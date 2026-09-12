// Expected-failure packed proof, `/types` subpath (g18.022).
//
// Same claim as the root negative, one import path over. Compiled on its own,
// with no compiler-suppression comment, escape-hatch type, or cast.
import type { SliderVariant } from "@inflatable-cookie/poodle-svelte/types";

export const variant: SliderVariant = "standard";
