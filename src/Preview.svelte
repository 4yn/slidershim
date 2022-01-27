<script lang="ts">
  let topDatas = Array(16).fill(0);
  let botDatas = Array(16).fill(0);
  let ledDatas = Array(31).fill(0).map((_, idx) => ({
    color: !!(idx % 2) ? "f0f" : "ff0",
    spec: idx % 2
  }))
</script>

<main class="preview">
  <div class="air"></div>
  <div class="ground">
    <div class="ground-led">
      <div class="ground-row">
        <div class="ground-led-2"></div>
        {#each ledDatas as {color, spec}, idx (idx)}
          <div class={`ground-led-${spec}`} style={`background-color: #${color}`}></div>
        {/each}
        <div class="ground-led-2"></div>
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
