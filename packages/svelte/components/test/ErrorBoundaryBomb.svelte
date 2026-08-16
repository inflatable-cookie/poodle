<script module lang="ts">
  // Throws once per module load: the boundary re-creates the failed subtree
  // from scratch on reset, so a per-instance counter would throw again.
  let hasThrown = false;

  export function resetBomb(): void {
    hasThrown = false;
  }
</script>

<script lang="ts">
  if (!hasThrown) {
    hasThrown = true;
    throw new Error("render exploded");
  }
</script>

<span class="harness-ok">ok</span>
