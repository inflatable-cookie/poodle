// Positive packed proof that `SliderAppearance` is exported from both public
// Svelte paths and the React package root (g16.046).
import type { SliderAppearance as RootAppearance } from "@inflatable-cookie/poodle-svelte";
import type { SliderAppearance as TypesAppearance } from "@inflatable-cookie/poodle-svelte/types";
import type { SliderAppearance as ReactAppearance } from "@inflatable-cookie/poodle-react";

const fromRoot: RootAppearance = "block";
const fromTypes: TypesAppearance = fromRoot;
const fromReact: ReactAppearance = fromTypes;

export const appearances: RootAppearance[] = [fromRoot, fromTypes, fromReact, "track"];
