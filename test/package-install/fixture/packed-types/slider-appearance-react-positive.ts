// Mapped assignability proof for packed `SliderAppearance` (g16.046).
//
// The installed React `types` condition is `src/index.ts`. Following it
// typechecks the whole React TSX graph, which is not tsc-clean. This compile
// maps the public specifier onto installed `src/types.ts`. Public-root
// resolution is proved separately in the pack harness.
import type { SliderAppearance } from "@inflatable-cookie/poodle-react";

const block: SliderAppearance = "block";
const track: SliderAppearance = "track";

export const appearances: SliderAppearance[] = [block, track];
