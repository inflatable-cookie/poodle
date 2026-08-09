import { getContext, setContext } from "svelte";
import { writable, type Readable, type Writable } from "svelte/store";
export { resolveIconNodes } from "@inflatable-cookie/poodle-core/icons";
export type { IconNodeElement, IconNodes, IconSet } from "@inflatable-cookie/poodle-core/icons";

import type { IconSet } from "@inflatable-cookie/poodle-core/icons";

// ---------------------------------------------------------------------------
// Context (for providing / overriding icon sets)
// ---------------------------------------------------------------------------

const POODLE_ICON_SET = Symbol("poodle-icon-set");
const DEFAULT_ICON_SET_STORE = writable<IconSet>({});

/** @internal Set an icon set via Svelte context. Used by `IconProvider`. */
export function setIconSet(icons: IconSet): Writable<IconSet> {
  const store = writable(icons);
  setContext(POODLE_ICON_SET, store);
  return store;
}

/** @internal Read the icon set store from Svelte context. */
export function getIconSetStore(): Readable<IconSet> {
  return getContext<Readable<IconSet>>(POODLE_ICON_SET) ?? DEFAULT_ICON_SET_STORE;
}
