<script lang="ts">
  import "@inflatable-cookie/poodle-styles/region.css";
  interface Props {
    label?: string;
    color?: string | null;
    minHeight?: string;
  }

  let {
    label = "",
    color = null,
    minHeight = "4rem",
  }: Props = $props();
</script>

<div
  class="poodle-region"
  style:--region-color={color}
  style:min-height={minHeight}
  role="presentation"
>
  {#if label}
    <span class="poodle-region__label">{label}</span>
  {/if}
</div>

