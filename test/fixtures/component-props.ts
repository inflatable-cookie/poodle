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
  Dialog: { open: true, title: "Dialog" },
  Drawer: { open: true, title: "Drawer" },
  ContextMenu: { items: menuItems, ariaLabel: "Actions" },
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
  Menu: { items: menuItems, ariaLabel: "Menu" },
  Menubar: { items: [{ value: "file", label: "File", items: menuItems }] },
  MediaPicker: { open: true, title: "Pick media" },
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
  Select: { options: opts, ariaLabel: "Choose" },
  SidebarNav: { groups: [{ id: "g1", label: "Group", items: [] }] },
  SplitButton: { items: menuItems, ariaLabel: "Split action" },
  Table: { columns: [{ id: "name", label: "Name" }] },
  Tabs: { items: opts },
  TextInput: { id: "t1", ariaLabel: "Search" },
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
  Button: { ariaLabel: "Action" },
  Checkbox: { label: "Accept" },
  Radio: { value: "a", label: "Option A" },
  Switch: { label: "Enabled" },
  NumberInput: { ariaLabel: "Amount" },
  TimeInput: { ariaLabel: "Time" },
  TokenInput: { ariaLabel: "Tags" },
  Slider: { ariaLabel: "Volume" },
  Progress: { ariaLabel: "Loading" },
  Rating: { ariaLabel: "Rating" },
  HoverCard: { ariaLabel: "Details" },
  Popover: { ariaLabel: "Popover" },
  Collapsible: { title: "Section" },
  TextLink: { href: "#", ariaLabel: "Link" },
  MediaPreview: { title: "Preview" },
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

// Components excluded from the axe sweep, with the reason.
export const A11Y_EXCLUDE: Record<string, string> = {};

// Accepted axe violations per component (rule ids), held as an explicit baseline
// so the gate stays green while the debt stays visible. Closing an issue means
// deleting its entry — never add one without a reason in the commit.
export const A11Y_BASELINE: Record<string, string[]> = {
  // --- Harness artifacts: the trigger's content is a consumer-supplied snippet,
  // so rendering bare leaves the trigger with no accessible name. Not a defect.
  ContextMenu: ["aria-command-name"],
  HoverCard: ["aria-command-name"],
  Menu: ["aria-command-name"],
  Popover: ["aria-command-name"],

  // --- Real defects found by this sweep. Each is tracked for reconciliation;
  // delete the entry when fixed. See the a11y follow-up task.
  //
  // Dialogs render `title` but never associate it: aria-label is set to
  // undefined when a title exists and no aria-labelledby points at it, so a
  // titled dialog has NO accessible name (Dialog.svelte:190 and friends).
  AlertDialog: ["aria-dialog-name"],
  Dialog: ["aria-dialog-name"],
  Drawer: ["aria-dialog-name"],
  FormDialog: ["aria-dialog-name"],
  MediaPicker: ["aria-dialog-name"],
  // <dt>/<dd> rendered with no <dl> ancestor — no parent supplies one.
  DetailItem: ["dlitem"],
  // Nested interactive controls (same class as the FilterToolbar defect).
  FileUpload: ["label", "nested-interactive"],
  Rating: ["nested-interactive"],
  VideoPlayer: ["nested-interactive"],
  // Role used without its required ARIA attributes.
  ResizeHandle: ["aria-required-attr"],
  SplitView: ["aria-required-attr"],
  // Role not permitted on the element it is applied to.
  ToastHost: ["aria-allowed-role"],
  ToastStack: ["aria-allowed-role"],
  // ARIA attribute not permitted on that role/element.
  BlockEditor: ["aria-prohibited-attr"],
};
