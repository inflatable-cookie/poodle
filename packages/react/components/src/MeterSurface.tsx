import { createContext, useContext, useEffect, useImperativeHandle, useRef, useState, forwardRef, type ReactNode } from "react";
import {
  createCanvas2dMeterSurfacePainter, createMeterSurfaceRegistry,
  type MeterBus, type MeterSurfacePainter, type MeterSurfaceRegistry,
} from "@inflatable-cookie/poodle-core";
import "@inflatable-cookie/poodle-core/styles/meter-surface.css";

const MeterSurfaceRegistryContext = createContext<MeterSurfaceRegistry | null>(null);

export function useMeterSurfaceRegistry(): MeterSurfaceRegistry | null {
  return useContext(MeterSurfaceRegistryContext);
}

export interface MeterSurfaceProps {
  bus?: MeterBus | null;
  painter?: MeterSurfacePainter | null;
  children?: ReactNode;
}

export interface MeterSurfaceHandle {
  invalidateLayout(): void;
  refreshPalette(): void;
}

export const MeterSurface = forwardRef<MeterSurfaceHandle, MeterSurfaceProps>(function MeterSurface({ bus = null, painter = null, children }, ref) {
  const rootRef = useRef<HTMLDivElement>(null);
  const viewportRef = useRef<HTMLDivElement>(null);
  const contentRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  // The registry exists from the first render so descendant surface-mode
  // AudioMeters can register before this component's effect connects it.
  const [registry] = useState(() => (bus === null ? null : createMeterSurfaceRegistry(bus)));
  useEffect(() => {
    const root = rootRef.current;
    const viewport = viewportRef.current;
    const content = contentRef.current;
    const canvas = canvasRef.current;
    if (registry === null || root === null || viewport === null || content === null || canvas === null) return;
    registry.connect({ root, viewport, content, canvas }, { painter: painter ?? createCanvas2dMeterSurfacePainter() });
    return () => registry.disconnect();
  }, [registry, painter]);
  useImperativeHandle(ref, () => ({
    invalidateLayout: () => registry?.invalidateLayout(),
    refreshPalette: () => registry?.refreshPalette(),
  }), [registry]);
  return <div className="poodle-meter-surface" data-scope="meter-surface" data-part="root" ref={rootRef}>
    <div className="poodle-meter-surface__viewport" data-part="viewport" ref={viewportRef}>
      <div className="poodle-meter-surface__content" data-part="content" ref={contentRef}>
        <MeterSurfaceRegistryContext.Provider value={registry}>{children}</MeterSurfaceRegistryContext.Provider>
      </div>
    </div>
    <canvas className="poodle-meter-surface__canvas" data-part="canvas" aria-hidden="true" ref={canvasRef} />
  </div>;
});
