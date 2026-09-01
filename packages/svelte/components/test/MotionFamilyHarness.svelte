<script lang="ts">
  import type { MotionPolicy } from "@inflatable-cookie/poodle-core";

  import IconButton from "../src/IconButton.svelte";
  import MotionPolicyProvider from "../src/MotionPolicyProvider.svelte";
  import Skeleton from "../src/Skeleton.svelte";
  import Spinner from "../src/Spinner.svelte";

  let {
    policy = "full",
    kind = "icon-button",
    animated = true,
  }: {
    policy?: MotionPolicy;
    kind?: "icon-button" | "spinner" | "skeleton";
    animated?: boolean;
  } = $props();
</script>

<MotionPolicyProvider {policy}>
  {#if kind === "icon-button"}
    <IconButton icon="star" ariaLabel="Star" />
  {:else if kind === "spinner"}
    <Spinner />
  {:else}
    <Skeleton {animated} />
  {/if}
</MotionPolicyProvider>
