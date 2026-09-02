// Positive packed proof that `SliderAppearance` is exported from both public
// Svelte paths (g16.046). React assignability lives in the mapped
// `slider-appearance-react-positive.ts` compile: the installed React `types`
// condition is `src/index.ts`, a value barrel that is not tsc-clean.
import type { SliderAppearance as RootAppearance } from "@inflatable-cookie/poodle-svelte";
import type { SliderAppearance as TypesAppearance } from "@inflatable-cookie/poodle-svelte/types";

const fromRoot: RootAppearance = "block";
const fromTypes: TypesAppearance = fromRoot;

export const appearances: RootAppearance[] = [fromRoot, fromTypes, "track"];
