// Positive packed proof that `SliderVariant` is exported from both public
// Svelte paths (g18.022). React assignability is compiled separately through
// the installed React package exports.
import type { SliderVariant as RootVariant } from "@inflatable-cookie/poodle-svelte";
import type { SliderVariant as TypesVariant } from "@inflatable-cookie/poodle-svelte/types";

const fromRoot: RootVariant = "block";
const fromTypes: TypesVariant = fromRoot;

export const variants: RootVariant[] = [fromRoot, fromTypes, "embedded"];
