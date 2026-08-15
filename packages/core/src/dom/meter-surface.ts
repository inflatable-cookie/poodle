import { normalizeMeterDb } from "../audio/meter";
import type { MeterBus } from "../audio/meter-bus";

export interface MeterSurfacePalette {
  trackFill: string;
  trackBorder: string;
  segmentOff: string;
  segmentOn: string;
  segmentWarning: string;
  segmentClip: string;
  barLow: string;
  barMid: string;
  barHigh: string;
  peakFill: string;
  clipOff: string;
  clipOn: string;
  disabledOpacity: number;
}

/**
 * One preallocated flat draw pass. The controller assembles it every frame
 * from cached geometry and the live bus view; a painter reads it and draws.
 * All rectangles are CSS-pixel content coordinates; `originX`/`originY` map
 * them into canvas coordinates for the current scroll offset.
 */
export interface MeterDrawPass {
  count: number;
  slot: Int32Array;
  trackX: Float64Array;
  trackY: Float64Array;
  trackW: Float64Array;
  trackH: Float64Array;
  clipX: Float64Array;
  clipY: Float64Array;
  clipW: Float64Array;
  clipH: Float64Array;
  /** 0 = bar, 1 = segments */
  style: Uint8Array;
  /** 0 = vertical, 1 = horizontal */
  orientation: Uint8Array;
  segments: Int32Array;
  enabled: Uint8Array;
  clip: Uint8Array;
  value: Float64Array;
  /** NaN while the channel has no peak hold. */
  peak: Float64Array;
  borderPx: Float64Array;
  radiusPx: Float64Array;
  clipRadiusPx: Float64Array;
  segmentGapPx: Float64Array;
  peakThicknessPx: Float64Array;
  dpr: number;
  viewportWidth: number;
  viewportHeight: number;
  originX: number;
  originY: number;
}

export interface MeterSurfacePainter {
  setup(canvas: HTMLCanvasElement): void;
  resize(widthPx: number, heightPx: number, dpr: number): void;
  setPalette(palette: MeterSurfacePalette): void;
  paint(pass: MeterDrawPass): void;
  destroy(): void;
}

export interface MeterSurfaceElements {
  /** Positioning root that hosts the overlay canvas. */
  root: HTMLElement;
  /** The one scroll container this surface owns. */
  viewport: HTMLElement;
  /** Scrolled content region the placeholders live in. */
  content: HTMLElement;
  canvas: HTMLCanvasElement;
}

export interface MeterPlaceholderSpec {
  slot: number;
  rightSlot: number | null;
  style: "bar" | "segments";
  orientation: "vertical" | "horizontal";
  segments: number;
}

export interface MeterPlaceholderHandle {
  update(spec: MeterPlaceholderSpec): void;
  detach(): void;
}

export interface MeterSurfaceControllerOptions {
  bus: MeterBus;
  painter?: MeterSurfacePainter;
  /** Shared aria sampling cadence; default 500 ms (2 Hz). */
  ariaIntervalMs?: number;
}

export interface MeterSurfaceController {
  attachMeter(
    element: HTMLElement,
    spec: MeterPlaceholderSpec,
    onAriaSample?: ((timeMs: number) => void) | null,
  ): MeterPlaceholderHandle;
  invalidateLayout(): void;
  refreshPalette(): void;
  /** Test/diagnostic access to the assembled pass of the last frame. */
  readonly drawPass: MeterDrawPass;
  destroy(): void;
}

export interface MeterSurfaceRegistryOptions {
  painter?: MeterSurfacePainter;
  ariaIntervalMs?: number;
}

/**
 * Mount-order-safe seam between a framework MeterSurface wrapper and the DOM
 * controller. Descendant placeholders may register before the wrapper's
 * elements exist (child-first mount order in both Svelte and React); the
 * registry queues them and attaches everything when `connect` runs.
 */
export interface MeterSurfaceRegistry {
  readonly bus: MeterBus;
  registerMeter(
    element: HTMLElement,
    spec: MeterPlaceholderSpec,
    onAriaSample?: ((timeMs: number) => void) | null,
  ): MeterPlaceholderHandle;
  connect(elements: MeterSurfaceElements, options?: MeterSurfaceRegistryOptions): void;
  /** Tear down the live controller but keep registrations for a reconnect. */
  disconnect(): void;
  invalidateLayout(): void;
  refreshPalette(): void;
  destroy(): void;
}

