// Expected-failure packed proof, package root (g18.022).
//
// The removed pre-g18.022 vocabulary has no alias: "standard" is not a
// SliderVariant. This file is compiled on its own and MUST fail with a real
// diagnostic: it carries no compiler-suppression comment, no escape-hatch
// type, and no cast.
import type { SliderVariant } from "@inflatable-cookie/poodle-svelte";

export const variant: SliderVariant = "standard";
