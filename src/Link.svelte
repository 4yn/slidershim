<script lang="ts">
  import { open } from "@tauri-apps/api/shell";

  export let href: string;
</script>

<a
  {href}
  on:click={async (e) => {
    e.preventDefault();
    await open(href);
  }}><slot /></a
>
