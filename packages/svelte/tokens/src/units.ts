function formatRelativeUnit(value: number, unit: "rem" | "em"): string {
  if (!Number.isFinite(value)) {
    return `0${unit}`;
  }

  const rounded = Number(value.toFixed(4));
  return `${rounded}${unit}`;
}

export function pxToRem(px: number, base = 16): string {
  return formatRelativeUnit(px / base, "rem");
}

export function pxToEm(px: number, base = 16): string {
  return formatRelativeUnit(px / base, "em");
}
