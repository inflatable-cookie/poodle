// Public-root resolution probe. The pack harness runs tsc --traceResolution
// against this file with no compiler paths map. Exit code is ignored: the
// value barrel is not tsc-clean and is not a compile proof.
import type { TreeProps } from "@inflatable-cookie/poodle-react";

export type Probe = TreeProps;
