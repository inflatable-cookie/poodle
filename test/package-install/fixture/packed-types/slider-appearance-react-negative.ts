// Expected-failure mapped assignability half of the packed SliderAppearance
// proof. `"pill"` is not a SliderAppearance. Compiled against installed
// `src/types.ts` via a paths map, not against the value barrel. No compiler-
// suppression comment, escape-hatch type, or cast.
import type { SliderAppearance } from "@inflatable-cookie/poodle-react";

export const appearance: SliderAppearance = "pill";
