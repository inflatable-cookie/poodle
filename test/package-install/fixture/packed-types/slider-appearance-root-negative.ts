// Expected-failure packed proof, package root (g16.046).
//
// `"pill"` is not a SliderAppearance. This file is compiled on its own and
// MUST fail with a real diagnostic: it carries no compiler-suppression
// comment, no escape-hatch type, and no cast.
import type { SliderAppearance } from "@inflatable-cookie/poodle-svelte";

export const appearance: SliderAppearance = "pill";
