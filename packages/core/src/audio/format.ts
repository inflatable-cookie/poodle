export type AudioValueFormat =
  | { type: "number"; decimals?: number; suffix?: string }
  | { type: "db"; decimals?: number }
  | { type: "hz"; decimals?: number }
  | { type: "khz"; decimals?: number }
  | { type: "percent"; decimals?: number }
  | { type: "ratio"; decimals?: number }
  | { type: "milliseconds"; decimals?: number }
  | { type: "note" }
  | { type: "semitones"; decimals?: number };

export const numberValueFormat: AudioValueFormat = { type: "number", decimals: 2 };

function decimals(format: { decimals?: number }, fallback: number): number {
  return Math.min(Math.max(format.decimals ?? fallback, 0), 12);
}

function fixed(value: number, places: number): string {
  return value.toFixed(places).replace(/(\.\d*?[1-9])0+$|\.0+$/u, "$1");
}

const NOTE_NAMES = ["C", "C♯", "D", "D♯", "E", "F", "F♯", "G", "G♯", "A", "A♯", "B"] as const;

export function formatAudioValue(value: number, format: AudioValueFormat = numberValueFormat): string {
  switch (format.type) {
    case "number":
      return `${fixed(value, decimals(format, 2))}${format.suffix ?? ""}`;
    case "db":
      return `${fixed(value, decimals(format, 1))} dB`;
    case "hz":
      return Math.abs(value) >= 1000
        ? `${fixed(value / 1000, decimals(format, 2))} kHz`
        : `${fixed(value, decimals(format, 1))} Hz`;
    case "khz":
      return `${fixed(value, decimals(format, 2))} kHz`;
    case "percent":
      return `${fixed(value * 100, decimals(format, 1))}%`;
    case "ratio":
      return `${fixed(value, decimals(format, 2))}:1`;
    case "milliseconds":
      return Math.abs(value) >= 1000
        ? `${fixed(value / 1000, decimals(format, 2))} s`
        : `${fixed(value, decimals(format, 1))} ms`;
    case "note": {
      const midi = Math.round(value);
      const name = NOTE_NAMES[((midi % 12) + 12) % 12];
      return `${name}${Math.floor(midi / 12) - 1}`;
    }
    case "semitones":
      return `${value > 0 ? "+" : ""}${fixed(value, decimals(format, 1))} st`;
  }
}

function parseNumber(text: string): number | null {
  const value = Number.parseFloat(text.trim().replace(",", "."));
  return Number.isFinite(value) ? value : null;
}

export function parseAudioValue(text: string, format: AudioValueFormat = numberValueFormat): number | null {
  if (format.type === "note") {
    const match = text.trim().match(/^([A-Ga-g])([#♯b♭]?)(-?\d+)$/u);
    if (!match) return parseNumber(text);
    const naturals: Record<string, number> = { C: 0, D: 2, E: 4, F: 5, G: 7, A: 9, B: 11 };
    const letter = match[1]!.toUpperCase();
    const accidental = match[2] === "#" || match[2] === "♯" ? 1 : match[2] === "b" || match[2] === "♭" ? -1 : 0;
    return (Number(match[3]) + 1) * 12 + naturals[letter]! + accidental;
  }

  const value = parseNumber(text);
  if (value === null) return null;
  const lower = text.toLowerCase();

  switch (format.type) {
    case "percent": return value / 100;
    case "hz": return lower.includes("khz") ? value * 1000 : value;
    case "khz": return lower.includes("hz") && !lower.includes("khz") ? value / 1000 : value;
    case "milliseconds": return /(^|\s)s\s*$/u.test(lower) && !lower.includes("ms") ? value * 1000 : value;
    default: return value;
  }
}
