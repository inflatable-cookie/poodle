<script lang="ts">
  import type { KeyboardVisualState } from "@inflatable-cookie/poodle-core";
  let { visualState }: { visualState: KeyboardVisualState } = $props();
</script>

<span class="poodle-keyboard-visual" aria-hidden="true" data-enabled={visualState.enabled}>
  {#each visualState.keys as key (key.note)}
    <span
      class="poodle-keyboard-visual__key"
      data-kind={key.kind}
      data-held={key.held}
      data-external={key.externallyHeld}
      data-focused={key.focused}
      style={visualState.orientation === "horizontal"
        ? `left:${key.startNorm * 100}%;width:${key.lengthNorm * 100}%;height:${key.breadthNorm * 100}%`
        : `top:${key.startNorm * 100}%;height:${key.lengthNorm * 100}%;width:${key.breadthNorm * 100}%`}
    ></span>
  {/each}
</span>
