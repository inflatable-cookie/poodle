// Expected-failure assignability half of the packed SliderVariant proof.
// The removed pre-g18.022 vocabulary has no alias: "standard" is not a
// SliderVariant. No compiler-suppression comment, escape-hatch type, or cast.
import type { SliderVariant } from "@inflatable-cookie/poodle-react";

export const variant: SliderVariant = "standard";
