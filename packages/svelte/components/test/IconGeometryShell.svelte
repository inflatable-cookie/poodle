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

  function paintFromProps(): { closed: boolean; d: string }[] {
    setIconGeometryPolicy(runtime, policy);
    const decision = activateIconGeometry(runtime, { owner, pairId, target, initial });
    if (progress !== null) {
      sampleIconGeometry(runtime, decision.key, progress);
    }
    const current = currentIconGeometryFrame(runtime);
    if (!current) return [];
    return current.contours.map((contour) => ({
      closed: contour.closed,
      d: contourPath(contour.closed, contour.points, contour.count),
    }));
  }

  let paths = $state(paintFromProps());

  function snapshot() {
    paths = paintFromProps();
  }

  function contourPath(
    closed: boolean,
    points: readonly (readonly [number, number])[],
    count: number,
  ): string {
    if (count === 0) return "";
    const commands: string[] = [];
    for (let index = 0; index < count; index += 1) {
      const point = points[index]!;
      commands.push(`${index === 0 ? "M" : "L"}${point[0] / 10_000} ${point[1] / 10_000}`);
    }
    if (closed) commands.push("Z");
    return commands.join(" ");
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
      const current = currentIconGeometryFrame(runtime);
      paths = current
        ? current.contours.map((contour) => ({
            closed: contour.closed,
            d: contourPath(contour.closed, contour.points, contour.count),
          }))
        : [];
    });
  });

  $effect(() => {
    return () => {
      teardownIconGeometry(runtime);
    };
  });
</script>

<svg
  class="poodle-icon-geometry"
  data-poodle-icon-geometry=""
  data-size="md"
  xmlns="http://www.w3.org/2000/svg"
  width="24"
  height="24"
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
  role="presentation"
  aria-hidden="true"
>
  {#each paths as contour, index (index)}
    <path d={contour.d} />
  {/each}
</svg>
