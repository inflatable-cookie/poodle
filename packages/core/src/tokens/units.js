function formatRelativeUnit(value, unit) {
    if (!Number.isFinite(value)) {
        return `0${unit}`;
    }
    const rounded = Number(value.toFixed(4));
    return `${rounded}${unit}`;
}
export function pxToRem(px, base = 16) {
    return formatRelativeUnit(px / base, "rem");
}
export function pxToEm(px, base = 16) {
    return formatRelativeUnit(px / base, "em");
}
