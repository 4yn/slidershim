<script lang="ts">
  import { onMount } from "svelte";
  import { emit, listen } from "@tauri-apps/api/event";
  import { getVersion } from "@tauri-apps/api/app";

  import Link from "./Link.svelte";
  import Preview from "./Preview.svelte";

  let deviceMode = "none";
  let outputMode = "none";
  let ledMode = "none";

  let disableAirStrings = false;
  let divaSerialPort = "COM1";
  let divaBrightness = 63;
  let keyboardSensitivity = 20;
  let outputPolling = "100";
  let outputWebsocketUrl = "http://localhost:3000";
  let ledFaster = false;
  let ledColorActive = "#ff00ff";
  let ledColorInactive = "#ffff00";
  let ledSensitivity = 20;
  let ledWebsocketUrl = "http://localhost:3001";
  let ledSerialPort = "COM5";

  let dirty = false;

  function markDirty() {
    dirty = true;
  }

  // let debugstr = "";
  let versionString = "";
  let ips: Array<string> = [];
  let polling = null;
  let tick = 0;
  let previewData = Array(131).fill(0);
  let timerData = "";

  function updatePolling(enabled) {
    if (!!polling) {
      clearInterval(polling);
      polling = null;
    }
    if (enabled) {
      polling = setInterval(async () => {
        tick += 1;
        await emit("queryState", "");
      }, 50);
    }
    // console.log(enabled, polling, tick);
  }

  // Receive events
  onMount(async () => {
    document.addEventListener("contextmenu", (event) => event.preventDefault());

    // console.log(emit, listen);
    await listen("showConfig", (event) => {
      const payload: any = JSON.parse(event.payload as any);
      deviceMode = payload.deviceMode || "none";
      outputMode = payload.outputMode || "none";
      ledMode = payload.ledMode || "none";

      disableAirStrings = payload.disableAirStrings || false;
      divaSerialPort = payload.divaSerialPort || "COM1";
      divaBrightness = payload.divaBrightness || 63;
      keyboardSensitivity = payload.keyboardSensitivity || 20;
      outputPolling = payload.outputPolling || "100";
      outputWebsocketUrl =
        payload.outputWebsocketUrl || "http://localhost:3000/";
      ledFaster = payload.ledFaster || false;
      ledColorActive = payload.ledColorActive || "#ff00ff";
      ledColorInactive = payload.ledColorInactive || "#ffff00";
      ledSensitivity = payload.ledSensitivity || 20;
      ledWebsocketUrl = payload.ledWebsocketUrl || "http://localhost:3001";
      ledSerialPort = payload.ledSerialPort || "COM5";
    });

    await listen("showState", (event) => {
      previewData = event.payload as any;
    });
    await listen("showTimerState", (event) => {
      timerData = event.payload as string;
    });

    await listen("listIps", (event) => {
      ips = (event.payload as Array<string>).filter(
        (x) => x.split(".").length == 4
      );
    });

    await emit("ready", "");

    updatePolling(true);
    await listen("ackShow", (event) => {
      console.log("ackShow");
      updatePolling(true);
    });
    await listen("ackHide", (event) => {
      console.log("ackHide");
      updatePolling(false);
    });

    versionString = ` ${await getVersion()}`;
  });

  // Emit events

  async function setConfig() {
    console.log("Updating config");
    console.log(disableAirStrings);
    await emit(
      "setConfig",
      JSON.stringify({
        deviceMode,
        outputMode,
        ledMode,
        disableAirStrings,
        divaSerialPort,
        divaBrightness,
        keyboardSensitivity,
        outputPolling,
        outputWebsocketUrl,
        ledFaster,
        ledColorActive,
        ledColorInactive,
        ledSensitivity,
        ledWebsocketUrl,
        ledSerialPort,
      })
    );
    dirty = false;
    console.log("Done");
  }

  async function hide() {
    await emit("hide", "");
  }

  async function quit() {
    await emit("quit", "");
  }

  async function logs() {
    await emit("openLogfile", "");
  }

  async function brokenithmQr() {
    await emit("openBrokenithmQr");
  }

  async function repo() {
    await emit("openRepo");
  }
</script>

<div class="titlebar">
  <div class="header-icon">
    <img src="/icon.png" alt="logo" />
  </div>
  <div class="header">
    &nbsp;slidershim{versionString}
  </div>
  <div class="header-space" />
  <div class="header-timer">
    {timerData}
  </div>