interface PendingRegistration {
  element: HTMLElement;
  spec: MeterPlaceholderSpec;
  onAriaSample: ((timeMs: number) => void) | null;
  live: MeterPlaceholderHandle | null;
}

export function createMeterSurfaceRegistry(bus: MeterBus): MeterSurfaceRegistry {
  const registrations: PendingRegistration[] = [];
  let controller: MeterSurfaceController | null = null;
  let destroyed = false;

  return {
    bus,
    registerMeter(element, spec, onAriaSample = null) {
      if (destroyed) throw new Error("MeterSurface: registry has been destroyed");
      const registration: PendingRegistration = { element, spec, onAriaSample, live: null };
      registrations.push(registration);
      if (controller !== null) {
        registration.live = controller.attachMeter(element, spec, onAriaSample);
      }
      return {
        update(next) {
          registration.spec = next;
          registration.live?.update(next);
        },
        detach() {
          const index = registrations.indexOf(registration);
          if (index >= 0) registrations.splice(index, 1);
          registration.live?.detach();
          registration.live = null;
        },
      };
    },
    connect(elements, options = {}) {
      if (destroyed) throw new Error("MeterSurface: registry has been destroyed");
      if (controller !== null) throw new Error("MeterSurface: registry is already connected");
      controller = createMeterSurfaceController(elements, {
        bus,
        painter: options.painter,
        ariaIntervalMs: options.ariaIntervalMs,
      });
      for (const registration of registrations) {
        registration.live = controller.attachMeter(registration.element, registration.spec, registration.onAriaSample);
      }
    },
    disconnect() {
      if (controller === null) return;
      controller.destroy();
      controller = null;
      for (const registration of registrations) registration.live = null;
    },
    invalidateLayout() {
      controller?.invalidateLayout();
    },
    refreshPalette() {
      controller?.refreshPalette();
    },
    destroy() {
      if (destroyed) return;
      destroyed = true;
      controller?.destroy();
      controller = null;
      registrations.length = 0;
    },
  };
}

interface MeterRecord {
  element: HTMLElement;
  slot: number;
  rightSlot: number;
  style: number;
  orientation: number;
  segments: number;
  onAriaSample: ((timeMs: number) => void) | null;
  /** [trackX, trackY, trackW, trackH, clipX, clipY, clipW, clipH] x 2 channels. */
  geometry: Float64Array;
  /** [x, y, w, h] culling bounds. */
  bounds: Float64Array;
  borderPx: number;
  radiusPx: number;
  clipRadiusPx: number;
  segmentGapPx: number;
  measured: boolean;
}

const CULL_MARGIN_PX = 8;

const PALETTE_PROBES: ReadonlyArray<[key: keyof MeterSurfacePalette, background: string]> = [
  ["trackFill", "var(--poodle-recipe-audio-meter-track-fill, var(--poodle-color-background-surface))"],
  ["trackBorder", "var(--poodle-recipe-audio-meter-track-border, var(--poodle-color-border-default))"],
  ["segmentOff", "var(--poodle-recipe-audio-meter-segment-off-fill, color-mix(in srgb, var(--poodle-color-background-surface) 82%, black))"],
  ["segmentOn", "var(--poodle-recipe-audio-meter-segment-on-fill, var(--poodle-color-status-success))"],
  ["segmentWarning", "var(--poodle-recipe-audio-meter-segment-warning-fill, var(--poodle-color-status-warning))"],
  ["segmentClip", "var(--poodle-recipe-audio-meter-segment-clip-fill, var(--poodle-color-status-danger))"],
  ["barLow", "var(--poodle-color-status-success)"],
  ["barMid", "var(--poodle-color-status-warning)"],
  ["barHigh", "var(--poodle-color-status-danger)"],
  ["peakFill", "var(--poodle-recipe-audio-meter-peak-fill, var(--poodle-color-text-primary))"],
  ["clipOff", "var(--poodle-recipe-audio-meter-clip-fill, var(--poodle-color-background-surface))"],
  ["clipOn", "var(--poodle-recipe-audio-meter-clip-fill, var(--poodle-color-status-danger))"],
];

