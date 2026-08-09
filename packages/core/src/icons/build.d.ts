import type { IconSet } from "./types";

export function selectIconSet(catalogue: Record<string, unknown>, names: string[]): IconSet;
export function renderIconSetModule(iconSet: IconSet): string;
