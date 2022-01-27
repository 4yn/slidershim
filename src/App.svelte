<script lang="ts">
  import { onMount } from "svelte";
  import { emit, listen } from "@tauri-apps/api/event";

  import Preview from "./Preview.svelte"

  let deviceMode = "none";
  let outputMode = "none";
  let lightingMode = "none";

  let keyboardSensitivity = 20;
  let websocketOutputUrl = "http://localhost:3000";
  let websocketLedUrl = "http://localhost:3001";

  onMount(async () => {
    await listen("showConfig", (event) => {
      const payload: any = event.payload;
      deviceMode = payload.deviceMode;
      outputMode = payload.outputMode;
      lightingMode = payload.lightingMode;
      keyboardSensitivity = payload.keyboardSensitivity;
      websocketOutputUrl = payload.websocketOutputUrl;
      websocketLedUrl = payload.websocketLedUrl;
    });
  });

  async function setConfig() {
    console.log("Updating config");
    await emit("setConfig", JSON.stringify({
      deviceMode,
      outputMode,
      lightingMode,
      keyboardSensitivity,
      websocketOutputUrl,
      websocketLedUrl
    }));
    console.log("Done");
  }

  async function hide() {
    await emit("hide", "")
  }

  async function quit() {
    await emit("quit", "")
  }
</script>

<main class="main">
  <div class="row">
    <div class="header">
      slidershim
      <!-- slidershim by @4yn -->
    </div>
  </div>
  <div class="row">
    <Preview />
  </div>
  <div class="row">
    <div class="label">Input Device</div>
    <div class="input">
      <select bind:value={deviceMode}>
        <option value="none">None</option>
        <option value="tasoller">GAMO2 Tasoller, HID Firmware</option>
        <option value="yuancon">Yuancon Laverita, HID Firmware</option>
        <option value="brokenithm">Brokenithm</option>
        <option value="brokenithm-ground">Brokenithm, Ground only</option>
      </select>
    </div>
  </div>
  <div class="row">
    <div class="label">Output Mode</div>
    <div class="input">
      <select bind:value={outputMode}>
        <option value="none">None</option>
        <option value="kb-32-tasoller">Keyboard 32-zone, Tasoller Layout</option
        >
        <option value="kb-32-yuancon">Keyboard 32-zone, Yuancon Layout</option>
        <option value="kb-6-deemo">Keyboard 6-zone, Deemo Layout</option>
        <option value="websocket">Websocket</option>
      </select>
    </div>
  </div>
  {#if outputMode.slice(0, 2) === "kb"}
    <div class="row">
      <div class="label">Sensitivity</div>
      <div class="input">
        <input
          type="number"
          min="1"
          max="255"
          step="1"
          bind:value={keyboardSensitivity}
        />
      </div>
    </div>
    <div class="row">
      <div class="label"></div>
      <div class="input">
        <input
          type="range"
          min="1"
          max="255"
          step="1"
          bind:value={keyboardSensitivity}
        />
      </div>
    </div>
  {/if}
  {#if outputMode === "websocket"}
    <div class="row">
      <div class="label">Output URL</div>
      <div class="input">
        <input placeholder="URL" bind:value={websocketOutputUrl} />
      </div>
    </div>
  {/if}
  <div class="row">
    <div class="label">LED Mode</div>
    <div class="input">
      <select bind:value={lightingMode}>
        <option value="none">None</option>
        <option value="reactive-4">Reactive, 4-Zone</option>
        <option value="reactive-8">Reactive, 8-Zone</option>
        <option value="reactive-16">Reactive, 16-Zone</option>
        <option value="attract">Rainbow Attract Mode</option>
        <option value="test">LED Test</option>
        <option value="websocket">Websocket</option>
      </select>
    </div>
  </div>
  {#if lightingMode === "websocket"}
    <div class="row">
      <div class="label">LED URL</div>
      <div class="input">
        <input placeholder="URL" bind:value={websocketLedUrl} />
      </div>
    </div>
  {/if}
  <div class="row">
    <button on:click={async () => await setConfig()}>Apply</button>
    <button on:click={async () => await hide()}>Hide</button>
    <button on:click={async () => await quit()}>Quit</button>
  </div>
</main>

<style>
</style>
