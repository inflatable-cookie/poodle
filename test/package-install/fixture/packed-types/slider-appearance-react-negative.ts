// Expected-failure assignability half of the packed SliderAppearance proof.
// `"pill"` is not a SliderAppearance. No compiler-suppression comment,
// escape-hatch type, or cast.
import type { SliderAppearance } from "@inflatable-cookie/poodle-react";

export const appearance: SliderAppearance = "pill";
