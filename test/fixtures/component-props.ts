// Minimum viable props per component, derived from the "Required" column of each
// contract's Public Props table. Components not listed here mount with no props.
//
// Framework-neutral: values here are plain data only. Children/slots are supplied
// by each harness (Svelte needs a Snippet, React a ReactNode), never here.

const opts = [
  { value: "a", label: "Alpha" },
  { value: "b", label: "Beta" },
];

const menuItems = [
  { value: "a", label: "Alpha" },
  { value: "b", label: "Beta" },
];

export const COMPONENT_PROPS: Record<string, Record<string, unknown>> = {
  Accordion: { items: [{ value: "a", label: "Alpha" }] },
  // Overlays render nothing while closed — open them so anatomy is assertable.
  AlertDialog: { title: "Heads up", open: true },
  AudioPlayer: { src: "/audio.mp3" },
  ConfirmAction: { title: "Are you sure?" },
  CommandPalette: { open: true },
  DebugDialog: { open: true, value: { a: 1 } },
  Dialog: { open: true },
  Drawer: { open: true },
  ContextMenu: { items: menuItems },
  EditableLabel: { value: "Label" },
  EmptyState: { title: "Nothing here" },
  Field: { id: "f1", label: "Name" },
  FilterBuilder: { fields: [{ key: "name", label: "Name", kind: "text" }] },
  // Non-collapsible variant: the collapsible header nests a <button> (the
  // CollapseToggle) inside a <button>, which is invalid HTML in BOTH
  // implementations. Tracked separately; smoke covers the valid variant.
  FilterToolbar: { collapsible: false },
  FormDialog: { title: "Edit", open: true },
  IconButton: { icon: "info", ariaLabel: "Info" },
  ListCard: { title: "Card" },
  ListContainer: { title: "Items" },
  Menu: { items: menuItems },
  Menubar: { items: [{ value: "file", label: "File", items: menuItems }] },
  MediaPicker: { open: true },
  // Empty list: rendering rows needs an `item` snippet/render-prop, which is
  // framework-specific. The empty state still exercises the section anatomy.
  InlineListSection: { title: "Section", items: [] },
  Pagination: { totalPages: 3, currentPage: 1 },
  TimeAgo: { datetime: "2026-01-01T00:00:00.000Z" },
  MetricTile: { label: "Streams", value: "1.2k" },
  NavCard: { title: "Nav" },
  NavigationMenu: { items: opts },
  OrderBy: { fields: [{ label: "Name", value: "name" }] },
  PickerShell: { title: "Pick one" },
  RadioGroup: { options: opts },
  SegmentedControl: { options: opts },
  Select: { options: opts },
  SidebarNav: { groups: [{ id: "g1", label: "Group", items: [] }] },
  SplitButton: { items: menuItems },
  Table: { columns: [{ id: "name", label: "Name" }] },
  Tabs: { items: opts },
  TextInput: { id: "t1" },
  ToastHost: {
    store: {
      // Minimal store protocol: `toasts` is subscribable, plus `dismiss`.
      toasts: {
        subscribe: (run: (v: unknown[]) => void) => {
          run([{ id: "t1", message: "Saved" }]);
          return () => {};
        },
      },
      dismiss: () => {},
    },
  },
  ToggleGroup: { options: opts },
  Tooltip: { content: "Tip" },
  Tree: { nodes: [{ value: "n1", label: "Node" }] },
  TriStateSwitch: { ariaLabel: "Toggle" },
  VideoPlayer: { src: "/video.mp4" },
};

// Components excluded from the auto-scaling smoke sweep, with the reason. Keep
// this list short and justified — it is the inverse of coverage.
// React-only exclusions (the Svelte counterpart does render its own anatomy).
export const SMOKE_EXCLUDE_REACT: Record<string, string> = {
  // Returns `children` unchanged in the happy path, so it emits no DOM of its
  // own. Svelte's version renders an EmptyState fallback, so it stays covered.
  ErrorBoundary: "passthrough — renders children only in the non-error path",
};

// Components excluded from the Svelte<->React parity gate, with the reason.
export const PARITY_EXCLUDE: Record<string, string> = {
  // Rendered with no children (the symmetric default), Svelte's boundary catches
  // the missing-children error and renders its EmptyState fallback while React
  // returns children unchanged. That is correct behaviour on both sides, so the
  // resulting class-set difference is a harness artifact, not an anatomy gap.
  ErrorBoundary: "asymmetric no-children behaviour — not an anatomy divergence",
};

export const SMOKE_EXCLUDE: Record<string, string> = {
  // Pure context providers: they render only their children and never emit a
  // poodle-* element of their own, so the anatomy assertion does not apply.
  IconProvider: "context provider — renders children only, no DOM of its own",
};
