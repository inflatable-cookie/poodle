import { getContext, setContext } from "svelte";
import type { MeterSurfaceRegistry } from "@inflatable-cookie/poodle-core";

const POODLE_METER_SURFACE = Symbol("poodle-meter-surface");

export function setMeterSurfaceRegistry(registry: MeterSurfaceRegistry): void {
  setContext(POODLE_METER_SURFACE, registry);
}

export function getMeterSurfaceRegistry(): MeterSurfaceRegistry | null {
  return getContext<MeterSurfaceRegistry>(POODLE_METER_SURFACE) ?? null;
}
