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
  // Agent transcript set. AgentMessage and ChangedFiles both render nothing
  // when empty — by contract, since a turn with no prose and a turn that
  // changed no files should not reserve space — so they need real content to
  // have any anatomy to assert.
  AgentMessage: { markdown: "Ran the sweep. `41` tests pass." },
  // Renders nothing without a question, by contract — the composer only shows
  // the region while one is live.
  AgentQuestionRecord: {
    question: {
      id: "placement",
      prompt: "Where should the question surface appear?",
      options: [
        { value: "inline", label: "Inline in the transcript" },
        { value: "composer", label: "Anchored above the composer" },
      ],
    },
    answer: { questionId: "placement", outcome: "selected", values: ["composer"], text: "" },
  },
  AgentQuestion: {
    questions: [
      {
        id: "placement",
        header: "Placement",
        prompt: "Where should the question surface appear?",
        options: [
          { value: "inline", label: "Inline in the transcript" },
          { value: "composer", label: "Anchored above the composer" },
        ],
      },
    ],
  },
  AgentTranscript: {
    items: [
      { kind: "message", id: "m1", role: "assistant", markdown: "Checking the parser." },
      { kind: "tool-call", id: "t1", label: "Ran command", detail: "bun test", status: "success" },
    ],
  },
  ChangedFiles: {
    id: "changed",
    files: [{ path: "src/lexer.rs", additions: 12, deletions: 3 }],
  },
  ToolCall: { id: "call", label: "Ran command", detail: "bun test", status: "success" },
  ToolCallGroup: {
    id: "run",
    calls: [
      { kind: "tool-call", id: "c1", label: "Ran command", detail: "cargo check", status: "success" },
      { kind: "tool-call", id: "c2", label: "Ran command", detail: "bun test", status: "success" },
    ],
  },
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
  // Exercises all four statuses, because the failed case is the one this
  // component exists to express and an all-pending fixture would never render
  // its danger treatment.
  Stepper: {
    ariaLabel: "Setup steps",
    steps: [
      { value: "one", label: "Current state", status: "complete" },
      { value: "two", label: "Recovery", status: "failed" },
      { value: "three", label: "Categories", status: "running" },
      { value: "four", label: "Apply and verify", status: "pending" },
    ],
    defaultValue: "three",
  },
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
  RefSelect: {
    refs: [{ value: "main", label: "main", kind: "branch" as const }],
    currentRef: "main",
    value: "main",
  },
  ModelPicker: {
    models: [{ value: "a", label: "Model A" }],
    axes: [{ key: "effort", label: "Effort", kind: "select", options: opts }],
  },
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
  // Infrastructure, not a component: without an `anchor` there is nothing to
  // position against, so it renders nothing. Covered through its consumers
  // (Popover, Select, ModelPicker, ...) and by test/overlays/portalling.test.tsx.
  AnchoredSurface: "overlay positioning primitive — inert without an anchor",
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

// Accepted axe violations per component (rule ids). Currently EMPTY: every
// component passes the sweep outright. Components whose trigger content comes
// from the consumer are given snippet fixtures in test/a11y/ rather than being
// excluded, so they are genuinely asserted. Never add an entry without a reason
// in the commit — an entry here is debt, not a fix.
export const A11Y_BASELINE: Record<string, string[]> = {};