</div>
<div data-tauri-drag-region class="titlebar titlebar-front" />
<main class="main">
  <div class="row">
    <Preview data={previewData} />
  </div>
  <div class="row">
    <div class="label">Input Device</div>
    <div class="input">
      <select bind:value={deviceMode} on:change={markDirty}>
        <option value="none">None</option>
        <option value="tasoller-one">GAMO2 Tasoller, 1.0 HID Firmware</option>
        <option value="tasoller-two">GAMO2 Tasoller, 2.0 HID Firmware</option>
        <option value="yuancon">Yuancon Laverita, HID Firmware</option>
        <option value="diva">Slider over Serial</option>
        <option value="brokenithm">Brokenithm</option>
        <option value="brokenithm-led">Brokenithm + Led</option>
        <option value="brokenithm-nostalgia">Brokestalgia (28k)</option>
      </select>
    </div>
  </div>
  {#if deviceMode.slice(0, 8) === "tasoller" || deviceMode.slice(0, 7) === "yuancon" || (deviceMode.slice(0, 10) === "brokenithm" && deviceMode !== "brokenithm-nostalgia")}
    <div class="row">
      <div class="label" />
      <div class="input">
        <span>
          <input
            type="checkbox"
            id="disable-air"
            style="width: unset;"
            bind:checked={disableAirStrings}
            on:change={markDirty}
          />
          <label for="disable-air">Disable Air Strings</label>
        </span>
      </div>
    </div>
  {/if}
  {#if deviceMode.slice(0, 10) === "brokenithm"}
    <div class="row">
      <div class="label" />
      <div class="input">
        <div class="serverlist">
          Brokenithm server running, access at one of:
          <div class="iplist">
            {ips
              .map((x) => `http://${x}:1606/`)
              .join("\n")
              .trim()}
          </div>
        </div>
      </div>
    </div>
  {/if}
  {#if deviceMode === "diva"}
    <div class="row">
      <div class="label">Slider Serial Port</div>
      <div class="input">
        <select bind:value={divaSerialPort} on:change={markDirty}>
          <option value="COM1">COM1</option>
          <option value="COM2">COM2</option>
          <option value="COM3">COM3</option>
          <option value="COM4">COM4</option>
          <option value="COM5">COM5</option>
          <option value="COM6">COM6</option>
          <option value="COM7">COM7</option>
          <option value="COM8">COM8</option>
          <option value="COM9">COM9</option>
        </select>
      </div>
    </div>
    <div class="row">
      <div class="label">Brightness</div>
      <div class="input">
        <input
          type="number"
          min="1"
          max="255"
          step="1"
          bind:value={divaBrightness}
          on:change={markDirty}
        />
      </div>
    </div>
    <div class="row">
      <div class="label" />
      <div class="input">
        <input
          type="range"
          min="1"
          max="255"
          step="1"
          bind:value={divaBrightness}
          on:change={markDirty}
        />
      </div>
    </div>
  {/if}

  <div class="row">
    <div class="label">Output Mode</div>
    <div class="input">
      <select bind:value={outputMode} on:change={markDirty}>
        <option value="none">None</option>
        <option value="kb-32-tasoller">Keyboard 32-zone, Tasoller Layout</option
        >
        <option value="kb-32-yuancon">Keyboard 32-zone, Yuancon Layout</option>
        <option value="kb-16">Keyboard 16-zone, Linear</option>
        <option value="kb-8">Keyboard 8-zone, Linear</option>
        <option value="kb-6">Keyboard 6-zone, Linear</option>
        <option value="kb-4">Keyboard 4-zone, Linear</option>
        <option value="kb-voltex">Keyboard 10-zone, Voltex Layout</option>
        <option value="kb-neardayo">Keyboard 10-zone, Neardayo Layout</option>
        <option value="gamepad-voltex">XBOX 360 Gamepad, Voltex Layout</option>
        <option value="gamepad-neardayo"
          >XBOX 360 Gamepad, Neardayo Layout</option
        >
        <!-- <option value="websocket">Websocket</option> -->
      </select>
    </div>
  </div>
  {#if outputMode !== "none"}
    <div class="row">
      <div class="label">Output Polling</div>
      <div class="input">
        <select bind:value={outputPolling} on:change={markDirty}>
          <option value="60">60 Hz</option>
          <option value="100">100 Hz</option>
          <option value="250">250 Hz</option>
          <option value="500">500 Hz</option>
          <option value="1000">1000 Hz</option>
        </select>
      </div>
    </div>
  {/if}
  {#if outputMode.slice(0, 7) === "gamepad"}
    <div class="row">
      <div class="label" />
      <div class="input">
        Gamepad emulation requires <Link
          href="https://github.com/ViGEm/ViGEmBus/releases">ViGEMBus</Link
        >
      </div>
    </div>
  {/if}
  {#if (outputMode.slice(0, 2) === "kb" || outputMode.slice(0, 7) === "gamepad") && deviceMode.slice(0, 10) !== "brokenithm"}
    <div class="row">
      <div class="label" title="Larger means harder to trigger">
        Sensitivity
      </div>
      <div class="input">
        <input
          type="number"
          min="1"
          max="255"
          step="1"
          bind:value={keyboardSensitivity}
          on:change={markDirty}
        />
      </div>
    </div>
    <div class="row">
      <div class="label" />
      <div class="input">
        <input
          type="range"
          min="1"
          max="255"
          step="1"
          bind:value={keyboardSensitivity}
          on:change={markDirty}
        />
      </div>
    </div>
  {/if}
  {#if outputMode === "websocket"}
    <div class="row">
      <div class="label">Output URL</div>
      <div class="input">
        <input
          placeholder="URL"
          bind:value={outputWebsocketUrl}
          on:change={markDirty}
        />
      </div>
    </div>
  {/if}

  <div class="row">
    <div class="label">LED Mode</div>
    <div class="input">
      <select bind:value={ledMode} on:change={markDirty}>
        <option value="none">None</option>
        <option value="reactive-16">Reactive, 16-Zone</option>
        <option value="reactive-8">Reactive, 8-Zone</option>
        <option value="reactive-6">Reactive, 6-Zone</option>
        <option value="reactive-4">Reactive, 4-Zone</option>
        <option value="reactive-rainbow">Reactive, 16-Zone Rainbow</option>
        <option value="reactive-voltex">Reactive, Voltex Layout</option>
        <option value="attract">Rainbow Attract Mode</option>
        <!-- <option value="websocket">Websocket</option> -->
        <option value="serial">Serial</option>
      </select>
    </div>
  </div>
  {#if ledMode !== "none"}
    <div class="row">
      <div class="label" />
      <div class="input">
        <span>
          <input
            type="checkbox"
            id="led-faster"
            style="width: unset;"
            bind:checked={ledFaster}
            on:change={markDirty}
          />
          <label for="led-faster">Update LED data faster</label>
        </span>
      </div>
    </div>
  {/if}
  {#if ledMode.slice(0, 8) === "reactive" && ["16", "8", "6", "4"].includes(ledMode.slice(9))}
    <div class="row">
      <div class="label">Active Color</div>
      <div class="input">
        <input type="color" bind:value={ledColorActive} on:change={markDirty} />
      </div>
    </div>
    <div class="row">
      <div class="label">Base Color</div>
      <div class="input">
        <input
          type="color"
          bind:value={ledColorInactive}
          on:change={markDirty}
        />
      </div>
    </div>
  {/if}
  {#if ledMode.slice(0, 8) === "reactive" && deviceMode.slice(0, 10) !== "brokenithm"}
    <div class="row">
      <div class="label" title="Larger means harder to trigger">
        Sensitivity
      </div>
      <div class="input">
        <input
          type="number"
          min="1"
          max="255"
          step="1"
          bind:value={ledSensitivity}
          on:change={markDirty}
        />
      </div>
    </div>
    <div class="row">
      <div class="label" />
      <div class="input">
        <input
          type="range"
          min="1"
          max="255"
          step="1"
          bind:value={ledSensitivity}
          on:change={markDirty}
        />
      </div>
    </div>
  {/if}
  {#if ledMode === "websocket"}
    <div class="row">
      <div class="label">LED URL</div>
      <div class="input">
        <input
          placeholder="URL"
          bind:value={ledWebsocketUrl}
          on:change={markDirty}
        />
      </div>
    </div>
  {/if}
  {#if ledMode === "serial"}
    <div class="row">
      <div class="label" />
      <div class="input">
        Serial LED may require <Link
          href="https://sourceforge.net/projects/com0com/files/com0com/2.2.2.0/com0com-2.2.2.0-x64-fre-signed.zip/download"
          >com0com</Link
        >
      </div>
    </div>
    <div class="row">
      <div class="label">LED Serial Port</div>
      <div class="input">
        <select bind:value={ledSerialPort} on:change={markDirty}>
          <option value="COM1">COM1</option>
          <option value="COM2">COM2</option>
          <option value="COM3">COM3</option>
          <option value="COM4">COM4</option>
          <option value="COM5">COM5</option>
          <option value="COM6">COM6</option>
          <option value="COM7">COM7</option>
          <option value="COM8">COM8</option>
          <option value="COM9">COM9</option>
        </select>
      </div>
    </div>
  {/if}
  <div class="row">
    <button
      on:click={async () => await setConfig()}
      class={`${dirty && "primary"}`}>Apply</button
    >
    <button on:click={async () => await hide()}>Hide</button>
    <button on:click={async () => await quit()}>Quit</button>
    <button on:click={async () => await logs()}>Logs</button>
    {#if deviceMode.slice(0, 10) === "brokenithm"}
      <button on:click={async () => await brokenithmQr()}>Brokenithm QR</button>
    {/if}
    <button on:click={async () => await repo()}>About</button>
  </div>
</main>

<style>
</style>
