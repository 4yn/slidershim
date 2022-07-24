<script lang="ts">
  export let data: Array<number>;

  let topDatas = Array(16).fill(0);
  let botDatas = Array(16).fill(0);
  let airDatas = Array(6).fill(0);
  let extraDatas = Array(3).fill(0);

  let ledDatas = Array(16).fill("#ff0");
  let ledDividerDatas = Array(15).fill("#ff0");
  let airLedLeftDatas = Array(3).fill(0);
  let airLedRightDatas = Array(3).fill(0);

  $: {
    if (data.length === 152) {
      // console.log(data);
      for (let i = 0; i < 16; i++) {
        topDatas[i] = data[i * 2 + 1];
        botDatas[i] = data[i * 2];
      }
      for (let i = 0; i < 6; i++) {
        airDatas[i] = data[32 + i];
      }
      for (let i = 0; i < 3; i++) {
        extraDatas[i] = data[38 + i];
      }

      for (let i = 0; i < 31; i++) {
        let rgbstr = `rgb(${data[41 + i * 3]}, ${data[42 + i * 3]}, ${
          data[43 + i * 3]
        })`;
        if (i % 2 == 0) {
          ledDatas[i / 2] = rgbstr;
        } else {
          ledDividerDatas[(i - 1) / 2] = rgbstr;
        }
      }
      for (let i = 0; i < 3; i++) {
        let rgbstr = `rgb(${data[134 + i * 3]}, ${data[135 + i * 3]}, ${
          data[136 + i * 3]
        })`;
        airLedLeftDatas[i] = rgbstr;
      }
      for (let i = 0; i < 3; i++) {
        let rgbstr = `rgb(${data[143 + i * 3]}, ${data[144 + i * 3]}, ${
          data[145 + i * 3]
        })`;
        airLedRightDatas[i] = rgbstr;
      }
    }
  }
</script>

<main class="preview">
  <div class="air">
    <div class="air-led">
      <div class="air-led-left">
        {#each airLedLeftDatas as airLedData, idx (idx)}
          <div class="air-led-data" style={`background-color: ${airLedData}`} />
        {/each}
      </div>
      <div class="air-led-space" />
      <div class="air-led-right">
        {#each airLedRightDatas as airLedData, idx (idx)}
          <div class="air-led-data" style={`background-color: ${airLedData}`} />
        {/each}
      </div>
    </div>
    <div class="air-btn">
      {#each airDatas as airData, idx (idx)}
        <div class={`air-data air-data-${airData}`} />
      {/each}
    </div>
  </div>
  <div class="ground">
    <div class="ground-led">
      <div class="ground-row">
        {#each ledDatas as ledData, idx (idx)}
          <div class={`ground-led-0`} style={`background-color: ${ledData}`} />
        {/each}
      </div>
    </div>
    <div class="ground-led">
      <div class="ground-row ground-row-divider">
        <div class="ground-led-2" />
        {#each ledDividerDatas as ledDividerData, idx (idx)}
          <div
            class="ground-led-1"
            style={`background-color: ${ledDividerData}`}
          />
        {/each}
        <div class="ground-led-2" />
      </div>
    </div>
    <div class="ground-btn">
      <div class="ground-row">
        {#each topDatas as topData, idx (idx)}
          <div class="ground-data">{topData}</div>
        {/each}
      </div>
      <div class="ground-row">
        {#each botDatas as botData, idx (idx)}
          <div class="ground-data">{botData}</div>
        {/each}
      </div>
    </div>
  </div>
  <div class="extra">
    {#each extraDatas as extraData, idx (idx)}
      <div class={`extra-data extra-data-${extraData}`} />
    {/each}
  </div>
</main>
