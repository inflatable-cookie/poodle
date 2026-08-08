<script lang="ts">
  import "@inflatable-cookie/poodle-styles/spacer.css";
  import { joinStyles } from "./internal";

  interface Props {
    grow?: number;
    minSize?: string | null;
  }

  let {
    grow = 1,
    minSize = null,
  }: Props = $props();

  const style = $derived(joinStyles([
    `flex: ${grow} 1 0%`,
    minSize ? `min-width: ${minSize}; min-height: ${minSize}` : null,
  ]));
</script>

<div class="poodle-spacer" aria-hidden="true" style={style}></div>