function allocateDrawPass(capacity: number): MeterDrawPass {
  return {
    count: 0,
    slot: new Int32Array(capacity),
    trackX: new Float64Array(capacity),
    trackY: new Float64Array(capacity),
    trackW: new Float64Array(capacity),
    trackH: new Float64Array(capacity),
    clipX: new Float64Array(capacity),
    clipY: new Float64Array(capacity),
    clipW: new Float64Array(capacity),
    clipH: new Float64Array(capacity),
    style: new Uint8Array(capacity),
    orientation: new Uint8Array(capacity),
    segments: new Int32Array(capacity),
    enabled: new Uint8Array(capacity),
    clip: new Uint8Array(capacity),
    value: new Float64Array(capacity),
    peak: new Float64Array(capacity),
    borderPx: new Float64Array(capacity),
    radiusPx: new Float64Array(capacity),
    clipRadiusPx: new Float64Array(capacity),
    segmentGapPx: new Float64Array(capacity),
    peakThicknessPx: new Float64Array(capacity),
    dpr: 1,
    viewportWidth: 0,
    viewportHeight: 0,
    originX: 0,
    originY: 0,
  };
}

function growDrawPass(pass: MeterDrawPass, capacity: number): void {
  const next = allocateDrawPass(capacity);
  pass.slot = next.slot;
  pass.trackX = next.trackX;
  pass.trackY = next.trackY;
  pass.trackW = next.trackW;
  pass.trackH = next.trackH;
  pass.clipX = next.clipX;
  pass.clipY = next.clipY;
  pass.clipW = next.clipW;
  pass.clipH = next.clipH;
  pass.style = next.style;
  pass.orientation = next.orientation;
  pass.segments = next.segments;
  pass.enabled = next.enabled;
  pass.clip = next.clip;
  pass.value = next.value;
  pass.peak = next.peak;
  pass.borderPx = next.borderPx;
  pass.radiusPx = next.radiusPx;
  pass.clipRadiusPx = next.clipRadiusPx;
  pass.segmentGapPx = next.segmentGapPx;
  pass.peakThicknessPx = next.peakThicknessPx;
}

function drawPassCapacity(pass: MeterDrawPass): number {
  return pass.slot.length;
}

/**
 * Shared DOM controller for one batched meter surface: it owns the overlay
 * canvas lifecycle, one ResizeObserver, scroll projection from cached content
 * coordinates, DPR backing-store sizing, theme/palette probes, viewport
 * culling, the bus frame subscription, and cleanup. Framework wrappers stay
 * thin: they mount the elements and register placeholder meters.
 */
