import type { MeterDrawPass, MeterSurfacePainter, MeterSurfacePalette } from "./meter-surface";

/**
 * Default Canvas2D painter for one meter surface. It reproduces the standalone
 * AudioMeter visual semantics — bar and segment styles, orientation,
 * mono/stereo channel tracks, peak hold, clip lamp, enabled opacity, and the
 * probed recipe palette — from one flat draw pass. Gradients are constructed
 * once per palette change in unit space and mapped onto each track through a
 * context scale, so the frame loop performs no gradient or style parsing.
 */
export function createCanvas2dMeterSurfacePainter(): MeterSurfacePainter {
  let canvas: HTMLCanvasElement | null = null;
  let context: CanvasRenderingContext2D | null = null;
  let palette: MeterSurfacePalette | null = null;
  let verticalGradient: CanvasGradient | null = null;
  let horizontalGradient: CanvasGradient | null = null;
  let width = 0;
  let height = 0;

  const rebuildGradients = (): void => {
    if (context === null || palette === null) return;
    // Mirror of the standalone bar-fill defaults:
    // linear-gradient(to top, success 0 70%, warning 85%, danger 100%).
    verticalGradient = context.createLinearGradient(0, 1, 0, 0);
    horizontalGradient = context.createLinearGradient(0, 0, 1, 0);
    for (const gradient of [verticalGradient, horizontalGradient]) {
      gradient.addColorStop(0, palette.barLow);
      gradient.addColorStop(0.7, palette.barLow);
      gradient.addColorStop(0.85, palette.barMid);
      gradient.addColorStop(1, palette.barHigh);
    }
  };

  return {
    setup(nextCanvas) {
      canvas = nextCanvas;
      context = nextCanvas.getContext("2d");
      rebuildGradients();
    },
    resize(widthPx, heightPx, dpr) {
      if (canvas === null) return;
      width = widthPx;
      height = heightPx;
      canvas.width = Math.max(Math.round(widthPx * dpr), 1);
      canvas.height = Math.max(Math.round(heightPx * dpr), 1);
    },
    setPalette(nextPalette) {
      palette = nextPalette;
      rebuildGradients();
    },
    paint(pass) {
      const ctx = context;
      if (ctx === null || palette === null) return;
      ctx.setTransform(pass.dpr, 0, 0, pass.dpr, 0, 0);
      ctx.clearRect(0, 0, width, height);
      ctx.translate(pass.originX, pass.originY);
      for (let index = 0; index < pass.count; index += 1) {
        const horizontal = pass.orientation[index] === 1;
        const border = pass.borderPx[index]!;
        const trackX = pass.trackX[index]!;
        const trackY = pass.trackY[index]!;
        const trackW = pass.trackW[index]!;
        const trackH = pass.trackH[index]!;
        const innerX = trackX + border;
        const innerY = trackY + border;
        const innerW = Math.max(trackW - border * 2, 0);
        const innerH = Math.max(trackH - border * 2, 0);
        const value = pass.value[index]!;
        ctx.globalAlpha = pass.enabled[index] === 1 ? 1 : palette.disabledOpacity;

        ctx.beginPath();
        ctx.roundRect(trackX, trackY, trackW, trackH, pass.radiusPx[index]!);
        ctx.fillStyle = palette.trackFill;
        ctx.fill();
        if (border > 0) {
          ctx.lineWidth = border;
          ctx.strokeStyle = palette.trackBorder;
          ctx.stroke();
        }

        ctx.save();
        ctx.beginPath();
        ctx.roundRect(innerX, innerY, innerW, innerH, Math.max(pass.radiusPx[index]! - border, 0));
        ctx.clip();

        if (pass.style[index] === 0) {
          // Bar style: unit-space gradient scaled onto this track.
          ctx.save();
          if (horizontal) {
            ctx.translate(innerX, innerY);
            ctx.scale(innerW, 1);
            ctx.fillStyle = horizontalGradient ?? palette.barLow;
            ctx.fillRect(0, 0, value, innerH);
          } else {
            ctx.translate(innerX, innerY);
            ctx.scale(1, innerH);
            ctx.fillStyle = verticalGradient ?? palette.barLow;
            ctx.fillRect(0, 1 - value, innerW, value);
          }
          ctx.restore();
        } else {
          const count = pass.segments[index]!;
          const gap = pass.segmentGapPx[index]!;
          const extent = horizontal ? innerW : innerH;
          const segmentExtent = (extent - gap * (count - 1)) / count;
          if (segmentExtent > 0) {
            for (let segment = 0; segment < count; segment += 1) {
              const active = (segment + 1) / count <= value;
              ctx.fillStyle = !active
                ? palette.segmentOff
                : segment / count >= 0.95
                  ? palette.segmentClip
                  : segment / count >= 0.75
                    ? palette.segmentWarning
                    : palette.segmentOn;
              const offset = segment * (segmentExtent + gap);
              if (horizontal) {
                ctx.fillRect(innerX + offset, innerY, segmentExtent, innerH);
              } else {
                ctx.fillRect(innerX, innerY + innerH - offset - segmentExtent, innerW, segmentExtent);
              }
            }
          }
        }

        const peak = pass.peak[index]!;
        if (!Number.isNaN(peak)) {
          const thickness = pass.peakThicknessPx[index]!;
          ctx.fillStyle = palette.peakFill;
          if (horizontal) {
            ctx.fillRect(innerX + Math.min(peak * innerW, innerW - thickness), innerY, thickness, innerH);
          } else {
            ctx.fillRect(innerX, innerY + Math.max(innerH - peak * innerH - thickness, 0), innerW, thickness);
          }
        }
        ctx.restore();

        ctx.beginPath();
        ctx.roundRect(pass.clipX[index]!, pass.clipY[index]!, pass.clipW[index]!, pass.clipH[index]!, pass.clipRadiusPx[index]!);
        ctx.fillStyle = pass.clip[index] === 1 ? palette.clipOn : palette.clipOff;
        ctx.fill();
      }
      ctx.globalAlpha = 1;
    },
    destroy() {
      if (context !== null && canvas !== null) {
        context.setTransform(1, 0, 0, 1, 0, 0);
        context.clearRect(0, 0, canvas.width, canvas.height);
      }
      canvas = null;
      context = null;
      verticalGradient = null;
      horizontalGradient = null;
    },
  };
}
