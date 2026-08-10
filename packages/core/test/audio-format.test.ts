import { describe, expect, test } from "bun:test";
import { formatAudioValue, parseAudioValue } from "../src/audio/format";

describe("audio value formatting", () => {
  test("formats domain units", () => {
    expect(formatAudioValue(-12.04, { type: "db", decimals: 1 })).toBe("-12 dB");
    expect(formatAudioValue(440, { type: "hz" })).toBe("440 Hz");
    expect(formatAudioValue(12_500, { type: "hz" })).toBe("12.5 kHz");
    expect(formatAudioValue(0.375, { type: "percent" })).toBe("37.5%");
    expect(formatAudioValue(4, { type: "ratio" })).toBe("4:1");
    expect(formatAudioValue(1500, { type: "milliseconds" })).toBe("1.5 s");
    expect(formatAudioValue(60, { type: "note" })).toBe("C4");
    expect(formatAudioValue(3, { type: "semitones" })).toBe("+3 st");
  });

  test("parses formatted entry", () => {
    expect(parseAudioValue("2.5 kHz", { type: "hz" })).toBe(2500);
    expect(parseAudioValue("50%", { type: "percent" })).toBe(0.5);
    expect(parseAudioValue("1.25 s", { type: "milliseconds" })).toBe(1250);
    expect(parseAudioValue("C4", { type: "note" })).toBe(60);
    expect(parseAudioValue("D♭4", { type: "note" })).toBe(61);
    expect(parseAudioValue("nope", { type: "db" })).toBeNull();
  });
});
