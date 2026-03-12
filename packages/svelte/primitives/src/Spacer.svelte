<script lang="ts">
  import { joinStyles } from "./internal";

  export let grow = 1;
  export let minSize: string | null = null;

  $: style = joinStyles([
    `flex: ${grow} 1 0%`,
    minSize ? `min-width: ${minSize}; min-height: ${minSize}` : null,
  ]);
</script>

<div class="spacer" aria-hidden="true" style={style}></div>

<style>
  .spacer {
    min-width: 0;
    min-height: 0;
  }
</style>
