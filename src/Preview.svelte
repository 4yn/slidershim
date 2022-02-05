<script lang="ts">
  export let data: Array<number>;

  let topDatas = Array(16).fill(0);
  let botDatas = Array(16).fill(0);
  let ledDatas = Array(31)
    .fill(0)
    .map((_, idx) => ({
      color: !!(idx % 2) ? "#f0f" : "#ff0",
      spec: idx % 2,
    }));

  $: {
    if (data.length === 131) {
      // console.log(data);
      for (let i = 0; i < 16; i++) {
        topDatas[i] = data[i * 2 + 1];
        botDatas[i] = data[i * 2];
      }
      for (let i = 0; i < 31; i++) {
        ledDatas[i].color = `rgb(${data[38 + i * 3]}, ${data[39 + i * 3]}, ${
          data[40 + i * 3]
        })`;
      }
    }
  }
</script>

<main class="preview">
  <div class="air" />
  <div class="ground">
    <div class="ground-led">
      <div class="ground-row">
        <div class="ground-led-2" />
        {#each ledDatas as { color, spec }, idx (idx)}
          <div
            class={`ground-led-${spec}`}
            style={`background-color: ${color}`}
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
</main>
