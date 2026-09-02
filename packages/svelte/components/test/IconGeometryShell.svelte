<script lang="ts">
  import {
    activateIconGeometry,
    currentIconGeometryFrame,
    sampleIconGeometry,
    setIconGeometryPolicy,
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

  $effect(() => {
    return () => {
      teardownIconGeometry(runtime);
    };
  });

  function frame() {
    setIconGeometryPolicy(runtime, policy);
    const decision = activateIconGeometry(runtime, { owner, pairId, target, initial });
    if (progress !== null) {
      sampleIconGeometry(runtime, decision.key, progress);
    }
    const current = currentIconGeometryFrame(runtime);
    if (!current) return null;
    return {
      contours: current.contours.map((contour) => ({
        closed: contour.closed,
        points: contour.points.map((point) => [point[0], point[1]] as const),
      })),
    };
  }

  function contourPath(closed: boolean, points: readonly (readonly [number, number])[]): string {
    if (points.length === 0) return "";
    const commands = points.map(([x, y], index) => {
      const prefix = index === 0 ? "M" : "L";
      return `${prefix}${x / 10_000} ${y / 10_000}`;
    });
    if (closed) commands.push("Z");
    return commands.join(" ");
  }
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
  {#each frame()?.contours ?? [] as contour, index (index)}
    <path d={contourPath(contour.closed, contour.points)} />
  {/each}
</svg>
