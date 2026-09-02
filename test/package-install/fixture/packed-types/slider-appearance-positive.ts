// Positive packed proof that `SliderAppearance` is exported from both public
// Svelte paths (g16.046). React assignability is compiled separately through
// the installed React package exports.
import type { SliderAppearance as RootAppearance } from "@inflatable-cookie/poodle-svelte";
import type { SliderAppearance as TypesAppearance } from "@inflatable-cookie/poodle-svelte/types";

const fromRoot: RootAppearance = "block";
const fromTypes: TypesAppearance = fromRoot;

export const appearances: RootAppearance[] = [fromRoot, fromTypes, "track"];
