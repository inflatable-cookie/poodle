// Expected-failure packed proof, `/types` subpath (g16.046).
//
// Same claim as the root negative, one import path over. Compiled on its own,
// with no compiler-suppression comment, escape-hatch type, or cast.
import type { SliderAppearance } from "@inflatable-cookie/poodle-svelte/types";

export const appearance: SliderAppearance = "pill";
