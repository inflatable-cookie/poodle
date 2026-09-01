// Positive half of the packed v3 `HistoryEntry` proof (g16.033).
//
// Both public Svelte import paths must hand a consumer the live core shape:
// the package root and the explicit `/types` subpath. Nothing here reaches
// into `src/`, aliases a workspace, or reads a declaration file as text — the
// specifiers below resolve through the installed tarball exactly as a customer
// would resolve them.
import type {
  HistoryEntry as RootHistoryEntry,
  HistoryEntryPosition as RootHistoryEntryPosition,
} from "@inflatable-cookie/poodle-svelte";
import type {
  HistoryEntry as TypesHistoryEntry,
  HistoryEntryPosition as TypesHistoryEntryPosition,
} from "@inflatable-cookie/poodle-svelte/types";

const position: RootHistoryEntryPosition = "past";
const samePosition: TypesHistoryEntryPosition = position;

const fromRoot: RootHistoryEntry = {
  id: "e1",
  label: "Import stems",
  position,
  continuationCount: 2,
};

const fromTypes: TypesHistoryEntry = {
  id: "e1",
  label: "Import stems",
  position: samePosition,
  continuationCount: 2,
};

// Both paths must describe one type, not two structurally similar ones.
const rootAcceptsTypes: RootHistoryEntry = fromTypes;
const typesAcceptsRoot: TypesHistoryEntry = fromRoot;

export const continuationCounts: number[] = [
  rootAcceptsTypes.continuationCount,
  typesAcceptsRoot.continuationCount,
];
