/**
 * tsc colourises diagnostics when `FORCE_COLOR=1` (Paseo's agent default).
 * Packed-type proofs compare a plain diagnostic string; strip at the
 * comparison rather than loosening the expected text.
 */
const ESC = "\u001B";
const ANSI_PATTERN = new RegExp(
  `${ESC}\\[[0-9;?]*[ -/]*[@-~]|${ESC}\\][^\\u0007${ESC}]*(?:\\u0007|${ESC}\\\\)|${ESC}[@-Z\\\\-_]`,
  "g",
);

export function stripAnsi(text: string): string {
  return text.replace(ANSI_PATTERN, "");
}

export function outputIncludes(output: string, expected: string): boolean {
  return stripAnsi(output).includes(expected);
}
