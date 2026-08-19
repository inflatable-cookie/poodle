<script lang="ts">
  import { Pill, type PillAppearance, type PillTone } from "@inflatable-cookie/poodle-svelte";

  let {
    appearance = undefined,
    tone = "neutral",
    label = "Packed",
  }: { appearance?: PillAppearance; tone?: PillTone; label?: string } = $props();
</script>

<Pill {appearance} {tone} dot>{label}</Pill>
