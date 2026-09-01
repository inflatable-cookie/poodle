<script lang="ts">
  import type { MotionPolicy } from "@inflatable-cookie/poodle-core";

  import IconButton from "../src/IconButton.svelte";
  import MotionPolicyProvider from "../src/MotionPolicyProvider.svelte";
  import Skeleton from "../src/Skeleton.svelte";
  import Spinner from "../src/Spinner.svelte";
  import ToastStack from "../src/ToastStack.svelte";
  import type { ToastItem } from "../src/types";

  let {
    policy = "full",
    kind = "icon-button",
    animated = true,
    items = [],
  }: {
    policy?: MotionPolicy;
    kind?: "icon-button" | "spinner" | "skeleton" | "toast";
    animated?: boolean;
    items?: ToastItem[];
  } = $props();
</script>

<MotionPolicyProvider {policy}>
  {#if kind === "icon-button"}
    <IconButton icon="star" ariaLabel="Star" />
  {:else if kind === "spinner"}
    <Spinner />
  {:else if kind === "toast"}
    <ToastStack {items} />
  {:else}
    <Skeleton {animated} />
  {/if}
</MotionPolicyProvider>
