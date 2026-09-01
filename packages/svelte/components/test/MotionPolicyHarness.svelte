<script lang="ts">
  import type { MotionPolicy } from "@inflatable-cookie/poodle-core";

  import MotionPolicyProvider from "../src/MotionPolicyProvider.svelte";
  import UiPresentationProvider from "../src/UiPresentationProvider.svelte";
  import MotionPolicyReader from "./MotionPolicyReader.svelte";

  let {
    policy = "full",
    nestedPolicy = undefined,
    wrapPresentation = false,
  }: {
    policy?: MotionPolicy;
    nestedPolicy?: MotionPolicy;
    wrapPresentation?: boolean;
  } = $props();
</script>

<MotionPolicyProvider {policy}>
  {#if wrapPresentation}
    <UiPresentationProvider sizeScale="xl" density="comfortable">
      {#if nestedPolicy}
        <MotionPolicyProvider policy={nestedPolicy}>
          <MotionPolicyReader />
        </MotionPolicyProvider>
      {:else}
        <MotionPolicyReader />
      {/if}
    </UiPresentationProvider>
  {:else if nestedPolicy}
    <MotionPolicyProvider policy={nestedPolicy}>
      <MotionPolicyReader />
    </MotionPolicyProvider>
  {:else}
    <MotionPolicyReader />
  {/if}
</MotionPolicyProvider>