export function createMeterSurfaceController(
  elements: MeterSurfaceElements,
  options: MeterSurfaceControllerOptions,
): MeterSurfaceController {
  const { root, viewport, content, canvas } = elements;
  const bus = options.bus;
  const ariaIntervalMs = options.ariaIntervalMs ?? 500;
  const painter = options.painter ?? null;
  const records: MeterRecord[] = [];
  const pass = allocateDrawPass(16);

  let destroyed = false;
  let needsMeasure = true;
  let needsPalette = true;
  let dpr = 0;
  let canvasWidth = -1;
  let canvasHeight = -1;
  let contentOffsetX = 0;
  let contentOffsetY = 0;
  let lastAriaMs: number | null = null;
  let palette: MeterSurfacePalette | null = null;

  painter?.setup(canvas);

  let probeContainer: HTMLElement | null = null;
  const ownerDocument = root.ownerDocument;

  const ensureProbes = (): HTMLElement => {
    if (probeContainer !== null) return probeContainer;
    const container = ownerDocument.createElement("div");
    container.setAttribute("aria-hidden", "true");
    container.setAttribute("data-part", "palette-probe");
    container.style.cssText = "position:absolute;visibility:hidden;width:0;height:0;overflow:hidden;pointer-events:none;";
    for (const [key, background] of PALETTE_PROBES) {
      const probe = ownerDocument.createElement("span");
      probe.setAttribute("data-probe", key);
      probe.style.backgroundColor = background;
      container.appendChild(probe);
    }
    const opacityProbe = ownerDocument.createElement("span");
    opacityProbe.setAttribute("data-probe", "disabledOpacity");
    opacityProbe.style.opacity = "var(--poodle-recipe-audio-meter-disabled-opacity, var(--poodle-state-opacity-disabled, 0.5))";
    container.appendChild(opacityProbe);
    root.appendChild(container);
    probeContainer = container;
    return container;
  };

  const probePalette = (): void => {
    needsPalette = false;
    const view = ownerDocument.defaultView;
    if (view === null) return;
    const container = ensureProbes();
    const next: Partial<MeterSurfacePalette> = {};
    let child = container.firstElementChild;
    let index = 0;
    while (child !== null) {
      const computed = view.getComputedStyle(child);
      if (index < PALETTE_PROBES.length) {
        next[PALETTE_PROBES[index]![0]] = computed.backgroundColor as never;
      } else {
        const opacity = Number.parseFloat(computed.opacity);
        next.disabledOpacity = Number.isFinite(opacity) ? opacity : 0.5;
      }
      child = child.nextElementSibling;
      index += 1;
    }
    palette = next as MeterSurfacePalette;
    painter?.setPalette(palette);
  };

  // One transient standalone-anatomy skeleton per channel gives the painter
  // exact track/clip geometry from the real stylesheet instead of a parallel
  // layout model. Injection happens only on cold measurement passes.
  const measure = (): void => {
    needsMeasure = false;
    const view = ownerDocument.defaultView;
    if (view === null) return;
    const skeletons: HTMLElement[][] = [];
    for (const record of records) {
      const channels = record.rightSlot >= 0 ? 2 : 1;
      const spans: HTMLElement[] = [];
      for (let channel = 0; channel < channels; channel += 1) {
        const visual = ownerDocument.createElement("span");
        visual.className = "poodle-audio-meter-visual";
        visual.setAttribute("aria-hidden", "true");
        const track = ownerDocument.createElement("span");
        track.className = "poodle-audio-meter-visual__track";
        const segmentA = ownerDocument.createElement("span");
        segmentA.className = "poodle-audio-meter-visual__segment";
        const segmentB = ownerDocument.createElement("span");
        segmentB.className = "poodle-audio-meter-visual__segment";
        track.appendChild(segmentA);
        track.appendChild(segmentB);
        const clip = ownerDocument.createElement("span");
        clip.className = "poodle-audio-meter-visual__clip";
        visual.appendChild(track);
        visual.appendChild(clip);
        record.element.appendChild(visual);
        spans.push(visual);
      }
      skeletons.push(spans);
    }

    const canvasRect = canvas.getBoundingClientRect();
    const contentRect = content.getBoundingClientRect();
    contentOffsetX = contentRect.left - canvasRect.left + viewport.scrollLeft;
    contentOffsetY = contentRect.top - canvasRect.top + viewport.scrollTop;

    for (let index = 0; index < records.length; index += 1) {
      const record = records[index]!;
      const spans = skeletons[index]!;
      let minX = Number.POSITIVE_INFINITY;
      let minY = Number.POSITIVE_INFINITY;
      let maxX = Number.NEGATIVE_INFINITY;
      let maxY = Number.NEGATIVE_INFINITY;
      for (let channel = 0; channel < spans.length; channel += 1) {
        const visual = spans[channel]!;
        const track = visual.firstElementChild as HTMLElement;
        const clip = track.nextElementSibling as HTMLElement;
        const trackRect = track.getBoundingClientRect();
        const clipRect = clip.getBoundingClientRect();
        const base = channel * 8;
        record.geometry[base] = trackRect.left - contentRect.left;
        record.geometry[base + 1] = trackRect.top - contentRect.top;
        record.geometry[base + 2] = trackRect.width;
        record.geometry[base + 3] = trackRect.height;
        record.geometry[base + 4] = clipRect.left - contentRect.left;
        record.geometry[base + 5] = clipRect.top - contentRect.top;
        record.geometry[base + 6] = clipRect.width;
        record.geometry[base + 7] = clipRect.height;
        minX = Math.min(minX, trackRect.left, clipRect.left);
        minY = Math.min(minY, trackRect.top, clipRect.top);
        maxX = Math.max(maxX, trackRect.right, clipRect.right);
        maxY = Math.max(maxY, trackRect.bottom, clipRect.bottom);
        if (channel === 0) {
          const trackStyle = view.getComputedStyle(track);
          record.borderPx = Number.parseFloat(trackStyle.borderTopWidth) || 0;
          record.radiusPx = Number.parseFloat(trackStyle.borderTopLeftRadius) || 0;
          record.clipRadiusPx = Number.parseFloat(view.getComputedStyle(clip).borderTopLeftRadius) || 0;
          const segmentA = track.firstElementChild!.getBoundingClientRect();
          const segmentB = track.lastElementChild!.getBoundingClientRect();
          record.segmentGapPx = record.orientation === 1
            ? Math.max(segmentB.left - segmentA.right, 0)
            : Math.max(segmentA.top - segmentB.bottom, 0);
        }
      }
      record.bounds[0] = minX - contentRect.left;
      record.bounds[1] = minY - contentRect.top;
      record.bounds[2] = maxX - minX;
      record.bounds[3] = maxY - minY;
      record.measured = true;
    }

    for (const spans of skeletons) {
      for (const span of spans) span.remove();
    }
  };

  const resizeCanvas = (nextDpr: number): void => {
    const width = viewport.clientWidth;
    const height = viewport.clientHeight;
    if (width === canvasWidth && height === canvasHeight && nextDpr === dpr) return;
    canvasWidth = width;
    canvasHeight = height;
    dpr = nextDpr;
    painter?.resize(width, height, nextDpr);
  };

  const assemble = (timeMs: number): void => {
    const busView = bus.view;
    const scrollX = viewport.scrollLeft;
    const scrollY = viewport.scrollTop;
    pass.dpr = dpr;
    pass.viewportWidth = canvasWidth;
    pass.viewportHeight = canvasHeight;
    pass.originX = contentOffsetX - scrollX;
    pass.originY = contentOffsetY - scrollY;
    const visibleMinX = scrollX - contentOffsetX - CULL_MARGIN_PX;
    const visibleMinY = scrollY - contentOffsetY - CULL_MARGIN_PX;
    const visibleMaxX = visibleMinX + canvasWidth + CULL_MARGIN_PX * 2;
    const visibleMaxY = visibleMinY + canvasHeight + CULL_MARGIN_PX * 2;
    let count = 0;
    for (let index = 0; index < records.length; index += 1) {
      const record = records[index]!;
      if (!record.measured) continue;
      const bounds = record.bounds;
      if (
        bounds[0]! > visibleMaxX || bounds[0]! + bounds[2]! < visibleMinX
        || bounds[1]! > visibleMaxY || bounds[1]! + bounds[3]! < visibleMinY
      ) continue;
      const channels = record.rightSlot >= 0 ? 2 : 1;
      for (let channel = 0; channel < channels; channel += 1) {
        const slot = channel === 0 ? record.slot : record.rightSlot;
        if (slot < 0 || slot >= busView.capacity || busView.active[slot] !== 1) continue;
        const base = channel * 8;
        pass.slot[count] = slot;
        pass.trackX[count] = record.geometry[base]!;
        pass.trackY[count] = record.geometry[base + 1]!;
        pass.trackW[count] = record.geometry[base + 2]!;
        pass.trackH[count] = record.geometry[base + 3]!;
        pass.clipX[count] = record.geometry[base + 4]!;
        pass.clipY[count] = record.geometry[base + 5]!;
        pass.clipW[count] = record.geometry[base + 6]!;
        pass.clipH[count] = record.geometry[base + 7]!;
        pass.style[count] = record.style;
        pass.orientation[count] = record.orientation;
        pass.segments[count] = record.segments;
        pass.enabled[count] = busView.enabled[slot]!;
        pass.clip[count] = busView.clip[slot]!;
        const minDb = busView.minDb[slot]!;
        const maxDb = busView.maxDb[slot]!;
        pass.value[count] = normalizeMeterDb(busView.ballisticDb[slot]!, minDb, maxDb);
        const holdDb = busView.peakHoldDb[slot]!;
        pass.peak[count] = Number.isNaN(holdDb) ? Number.NaN : normalizeMeterDb(holdDb, minDb, maxDb);
        pass.borderPx[count] = record.borderPx;
        pass.radiusPx[count] = record.radiusPx;
        pass.clipRadiusPx[count] = record.clipRadiusPx;
        pass.segmentGapPx[count] = record.segmentGapPx;
        pass.peakThicknessPx[count] = 1;
        count += 1;
      }
    }
    pass.count = count;
    if (lastAriaMs === null || timeMs - lastAriaMs >= ariaIntervalMs) {
      lastAriaMs = timeMs;
      for (let index = 0; index < records.length; index += 1) {
        records[index]!.onAriaSample?.(timeMs);
      }
    }
  };

  const tick = (timeMs: number): void => {
    if (destroyed) return;
    const view = ownerDocument.defaultView;
    const nextDpr = view === null ? 1 : view.devicePixelRatio || 1;
    if (nextDpr !== dpr || viewport.clientWidth !== canvasWidth || viewport.clientHeight !== canvasHeight) {
      resizeCanvas(nextDpr);
      needsMeasure = true;
    }
    if (needsPalette) probePalette();
    if (needsMeasure) measure();
    assemble(timeMs);
    painter?.paint(pass);
  };

  let resizeObserver: ResizeObserver | null = null;
  if (typeof ResizeObserver !== "undefined") {
    resizeObserver = new ResizeObserver((entries) => {
      for (const entry of entries) {
        if (entry.target === viewport) {
          canvasWidth = -1;
        }
        needsMeasure = true;
      }
    });
    resizeObserver.observe(viewport);
    resizeObserver.observe(content);
  }

  let themeObserver: MutationObserver | null = null;
  if (typeof MutationObserver !== "undefined") {
    themeObserver = new MutationObserver(() => {
      needsPalette = true;
      needsMeasure = true;
    });
    themeObserver.observe(ownerDocument.documentElement, {
      attributes: true,
      subtree: true,
      attributeFilter: ["data-theme", "data-density", "data-control-size", "class"],
    });
  }

  const unsubscribe = bus.subscribe(tick);

  return {
    get drawPass() {
      return pass;
    },
    attachMeter(element, spec, onAriaSample = null) {
      if (destroyed) throw new Error("MeterSurface: controller has been destroyed");
      const record: MeterRecord = {
        element,
        slot: spec.slot,
        rightSlot: spec.rightSlot ?? -1,
        style: spec.style === "bar" ? 0 : 1,
        orientation: spec.orientation === "horizontal" ? 1 : 0,
        segments: Math.max(spec.segments, 1),
        onAriaSample,
        geometry: new Float64Array(16),
        bounds: new Float64Array(4),
        borderPx: 0,
        radiusPx: 0,
        clipRadiusPx: 0,
        segmentGapPx: 0,
        measured: false,
      };
      records.push(record);
      if (records.length * 2 > drawPassCapacity(pass)) {
        growDrawPass(pass, Math.max(drawPassCapacity(pass) * 2, records.length * 2));
      }
      resizeObserver?.observe(element);
      needsMeasure = true;
      return {
        update(next) {
          record.slot = next.slot;
          record.rightSlot = next.rightSlot ?? -1;
          record.style = next.style === "bar" ? 0 : 1;
          record.orientation = next.orientation === "horizontal" ? 1 : 0;
          record.segments = Math.max(next.segments, 1);
          needsMeasure = true;
        },
        detach: () => {
          const index = records.indexOf(record);
          if (index >= 0) records.splice(index, 1);
          resizeObserver?.unobserve(element);
        },
      };
    },
    invalidateLayout() {
      needsMeasure = true;
    },
    refreshPalette() {
      needsPalette = true;
    },
    destroy() {
      if (destroyed) return;
      destroyed = true;
      unsubscribe();
      resizeObserver?.disconnect();
      themeObserver?.disconnect();
      probeContainer?.remove();
      probeContainer = null;
      records.length = 0;
      painter?.destroy();
    },
  };
}
