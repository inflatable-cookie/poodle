<script lang="ts">
  import {
    activateIconGeometry,
    currentIconGeometryFrame,
    sampleIconGeometry,
    setIconGeometryPolicy,
    startIconGeometryFrameLoop,
    teardownIconGeometry,
    createIconGeometryRuntime,
    type GeometryEndpoint,
  } from "../../../core/src/icons/geometry-runtime";
  import { realize } from "../../../core/src/icons/geometry-realize";
  import type { MotionPolicy } from "@inflatable-cookie/poodle-core";

  let {
    owner = "icon-geometry-shell",
    pairId,
    target = "from",
    policy = "full",
    progress = null,
    initial = false,
  }: {
    owner?: string;
    pairId: string;
    target?: GeometryEndpoint;
    policy?: MotionPolicy;
    progress?: number | null;
    initial?: boolean;
  } = $props();

  const runtime = createIconGeometryRuntime("full");
  const svgAttributes = realize({ name: "frame", frame: null }).svgAttributes;

  function paintFromProps(): { d: string }[] {
    setIconGeometryPolicy(runtime, policy);
    const decision = activateIconGeometry(runtime, { owner, pairId, target, initial });
    if (progress !== null) {
      sampleIconGeometry(runtime, decision.key, progress);
    }
    return realize({
      name: "frame",
      frame: currentIconGeometryFrame(runtime),
    }).paths;
  }

  let paths = $state(paintFromProps());

  function snapshot() {
    paths = paintFromProps();
  }

  $effect(() => {
    setIconGeometryPolicy(runtime, policy);
    const decision = activateIconGeometry(runtime, { owner, pairId, target, initial });
    if (progress !== null) {
      sampleIconGeometry(runtime, decision.key, progress);
    }
    snapshot();
    if (progress !== null || !decision.liveClock || typeof requestAnimationFrame !== "function") {
      return;
    }
    return startIconGeometryFrameLoop(runtime, decision.key, () => {
      paths = realize({
        name: "frame",
        frame: currentIconGeometryFrame(runtime),
      }).paths;
    });
  });

  $effect(() => {
    return () => {
      teardownIconGeometry(runtime);
    };
  });
</script>

<svg {...svgAttributes}>
  {#each paths as contour, index (index)}
    <path d={contour.d} />
  {/each}
</svg>
