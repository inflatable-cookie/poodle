<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/motion-policy-provider.css";
  import { restrictMotionPolicy, type MotionPolicy } from "@inflatable-cookie/poodle-core";
  import type { Snippet } from "svelte";
  import { get } from "svelte/store";

  import { getMotionPolicy, setMotionPolicy } from "./motion-policy";

  interface Props {
    policy?: MotionPolicy;
    children?: Snippet;
  }

  let { policy = "full", children }: Props = $props();

  const ancestor = getMotionPolicy();
  const motion = setMotionPolicy(get(ancestor));
  const effective = $derived(restrictMotionPolicy($ancestor, policy));

  $effect.pre(() => {
    motion.set(effective);
  });
</script>

<div class="poodle-motion-policy-provider" data-poodle-motion-policy={effective}>
  {@render children?.()}
</div>
