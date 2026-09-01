/** Long markdown that renders taller than a 16rem host when unconstrained. */
export const LONG_MARKDOWN = Array.from({ length: 40 }, (_, index) => {
  const n = index + 1;
  return `## Heading ${n}\n\nParagraph ${n} expands the preview intrinsic height so a definite host must scroll internally rather than grow the surrounding layout.\n`;
}).join("\n");

export const SHORT_MARKDOWN = "One short paragraph stays naturally sized.";
