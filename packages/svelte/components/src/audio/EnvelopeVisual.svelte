<script lang="ts">
  import { envelopeSegmentValueAt, type EnvelopeVisualPoint, type EnvelopeVisualState } from "@inflatable-cookie/poodle-core";

  let { visualState }: { visualState: EnvelopeVisualState } = $props();

  const segments = $derived(visualState.points.slice(0, -1).map((point, index) => ({
    point,
    next: visualState.points[index + 1]!,
  })));

  function segmentPath(point: EnvelopeVisualPoint, next: EnvelopeVisualPoint): string {
    const samples = 24;
    const from = { id: point.id, x: point.xNorm, y: point.yNorm, curve: point.curve };
    const to = { id: next.id, x: next.xNorm, y: next.yNorm, curve: next.curve };
    return Array.from({ length: samples + 1 }, (_, index) => {
      const t = index / samples;
      const x = (point.xNorm + (next.xNorm - point.xNorm) * t) * 100;
      const y = (1 - envelopeSegmentValueAt(from, to, t)) * 100;
      return `${index === 0 ? "M" : "L"} ${x} ${y}`;
    }).join(" ");
  }
</script>

<svg
  class="poodle-envelope-editor-visual"
  viewBox="0 0 100 100"
  preserveAspectRatio="none"
  data-focus={visualState.focus}
  data-enabled={visualState.enabled}
  aria-hidden="true"
>
  <path class="poodle-envelope-editor-visual__grid" d="M 25 0 V 100 M 50 0 V 100 M 75 0 V 100 M 0 25 H 100 M 0 50 H 100 M 0 75 H 100" />
  {#each segments as segment (segment.point.id)}
    <path class="poodle-envelope-editor-visual__curve" d={segmentPath(segment.point, segment.next)} />
  {/each}
  {#each visualState.points as point (point.id)}
    <circle
      class="poodle-envelope-editor-visual__point"
      cx={point.xNorm * 100}
      cy={(1 - point.yNorm) * 100}
      r="2.8"
      data-selected={point.selected}
      data-dragging={point.dragging}
      data-hover={visualState.hoverPointId === point.id}
    />
  {/each}
</svg>
