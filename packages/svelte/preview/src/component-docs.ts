export type PropDoc = {
  name: string;
  type: string;
  default?: string;
  required?: boolean;
  description: string;
};

export type SlotDoc = {
  name: string;
  description: string;
};

export type EventDoc = {
  name: string;
  payload: string;
  description: string;
};

export type ComponentDocs = {
  props: PropDoc[];
  slots?: SlotDoc[];
  events?: EventDoc[];
  usage?: string;
};

export const componentDocsMap: Record<string, ComponentDocs> = {
  accordion: {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "items", type: "AccordionItem[]", default: "[]", description: "Array of accordion panel items to render." },
      { name: "value", type: "string | string[] | null", default: "null", description: "Controlled open panel value(s)." },
      { name: "defaultValue", type: "string | string[] | null", default: "null", description: "Initial open panel value(s) for uncontrolled mode." },
      { name: "selectionMode", type: '"single" | "multiple"', default: '"single"', description: "Whether one or many panels can be open simultaneously." },
      { name: "collapsible", type: "boolean", default: "true", description: "Whether the open panel can be collapsed by clicking its trigger again." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the accordion region." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
    ],
    slots: [
      { name: "default", description: "Custom panel content. Receives slot props: item, isOpen." },
    ],
    events: [
      { name: "valueChange", payload: "{ value: string | string[] | null }", description: "Fires when the open panel(s) change." },
    ],
    usage: `<script lang="ts">
  import { Accordion } from "@poodle/svelte-primitives";

  const items = [
    { value: "one", title: "Section One", content: "Content for section one." },
    { value: "two", title: "Section Two", content: "Content for section two." },
  ];
</script>

<Accordion {items} selectionMode="single" collapsible />`,
  },

  "alert-dialog": {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "open", type: "boolean | null", default: "null", description: "Controlled open state of the dialog." },
      { name: "title", type: "string", required: true, description: "Title displayed at the top of the alert dialog." },
      { name: "description", type: "string | null", default: "null", description: "Descriptive text shown below the title." },
      { name: "itemLabel", type: "string | null", default: "null", description: "Label for an optional detail line showing the affected item." },
      { name: "itemValue", type: "string | null", default: "null", description: "Value for the optional detail line (shown when itemLabel is also set)." },
      { name: "tone", type: "AlertDialogTone", default: '"danger"', description: "Visual tone of the dialog (e.g. danger, warning)." },
      { name: "confirmLabel", type: "string", default: '"Confirm"', description: "Label for the confirm action button." },
      { name: "cancelLabel", type: "string", default: '"Cancel"', description: "Label for the cancel action button." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the dialog." },
      { name: "workingLabel", type: "string", default: '"Working…"', description: "Label displayed while the built-in confirm flow is working." },
      { name: "onConfirm", type: "(() => void | Promise<void>) | null", default: "null", description: "Optional built-in confirm callback. Promise results keep the dialog open and working until resolved." },
      { name: "onCancel", type: "(() => void) | null", default: "null", description: "Optional built-in cancel callback used for cancel and dismiss paths." },
    ],
    slots: [
      { name: "default", description: "Optional body content rendered between description and the built-in action row." },
    ],
    events: [
      { name: "confirm", payload: "void", description: "Fires when the confirm button is clicked and no onConfirm callback is supplied." },
      { name: "cancel", payload: "void", description: "Fires when the dialog is cancelled and no onCancel callback is supplied." },
      { name: "openChange", payload: "{ open: boolean }", description: "Fires when the open state changes." },
    ],
    usage: `<script lang="ts">
  import { AlertDialog } from "@poodle/svelte-primitives";

  let open = false;
</script>

<AlertDialog
  bind:open
  title="Delete item?"
  description="This action cannot be undone."
  tone="danger"
  onConfirm={async () => {
    await deleteItem();
  }}
/>`,
  },

  "audio-player": {
    props: [
      { name: "src", type: "string", required: true, description: "URL of the audio file to play." },
      { name: "ariaLabel", type: "string", default: '"Audio player"', description: "Accessible label for the player." },
      { name: "showSpeedControl", type: "boolean", default: "false", description: "Whether to show playback speed controls." },
      { name: "size", type: 'ControlSize | null', default: "null", description: "Explicit semantic size override for transport controls." },
      { name: "sizeRole", type: 'SemanticControlSizeRole', default: '"control"', description: "Semantic size role used when inheriting from presentation context." },
      { name: "density", type: 'ControlDensity | null', default: "null", description: "Explicit density override for player spacing." },
    ],
    slots: [],
    events: [],
    usage: `<script lang="ts">
  import { AudioPlayer } from "@poodle/svelte-composites";
</script>

<AudioPlayer src="/audio/podcast-episode.mp3" showSpeedControl />`,
  },

  "block-editor": {
    props: [
      { name: "blocks", type: "EditorBlock[]", default: "[default block]", description: "Array of content blocks in the editor." },
      { name: "blockTypes", type: "BlockTypeDefinition[]", default: "[defaults]", description: "Available block type definitions." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the editor is disabled." },
      { name: "ariaLabel", type: "string", default: '"Block editor"', description: "Accessible label for the editor region." },
      { name: "size", type: 'ControlSize | null', default: "null", description: "Explicit semantic control size override for editor chrome and nested controls." },
      { name: "sizeRole", type: 'SemanticControlSizeRole', default: '"control"', description: "Semantic size role used when inheriting from presentation context." },
      { name: "density", type: 'ControlDensity | null', default: "null", description: "Explicit density override for editor shell and toolbar spacing." },
    ],
    slots: [
      { name: "block", description: "Custom block renderer. Receives slot props: block, index, disabled, update." },
    ],
    events: [
      { name: "change", payload: "{ blocks: EditorBlock[] }", description: "Fires when any block content changes." },
    ],
    usage: `<script lang="ts">
  import { BlockEditor } from "@poodle/svelte-composites";

  let blocks = [{ type: "paragraph", content: "Start writing..." }];
</script>

<BlockEditor {blocks} on:change={(e) => (blocks = e.detail.blocks)} />`,
  },

  box: {
    props: [
      { name: "padding", type: "SpaceScale", default: '"none"', description: "Inner padding using the design system space scale." },
      { name: "width", type: "string | null", default: "null", description: "Explicit CSS width." },
      { name: "height", type: "string | null", default: "null", description: "Explicit CSS height." },
      { name: "minWidth", type: "string | null", default: "null", description: "Minimum CSS width." },
      { name: "minHeight", type: "string | null", default: "null", description: "Minimum CSS height." },
      { name: "overflow", type: "OverflowMode", default: '"visible"', description: "CSS overflow behavior." },
      { name: "asRole", type: "string | null", default: "null", description: "Semantic ARIA role for the container." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the box." },
    ],
    slots: [
      { name: "default", description: "Content rendered inside the box." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { Box } from "@poodle/svelte-primitives";
</script>

<Box padding="md" width="100%">
  <p>Contained content with padding.</p>
</Box>`,
  },

  breadcrumbs: {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"chrome"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "items", type: "BreadcrumbItem[]", default: "[]", description: "Array of breadcrumb navigation items." },
      { name: "ariaLabel", type: "string", default: '"Breadcrumb"', description: "Accessible label for the breadcrumb nav." },
      { name: "maxVisibleItems", type: "number | null", default: "null", description: "Maximum number of visible items before collapsing." },
    ],
    slots: [],
    events: [
      { name: "navigate", payload: "{ value: string }", description: "Fires when a breadcrumb item is clicked." },
    ],
    usage: `<script lang="ts">
  import { Breadcrumbs } from "@poodle/svelte-primitives";

  const items = [
    { value: "home", label: "Home" },
    { value: "products", label: "Products" },
    { value: "widgets", label: "Widgets" },
  ];
</script>

<Breadcrumbs {items} on:navigate={(e) => goto(e.detail.value)} />`,
  },

  "bulk-action-bar": {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "selectionCount", type: "number", default: "0", description: "Number of currently selected items." },
      { name: "totalCount", type: "number | null", default: "null", description: "Total number of selectable items." },
      { name: "actions", type: "BulkAction[]", default: "[]", description: "Array of bulk action definitions." },
      { name: "loading", type: "boolean", default: "false", description: "Disables interactions and shows busy state for bulk workflows." },
      { name: "disabled", type: "boolean", default: "false", description: "Disables the whole bar without changing selection state." },
      { name: "showSelectAll", type: "boolean", default: "false", description: "Shows a select-all or deselect-all action before the bulk actions." },
      { name: "allSelected", type: "boolean", default: "false", description: "Controls whether the select-all action presents as deselect-all." },
      { name: "selectAllLabel", type: "string", default: '"Select all"', description: "Label for the select-all button." },
      { name: "deselectAllLabel", type: "string", default: '"Deselect all"', description: "Label for the deselect-all button." },
    ],
    slots: [],
    events: [
      { name: "action", payload: "{ id: string }", description: "Fires when a bulk action is triggered." },
      { name: "clear", payload: "void", description: "Fires when the selection is cleared." },
      { name: "selectAll", payload: "void", description: "Fires when the select-all or deselect-all control is triggered." },
    ],
    usage: `<script lang="ts">
  import { BulkActionBar } from "@poodle/svelte-primitives";

  const actions = [
    { id: "delete", label: "Delete", tone: "danger" },
    { id: "archive", label: "Archive" },
  ];
</script>

<BulkActionBar selectionCount={3} totalCount={50} {actions} showSelectAll />`,
  },

  button: {
    props: [
      { name: "variant", type: "ButtonVariant", default: '"secondary"', description: "Visual variant of the button." },
      { name: "tone", type: "ButtonTone", default: '"default"', description: "Color tone of the button." },
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "type", type: 'HTMLButtonElement["type"]', default: '"button"', description: "HTML button type attribute." },
      { name: "form", type: "string | null", default: "null", description: "External form id to associate with." },
      { name: "formaction", type: "string | null", default: "null", description: "Per-button form action override." },
      { name: "formenctype", type: '"application/x-www-form-urlencoded" | "multipart/form-data" | "text/plain" | null', default: "null", description: "Per-button form encoding override." },
      { name: "formmethod", type: '"get" | "post" | "dialog" | null', default: "null", description: "Per-button form method override." },
      { name: "formnovalidate", type: "boolean", default: "false", description: "Skips validation for this submit action." },
      { name: "formtarget", type: '"_self" | "_blank" | "_parent" | "_top" | string | null', default: "null", description: "Per-button form target override." },
      { name: "pressed", type: "boolean | null", default: "null", description: "Pressed state for toggle buttons. When non-null, button renders aria-pressed and fires pressedChange." },
      { name: "defaultPressed", type: "boolean", default: "false", description: "Initial pressed state for uncontrolled toggle mode." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the button is disabled." },
      { name: "loading", type: "boolean", default: "false", description: "Whether the button shows a loading spinner." },
      { name: "leadingIcon", type: "string | null", default: "null", description: "Icon name displayed before the label." },
      { name: "trailingIcon", type: "string | null", default: "null", description: "Icon name displayed after the label." },
      { name: "chevron", type: "boolean", default: "false", description: "Whether to show a dropdown chevron." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the button." },
      { name: "ariaExpanded", type: "boolean | null", default: "null", description: "Disclosure state for expandable triggers." },
      { name: "describedBy", type: "string | null", default: "null", description: "ID of the element describing this button." },
      { name: "className", type: "string", default: '""', description: "Additional CSS class name." },
      { name: "style", type: "string | null", default: "null", description: "Inline style passthrough for dynamic sizing or CSS variables." },
    ],
    slots: [
      { name: "default", description: "Button label content." },
      { name: "leading", description: "Custom leading content replacing the leading icon." },
      { name: "trailing", description: "Custom trailing content replacing the trailing icon." },
    ],
    events: [
      { name: "click", payload: "MouseEvent", description: "Fires when the button is clicked." },
      { name: "focus", payload: "FocusEvent", description: "Fires when the button receives focus." },
      { name: "blur", payload: "FocusEvent", description: "Fires when the button loses focus." },
    ],
    usage: `<script lang="ts">
  import { Button } from "@poodle/svelte-primitives";
</script>

<Button variant="primary" tone="default" leadingIcon="plus" on:click={() => console.log("clicked")}>
  Create Item
</Button>`,
  },

  calendar: {
    props: [
      { name: "mode", type: '"single" | "range"', default: '"single"', description: "Selection mode. 'single' selects one date, 'range' selects a start/end pair." },
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "value", type: "string | DateRangeValue | null", default: "null", description: "Controlled selected value. String in single mode, DateRangeValue in range mode." },
      { name: "defaultValue", type: "string | DateRangeValue | null", default: "null", description: "Initial value for uncontrolled mode." },
      { name: "visibleMonth", type: "string | null", default: "null", description: "Controlled visible month in YYYY-MM format." },
      { name: "weekStartsOn", type: "CalendarWeekStart", default: '"monday"', description: "Which day of the week the calendar starts on." },
      { name: "locale", type: "string", default: '"en-US"', description: "Locale for date formatting." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the calendar is disabled." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the calendar." },
    ],
    slots: [],
    events: [
      { name: "valueChange", payload: "{ value: string } | { value: DateRangeValue }", description: "Fires when the selected date or range changes." },
      { name: "monthChange", payload: "{ month: string }", description: "Fires when the visible month changes." },
    ],
    usage: `<script lang="ts">
  import { Calendar } from "@poodle/svelte-primitives";

  let selectedDate: string | null = null;
</script>

<Calendar bind:value={selectedDate} weekStartsOn="monday" locale="en-US" />

<!-- Range mode -->
<Calendar mode="range" ariaLabel="Select a date range" />`,
  },

  callout: {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "tone", type: 'StatusTone | "neutral"', default: '"neutral"', description: "Visual tone indicating the callout severity." },
      { name: "title", type: "string | null", default: "null", description: "Title text for the callout." },
      { name: "message", type: "string | null", default: "null", description: "Body message text." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the callout." },
      { name: "announceMode", type: '"none" | "polite" | "assertive"', default: '"none"', description: "ARIA live region announcement mode." },
      { name: "dismissible", type: "boolean", default: "false", description: "Whether the callout can be dismissed." },
      { name: "dismissLabel", type: "string", default: '"Dismiss message"', description: "Accessible label for the dismiss button." },
    ],
    slots: [
      { name: "icon", description: "Custom icon replacing the default tone icon." },
      { name: "actions", description: "Action buttons rendered in the callout." },
      { name: "default", description: "Rich body content replacing the message prop." },
    ],
    events: [
      { name: "dismiss", payload: "void", description: "Fires when the callout is dismissed." },
    ],
    usage: `<script lang="ts">
  import { Callout } from "@poodle/svelte-primitives";
</script>

<Callout tone="warning" title="Unsaved changes" message="You have unsaved changes that will be lost." dismissible />`,
  },

  card: {
    props: [
      { name: "variant", type: "CardVariant", default: '"default"', description: "Visual variant of the card." },
      { name: "layout", type: '"vertical" | "horizontal" | "compact"', default: '"vertical"', description: "Layout direction of card content." },
      { name: "interactive", type: "boolean", default: "false", description: "Whether the card responds to hover and click." },
      { name: "selected", type: "boolean", default: "false", description: "Whether the card is in a selected state." },
      { name: "media", type: "boolean", default: "false", description: "Whether the card contains a media slot." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the card." },
    ],
    slots: [
      { name: "media", description: "Media content (image, video) rendered at the top or side." },
      { name: "header", description: "Header content with title and metadata." },
      { name: "footer", description: "Footer content with actions or metadata." },
      { name: "default", description: "Main body content of the card." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { Card } from "@poodle/svelte-primitives";
</script>

<Card variant="default" layout="vertical">
  <svelte:fragment slot="header">Project Alpha</svelte:fragment>
  <p>A brief description of the project and its current status.</p>
  <svelte:fragment slot="footer">Updated 2 hours ago</svelte:fragment>
</Card>`,
  },

  "card-radio-group": {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "items", type: "CardRadioItem[]", default: "[]", description: "Array of card radio option items." },
      { name: "value", type: "string | null", default: "null", description: "Currently selected item value." },
      { name: "columns", type: "1 | 2 | 3 | 4", default: "2", description: "Number of columns in the grid layout." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the radio group." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the entire group is disabled." },
    ],
    slots: [
      { name: "card", description: "Custom card renderer. Receives slot props: item, checked, disabled." },
    ],
    events: [
      { name: "change", payload: "{ value: string }", description: "Fires when the selected card changes." },
    ],
    usage: `<script lang="ts">
  import { CardRadioGroup } from "@poodle/svelte-composites";

  const items = [
    { value: "basic", label: "Basic", description: "For personal use" },
    { value: "pro", label: "Pro", description: "For teams" },
  ];

  let plan = "basic";
</script>

<CardRadioGroup {items} bind:value={plan} columns={2} />`,
  },

  checkbox: {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "id", type: "string | undefined", default: "undefined", description: "HTML id attribute for the checkbox input." },
      { name: "checked", type: "boolean", default: "false", description: "Controlled checked state." },
      { name: "defaultChecked", type: "boolean", default: "false", description: "Initial checked state for uncontrolled mode." },
      { name: "mixed", type: "boolean", default: "false", description: "Whether the checkbox is in an indeterminate state." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the checkbox is disabled." },
      { name: "readOnly", type: "boolean", default: "false", description: "Whether the checkbox is read-only." },
      { name: "label", type: "string | null", default: "null", description: "Label text displayed next to the checkbox." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label when no visible label is used." },
      { name: "describedBy", type: "string | null", default: "null", description: "ID of the element describing this checkbox." },
      { name: "selectedColor", type: "string | null", default: "null", description: "Optional selected-state color override used for the checked and mixed indicator fill/border." },
    ],
    slots: [],
    events: [
      { name: "checkedChange", payload: "{ checked: boolean }", description: "Fires when the checked state changes." },
    ],
    usage: `<script lang="ts">
  import { Checkbox } from "@poodle/svelte-primitives";

  let agreed = false;
</script>

<Checkbox label="I agree to the terms" bind:checked={agreed} selectedColor="#22c55e" />`,
  },

  code: {
    props: [
      { name: "source", type: "string", default: '""', description: "Source code string to display." },
      { name: "language", type: "string | null", default: "null", description: "Programming language for syntax highlighting." },
      { name: "showLineNumbers", type: "boolean", default: "false", description: "Whether to display line numbers." },
      { name: "highlightLines", type: "number[]", default: "[]", description: "Array of line numbers to highlight." },
      { name: "showCopyButton", type: "boolean", default: "true", description: "Whether to show the copy-to-clipboard button." },
      { name: "maxHeight", type: "string | null", default: "null", description: "Maximum height before scrolling." },
      { name: "inline", type: "boolean", default: "false", description: "Whether to render as an inline code element." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the code block." },
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"chrome"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
    ],
    slots: [],
    events: [],
    usage: `<script lang="ts">
  import { Code } from "@poodle/svelte-primitives";

  const source = \`function greet(name: string) {
  return \\\`Hello, \\\${name}!\\\`;
}\`;
</script>

<Code {source} language="typescript" showLineNumbers />`,
  },

  "collapse-toggle": {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"chrome"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "collapsed", type: "boolean", default: "false", description: "Whether the target region is collapsed." },
      { name: "direction", type: "CollapseDirection", default: '"left"', description: "Direction the toggle chevron points when collapsed." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the toggle is disabled." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the toggle button." },
    ],
    slots: [],
    events: [
      { name: "toggle", payload: "{ isCollapsed: boolean }", description: "Fires when the collapsed state changes." },
    ],
    usage: `<script lang="ts">
  import { CollapseToggle } from "@poodle/svelte-primitives";

  let collapsed = false;
</script>

<CollapseToggle {collapsed} direction="left" ariaLabel="Toggle sidebar" on:toggle={(e) => (collapsed = e.detail.isCollapsed)} />`,
  },

  collapsible: {
    props: [
      { name: "open", type: "boolean | null", default: "null", description: "Controlled open state." },
      { name: "defaultOpen", type: "boolean", default: "false", description: "Initial open state for uncontrolled mode." },
      { name: "title", type: "string | null", default: "null", description: "Title displayed in the trigger." },
      { name: "description", type: "string | null", default: "null", description: "Description shown below the title." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the collapsible is disabled." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the collapsible region." },
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
    ],
    slots: [
      { name: "trigger", description: "Custom trigger content. Receives slot props: isOpen." },
      { name: "default", description: "Content revealed when the collapsible is open." },
    ],
    events: [
      { name: "openChange", payload: "{ open: boolean }", description: "Fires when the open state changes." },
    ],
    usage: `<script lang="ts">
  import { Collapsible } from "@poodle/svelte-primitives";
</script>

<Collapsible title="Advanced options" description="Configure additional settings">
  <p>Advanced configuration fields go here.</p>
</Collapsible>`,
  },

  "color-picker": {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "value", type: "string", default: '"#6366f1"', description: "Current color value in hex format." },
      { name: "swatches", type: "string[]", default: "[]", description: "Preset color swatches to display." },
      { name: "showInput", type: "boolean", default: "true", description: "Whether to show the hex input field." },
      { name: "showAlpha", type: "boolean", default: "false", description: "Whether to show the alpha channel slider." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the picker is disabled." },
      { name: "ariaLabel", type: "string", default: '"Color picker"', description: "Accessible label for the color picker." },
      { name: "open", type: "boolean | null", default: "null", description: "Controlled open state of the picker popover." },
      { name: "defaultOpen", type: "boolean", default: "false", description: "Initial open state for uncontrolled mode." },
      { name: "defaultMode", type: "ColorInputMode", default: '"hex"', description: "Default color input mode." },
    ],
    slots: [],
    events: [
      { name: "change", payload: "{ value: string }", description: "Fires when the selected color changes." },
      { name: "openChange", payload: "{ open: boolean }", description: "Fires when the picker open state changes." },
    ],
    usage: `<script lang="ts">
  import { ColorPicker } from "@poodle/svelte-primitives";

  let color = "#6366f1";
  const swatches = ["#ef4444", "#f59e0b", "#10b981", "#3b82f6", "#8b5cf6"];
</script>

<ColorPicker bind:value={color} {swatches} showAlpha />`,
  },


  "command-palette": {
    props: [
      { name: "open", type: "boolean", default: "false", description: "Whether the command palette is open." },
      { name: "title", type: "string", default: '"Command palette"', description: "Title displayed at the top of the palette." },
      { name: "description", type: "string | null", default: "null", description: "Description text below the title." },
      { name: "query", type: "string", default: '""', description: "Current search query." },
      { name: "items", type: "CommandActionItem[]", default: "[]", description: "Array of command items to display." },
      { name: "state", type: "DiscoveryState", default: '"ready"', description: "Loading state of the palette content." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the palette." },
      { name: "invocationHint", type: "string | null", default: "null", description: "Keyboard shortcut hint shown in the UI." },
      { name: "size", type: 'ControlSize | null', default: "null", description: "Explicit semantic size override for hint, close control, and nested search/results chrome." },
      { name: "sizeRole", type: 'SemanticControlSizeRole', default: '"control"', description: "Semantic size role used when inheriting from presentation context." },
      { name: "density", type: 'ControlDensity | null', default: "null", description: "Explicit density override for palette shell and nested result spacing." },
    ],
    slots: [],
    events: [
      { name: "queryChange", payload: "{ value: string }", description: "Fires when the search query changes." },
      { name: "commandSelect", payload: "{ id: string }", description: "Fires when a command is selected." },
      { name: "openChange", payload: "{ open: boolean }", description: "Fires when the open state changes." },
      { name: "activeChange", payload: "{ id: string | null }", description: "Fires when the active (highlighted) item changes." },
    ],
    usage: `<script lang="ts">
  import { CommandPalette } from "@poodle/svelte-composites";

  let open = false;
  const items = [
    { id: "new-file", label: "New File", shortcut: "Ctrl+N" },
    { id: "open-file", label: "Open File", shortcut: "Ctrl+O" },
  ];
</script>

<CommandPalette bind:open {items} on:commandSelect={(e) => handleCommand(e.detail.id)} />`,
  },

  "confirm-action": {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "title", type: "string", required: true, description: "Title of the confirmation dialog." },
      { name: "description", type: "string | null", default: "null", description: "Description text in the confirmation dialog." },
      { name: "tone", type: "AlertDialogTone", default: '"danger"', description: "Visual tone of the confirmation dialog." },
      { name: "triggerLabel", type: "string", default: '"Delete"', description: "Label for the trigger button." },
      { name: "confirmLabel", type: "string", default: '"Confirm"', description: "Label for the confirm button in the dialog." },
      { name: "cancelLabel", type: "string", default: '"Cancel"', description: "Label for the cancel button in the dialog." },
    ],
    slots: [
      { name: "trigger", description: "Custom trigger element replacing the default button." },
      { name: "default", description: "Additional content rendered in the confirmation dialog body." },
    ],
    events: [
      { name: "confirm", payload: "void", description: "Fires when the action is confirmed." },
      { name: "cancel", payload: "void", description: "Fires when the action is cancelled." },
    ],
    usage: `<script lang="ts">
  import { ConfirmAction } from "@poodle/svelte-composites";
</script>

<ConfirmAction
  title="Delete this record?"
  description="This action is permanent and cannot be undone."
  tone="danger"
  triggerLabel="Delete"
  on:confirm={() => deleteRecord()}
/>`,
  },

  "context-menu": {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"chrome"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "items", type: "MenuItem[]", default: "[]", description: "Array of menu items to display." },
      { name: "open", type: "boolean | null", default: "null", description: "Controlled open state." },
      { name: "defaultOpen", type: "boolean", default: "false", description: "Initial open state for uncontrolled mode." },
      { name: "anchorPoint", type: "{ x: number; y: number } | null", default: "null", description: "Anchor position for programmatic opening." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the context menu." },
    ],
    slots: [
      { name: "default", description: "Trigger area that activates the context menu on right-click." },
    ],
    events: [
      { name: "openChange", payload: "{ open: boolean }", description: "Fires when the open state changes." },
      { name: "action", payload: "{ value: string }", description: "Fires when a menu item is selected." },
    ],
    usage: `<script lang="ts">
  import { ContextMenu } from "@poodle/svelte-primitives";

  const items = [
    { value: "copy", label: "Copy" },
    { value: "paste", label: "Paste" },
    { type: "separator" },
    { value: "delete", label: "Delete", tone: "danger" },
  ];
</script>

<ContextMenu {items} on:action={(e) => handleAction(e.detail.value)}>
  <div>Right-click me</div>
</ContextMenu>`,
  },

  "data-table": {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "ariaLabel", type: "string", default: '"Data table"', description: "Accessible label for the table." },
      { name: "columns", type: "TableColumn[]", default: "[]", description: "Column definitions for the table." },
      { name: "rows", type: "TableRow[]", default: "[]", description: "Row data to display." },
      { name: "selectable", type: "boolean", default: "false", description: "Whether the selection column is shown." },
      { name: "selectedRowIds", type: "string[]", default: "[]", description: "Array of selected row IDs." },
      { name: "sortColumnId", type: "string | null", default: "null", description: "Currently sorted column ID." },
      { name: "sortDirection", type: "TableSortDirection", default: '"asc"', description: "Current sort direction." },
      { name: "rowActionLabel", type: "string", default: '"Open"', description: "Label for the row action button." },
      { name: "showRowActions", type: "boolean", default: "true", description: "Whether to show row action buttons." },
      { name: "rowActions", type: "TableRowAction[] | ((row: TableRow) => TableRowAction[])", default: "[]", description: "Richer per-row action model for single actions or action menus." },
      { name: "expandedRowWhen", type: "(row: TableRow) => boolean", default: "() => false", description: "Controls when the expanded row slot renders." },
      { name: "filters", type: "TableFilters", default: "{}", description: "Controlled filter values keyed by column ID." },
      { name: "pagination", type: "TablePagination | null", default: "null", description: "Controlled pagination state for the footer." },
      { name: "loading", type: "boolean", default: "false", description: "Shows loading rows when true and no rows are currently available." },
      { name: "loadingRows", type: "number", default: "5", description: "Number of skeleton rows to show in the loading posture." },
      { name: "emptyMessage", type: "string", default: '"No rows match the current view."', description: "Message shown when no rows are present." },
      { name: "hiddenColumnIds", type: "string[]", default: "[]", description: "Array of column IDs to hide." },
      { name: "showColumnVisibility", type: "boolean", default: "false", description: "Whether to show the column visibility toggle." },
      { name: "showExport", type: "boolean", default: "false", description: "Whether to show the CSV export button." },
      { name: "exportFilename", type: "string", default: '"export.csv"', description: "Default filename for CSV export." },
      { name: "limitOptions", type: "number[]", default: "[10, 20, 50, 100]", description: "Available page-size options when the limit selector is visible." },
      { name: "showLimitSelector", type: "boolean", default: "true", description: "Whether to show the per-page selector in the footer." },
      { name: "compact", type: "boolean", default: "false", description: "Tightens table spacing for denser operational tables." },
      { name: "striped", type: "boolean", default: "false", description: "Alternates row backgrounds for easier scanning." },
      { name: "stickyHeader", type: "boolean", default: "false", description: "Pins table headers while the table scrolls." },
    ],
    slots: [
      { name: "cell", description: "Custom cell rendering with `let:column`, `let:row`, and `let:value`." },
      { name: "expandedRow", description: "Expanded detail row content with `let:row`." },
      { name: "empty", description: "Custom empty-state content." },
    ],
    events: [
      { name: "sortChange", payload: "{ columnId: string; direction: TableSortDirection }", description: "Fires when the sort column or direction changes." },
      { name: "rowToggle", payload: "{ rowId: string; selected: boolean }", description: "Fires when a row selection is toggled." },
      { name: "toggleAll", payload: "{ selected: boolean }", description: "Fires when the select-all checkbox is toggled." },
      { name: "rowAction", payload: "{ rowId: string }", description: "Fires when a row action button is clicked." },
      { name: "rowActionSelect", payload: "{ rowId: string; row: TableRow; action: TableRowAction }", description: "Fires when a rich row action is selected." },
      { name: "columnVisibilityChange", payload: "{ columnId: string; visible: boolean }", description: "Fires when column visibility changes." },
      { name: "exportCsv", payload: "{ filename: string }", description: "Fires when CSV export is triggered." },
      { name: "rowClick", payload: "{ rowId: string; row: TableRow }", description: "Fires when a non-interactive row surface is clicked." },
      { name: "filterChange", payload: "{ filters: TableFilters }", description: "Fires when a column filter value changes." },
      { name: "pageChange", payload: "{ page: number }", description: "Fires when the requested page changes." },
      { name: "limitChange", payload: "{ limit: number }", description: "Fires when the requested page size changes." },
    ],
    usage: `<script lang="ts">
  import { DataTable } from "@poodle/svelte-composites";

  const columns = [
    { id: "name", label: "Name", sortable: true },
    { id: "email", label: "Email" },
    { id: "role", label: "Role" },
  ];

  const rows = [
    { id: "1", cells: { name: "Alice", email: "alice@example.com", role: "Admin" } },
    { id: "2", cells: { name: "Bob", email: "bob@example.com", role: "Editor" } },
  ];
</script>

<DataTable {columns} {rows} selectable showColumnVisibility showExport />`,
  },

  "date-picker": {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "value", type: "string | null", default: "null", description: "Controlled selected date in ISO format." },
      { name: "defaultValue", type: "string | null", default: "null", description: "Initial date for uncontrolled mode." },
      { name: "open", type: "boolean | null", default: "null", description: "Controlled open state of the calendar popup." },
      { name: "defaultOpen", type: "boolean", default: "false", description: "Initial open state for uncontrolled mode." },
      { name: "placeholder", type: "string", default: '"Select date"', description: "Placeholder text when no date is selected." },
      { name: "weekStartsOn", type: "CalendarWeekStart", default: '"monday"', description: "Which day the calendar week starts on." },
      { name: "locale", type: "string", default: '"en-US"', description: "Locale for date formatting." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the picker is disabled." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the date picker." },
    ],
    slots: [
      { name: "default", description: "Custom trigger content." },
    ],
    events: [
      { name: "valueChange", payload: "{ value: string }", description: "Fires when the selected date changes." },
      { name: "openChange", payload: "{ open: boolean }", description: "Fires when the calendar open state changes." },
    ],
    usage: `<script lang="ts">
  import { DatePicker } from "@poodle/svelte-primitives";

  let date: string | null = null;
</script>

<DatePicker bind:value={date} placeholder="Pick a date" />`,
  },

  "date-range-picker": {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "value", type: "DateRangeValue | null", default: "null", description: "Controlled date range value." },
      { name: "defaultValue", type: "DateRangeValue", default: "{ start: null, end: null }", description: "Initial date range for uncontrolled mode." },
      { name: "open", type: "boolean | null", default: "null", description: "Controlled open state." },
      { name: "defaultOpen", type: "boolean", default: "false", description: "Initial open state for uncontrolled mode." },
      { name: "placeholder", type: "string", default: '"Select date range"', description: "Placeholder text when no range is selected." },
      { name: "weekStartsOn", type: "CalendarWeekStart", default: '"monday"', description: "Which day the calendar week starts on." },
      { name: "locale", type: "string", default: '"en-US"', description: "Locale for date formatting." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the picker is disabled." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the date range picker." },
    ],
    slots: [
      { name: "default", description: "Custom trigger content." },
    ],
    events: [
      { name: "valueChange", payload: "{ value: DateRangeValue }", description: "Fires when the date range changes." },
      { name: "openChange", payload: "{ open: boolean }", description: "Fires when the picker open state changes." },
    ],
    usage: `<script lang="ts">
  import { DateRangePicker } from "@poodle/svelte-primitives";

  let range = { start: null, end: null };
</script>

<DateRangePicker bind:value={range} placeholder="Select date range" />`,
  },

  "date-time-picker": {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "value", type: "DateTimeValue | null", default: "null", description: "Controlled date-time value." },
      { name: "defaultValue", type: "DateTimeValue", default: "{ date: null, time: null }", description: "Initial date-time for uncontrolled mode." },
      { name: "open", type: "boolean | null", default: "null", description: "Controlled open state." },
      { name: "defaultOpen", type: "boolean", default: "false", description: "Initial open state for uncontrolled mode." },
      { name: "placeholder", type: "string", default: '"Select date and time"', description: "Placeholder text when no value is selected." },
      { name: "weekStartsOn", type: "CalendarWeekStart", default: '"monday"', description: "Which day the calendar week starts on." },
      { name: "locale", type: "string", default: '"en-US"', description: "Locale for date formatting." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the picker is disabled." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the date-time picker." },
    ],
    slots: [
      { name: "default", description: "Custom trigger content." },
    ],
    events: [
      { name: "valueChange", payload: "{ value: DateTimeValue }", description: "Fires when the date-time value changes." },
      { name: "openChange", payload: "{ open: boolean }", description: "Fires when the picker open state changes." },
    ],
    usage: `<script lang="ts">
  import { DateTimePicker } from "@poodle/svelte-primitives";

  let dateTime = { date: null, time: null };
</script>

<DateTimePicker bind:value={dateTime} placeholder="Select date and time" />`,
  },

  "date-time-range-picker": {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "value", type: "DateTimeRangeValue | null", default: "null", description: "Controlled date-time range value." },
      { name: "defaultValue", type: "DateTimeRangeValue", default: "{ start: { date: null, time: null }, end: { date: null, time: null } }", description: "Initial date-time range for uncontrolled mode." },
      { name: "open", type: "boolean | null", default: "null", description: "Controlled open state." },
      { name: "defaultOpen", type: "boolean", default: "false", description: "Initial open state for uncontrolled mode." },
      { name: "placeholder", type: "string", default: '"Select date and time range"', description: "Placeholder text when no range is selected." },
      { name: "weekStartsOn", type: "CalendarWeekStart", default: '"monday"', description: "Which day the calendar week starts on." },
      { name: "locale", type: "string", default: '"en-US"', description: "Locale for date formatting." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the picker is disabled." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the date-time range picker." },
    ],
    slots: [
      { name: "default", description: "Custom trigger content." },
    ],
    events: [
      { name: "valueChange", payload: "{ value: DateTimeRangeValue }", description: "Fires when the date-time range changes." },
      { name: "openChange", payload: "{ open: boolean }", description: "Fires when the picker open state changes." },
    ],
    usage: `<script lang="ts">
  import { DateTimeRangePicker } from "@poodle/svelte-primitives";

  let range = { start: { date: null, time: null }, end: { date: null, time: null } };
</script>

<DateTimeRangePicker bind:value={range} placeholder="Select date and time range" />`,
  },

  "detail-item": {
    props: [
      { name: "label", type: "string", required: true, description: "Label text for the detail row." },
      { name: "value", type: "string | null", default: "null", description: "Value text displayed next to the label." },
      { name: "description", type: "string | null", default: "null", description: "Description text shown below the label." },
      { name: "truncateValue", type: "boolean", default: "false", description: "Whether to truncate long values with an ellipsis." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the detail row." },
      { name: "layout", type: '"inline" | "stacked"', default: '"inline"', description: "Layout mode for label/value arrangement." },
    ],
    slots: [
      { name: "value", description: "Custom value content replacing the value prop." },
      { name: "action", description: "Action button rendered at the end of the row." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { DetailItem } from "@poodle/svelte-primitives";
</script>

<DetailItem label="Email" value="alice@example.com" />
<DetailItem label="Role" value="Administrator" />
<DetailItem label="Notes" emptyText="No notes provided" />`,
  },

  "detail-section": {
    props: [
      { name: "title", type: "string | null", default: "null", description: "Section title." },
      { name: "description", type: "string | null", default: "null", description: "Description shown below the title." },
      { name: "separated", type: "boolean", default: "true", description: "Whether a separator is shown above the section." },
      { name: "columns", type: "1 | 2 | 3", default: "1", description: "Number of columns for the section body grid layout." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the section region." },
    ],
    slots: [
      { name: "actions", description: "Action buttons rendered in the section header." },
      { name: "default", description: "Body content of the section." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { DetailSection, DetailItem } from "@poodle/svelte-composites";
</script>

<DetailSection title="Account Information" description="Basic account details">
  <DetailItem label="Name" value="Alice Johnson" />
  <DetailItem label="Email" value="alice@example.com" />
</DetailSection>`,
  },

  "detail-shell": {
    props: [
      { name: "title", type: "string | null", default: "null", description: "Title displayed in the shell header." },
      { name: "scrollMode", type: '"shell" | "body"', default: '"body"', description: "Whether the shell or body region scrolls." },
      { name: "state", type: "BrowseState", default: '"ready"', description: "Loading state of the shell content." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the shell." },
      { name: "stateTitle", type: "string | null", default: "null", description: "Title shown in loading/error state." },
      { name: "stateMessage", type: "string | null", default: "null", description: "Message shown in loading/error state." },
    ],
    slots: [
      { name: "header", description: "Custom header content." },
      { name: "default", description: "Main body content." },
      { name: "state", description: "Custom state content for loading/error views." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { DetailShell, DetailSection } from "@poodle/svelte-composites";
</script>

<DetailShell title="User Profile" state="ready">
  <DetailSection title="Overview">
    <p>User profile content goes here.</p>
  </DetailSection>
</DetailShell>`,
  },

  dialog: {
    props: [
      { name: "open", type: "boolean | null", default: "null", description: "Controlled open state." },
      { name: "defaultOpen", type: "boolean", default: "false", description: "Initial open state for uncontrolled mode." },
      { name: "title", type: "string | null", default: "null", description: "Dialog title rendered in the built-in header." },
      { name: "description", type: "string | null", default: "null", description: "Dialog description text shown below the title." },
      { name: "role", type: '"dialog" | "alertdialog"', default: '"dialog"', description: "ARIA role for the dialog surface." },
      { name: "width", type: '"sm" | "md" | "lg" | "xl" | "full"', default: '"md"', description: "Surface width preset controlling the max inline size." },
      { name: "bare", type: "boolean", default: "false", description: "When true, the surface has no internal padding or structure; consumers control all layout." },
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "dismissOnEscape", type: "boolean", default: "true", description: "Whether pressing Escape dismisses the dialog." },
      { name: "dismissOnBackdrop", type: "boolean", default: "true", description: "Whether clicking the backdrop dismisses the dialog." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the dialog." },
      { name: "contentClassName", type: "string", default: '""', description: "Additional class name applied to the dialog surface." },
      { name: "contentStyle", type: "string", default: '""', description: "Custom inline style applied to the dialog surface element." },
      { name: "overlayClassName", type: "string", default: '""', description: "Additional class name applied to the backdrop button." },
      { name: "showCloseButton", type: "boolean", default: "false", description: "Whether to render the built-in close button." },
      { name: "closeLabel", type: "string", default: '"Close dialog"', description: "Accessible label for the built-in close button." },
      { name: "kind", type: '"dialog" | "alertdialog"', default: "undefined", description: "Deprecated. Use role instead." },
    ],
    slots: [
      { name: "default", description: "Dialog body content." },
      { name: "header", description: "Custom header content replacing the built-in title/description." },
      { name: "actions", description: "Footer actions (buttons) for the dialog." },
      { name: "footer", description: "Custom footer replacing the default actions slot." },
    ],
    events: [
      { name: "openChange", payload: "{ open: boolean }", description: "Fires when the open state changes." },
      { name: "requestClose", payload: "void", description: "Fires when a close is requested (Escape or backdrop click)." },
    ],
    usage: `<script lang="ts">
  import { Dialog, Button } from "@poodle/svelte-primitives";

  let open = false;
</script>

<Button on:click={() => (open = true)}>Open Dialog</Button>

<Dialog
  bind:open
  title="Edit Settings"
  description="Update your preferences below."
  showCloseButton
  width="lg"
>
  <p>Dialog content here.</p>
  <svelte:fragment slot="actions">
    <Button variant="secondary" on:click={() => (open = false)}>Cancel</Button>
    <Button variant="primary">Save</Button>
  </svelte:fragment>
</Dialog>

<!-- Bare mode: no padding, consumer controls all layout -->
<Dialog bind:open bare width="sm">
  <div style="padding: 2rem;">Custom bare content</div>
</Dialog>`,
  },

  "dock-region": {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"chrome"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "edge", type: "DockEdge", default: '"left"', description: "Which edge of the viewport the dock attaches to." },
      { name: "sizing", type: "DockSizing", default: '"flexible"', description: "Sizing behavior of the dock region." },
      { name: "collapsed", type: "boolean", default: "false", description: "Whether the dock is collapsed." },
      { name: "collapsedPosture", type: "DockCollapsedPosture", default: '"icon-strip"', description: "Visual posture when collapsed." },
      { name: "emphasis", type: "DockEmphasis", default: '"standard"', description: "Visual emphasis level of the dock." },
      { name: "items", type: "PanelTabItem[]", default: "[]", description: "Array of panel tab items in the dock." },
      { name: "value", type: "string | null", default: "null", description: "Currently active panel ID." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the dock region." },
      { name: "canAcceptPanel", type: "((panelId: string, sourceEdge: DockEdge) => boolean) | null", default: "null", description: "Callback to determine if a dragged panel can be dropped here." },
    ],
    slots: [
      { name: "panel", description: "Panel content renderer. Receives slot props: item." },
    ],
    events: [
      { name: "valueChange", payload: "{ value: string }", description: "Fires when the active panel changes." },
      { name: "collapsedChange", payload: "{ isCollapsed: boolean }", description: "Fires when the collapsed state changes." },
      { name: "close", payload: "{ value: string }", description: "Fires when a panel tab is closed." },
      { name: "reorder", payload: "{ items: string[] }", description: "Fires when panel tabs are reordered." },
      { name: "panelDrop", payload: "{ panel: PanelDragData; targetEdge: DockEdge }", description: "Fires when a panel is dropped onto this dock." },
    ],
    usage: `<script lang="ts">
  import { DockRegion } from "@poodle/svelte-composites";

  const items = [
    { id: "explorer", label: "Explorer", icon: "folder" },
    { id: "search", label: "Search", icon: "search" },
  ];
</script>

<DockRegion edge="left" {items} value="explorer">
  <svelte:fragment slot="panel" let:item>
    {#if item.id === "explorer"}
      <p>File explorer content</p>
    {:else}
      <p>Search panel content</p>
    {/if}
  </svelte:fragment>
</DockRegion>`,
  },

  drawer: {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "open", type: "boolean | null", default: "null", description: "Controlled open state." },
      { name: "defaultOpen", type: "boolean", default: "false", description: "Initial open state for uncontrolled mode." },
      { name: "edge", type: "DrawerEdge", default: '"right"', description: "Which edge the drawer slides from." },
      { name: "modal", type: "boolean", default: "true", description: "Whether the drawer is modal with a backdrop." },
      { name: "title", type: "string | null", default: "null", description: "Drawer title." },
      { name: "description", type: "string | null", default: "null", description: "Drawer description text." },
      { name: "dismissOnEscape", type: "boolean", default: "true", description: "Whether pressing Escape dismisses the drawer." },
      { name: "dismissOnBackdrop", type: "boolean", default: "true", description: "Whether clicking the backdrop dismisses the drawer." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the drawer." },
    ],
    slots: [
      { name: "default", description: "Drawer body content." },
      { name: "actions", description: "Footer actions for the drawer." },
    ],
    events: [
      { name: "openChange", payload: "{ open: boolean }", description: "Fires when the open state changes." },
      { name: "requestClose", payload: "void", description: "Fires when a close is requested." },
    ],
    usage: `<script lang="ts">
  import { Drawer, Button } from "@poodle/svelte-primitives";

  let open = false;
</script>

<Button on:click={() => (open = true)}>Open Drawer</Button>

<Drawer bind:open edge="right" title="Item Details">
  <p>Detail content goes here.</p>
  <svelte:fragment slot="actions">
    <Button variant="primary">Save</Button>
  </svelte:fragment>
</Drawer>`,
  },

  "duration-input": {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "hours", type: "number", default: "0", description: "Hours value." },
      { name: "minutes", type: "number", default: "0", description: "Minutes value." },
      { name: "seconds", type: "number", default: "0", description: "Seconds value." },
      { name: "showSeconds", type: "boolean", default: "true", description: "Whether to show the seconds field." },
      { name: "maxHours", type: "number", default: "99", description: "Maximum allowed hours value." },
      { name: "minTotalSeconds", type: "number", default: "0", description: "Minimum total duration in seconds." },
      { name: "maxTotalSeconds", type: "number | null", default: "null", description: "Maximum total duration in seconds." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the input is disabled." },
      { name: "ariaLabel", type: "string", default: '"Duration"', description: "Accessible label for the duration input." },
    ],
    slots: [
      { name: "default", description: "Custom content rendered alongside the duration fields." },
    ],
    events: [
      { name: "change", payload: "{ hours: number; minutes: number; seconds: number; totalSeconds: number }", description: "Fires when the duration value changes." },
    ],
    usage: `<script lang="ts">
  import { DurationInput } from "@poodle/svelte-primitives";

  let hours = 1;
  let minutes = 30;
  let seconds = 0;
</script>

<DurationInput bind:hours bind:minutes bind:seconds showSeconds={false} />`,
  },

  "editable-label": {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "value", type: "string", required: true, description: "Current text value of the editable label." },
      { name: "ariaLabel", type: "string", default: '"Edit label"', description: "Accessible label for the editable input." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the label is disabled for editing." },
      { name: "activationMode", type: "EditableLabelActivationMode", default: '"doubleClick"', description: "How to activate edit mode (doubleClick or click)." },
      { name: "selectOnFocus", type: "boolean", default: "true", description: "Whether to select all text when entering edit mode." },
      { name: "variant", type: '"default" | "flush"', default: '"default"', description: "Visual variant of the editable label." },
      { name: "emptyText", type: "string | null", default: "null", description: "Text shown when the value is empty." },
      { name: "placeholder", type: "string | null", default: "null", description: "Placeholder text shown in the input." },
      { name: "maxLength", type: "number | null", default: "null", description: "Maximum character length for the input." },
      { name: "showEditIcon", type: "boolean", default: "false", description: "Whether to show an edit icon indicator." },
    ],
    slots: [
      { name: "default", description: "Custom display content when not in edit mode." },
    ],
    events: [
      { name: "editStart", payload: "void", description: "Fires when edit mode is activated." },
      { name: "commit", payload: "{ value: string; previousValue: string }", description: "Fires when the edit is committed." },
      { name: "cancel", payload: "void", description: "Fires when the edit is cancelled." },
    ],
    usage: `<script lang="ts">
  import { EditableLabel } from "@poodle/svelte-primitives";

  let name = "Untitled Document";
</script>

<EditableLabel bind:value={name} activationMode="doubleClick" showEditIcon />`,
  },

  "editable-list": {
    props: [
      { name: "items", type: "ReorderableItem[]", default: "[]", description: "Array of list items." },
      { name: "addLabel", type: "string", default: '"Add item"', description: "Label for the add button." },
      { name: "placeholder", type: "string", default: '"New item"', description: "Placeholder text for new item input." },
      { name: "maxItems", type: "number | null", default: "null", description: "Maximum number of items allowed." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the list is disabled." },
      { name: "ariaLabel", type: "string", default: '"List"', description: "Accessible label for the list." },
      { name: "reorderable", type: "boolean", default: "true", description: "Whether items can be reordered by dragging." },
      { name: "size", type: 'ControlSize | null', default: "null", description: "Explicit semantic control size override for list rows and add controls." },
      { name: "sizeRole", type: 'SemanticControlSizeRole', default: '"control"', description: "Semantic size role used when inheriting from presentation context." },
      { name: "density", type: 'ControlDensity | null', default: "null", description: "Explicit density override for row and add-control spacing." },
    ],
    slots: [],
    events: [
      { name: "change", payload: "{ items: ReorderableItem[] }", description: "Fires when the list items change." },
    ],
    usage: `<script lang="ts">
  import { EditableList } from "@poodle/svelte-composites";

  let items = [
    { id: "1", label: "First item" },
    { id: "2", label: "Second item" },
  ];
</script>

<EditableList bind:items addLabel="Add step" placeholder="New step" />`,
  },

  "embed-input": {
    props: [
      { name: "id", type: "string", default: '"embed-input"', description: "HTML id attribute for the textarea." },
      { name: "value", type: "string", default: '""', description: "Current input value." },
      { name: "parsed", type: "ParsedEmbed | null", default: "null", description: "Parsed embed result from the input value." },
      { name: "placeholder", type: "string", default: '"Paste a URL or embed code..."', description: "Placeholder text for the input." },
      { name: "parseDebounce", type: "number", default: "300", description: "Debounce delay in ms before parsing the input." },
      { name: "providers", type: "string[]", default: "[]", description: "Allowed embed provider names." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the input is disabled." },
      { name: "error", type: "string | null", default: "null", description: "Error message to display." },
      { name: "resolveParseState", type: "((value: string, providers: string[]) => EmbedParseState) | undefined", default: "undefined", description: "Custom parse function override. When provided, replaces the built-in URL/embed parser." },
    ],
    slots: [],
    events: [
      { name: "parse", payload: "{ parsed: ParsedEmbed | null; error: string | null }", description: "Fires when the input value is parsed." },
      { name: "change", payload: "{ value: string }", description: "Fires when the input value changes." },
    ],
    usage: `<script lang="ts">
  import { EmbedInput } from "@poodle/svelte-composites";

  let url = "";
  let parsed = null;
</script>

<EmbedInput bind:value={url} on:parse={(e) => (parsed = e.detail.parsed)} placeholder="Paste a YouTube or Vimeo URL..." />`,
  },

  "embed-preview": {
    props: [
      { name: "parsed", type: "ParsedEmbed | null", default: "null", description: "Parsed embed data to preview." },
      { name: "aspectRatio", type: 'number | "auto"', default: "16/9", description: "Aspect ratio for the embed preview." },
      { name: "loading", type: "boolean", default: "false", description: "Whether the preview is in a loading state." },
      { name: "error", type: "string | null", default: "null", description: "Error message to display instead of preview." },
      { name: "emptyMessage", type: "string", default: '"No embed to preview"', description: "Message shown when no embed data is provided." },
    ],
    slots: [],
    events: [],
    usage: `<script lang="ts">
  import { EmbedPreview } from "@poodle/svelte-composites";

  export let parsed;
</script>

<EmbedPreview {parsed} aspectRatio={16/9} />`,
  },

  "empty-state": {
    props: [
      { name: "title", type: "string", required: true, description: "Title text for the empty state." },
      { name: "message", type: "string | null", default: "null", description: "Descriptive message below the title." },
      { name: "variant", type: "EmptyStateVariant", default: '"neutral"', description: "Visual variant of the empty state." },
      { name: "size", type: '"default" | "compact"', default: '"default"', description: "Layout density for page-level or inline empty states." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the empty state region." },
    ],
    slots: [
      { name: "visual", description: "Optional custom visual shown instead of the built-in variant icon." },
      { name: "actions", description: "Action buttons displayed below the message." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { EmptyState } from "@poodle/svelte-composites";
  import { Button } from "@poodle/svelte-primitives";
</script>

<EmptyState title="No results found" message="Try adjusting your search or filters." size="compact">
  <svelte:fragment slot="visual">
    <Search size={16} />
  </svelte:fragment>
  <svelte:fragment slot="actions">
    <Button variant="secondary">Clear filters</Button>
  </svelte:fragment>
</EmptyState>`,
  },

  eyebrow: {
    props: [
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the eyebrow element." },
    ],
    slots: [
      { name: "default", description: "Eyebrow text content." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { Eyebrow } from "@poodle/svelte-primitives";
</script>

<Eyebrow>Featured</Eyebrow>`,
  },

  field: {
    props: [
      { name: "id", type: "string", required: true, description: "Unique identifier for the field, used to connect label and input." },
      { name: "label", type: "string", required: true, description: "Label text for the field." },
      { name: "description", type: "string | null", default: "null", description: "Help text shown below the label (always visible)." },
      { name: "hint", type: "string | null", default: "null", description: "Progressive-disclosure help text shown in a tooltip when the user hovers/focuses the info icon next to the label." },
      { name: "error", type: "string | null", default: "null", description: "Error message displayed when validation fails." },
      { name: "pendingMessage", type: "string | null", default: "null", description: "Message shown while validation is pending." },
      { name: "validationState", type: "ValidationState", default: '"none"', description: "Current validation state of the field." },
      { name: "required", type: "boolean", default: "false", description: "Whether the field is required." },
      { name: "optionalLabel", type: "string | null", default: "null", description: "Opt-in label shown for optional fields." },
      { name: "span", type: 'number | "full" | null', default: "null", description: "Column span within a form layout grid." },
      { name: "gridArea", type: "string | null", default: "null", description: "CSS grid-area for custom grid placement." },
    ],
    slots: [
      { name: "default", description: "Field input content. Receives: { describedBy, descriptionId, errorId, messageId, validationState }." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { Field } from "@poodle/svelte-primitives";

  let email = "";
</script>

<Field id="email" label="Email address" description="We'll never share your email." required>
  <input id="email" type="email" bind:value={email} />
</Field>

<!-- With progressive-disclosure hint -->
<Field id="slug" label="URL Slug" hint="A URL-friendly identifier. Lowercase letters, numbers, and hyphens only.">
  <input id="slug" type="text" />
</Field>`,
  },

  "field-set": {
    props: [
      { name: "legend", type: "string | null", default: "null", description: "Group label rendered as a <legend> element. Styled as an uppercase eyebrow. Screen readers announce this as the group name." },
      { name: "columns", type: "number", default: "1", description: "Number of grid columns for the child fields." },
      { name: "gap", type: "SpaceScale", default: '"md"', description: "Gap between child fields." },
      { name: "span", type: 'number | "full" | null', default: "null", description: "Column span within a parent form grid layout." },
    ],
    slots: [
      { name: "default", description: "Child Field components and other form content." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { FieldSet, Field, TextInput } from "@poodle/svelte-primitives";
</script>

<FieldSet legend="Address" columns={2}>
  <Field id="street" label="Street" span="full">
    <TextInput id="street" />
  </Field>
  <Field id="city" label="City">
    <TextInput id="city" />
  </Field>
  <Field id="zip" label="ZIP Code">
    <TextInput id="zip" />
  </Field>
</FieldSet>`,
  },

  "file-upload": {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "accept", type: "string | null", default: "null", description: "Accepted file types (e.g. 'image/*,.pdf')." },
      { name: "maxSize", type: "number", default: "10485760", description: "Maximum file size in bytes (default 10 MB)." },
      { name: "multiple", type: "boolean", default: "false", description: "Whether multiple files can be uploaded." },
      { name: "maxFiles", type: "number", default: "10", description: "Maximum number of files when multiple is true." },
      { name: "showPreview", type: "boolean", default: "true", description: "Whether to show file previews." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the upload input is disabled." },
      { name: "files", type: "FileUploadItem[]", default: "[]", description: "Current array of uploaded file items." },
      { name: "validate", type: "(file: File) => string | null", default: "undefined", description: "Optional custom file validation callback." },
      { name: "compress", type: "boolean", default: "false", description: "Whether raster images should be compressed before upload." },
      { name: "compressionOptions", type: "ImageCompressionOptions", default: "DEFAULT_COMPRESSION", description: "Compression settings for raster images." },
    ],
    events: [
      { name: "change", payload: "{ files: FileUploadItem[] }", description: "Fires when the file list changes." },
      { name: "upload", payload: "{ files: File[] }", description: "Fires when new validated files are ready for app-owned upload work." },
      { name: "error", payload: "{ file: File; message: string }", description: "Fires when a file fails validation." },
      { name: "remove", payload: "{ item: FileUploadItem }", description: "Fires when a file is removed." },
    ],
    usage: `<script lang="ts">
  import { DEFAULT_COMPRESSION, FileUpload } from "@poodle/svelte-primitives";

  let files = [];
</script>

<FileUpload
  accept="image/*,.pdf"
  multiple
  maxSize={5242880}
  compress
  compressionOptions={DEFAULT_COMPRESSION}
  bind:files
/>`,
  },

  "filter-toolbar": {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"chrome"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "ariaLabel", type: "string", default: '"Filters"', description: "Accessible label for the toolbar." },
      { name: "summaryText", type: "string | null", default: "null", description: "Summary text describing active filters." },
      { name: "collapsible", type: "boolean", default: "false", description: "Whether the toolbar can be collapsed." },
      { name: "collapsed", type: "boolean", default: "false", description: "Whether the toolbar is currently collapsed." },
      { name: "columns", type: "number", default: "4", description: "Number of columns in the filter grid." },
      { name: "minItemWidth", type: "string", default: '"10rem"', description: "Minimum width of each filter item." },
      { name: "sticky", type: "boolean", default: "false", description: "Whether the toolbar sticks to the top on scroll." },
    ],
    slots: [
      { name: "actions", description: "Action buttons in the toolbar header." },
      { name: "default", description: "Filter controls placed in the grid." },
      { name: "secondary", description: "Secondary content below the main filters." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { FilterToolbar } from "@poodle/svelte-composites";
  import { Select } from "@poodle/svelte-primitives";
</script>

<FilterToolbar ariaLabel="User filters" collapsible>
  <Select options={[{ value: "admin", label: "Admin" }, { value: "user", label: "User" }]} placeholder="Role" />
</FilterToolbar>`,
  },

  "form-actions": {
    props: [
      { name: "align", type: "FormActionAlign", default: '"end"', description: "Horizontal alignment of the action buttons." },
      { name: "dangerItems", type: "FormActionDangerItem[]", default: "[]", description: "Optional overflow actions used when danger content collapses on narrow containers." },
    ],
    slots: [
      { name: "default", description: "Action buttons (e.g. Submit, Cancel)." },
      { name: "danger", description: "Optional destructive or cancel action content shown inline on wider containers." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { FormActions } from "@poodle/svelte-primitives";
  import { Button } from "@poodle/svelte-primitives";

  const dangerItems = [
    {
      label: "Discard draft",
      onSelect: () => console.log("discard"),
    },
  ];
</script>

<FormActions align="end" {dangerItems}>
  <Button variant="secondary">Back</Button>
  <Button variant="primary">Save</Button>
  <svelte:fragment slot="danger">
    <Button variant="ghost" tone="danger">Discard draft</Button>
  </svelte:fragment>
</FormActions>`,
  },

  "form-dialog": {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "open", type: "boolean | null", default: "null", description: "Controlled open state." },
      { name: "title", type: "string", required: true, description: "Title of the form dialog." },
      { name: "subtitle", type: "string | null", default: "null", description: "Primary subtitle text below the title." },
      { name: "description", type: "string | null", default: "null", description: "Fallback description text when subtitle is not supplied." },
      { name: "submitLabel", type: "string", default: '"Submit"', description: "Label for the submit button." },
      { name: "cancelLabel", type: "string", default: '"Cancel"', description: "Label for the cancel button." },
      { name: "submitting", type: "boolean", default: "false", description: "Whether the form is currently submitting." },
      { name: "error", type: "string | null", default: "null", description: "Error message shown in the dialog." },
      { name: "success", type: "string | null", default: "null", description: "Success message shown in the dialog." },
      { name: "width", type: "string | null", default: "null", description: "Optional custom dialog width CSS value." },
      { name: "showDefaultActions", type: "boolean", default: "true", description: "When false, hides the built-in submit/cancel buttons and expects an actions slot." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the dialog." },
    ],
    slots: [
      { name: "default", description: "Form content. Receives: { submitting }." },
      { name: "subtitle", description: "Optional rich subtitle content. Receives: { submitting }." },
      { name: "actions", description: "Optional custom footer actions. Receives: { submitting }." },
    ],
    events: [
      { name: "submit", payload: "void", description: "Fires when the built-in submit button is used." },
      { name: "cancel", payload: "void", description: "Fires when the dialog is cancelled or closed." },
      { name: "openChange", payload: "{ open: boolean }", description: "Fires when the open state changes." },
    ],
    usage: `<script lang="ts">
  import { FormDialog } from "@poodle/svelte-composites";

  let open = false;
  let submitting = false;
</script>

<FormDialog bind:open title="Create User" subtitle="Invite a new teammate." {submitting} on:submit={() => { submitting = true; }}>
  <input type="text" placeholder="Name" />
  <input type="email" placeholder="Email" />
</FormDialog>`,
  },

  "form-layout": {
    props: [
      { name: "columns", type: "number", default: "6", description: "Number of grid columns for the form layout." },
      { name: "error", type: "string | null", default: "null", description: "Top-level form error message." },
      { name: "success", type: "string | null", default: "null", description: "Top-level success message." },
      { name: "fieldErrors", type: "Record<string, string> | null", default: "null", description: "Map of field IDs to error messages." },
      { name: "description", type: "string | null", default: "null", description: "Description text shown above the form fields." },
    ],
    slots: [
      { name: "default", description: "Form field components." },
      { name: "actions", description: "Form action buttons." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { FormLayout } from "@poodle/svelte-composites";
  import { Field, Button } from "@poodle/svelte-primitives";
</script>

<FormLayout columns={2} description="Enter your details below.">
  <Field id="first" label="First Name" required>
    <input id="first" type="text" />
  </Field>
  <Field id="last" label="Last Name" required>
    <input id="last" type="text" />
  </Field>
  <svelte:fragment slot="actions">
    <Button variant="primary">Submit</Button>
  </svelte:fragment>
</FormLayout>`,
  },

  grid: {
    props: [
      { name: "columns", type: "string", default: '"1fr"', description: "CSS grid-template-columns value." },
      { name: "rows", type: "string | null", default: "null", description: "CSS grid-template-rows value." },
      { name: "gap", type: "SpaceScale", default: '"md"', description: "Gap between grid items." },
      { name: "padding", type: "SpaceScale", default: '"none"', description: "Padding around the grid." },
      { name: "asRole", type: "string | null", default: "null", description: "ARIA role for the grid container." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the grid." },
    ],
    slots: [
      { name: "default", description: "Grid item content." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { Grid } from "@poodle/svelte-primitives";
</script>

<Grid columns="1fr 1fr 1fr" gap="lg" padding="md">
  <div>Column 1</div>
  <div>Column 2</div>
  <div>Column 3</div>
</Grid>`,
  },

  "hover-card": {
    props: [
      { name: "open", type: "boolean | null", default: "null", description: "Controlled open state." },
      { name: "defaultOpen", type: "boolean", default: "false", description: "Initial open state for uncontrolled mode." },
      { name: "openDelayMs", type: "number", default: "180", description: "Delay in ms before the card opens on hover." },
      { name: "closeDelayMs", type: "number", default: "120", description: "Delay in ms before the card closes on mouse leave." },
      { name: "placement", type: "OverlayPlacement", default: '"top"', description: "Preferred placement of the hover card." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the hover card." },
    ],
    slots: [
      { name: "trigger", description: "Element that triggers the hover card." },
      { name: "default", description: "Hover card content." },
    ],
    events: [
      { name: "openChange", payload: "{ open: boolean }", description: "Fires when the open state changes." },
    ],
    usage: `<script lang="ts">
  import { HoverCard } from "@poodle/svelte-primitives";
</script>

<HoverCard placement="top">
  <svelte:fragment slot="trigger">
    <a href="/profile">@alice</a>
  </svelte:fragment>
  <div>
    <strong>Alice Johnson</strong>
    <p>Software Engineer</p>
  </div>
</HoverCard>`,
  },

  icon: {
    props: [
      { name: "icon", type: "IconNodes | string | null", required: true, description: "The icon to display. Pass an IconNodes array (from @poodle/icons-lucide or lucide-static) for tree-shaking, or a string name to resolve from an IconProvider set or the built-in internals." },
      { name: "name", type: "string | null", default: "null", description: "Deprecated. Use icon instead. Alias kept for internal convenience." },
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"chrome"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the icon. When set, the SVG receives role=\"img\"; otherwise it is role=\"presentation\"." },
    ],
    slots: [],
    events: [],
    usage: `<!-- Tree-shakeable: import individual icons from @poodle/icons-lucide -->
<script lang="ts">
  import { Icon } from "@poodle/svelte-primitives";
  import { search, heart, star } from "@poodle/icons-lucide";
</script>

<Icon icon={search} size="lg" ariaLabel="Search" />
<Icon icon={heart} size="sm" />
<Icon icon={star} size="xl" />

<!-- String name: resolves from IconProvider or built-in internals -->
<Icon icon="chevron-down" sizeRole="chrome" />

<!-- Bulk icon set via provider -->
<script lang="ts">
  import { Icon, IconProvider } from "@poodle/svelte-primitives";
  import iconNodes from "lucide-static/icon-nodes.json";
</script>

<IconProvider icons={iconNodes}>
  <Icon icon="rocket" />
  <Icon icon="flame" />
</IconProvider>`,
  },

  "icon-provider": {
    props: [
      { name: "icons", type: "IconSet", required: true, description: "A complete icon set mapping kebab-case names to SVG node arrays. Any icon set in this format works — lucide-static/icon-nodes.json, a Phosphor equivalent, or a custom set. String-based icon lookups resolve from this set first, then fall back to the 35 built-in internal icons." },
    ],
    slots: [
      { name: "default", description: "Child content. All descendant Icon components will resolve string names from this icon set." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { IconProvider, Icon } from "@poodle/svelte-primitives";
  import iconNodes from "lucide-static/icon-nodes.json";
</script>

<!-- All 1700+ Lucide icons available by name within the provider -->
<IconProvider icons={iconNodes}>
  <Icon icon="rocket" />
  <Icon icon="flame" />
  <Icon icon="shield-check" />
</IconProvider>`,
  },

  "icon-button": {
    props: [
      { name: "variant", type: "ButtonVariant", default: '"ghost"', description: "Visual variant of the button." },
      { name: "tone", type: "ButtonTone", default: '"default"', description: "Color tone of the button." },
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "icon", type: "IconProp", required: true, description: "The icon to display. Accepts an IconNodes array or a string name." },
      { name: "ariaLabel", type: "string", required: true, description: "Accessible label for the button (required since there is no text)." },
      { name: "tooltip", type: "string | null", default: "null", description: "Tooltip text shown on hover." },
      { name: "tooltipPlacement", type: "OverlayPlacement", default: '"top"', description: "Placement of the tooltip." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the button is disabled." },
      { name: "loading", type: "boolean", default: "false", description: "Whether the button shows a loading indicator." },
      { name: "pressed", type: "boolean | null", default: "null", description: "Pressed state for toggle buttons." },
      { name: "describedBy", type: "string | null", default: "null", description: "ID of an element that describes the button." },
      { name: "type", type: 'HTMLButtonElement["type"]', default: '"button"', description: "HTML button type attribute." },
    ],
    slots: [
      { name: "default", description: "Custom icon content replacing the icon prop." },
    ],
    events: [
      { name: "click", payload: "MouseEvent", description: "Fires when the button is clicked." },
      { name: "focus", payload: "FocusEvent", description: "Fires when the button receives focus." },
      { name: "blur", payload: "FocusEvent", description: "Fires when the button loses focus." },
    ],
    usage: `<script lang="ts">
  import { IconButton } from "@poodle/svelte-primitives";
  import { trash2, plus, settings } from "@poodle/icons-lucide";
</script>

<!-- Direct import (tree-shakeable) -->
<IconButton icon={trash2} ariaLabel="Delete item" variant="ghost" tone="danger" tooltip="Delete" />
<IconButton icon={plus} ariaLabel="Add" variant="primary" />

<!-- String name (built-in internals) -->
<IconButton icon="search" ariaLabel="Search" variant="secondary" />`,
  },

  "list-card": {
    props: [
      { name: "title", type: "string", required: true, description: "Primary title of the card." },
      { name: "subtitle", type: "string | null", default: "null", description: "Subtitle text below the title." },
      { name: "meta", type: "string | null", default: "null", description: "Meta text shown alongside the title." },
      { name: "href", type: "string | null", default: "null", description: "Optional link destination. Renders a real anchor root when provided." },
      { name: "leadingShape", type: '"circle" | "rounded-square"', default: '"circle"', description: "Shape of the leading visual element." },
      { name: "leadingFill", type: '"tint" | "solid"', default: '"tint"', description: "Fill style of the leading visual." },
      { name: "accentColor", type: "string | null", default: "null", description: "Accent color for the card." },
      { name: "layout", type: '"default" | "compact"', default: '"default"', description: "Layout density. Compact is intended for dense list and reorder contexts." },
      { name: "interactive", type: "boolean", default: "false", description: "Whether the card is clickable." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the card is disabled." },
      { name: "selectable", type: "boolean", default: "false", description: "Whether the card toggles selected state when activated." },
      { name: "selected", type: "boolean", default: "false", description: "Selected visual state for selectable cards." },
      { name: "showReorderHandle", type: "boolean", default: "false", description: "Whether to show the visual reorder affordance." },
      { name: "notLive", type: "boolean", default: "false", description: "Whether to show the card in a non-live state." },
      { name: "sash", type: "string | null", default: "null", description: "Sash label text." },
      { name: "sashColor", type: "string | null", default: "null", description: "Color of the sash." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the card." },
    ],
    slots: [
      { name: "leading", description: "Leading visual content (icon, avatar, etc.)." },
      { name: "badges", description: "Badge elements displayed on the card." },
      { name: "footer", description: "Footer content below the main body." },
      { name: "actions", description: "Explicit action trigger composition near the trailing edge." },
      { name: "trailing", description: "Trailing content on the right side." },
    ],
    events: [
      { name: "click", payload: "MouseEvent", description: "Fires when the card is clicked." },
      { name: "selectedChange", payload: "{ selected: boolean }", description: "Fires when a selectable card toggles state." },
    ],
    usage: `<script lang="ts">
  import { ListCard } from "@poodle/svelte-primitives";
</script>

<ListCard title="Alice Johnson" subtitle="Software Engineer" meta="Active" href="/team/alice">
  <svelte:fragment slot="leading">
    <img src="/avatars/alice.jpg" alt="Alice" />
  </svelte:fragment>
</ListCard>`,
  },

  "list-card-counter": {
    props: [
      { name: "icon", type: "IconProp", required: true, description: "Icon displayed before the count." },
      { name: "count", type: "number", required: true, description: "Numeric count to display." },
      { name: "tooltip", type: "string | null", default: "null", description: "Tooltip text shown on hover." },
      { name: "href", type: "string | null", default: "null", description: "Optional link destination. Renders an anchor when provided." },
      { name: "onClick", type: "((event: MouseEvent) => void) | null", default: "null", description: "Click handler callback." },
    ],
    slots: [],
    events: [],
    usage: `<script lang="ts">
  import { ListCardCounter } from "@poodle/svelte-primitives";
  import { messageSquare } from "lucide-svelte";
</script>

<ListCardCounter icon={messageSquare} count={12} tooltip="12 comments" href="/comments" />`,
  },

  "log-list": {
    props: [
      { name: "entries", type: "LogEntry[]", default: "[]", description: "Array of log entries to display." },
      { name: "variant", type: '"auto" | "stream" | "audit"', default: '"auto"', description: "Selects the stream-viewer or audit-list presentation. Auto mode detects the entry shape." },
      { name: "maxEntries", type: "number", default: "500", description: "Maximum number of entries to retain." },
      { name: "autoScroll", type: "boolean", default: "true", description: "Whether to auto-scroll to the latest entry." },
      { name: "filterLevel", type: "LogLevel | null", default: "null", description: "Filter entries by log level." },
      { name: "filterText", type: "string", default: '""', description: "Text filter applied to log entries." },
      { name: "loading", type: "boolean", default: "false", description: "Whether the audit list is loading." },
      { name: "error", type: "string | null", default: "null", description: "Error message shown in audit mode." },
      { name: "emptyMessage", type: "string", default: '"No log entries found"', description: "Empty-state copy for audit mode." },
      { name: "filters", type: "LogFilter[]", default: "[]", description: "Callback-driven audit filter definitions." },
      { name: "filterValues", type: "Record<string, string>", default: "{}", description: "Current audit filter values keyed by filter field." },
      { name: "page", type: "number", default: "1", description: "Current audit-list page (1-indexed)." },
      { name: "pageSize", type: "number", default: "50", description: "Audit-list page size used for pagination summary and controls." },
      { name: "total", type: "number | undefined", default: "undefined", description: "Total audit-list row count. When provided beyond pageSize, pagination controls appear." },
      { name: "onFilterChange", type: "((field: string, value: string) => void) | undefined", default: "undefined", description: "Callback fired when an audit filter changes." },
      { name: "onClearFilters", type: "(() => void) | undefined", default: "undefined", description: "Callback used to reset active audit filters." },
      { name: "onPageChange", type: "((page: number) => void) | undefined", default: "undefined", description: "Callback fired when the audit pagination controls request a new page." },
      { name: "onRefresh", type: "(() => void) | undefined", default: "undefined", description: "Optional audit toolbar refresh action." },
      { name: "onExport", type: "(() => void) | undefined", default: "undefined", description: "Optional audit toolbar export action." },
      { name: "getActorHref", type: "((actor: LogActor) => string) | undefined", default: "undefined", description: "Builds actor links for audit entries." },
      { name: "getResourceHref", type: "((resourceType: string, resourceId: string, action: string) => string | null) | undefined", default: "undefined", description: "Builds resource links for audit entries." },
      { name: "ariaLabel", type: "string", default: '"Log output"', description: "Accessible label for the log list." },
      { name: "size", type: 'ControlSize | null', default: "null", description: "Explicit semantic control size override for toolbar controls." },
      { name: "sizeRole", type: 'SemanticControlSizeRole', default: '"control"', description: "Semantic size role used when inheriting from presentation context." },
      { name: "density", type: 'ControlDensity | null', default: "null", description: "Explicit density override for toolbar and entry spacing." },
    ],
    slots: [],
    events: [],
    usage: `<script lang="ts">
  import { LogList, type LogEntry, type LogFilter } from "@poodle/svelte-composites";

  const filters: LogFilter[] = [
    {
      field: "action",
      label: "Action",
      type: "select",
      options: [
        { value: "create", label: "Create" },
        { value: "update", label: "Update" },
        { value: "delete", label: "Delete" },
      ],
    },
  ];

  const entries: LogEntry[] = [
    {
      id: "1",
      occurredAt: new Date().toISOString(),
      actor: { id: "u-1", name: "Alice Johnson" },
      action: "create",
      resourceType: "project",
      resourceId: "project-1",
      resourceLabel: "Launch Plan",
    },
  ];

  let filterValues = {};
</script>

<LogList
  {entries}
  {filters}
  {filterValues}
  onFilterChange={(field, value) => {
    filterValues = { ...filterValues, [field]: value };
  }}
  getActorHref={(actor) => \`/users/\${actor.id}\`}
  getResourceHref={(resourceType, resourceId) => \`/\${resourceType}/\${resourceId}\`}
/>`,
  },

  "markdown-editor": {
    props: [
      { name: "value", type: "string", default: '""', description: "Current markdown content." },
      { name: "placeholder", type: "string", default: '"Write markdown..."', description: "Placeholder text when empty." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the editor is disabled." },
      { name: "ariaLabel", type: "string", default: '"Markdown editor"', description: "Accessible label for the editor." },
      { name: "minHeight", type: "string", default: '"12rem"', description: "Minimum height of the editor." },
      { name: "mode", type: '"edit" | "preview" | "split"', default: '"edit"', description: "Display mode of the editor." },
      { name: "size", type: 'ControlSize | null', default: "null", description: "Explicit semantic control size override for toolbar and mode controls." },
      { name: "sizeRole", type: 'SemanticControlSizeRole', default: '"control"', description: "Semantic size role used when inheriting from presentation context." },
      { name: "density", type: 'ControlDensity | null', default: "null", description: "Explicit density override for toolbar and pane spacing." },
    ],
    slots: [],
    events: [
      { name: "change", payload: "{ value: string }", description: "Fires when the markdown content changes." },
    ],
    usage: `<script lang="ts">
  import { MarkdownEditor } from "@poodle/svelte-composites";

  let content = "# Hello World\n\nStart writing here...";
</script>

<MarkdownEditor bind:value={content} mode="split" minHeight="20rem" />`,
  },

  "media-picker": {
    props: [
      { name: "open", type: "boolean | null", default: "null", description: "Controlled open state of the picker." },
      { name: "items", type: "MediaPickerItem[]", default: "[]", description: "Available media items to choose from." },
      { name: "accept", type: "string", default: '"image/*"', description: "Accepted file types for upload." },
      { name: "maxFileSize", type: "number", default: "26214400", description: "Maximum upload file size in bytes (default 25 MB)." },
      { name: "title", type: "string", default: '"Select media"', description: "Title of the picker dialog." },
      { name: "emptyMessage", type: "string", default: '"No media items found."', description: "Message shown when no items are available." },
      { name: "size", type: 'ControlSize | null', default: "null", description: "Explicit semantic size override for tabs, search field, and grid item geometry." },
      { name: "sizeRole", type: 'SemanticControlSizeRole', default: '"control"', description: "Semantic size role used when inheriting from presentation context." },
      { name: "density", type: 'ControlDensity | null', default: "null", description: "Explicit density override for grid spacing and item padding." },
    ],
    slots: [],
    events: [
      { name: "select", payload: "{ item: MediaPickerItem }", description: "Fires when a media item is selected." },
      { name: "upload", payload: "{ files: FileUploadItem[] }", description: "Fires when files are uploaded." },
      { name: "openChange", payload: "{ open: boolean }", description: "Fires when the open state changes." },
    ],
    usage: `<script lang="ts">
  import { MediaPicker } from "@poodle/svelte-composites";

  let open = false;
  const items = [
    { id: "1", name: "hero.jpg", url: "/images/hero.jpg", type: "image" },
    { id: "2", name: "logo.png", url: "/images/logo.png", type: "image" },
  ];
</script>

<MediaPicker bind:open {items} accept="image/*" on:select={(e) => console.log(e.detail.item)} />`,
  },

  "media-browse-panel": {
    props: [
      { name: "loading", type: "boolean", default: "false", description: "Whether the browse panel is loading." },
      { name: "error", type: "string | null", default: "null", description: "Error message shown instead of the grid." },
      { name: "items", type: "MediaPickerItem[]", default: "[]", description: "Media items rendered in the browse grid." },
      { name: "hasMore", type: "boolean", default: "false", description: "Whether to show the load-more action." },
      { name: "emptyMessage", type: "string", default: '"No media found"', description: "Message shown when there are no items." },
      { name: "loadMoreLabel", type: "string", default: '"Load more"', description: "Label used for the load-more button." },
      { name: "size", type: 'ControlSize | null', default: "null", description: "Explicit semantic size override for browse cards and the load-more action." },
      { name: "sizeRole", type: 'SemanticControlSizeRole', default: '"control"', description: "Semantic size role used when inheriting from presentation context." },
      { name: "density", type: 'ControlDensity | null', default: "null", description: "Explicit density override for card and grid spacing." },
    ],
    slots: [],
    events: [
      { name: "select", payload: "{ item: MediaPickerItem }", description: "Fires when a media item is selected." },
      { name: "loadMore", payload: "void", description: "Fires when the load-more action is triggered." },
    ],
    usage: `<script lang="ts">
  import { MediaBrowsePanel, type MediaPickerItem } from "@poodle/svelte-composites";

  const items: MediaPickerItem[] = [
    { id: "1", label: "Hero banner", kind: "image", meta: "Image" },
    { id: "2", label: "Launch trailer", kind: "video", meta: "Video" },
  ];
</script>

<MediaBrowsePanel
  {items}
  hasMore
  on:select={(event) => console.log(event.detail.item)}
  on:loadMore={() => console.log("load more")}
/>`,
  },

  "media-upload-status-panel": {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "uploadStep", type: "MediaUploadWorkflowStep", default: '"checking"', description: "Current workflow posture for the upload state surface." },
      { name: "duplicateLabel", type: "string | null", default: "null", description: "Label for the duplicate media item." },
      { name: "uploadProgress", type: "number", default: "0", description: "Upload percentage shown during the uploading posture." },
      { name: "uploadError", type: "string | null", default: "null", description: "Error message shown during the error posture." },
    ],
    slots: [],
    events: [
      { name: "uploadAnyway", payload: "void", description: "Fires when the user chooses to upload a duplicate as new." },
      { name: "selectDuplicate", payload: "void", description: "Fires when the user chooses the existing duplicate media item." },
      { name: "clearUpload", payload: "void", description: "Fires when the user resets the upload flow." },
      { name: "selectUploaded", payload: "void", description: "Fires when the user chooses the newly uploaded media item." },
    ],
    usage: `<script lang="ts">
  import { MediaUploadStatusPanel } from "@poodle/svelte-composites";
</script>

<MediaUploadStatusPanel
  uploadStep="duplicate"
  duplicateLabel="hero-banner.jpg"
  on:uploadAnyway={() => console.log("upload anyway")}
  on:selectDuplicate={() => console.log("select duplicate")}
/>`,
  },

  "media-preview": {
    props: [
      { name: "title", type: "string", required: true, description: "Title of the media preview." },
      { name: "description", type: "string | null", default: "null", description: "Description text below the title." },
      { name: "eyebrow", type: "string | null", default: "null", description: "Eyebrow text above the title." },
      { name: "caption", type: "string | null", default: "null", description: "Caption text for the media." },
      { name: "meta", type: "string[]", default: "[]", description: "Array of meta text items." },
      { name: "badge", type: "string | null", default: "null", description: "Badge label shown on the preview." },
      { name: "thumbnailMeta", type: "string | null", default: "null", description: "Meta text overlaid on the thumbnail." },
      { name: "kind", type: "MediaKind", default: '"image"', description: "Kind of media (image, video, audio, etc.)." },
      { name: "state", type: "MediaState", default: '"ready"', description: "Current state of the media." },
      { name: "aspectRatio", type: "AspectRatio", default: '"landscape"', description: "Aspect ratio of the media preview." },
      { name: "variant", type: "CardVariant", default: '"default"', description: "Visual variant of the card." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the preview." },
      { name: "stateTitle", type: "string | null", default: "null", description: "Title shown during loading/error state." },
      { name: "stateMessage", type: "string | null", default: "null", description: "Message shown during loading/error state." },
    ],
    slots: [
      { name: "media", description: "Custom media content (image, video element, etc.)." },
      { name: "default", description: "Body content below the media." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { MediaPreview } from "@poodle/svelte-composites";
</script>

<MediaPreview
  title="Sunset at the Beach"
  eyebrow="Photography"
  description="A beautiful sunset captured at the coast."
  kind="image"
  aspectRatio="landscape"
>
  <svelte:fragment slot="media">
    <img src="/photos/sunset.jpg" alt="Sunset" />
  </svelte:fragment>
</MediaPreview>`,
  },

  "media-thumbnail": {
    props: [
      { name: "kind", type: "MediaKind", default: '"image"', description: "Kind of media represented." },
      { name: "state", type: "MediaState", default: '"ready"', description: "Current state of the media." },
      { name: "aspectRatio", type: "AspectRatio", default: '"landscape"', description: "Aspect ratio of the thumbnail." },
      { name: "title", type: "string | null", default: "null", description: "Title text for the thumbnail." },
      { name: "badge", type: "string | null", default: "null", description: "Badge label shown on the thumbnail." },
      { name: "meta", type: "string | null", default: "null", description: "Meta text overlaid on the thumbnail." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the thumbnail." },
      { name: "stateTitle", type: "string | null", default: "null", description: "Title shown during loading/error state." },
      { name: "stateMessage", type: "string | null", default: "null", description: "Message shown during loading/error state." },
      { name: "presentation", type: '"default" | "compact"', default: '"default"', description: "Visual presentation mode." },
    ],
    slots: [
      { name: "default", description: "Media content inside the thumbnail." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { MediaThumbnail } from "@poodle/svelte-composites";
</script>

<MediaThumbnail kind="video" aspectRatio="landscape" badge="HD" meta="3:24">
  <img src="/thumbnails/video-preview.jpg" alt="Video preview" />
</MediaThumbnail>`,
  },

  menu: {
    props: [
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "items", type: "MenuItem[]", default: "[]", description: "Array of menu items to display." },
      { name: "open", type: "boolean | null", default: "null", description: "Controlled open state." },
      { name: "defaultOpen", type: "boolean", default: "false", description: "Initial open state for uncontrolled mode." },
      { name: "placement", type: "OverlayPlacement", default: '"bottom-start"', description: "Preferred placement of the menu." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the menu." },
      { name: "triggerAriaLabel", type: "string | null", default: "null", description: "Accessible label for the trigger wrapper, useful for icon-only triggers." },
    ],
    slots: [
      { name: "trigger", description: "Element that opens the menu." },
    ],
    events: [
      { name: "openChange", payload: "{ open: boolean }", description: "Fires when the open state changes." },
      { name: "action", payload: "{ value: string }", description: "Fires when a menu item is selected." },
    ],
    usage: `<script lang="ts">
  import { Menu } from "@poodle/svelte-primitives";
  import { Button } from "@poodle/svelte-primitives";

  const items = [
    { value: "edit", label: "Edit" },
    { value: "duplicate", label: "Duplicate" },
    { type: "separator" },
    { value: "delete", label: "Delete", tone: "danger" },
  ];
</script>

<Menu {items} on:action={(e) => console.log(e.detail.value)}>
  <svelte:fragment slot="trigger">
    <Button variant="secondary">Actions</Button>
  </svelte:fragment>
</Menu>`,
  },

  menubar: {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"chrome"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "value", type: "string | null", default: "null", description: "Controlled active menu value." },
      { name: "defaultValue", type: "string | null", default: "null", description: "Initial active menu for uncontrolled mode." },
      { name: "items", type: "MenubarItem[]", default: "[]", description: "Array of top-level menubar items." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the menubar." },
    ],
    slots: [],
    events: [
      { name: "valueChange", payload: "{ value: string | null }", description: "Fires when the active menu changes." },
      { name: "action", payload: "{ value: string }", description: "Fires when a menu action is selected." },
    ],
    usage: `<script lang="ts">
  import { Menubar } from "@poodle/svelte-primitives";

  const items = [
    { value: "file", label: "File", children: [
      { value: "new", label: "New" },
      { value: "open", label: "Open" },
    ]},
    { value: "edit", label: "Edit", children: [
      { value: "undo", label: "Undo" },
      { value: "redo", label: "Redo" },
    ]},
  ];
</script>

<Menubar {items} on:action={(e) => console.log(e.detail.value)} />`,
  },

  "meta-bar": {
    props: [
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the meta bar region." },
      { name: "showSeparators", type: "boolean", default: "true", description: "Whether to show dot separators between items." },
    ],
    slots: [
      { name: "default", description: "MetaItem children rendered as separated inline items." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { MetaBar, MetaItem } from "@poodle/svelte-primitives";
</script>

<MetaBar ariaLabel="Document metadata">
  <MetaItem label="Status">Published</MetaItem>
  <MetaItem label="Author">Alice</MetaItem>
  <MetaItem>3 min read</MetaItem>
</MetaBar>`,
  },

  "meta-item": {
    props: [
      { name: "label", type: "string | null", default: "null", description: "Uppercase label text displayed before the value." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the meta item." },
    ],
    slots: [
      { name: "default", description: "Value content rendered after the optional label." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { MetaItem } from "@poodle/svelte-primitives";
</script>

<MetaItem label="Updated">March 30, 2026</MetaItem>`,
  },

  meter: {
    props: [
      { name: "value", type: "number", default: "0", description: "Current meter value." },
      { name: "min", type: "number", default: "0", description: "Minimum value of the meter." },
      { name: "max", type: "number", default: "100", description: "Maximum value of the meter." },
      { name: "low", type: "number | null", default: "null", description: "Threshold below which the value is considered low." },
      { name: "high", type: "number | null", default: "null", description: "Threshold above which the value is considered high." },
      { name: "optimum", type: "number | null", default: "null", description: "Optimum value for the meter." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the meter." },
    ],
    slots: [],
    events: [],
    usage: `<script lang="ts">
  import { Meter } from "@poodle/svelte-primitives";
</script>

<Meter value={72} min={0} max={100} low={25} high={75} optimum={50} ariaLabel="Disk usage" />`,
  },

  "metric-tile": {
    props: [
      { name: "label", type: "string", required: true, description: "Label describing the metric." },
      { name: "value", type: "string", required: true, description: "Formatted metric value to display." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the tile." },
      { name: "trend", type: '"up" | "down" | "flat" | null', default: "null", description: "Trend direction indicator." },
      { name: "trendLabel", type: "string | null", default: "null", description: "Text label for the trend (e.g. '+12%')." },
      { name: "sparklineData", type: "number[] | null", default: "null", description: "Array of values for a sparkline chart." },
    ],
    slots: [],
    events: [],
    usage: `<script lang="ts">
  import { MetricTile } from "@poodle/svelte-composites";
</script>

<MetricTile label="Revenue" value="$48,200" trend="up" trendLabel="+12.5%" sparklineData={[20, 35, 28, 42, 48]} />`,
  },

  "nav-card": {
    props: [
      { name: "title", type: "string", required: true, description: "Title text displayed on the card." },
      { name: "description", type: "string | null", default: "null", description: "Description text below the title." },
      { name: "href", type: "string | null", default: "null", description: "Link URL for the card navigation." },
      { name: "badge", type: "string | null", default: "null", description: "Badge label shown on the card." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the card is disabled." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the nav card." },
    ],
    slots: [
      { name: "icon", description: "Icon content displayed in the card." },
    ],
    events: [
      { name: "click", payload: "MouseEvent", description: "Fires when the card is clicked." },
    ],
    usage: `<script lang="ts">
  import { NavCard } from "@poodle/svelte-primitives";
</script>

<NavCard title="Dashboard" description="View your analytics" href="/dashboard" badge="New">
  <svelte:fragment slot="icon">📊</svelte:fragment>
</NavCard>`,
  },

  "navigation-menu": {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"chrome"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "value", type: "string | null", default: "null", description: "Controlled active menu value." },
      { name: "defaultValue", type: "string | null", default: "null", description: "Initial active value for uncontrolled mode." },
      { name: "items", type: "NavigationMenuItem[]", default: "[]", description: "Array of navigation menu items." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the navigation menu." },
    ],
    slots: [
      { name: "default", description: "Custom menu content. Receives slot props: activeValue, activeItem." },
    ],
    events: [
      { name: "valueChange", payload: "{ value: string | null }", description: "Fires when the active menu value changes." },
    ],
    usage: `<script lang="ts">
  import { NavigationMenu } from "@poodle/svelte-primitives";

  const items = [
    { value: "products", label: "Products" },
    { value: "pricing", label: "Pricing" },
    { value: "docs", label: "Documentation" },
  ];
</script>

<NavigationMenu {items} ariaLabel="Main navigation" />`,
  },

  "number-input": {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "id", type: "string", default: "\"\"", description: "HTML id attribute for the input." },
      { name: "value", type: "number | string | null", default: "null", description: "Controlled value. Numeric bindings stay numeric; string-form bindings round-trip as strings." },
      { name: "defaultValue", type: "number | string | null", default: "null", description: "Initial value for uncontrolled mode." },
      { name: "placeholder", type: "string | null", default: "null", description: "Placeholder text when empty." },
      { name: "name", type: "string | undefined", default: "undefined", description: "Form field name." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the input is disabled." },
      { name: "readOnly", type: "boolean", default: "false", description: "Whether the input is read-only." },
      { name: "required", type: "boolean", default: "false", description: "Whether the input is required for form submission." },
      { name: "min", type: "number | null", default: "null", description: "Minimum allowed value." },
      { name: "max", type: "number | null", default: "null", description: "Maximum allowed value." },
      { name: "step", type: "number | string | null", default: "null", description: "Step increment for value changes." },
      { name: "precision", type: "number | null", default: "null", description: "Number of decimal places to display." },
      { name: "prefix", type: "string | null", default: "null", description: "Optional prefix chip rendered before the control." },
      { name: "validationState", type: "ValidationState", default: '"none"', description: "Validation state of the input." },
      { name: "validate", type: "InputValidator | undefined", default: "undefined", description: "Optional async/sync validator for string-form workflows." },
      { name: "validationContext", type: "unknown", default: "undefined", description: "Optional validation context passed to the validator." },
      { name: "showSteppers", type: "boolean", default: "false", description: "Whether to show increment/decrement stepper buttons." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the input." },
      { name: "describedBy", type: "string | null", default: "null", description: "ID of the element describing this input." },
    ],
    slots: [],
    events: [
      { name: "valueChange", payload: "{ value: number | string | null }", description: "Fires when the value changes." },
      { name: "validationChange", payload: "{ status: InputValidationStatus; valid: boolean; message: string }", description: "Fires when validation status changes." },
      { name: "submit", payload: "{ value: number | string | null }", description: "Fires when the value is submitted (e.g. Enter key)." },
      { name: "increment", payload: "{ value: number | string | null }", description: "Fires when the value is incremented." },
      { name: "decrement", payload: "{ value: number | string | null }", description: "Fires when the value is decremented." },
      { name: "focus", payload: "FocusEvent", description: "Fires when the input receives focus." },
      { name: "blur", payload: "FocusEvent", description: "Fires when the input loses focus." },
    ],
    usage: `<script lang="ts">
  import { NumberInput } from "@poodle/svelte-primitives";

  let quantity: number | null = 1;
</script>

<NumberInput id="quantity" bind:value={quantity} min={0} max={100} step={1} showSteppers />`,
  },

  "order-by": {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "fields", type: "SortField[]", default: "[]", description: "Array of sortable field definitions. Multi-field callers should use key plus optional defaultDirection." },
      { name: "value", type: "OrderByValue", default: "[]", description: "Ordered multi-field sort state." },
      { name: "activeSort", type: "ActiveSort | null", default: "null", description: "Legacy single-sort compatibility signal derived from the first sort item." },
      { name: "maxFields", type: "number | null", default: "null", description: "Optional cap on number of active sort fields." },
      { name: "compact", type: "boolean", default: "false", description: "Collapses long trigger summaries after the first two items." },
      { name: "ariaLabel", type: "string", default: '"Sort by"', description: "Accessible label for the sort control." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the sort control is disabled." },
    ],
    slots: [],
    events: [
      { name: "change", payload: "{ value: OrderByValue; sort: ActiveSort | null }", description: "Fires when the sort configuration changes." },
    ],
    usage: `<script lang="ts">
  import { OrderBy, type OrderByValue } from "@poodle/svelte-primitives";

  const fields = [
    { key: "name", label: "Name" },
    { key: "date", label: "Date", defaultDirection: "desc" },
    { key: "priority", label: "Priority" },
  ];

  let value: OrderByValue = [];
</script>

<OrderBy {fields} bind:value on:change={(e) => console.log(e.detail.value)} />`,
  },

  "page-header": {
    props: [
      { name: "title", type: "string | null", default: "null", description: "Primary page title." },
      { name: "section", type: "string | null", default: "null", description: "Optional section label rendered above the title." },
      { name: "count", type: "number | null", default: "null", description: "Optional count badge rendered inline with the title." },
      { name: "subtitle", type: "string | null", default: "null", description: "Subtitle text below the title." },
      { name: "eyebrow", type: "string | null", default: "null", description: "Eyebrow label above the title." },
      { name: "backHref", type: "string | null", default: "null", description: "Optional back-link URL rendered above the title block." },
      { name: "backLabel", type: "string | null", default: "null", description: "Optional text for the back link; defaults to Back." },
      { name: "backIsContextual", type: "boolean", default: "false", description: "Whether the back link should show the contextual indicator dot." },
      { name: "bannerMessage", type: "string | null", default: "null", description: "Optional shortcut banner rendered below the header." },
      { name: "bannerTone", type: '"neutral" | "info" | "success" | "warning" | "danger"', default: '"warning"', description: "Tone for the shortcut banner." },
      { name: "align", type: '"start" | "between"', default: '"between"', description: "Alignment of the header content and actions." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the header region." },
    ],
    slots: [
      { name: "breadcrumbs", description: "Breadcrumb navigation rendered above the title." },
      { name: "actions", description: "Action buttons rendered on the right side of the header." },
      { name: "banner", description: "Optional custom banner content rendered below the header body." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { PageHeader } from "@poodle/svelte-composites";
  import { Button } from "@poodle/svelte-primitives";
</script>

<PageHeader
  section="Catalog"
  title="All Products"
  count={42}
  subtitle="Manage your product listings"
  backHref="/dashboard"
  backLabel="Back to dashboard"
  bannerMessage="Some products are currently hidden."
  bannerTone="info"
>
  <svelte:fragment slot="actions">
    <Button variant="primary" leadingIcon="plus">Add Product</Button>
  </svelte:fragment>
</PageHeader>`,
  },

  "page-loading": {
    props: [
      { name: "visible", type: "boolean", default: "true", description: "Whether the loading overlay is visible." },
      { name: "presentation", type: '"overlay" | "inline"', default: '"overlay"', description: "Whether the component renders as a full-screen overlay or an inline centered loading state." },
      { name: "value", type: "number | null", default: "null", description: "Determinate progress value." },
      { name: "max", type: "number", default: "100", description: "Maximum progress value." },
      { name: "message", type: "string | null", default: "null", description: "Loading message displayed below the spinner." },
      { name: "canCancel", type: "boolean", default: "false", description: "Whether the user can cancel the loading operation." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the loading indicator." },
    ],
    slots: [],
    events: [
      { name: "cancel", payload: "void", description: "Fires when the cancel button is clicked." },
    ],
    usage: `<script lang="ts">
  import { PageLoading } from "@poodle/svelte-composites";
</script>

<PageLoading visible presentation="inline" message="Loading your data..." />

<PageLoading visible message="Loading your data..." canCancel on:cancel={() => abortRequest()} />`,
  },

  "list-container": {
    props: [
      { name: "title", type: "string", required: true, description: "Primary list title." },
      { name: "subtitle", type: "string | null", default: "null", description: "Optional supporting text shown under the title." },
      { name: "eyebrow", type: "string | null", default: "null", description: "Eyebrow label rendered above the title." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the list region." },
      { name: "state", type: '"ready" | "loading" | "error" | "empty"', default: '"ready"', description: "Current list state." },
      { name: "loadingMessage", type: "string | null", default: '"Loading items..."', description: "Fallback loading message when no loading slot is provided." },
      { name: "errorTitle", type: "string | null", default: '"Unable to load list"', description: "Fallback error title when no error slot is provided." },
      { name: "errorMessage", type: "string | null", default: "null", description: "Fallback error message when no error slot is provided." },
      { name: "emptyTitle", type: "string | null", default: '"Nothing here yet"', description: "Fallback empty-state title when no empty slot is provided." },
      { name: "emptyMessage", type: "string | null", default: "null", description: "Fallback empty-state message when no empty slot is provided." },
      { name: "emptyVariant", type: '"neutral" | "search" | "firstRun"', default: '"neutral"', description: "Fallback empty-state visual treatment." },
      { name: "currentPage", type: "number", default: "1", description: "Current page number for built-in pagination." },
      { name: "totalPages", type: "number", default: "1", description: "Total page count for built-in pagination." },
      { name: "totalItems", type: "number | null", default: "null", description: "Total item count for the built-in pagination summary." },
      { name: "pageSize", type: "number | null", default: "null", description: "Items per page for the built-in pagination summary." },
      { name: "siblingCount", type: "number", default: "1", description: "Visible sibling pages around the current page." },
      { name: "paginationAriaLabel", type: "string | null", default: "null", description: "Accessible label for the built-in pagination nav." },
      { name: "showPagination", type: "boolean", default: "true", description: "Whether the built-in pagination footer is shown when multiple pages exist." },
      { name: "showPaginationSummary", type: "boolean", default: "true", description: "Whether the built-in pagination summary is shown alongside pagination controls." },
    ],
    slots: [
      { name: "breadcrumbs", description: "Optional breadcrumbs rendered in the header." },
      { name: "actions", description: "Header actions rendered next to the title block." },
      { name: "filters", description: "Filter controls rendered between the header and content." },
      { name: "batch", description: "Batch-action strip rendered above the main content." },
      { name: "default", description: "Primary list content." },
      { name: "pagination", description: "Optional override for the full pagination footer." },
      { name: "loading", description: "Optional override for the loading state." },
      { name: "error", description: "Optional override for the error state." },
      { name: "empty", description: "Optional override for the empty state." },
    ],
    events: [
      { name: "pageChange", payload: "{ page: number }", description: "Fires when the built-in pagination or pagination summary changes page." },
    ],
    usage: `<script lang="ts">
  import { ListContainer } from "@poodle/svelte-composites";
  import { Button, Field, TextInput, Select } from "@poodle/svelte-primitives";

  let page = 1;
</script>

<ListContainer
  title="Workflow incidents"
  subtitle="Monitor, filter, and page through operational events."
  currentPage={page}
  totalPages={12}
  totalItems={120}
  pageSize={10}
  on:pageChange={(event) => (page = event.detail.page)}
>
  <svelte:fragment slot="actions">
    <Button>Add incident</Button>
  </svelte:fragment>

  <svelte:fragment slot="filters">
    <Field label="Search">
      <TextInput type="search" placeholder="Search incidents" />
    </Field>
  </svelte:fragment>

  <div>Your list content goes here.</div>
</ListContainer>`,
  },

  pagination: {
    props: [
      { name: "controller", type: "PaginationControllerLike | null", default: "null", description: "Optional controller object with currentPage, total, pageSize, prevPage(), nextPage(), goToPage(), setPageSize(). When provided, overrides individual props." },
      { name: "currentPage", type: "number | null", default: "null", description: "Active page number (1-indexed). Overridden by controller." },
      { name: "totalPages", type: "number | null", default: "null", description: "Total number of pages. Overridden by controller." },
      { name: "page", type: "number | null", default: "null", description: "Alias for currentPage — used with item-based pagination (page + limit + total)." },
      { name: "limit", type: "number | null", default: "null", description: "Page size. Used with page + total to compute totalPages." },
      { name: "total", type: "number | null", default: "null", description: "Total item count. Used with page + limit to compute totalPages and info row." },
      { name: "variant", type: '"numbered" | "full" | "simple"', default: '"numbered"', description: "Navigation layout. numbered: page buttons with ellipsis. full: first/last + \"Page X of Y\" summary. simple: \"X-Y of Z\" range with Prev/Next." },
      { name: "siblingCount", type: "number", default: "1", description: "Number of sibling page buttons shown around the current page (numbered variant)." },
      { name: "showInfo", type: "boolean", default: "true", description: "Show \"Showing X to Y of Z\" info row above the controls." },
      { name: "showLimitSelector", type: "boolean", default: "false", description: "Show a page size dropdown (\"Show N per page\")." },
      { name: "limitOptions", type: "number[]", default: "[30, 50, 100]", description: "Options for the page size dropdown." },
      { name: "compact", type: "boolean", default: "false", description: "Tighter padding and gap. Hides the info row." },
      { name: "standalone", type: "boolean", default: "false", description: "Removes container padding, border-top, and background for embedding in custom layouts." },
      { name: "loading", type: "boolean", default: "false", description: "Shows loading state (reduced opacity, pointer-events disabled). Overridden by controller.loading." },
      { name: "scrollTarget", type: "HTMLElement | string | false", default: "false", description: "Element or CSS selector to scroll into view after page change. false disables." },
      { name: "scrollOffset", type: "number", default: "16", description: "Pixel offset from scroll container top when scrolling to target." },
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the pagination nav element." },
      { name: "className", type: "string", default: '""', description: "Additional CSS class on the root element." },
    ],
    slots: [],
    events: [
      { name: "pageChange", payload: "{ page: number }", description: "Fires when the current page changes. When a controller is provided, the controller method is called instead." },
      { name: "limitChange", payload: "{ limit: number }", description: "Fires when the page size changes via the limit selector." },
    ],
    usage: `<script lang="ts">
  import { Pagination } from "@poodle/svelte-primitives";

  let page = 1;
  let limit = 25;
</script>

<!-- Basic numbered pagination -->
<Pagination currentPage={page} totalPages={10} on:pageChange={(e) => (page = e.detail.page)} />

<!-- Simple variant with page size selector -->
<Pagination
  {page}
  {limit}
  total={248}
  variant="simple"
  showLimitSelector
  limitOptions={[10, 25, 50, 100]}
  on:pageChange={(e) => (page = e.detail.page)}
  on:limitChange={(e) => { limit = e.detail.limit; page = 1; }}
/>

<!-- Standalone (no container chrome) -->
<Pagination currentPage={1} totalPages={10} standalone />`,
  },

  "pagination-summary": {
    props: [
      { name: "currentPage", type: "number", default: "1", description: "Currently active page number." },
      { name: "totalPages", type: "number", default: "1", description: "Total number of pages." },
      { name: "totalItems", type: "number", default: "0", description: "Total number of items across all pages." },
      { name: "pageSize", type: "number", default: "5", description: "Number of items per page." },
    ],
    slots: [],
    events: [
      { name: "pageChange", payload: "{ page: number }", description: "Fires when the page changes." },
    ],
    usage: `<script lang="ts">
  import { PaginationSummary } from "@poodle/svelte-primitives";
</script>

<PaginationSummary currentPage={1} totalPages={10} totalItems={50} pageSize={5} />`,
  },

  "picker-shell": {
    props: [
      { name: "title", type: "string", required: true, description: "Title displayed at the top of the picker." },
      { name: "description", type: "string | null", default: "null", description: "Description text below the title." },
      { name: "variant", type: "PickerVariant", default: '"inline"', description: "Visual variant of the picker layout." },
      { name: "state", type: "BrowseState", default: '"ready"', description: "Loading state of the picker content." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the picker." },
      { name: "resultCount", type: "number | null", default: "null", description: "Total number of results found." },
      { name: "selectionCount", type: "number", default: "0", description: "Number of currently selected items." },
      { name: "stateTitle", type: "string | null", default: "null", description: "Title shown during loading/error states." },
      { name: "stateMessage", type: "string | null", default: "null", description: "Message shown during loading/error states." },
      { name: "statusText", type: "string | null", default: "null", description: "Status text displayed in the picker footer." },
      { name: "statusId", type: "string | null", default: "null", description: "ID for the status element for ARIA references." },
    ],
    slots: [
      { name: "toolbar", description: "Toolbar content for search and filter controls." },
      { name: "selection", description: "Selection summary display area." },
      { name: "default", description: "Main body content with browsable items." },
      { name: "state", description: "Custom state content for loading/error views." },
      { name: "footer", description: "Footer content with actions." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { PickerShell } from "@poodle/svelte-composites";
</script>

<PickerShell title="Select Items" description="Browse and pick items" resultCount={42} selectionCount={3}>
  <svelte:fragment slot="toolbar">
    <input type="search" placeholder="Search..." />
  </svelte:fragment>
  <p>Item list goes here.</p>
</PickerShell>`,
  },

  pill: {
    props: [
      { name: "tone", type: "PillTone", default: '"neutral"', description: "Color tone of the pill." },
      { name: "appearance", type: "PillAppearance", default: '"solid"', description: "Visual appearance variant." },
      { name: "size", type: "PillSize", default: '"md"', description: "Size of the pill." },
      { name: "font", type: "PillFont", default: '"normal"', description: "Font style of the pill label." },
      { name: "muted", type: "boolean", default: "false", description: "Whether the pill uses muted styling." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the pill." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
    ],
    slots: [
      { name: "default", description: "Pill label content." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { Pill } from "@poodle/svelte-primitives";
</script>

<Pill tone="success" appearance="solid">Active</Pill>
<Pill tone="warning" appearance="solid" muted>Pending</Pill>`,
  },

  "code-input": {
    props: [
      { name: "id", type: "string | null", default: "null", description: "Optional control id. Falls back to a derived id from the name when omitted." },
      { name: "value", type: "string | null", default: "null", description: "Controlled code value." },
      { name: "defaultValue", type: "string", default: '""', description: "Initial value for uncontrolled mode." },
      { name: "name", type: "string", default: '"code"', description: "Hidden input name for form submission." },
      { name: "label", type: "string", default: '"Authenticator code"', description: "Field label rendered by the built-in Field wrapper." },
      { name: "hint", type: "string | null", default: "null", description: "Optional help text below the label." },
      { name: "error", type: "string | null", default: "null", description: "Optional error message; also drives invalid validation state." },
      { name: "disabled", type: "boolean", default: "false", description: "Disables the hidden real input and visual slot interaction." },
      { name: "length", type: "number", default: "6", description: "Number of visible digit slots and max code length." },
      { name: "mask", type: "boolean", default: "false", description: "When true, display dots instead of digits in the visual slots." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Optional accessible label override for the grouped control." },
      { name: "autocomplete", type: "string", default: '"one-time-code"', description: "Autocomplete hint for verification-code autofill." },
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "validationState", type: 'ValidationState', default: '"none"', description: "Field-level validation state. Invalid is also inferred automatically when error is present." },
    ],
    slots: [],
    events: [
      { name: "valueChange", payload: "{ value: string }", description: "Fires whenever the sanitized code value changes." },
      { name: "complete", payload: "{ value: string }", description: "Fires when the value reaches the configured length." },
    ],
    usage: `<script lang="ts">
  import { CodeInput } from "@poodle/svelte-primitives";

  let code = "";
</script>

<CodeInput
  id="verify-code"
  value={code}
  label="Verification code"
  hint="Enter the 6-digit code from your authenticator app."
  on:valueChange={(e) => (code = e.detail.value)}
  on:complete={(e) => verify(e.detail.value)}
/>`,
  },

  "password-requirements": {
    props: [
      { name: "password", type: "string", default: '""', description: "Current password value to evaluate against the supplied rules." },
      { name: "requirements", type: "PasswordRequirementsPolicy | null", default: "null", description: "Caller-supplied password policy rules. When omitted, the component renders loading or error state instead of the checklist." },
      { name: "loading", type: "boolean", default: "false", description: "Shows the loading label instead of the checklist." },
      { name: "error", type: "string | null", default: "null", description: "Optional inline error message shown when requirements are unavailable." },
      { name: "title", type: "string", default: '"Password requirements"', description: "Heading shown above the checklist." },
      { name: "hint", type: "string | null", default: '"Avoid common words, patterns, and personal information."', description: "Optional supporting text shown below the checklist." },
      { name: "loadingLabel", type: "string", default: '"Loading requirements..."', description: "Text shown while requirements are loading." },
    ],
    slots: [],
    events: [],
    usage: `<script lang="ts">
  import { PasswordRequirements } from "@poodle/svelte-primitives";

  let password = "";

  const requirements = {
    minLength: 12,
    requireMixedCase: true,
    requireDigit: true,
    requireSpecial: true,
    description: "Use a long password with varied character types."
  };
</script>

<PasswordRequirements
  {password}
  {requirements}
/>`,
  },

  popover: {
    props: [
      { name: "open", type: "boolean | null", default: "null", description: "Controlled open state." },
      { name: "defaultOpen", type: "boolean", default: "false", description: "Initial open state for uncontrolled mode." },
      { name: "placement", type: "OverlayPlacement", default: '"bottom-start"', description: "Preferred placement of the popover relative to its trigger." },
      { name: "offset", type: "number", default: "8", description: "Pixel offset from the trigger element." },
      { name: "dismissOnOutsideInteract", type: "boolean", default: "true", description: "Whether clicking outside dismisses the popover." },
      { name: "initialFocus", type: "PopoverInitialFocus", default: '"first-focusable"', description: "Which element receives focus when the popover opens." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the popover." },
    ],
    slots: [
      { name: "trigger", description: "Trigger element that toggles the popover." },
      { name: "default", description: "Content rendered inside the popover." },
    ],
    events: [
      { name: "openChange", payload: "{ open: boolean }", description: "Fires when the open state changes." },
    ],
    usage: `<script lang="ts">
  import { Popover, Button } from "@poodle/svelte-primitives";
</script>

<Popover placement="bottom-start">
  <Button slot="trigger">Open Popover</Button>
  <div style="padding: 1rem;">
    <p>Popover content goes here.</p>
  </div>
</Popover>`,
  },

  progress: {
    props: [
      { name: "value", type: "number | null", default: "null", description: "Current progress value." },
      { name: "max", type: "number", default: "100", description: "Maximum progress value." },
      { name: "indeterminate", type: "boolean", default: "false", description: "Whether the progress is indeterminate (no specific value, shows animated bar)." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the progress bar." },
      { name: "size", type: 'ControlSize | null', default: "null", description: "Explicit height override for the progress bar." },
      { name: "sizeRole", type: 'SemanticControlSizeRole', default: '"control"', description: "Semantic size role used when inheriting from presentation context." },
      { name: "valueText", type: "string | null", default: "null", description: "Human-readable text for the current value." },
    ],
    slots: [],
    events: [],
    usage: `<script lang="ts">
  import { Progress } from "@poodle/svelte-primitives";
</script>

<Progress value={65} max={100} size="sm" ariaLabel="Upload progress" />
<Progress indeterminate ariaLabel="Loading..." />`,
  },

  "radio-group": {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "value", type: "string | null", default: "null", description: "Controlled selected value." },
      { name: "defaultValue", type: "string | null", default: "null", description: "Initial value for uncontrolled mode." },
      { name: "options", type: "RadioGroupOption[]", default: "[]", description: "Array of radio option definitions." },
      { name: "orientation", type: "Orientation", default: '"vertical"', description: "Layout orientation of the radio options." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the group is disabled." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the radio group." },
      { name: "describedBy", type: "string | null", default: "null", description: "ID of the element describing this group." },
      { name: "name", type: "string | undefined", default: "undefined", description: "Form field name." },
      { name: "selectedColor", type: "string | null", default: "null", description: "Optional selected-state color override used for the selected indicator border and dot." },
    ],
    slots: [],
    events: [
      { name: "valueChange", payload: "{ value: string }", description: "Fires when the selected value changes." },
    ],
    usage: `<script lang="ts">
  import { RadioGroup } from "@poodle/svelte-primitives";

  const options = [
    { value: "sm", label: "Small" },
    { value: "md", label: "Medium" },
    { value: "lg", label: "Large" },
  ];

  let size = "md";
</script>

<RadioGroup {options} bind:value={size} orientation="vertical" selectedColor="#22c55e" />`,
  },

  "range-slider": {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "value", type: "[number, number]", default: "[0, 100]", description: "Controlled range value as [min, max] tuple." },
      { name: "min", type: "number", default: "0", description: "Minimum allowed value." },
      { name: "max", type: "number", default: "100", description: "Maximum allowed value." },
      { name: "step", type: "number", default: "1", description: "Step increment between values." },
      { name: "orientation", type: "Orientation", default: '"horizontal"', description: "Layout orientation of the slider." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the slider is disabled." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the range slider." },
      { name: "lowerValueText", type: "string | null", default: "null", description: "Human-readable text for the lower thumb value." },
      { name: "upperValueText", type: "string | null", default: "null", description: "Human-readable text for the upper thumb value." },
    ],
    slots: [],
    events: [
      { name: "valueChange", payload: "{ value: [number, number] }", description: "Fires when the range value changes during drag." },
      { name: "valueCommit", payload: "{ value: [number, number] }", description: "Fires when the range value is committed (drag end)." },
    ],
    usage: `<script lang="ts">
  import { RangeSlider } from "@poodle/svelte-primitives";

  let priceRange: [number, number] = [20, 80];
</script>

<RangeSlider bind:value={priceRange} min={0} max={100} step={5} />`,
  },

  rating: {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "value", type: "number | null", default: "null", description: "Controlled rating value." },
      { name: "defaultValue", type: "number | null", default: "null", description: "Initial value for uncontrolled mode." },
      { name: "max", type: "number", default: "5", description: "Maximum number of stars." },
      { name: "allowClear", type: "boolean", default: "false", description: "Whether clicking the current value clears the rating." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the rating input is disabled." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the rating." },
    ],
    slots: [],
    events: [
      { name: "valueChange", payload: "{ value: number | null }", description: "Fires when the rating value changes." },
    ],
    usage: `<script lang="ts">
  import { Rating } from "@poodle/svelte-primitives";

  let stars: number | null = 3;
</script>

<Rating bind:value={stars} max={5} allowClear />`,
  },

  region: {
    props: [
      { name: "label", type: "string", default: '""', description: "Label text displayed inside the region placeholder." },
      { name: "color", type: "string | null", default: "null", description: "Border and label color." },
      { name: "minHeight", type: "string", default: '"4rem"', description: "Minimum height of the region block." },
    ],
    slots: [
      { name: "default", description: "Content rendered inside the region." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { Region } from "@poodle/svelte-primitives";
</script>

<Region label="Sidebar" color="blue" minHeight="200px" />`,
  },

  "relation-picker": {
    props: [
      { name: "title", type: "string", default: '"Select items"', description: "Title of the picker." },
      { name: "description", type: "string | null", default: "null", description: "Description text below the title." },
      { name: "items", type: "PickerItem[]", default: "[]", description: "Array of selectable items." },
      { name: "selectedIds", type: "string[]", default: "[]", description: "Array of currently selected item IDs." },
      { name: "query", type: "string", default: '""', description: "Current search query." },
      { name: "selectionMode", type: "SelectionMode", default: '"multiple"', description: "Whether single or multiple items can be selected." },
      { name: "variant", type: "PickerVariant", default: '"inline"', description: "Visual variant of the picker." },
      { name: "state", type: "BrowseState", default: '"ready"', description: "Loading state of the picker." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the picker." },
      { name: "confirmLabel", type: "string", default: '"Confirm selection"', description: "Label for the confirm button." },
      { name: "cancelLabel", type: "string", default: '"Cancel"', description: "Label for the cancel button." },
      { name: "drillDown", type: "DrillDownConfig | null", default: "null", description: "Configuration for drill-down navigation." },
      { name: "size", type: 'ControlSize | null', default: "null", description: "Explicit semantic control size override for picker actions and drill-down controls." },
      { name: "sizeRole", type: 'SemanticControlSizeRole', default: '"control"', description: "Semantic size role used when inheriting from presentation context." },
      { name: "density", type: 'ControlDensity | null', default: "null", description: "Explicit density override for breadcrumb and drill-list spacing." },
    ],
    slots: [
      { name: "state", description: "Custom state content for loading/error views." },
    ],
    events: [
      { name: "queryChange", payload: "{ value: string }", description: "Fires when the search query changes." },
      { name: "selectionChange", payload: "{ selectedIds: string[] }", description: "Fires when the selection changes." },
      { name: "confirm", payload: "{ selectedIds: string[] }", description: "Fires when the selection is confirmed." },
      { name: "cancel", payload: "void", description: "Fires when the picker is cancelled." },
      { name: "drillContext", payload: "{ context: DrillDownContext }", description: "Fires when a drill-down context changes." },
    ],
    usage: `<script lang="ts">
  import { RelationPicker } from "@poodle/svelte-composites";

  const items = [
    { id: "1", label: "Alice" },
    { id: "2", label: "Bob" },
    { id: "3", label: "Charlie" },
  ];

  let selected: string[] = [];
</script>

<RelationPicker title="Select Users" {items} bind:selectedIds={selected} on:confirm={(e) => save(e.detail.selectedIds)} />`,
  },

  "reorderable-list": {
    props: [
      { name: "items", type: "ReorderableItem[]", default: "[]", description: "Array of reorderable items." },
      { name: "ariaLabel", type: "string", default: '"Reorderable list"', description: "Accessible label for the list." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether reordering is disabled." },
      { name: "dirty", type: "boolean", default: "false", description: "Whether the current order differs from the committed baseline." },
      { name: "submitting", type: "boolean", default: "false", description: "Whether submit is in progress; disables interaction and shows saving state." },
      { name: "errorMessage", type: "string | null", default: "null", description: "Optional submit error surfaced above the list." },
      { name: "infoMessage", type: "string | null", default: "null", description: "Optional workflow guidance surfaced above the list." },
      { name: "longListThreshold", type: "number | null", default: "50", description: "Shows the built-in large-list guidance when item count exceeds the threshold. Pass null or 0 to disable." },
      { name: "longListWarningText", type: "string | null", default: "null", description: "Optional custom copy for the large-list guidance panel." },
      { name: "windowSize", type: "number | null", default: "null", description: "Optional page window size for very large reorder sessions." },
      { name: "submitLabel", type: "string", default: '"Save Order"', description: "Submit button label when workflow chrome is shown." },
      { name: "cancelLabel", type: "string", default: '"Cancel"', description: "Cancel button label when workflow chrome is shown." },
      { name: "onsubmit", type: "() => void | Promise<void>", default: "null", description: "Optional submit callback; enables workflow chrome." },
      { name: "oncancel", type: "() => void", default: "null", description: "Optional cancel callback; enables workflow chrome." },
      { name: "size", type: 'ControlSize | null', default: "null", description: "Explicit semantic control size override for row and handle geometry." },
      { name: "sizeRole", type: 'SemanticControlSizeRole', default: '"control"', description: "Semantic size role used when inheriting from presentation context." },
      { name: "density", type: 'ControlDensity | null', default: "null", description: "Explicit density override for row padding and list spacing." },
    ],
    slots: [
      { name: "item", description: "Custom item renderer. Receives slot props: item, index." },
    ],
    events: [
      { name: "reorder", payload: "{ items: ReorderableItem[] }", description: "Fires when items are reordered." },
      { name: "submit", payload: "void", description: "Fires when the submit action is triggered." },
      { name: "cancel", payload: "void", description: "Fires when the cancel action is triggered." },
    ],
    usage: `<script lang="ts">
  import { ReorderableList } from "@poodle/svelte-composites";

  let items = [
    { id: "1", label: "First" },
    { id: "2", label: "Second" },
    { id: "3", label: "Third" },
  ];
  let baseline = items.map((item) => item.id);
  $: dirty = items.some((item, index) => item.id !== baseline[index]);

  async function saveOrder() {
    baseline = items.map((item) => item.id);
  }
</script>

<ReorderableList
  bind:items
  {dirty}
  onsubmit={saveOrder}
  oncancel={() => (items = baseline.map((id) => items.find((item) => item.id === id)!))}
  on:reorder={(e) => (items = e.detail.items)}
/>`,
  },

  "resize-handle": {
    props: [
      { name: "orientation", type: "SplitOrientation", default: '"horizontal"', description: "Orientation of the resize handle." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the handle is disabled." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the resize handle." },
      { name: "ariaValueNow", type: "number | null", default: "null", description: "Current value for accessibility." },
      { name: "ariaValueMin", type: "number", default: "0", description: "Minimum value for accessibility." },
      { name: "ariaValueMax", type: "number", default: "100", description: "Maximum value for accessibility." },
    ],
    slots: [],
    events: [
      { name: "resizeStart", payload: "{ position: number }", description: "Fires when a resize drag begins." },
      { name: "resizeMove", payload: "{ delta: number }", description: "Fires during resize drag with the delta." },
      { name: "resizeEnd", payload: "{ position: number }", description: "Fires when a resize drag ends." },
      { name: "resizeStep", payload: "{ delta: number }", description: "Fires when resized via keyboard step." },
    ],
    usage: `<script lang="ts">
  import { ResizeHandle } from "@poodle/svelte-primitives";
</script>

<ResizeHandle orientation="horizontal" ariaLabel="Resize sidebar" ariaValueNow={50} />`,
  },

  "scroll-shell": {
    props: [
      { name: "direction", type: "ScrollDirection", default: '"vertical"', description: "Scroll direction of the container." },
      { name: "padding", type: "SpaceScale", default: '"none"', description: "Inner padding of the scroll area." },
      { name: "asRole", type: '"region" | "group" | null', default: "null", description: "Semantic ARIA role for the container." },
      { name: "label", type: "string | null", default: "null", description: "Accessible label for the scrollable region." },
      { name: "focusable", type: "boolean", default: "false", description: "Whether the scroll region is keyboard-focusable." },
    ],
    slots: [
      { name: "default", description: "Scrollable content." },
    ],
    events: [
      { name: "scroll", payload: "Event", description: "Fires when the container is scrolled." },
    ],
    usage: `<script lang="ts">
  import { ScrollShell } from "@poodle/svelte-primitives";
</script>

<ScrollShell direction="vertical" padding="md" style="max-height: 300px;">
  <p>Long scrollable content goes here...</p>
</ScrollShell>`,
  },

  "segmented-control": {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "value", type: "string | null", default: "null", description: "Controlled selected value." },
      { name: "defaultValue", type: "string | null", default: "null", description: "Initial value for uncontrolled mode." },
      { name: "options", type: "SegmentedControlOption[]", default: "[]", description: "Array of segment option definitions." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the control is disabled." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the segmented control." },
      { name: "name", type: "string | undefined", default: "undefined", description: "Form field name." },
      { name: "equalWidth", type: "boolean", default: "true", description: "Whether all segments are rendered with equal width." },
    ],
    slots: [],
    events: [
      { name: "valueChange", payload: "{ value: string }", description: "Fires when the selected segment changes." },
    ],
    usage: `<script lang="ts">
  import { SegmentedControl } from "@poodle/svelte-primitives";

  const options = [
    { value: "list", label: "List" },
    { value: "grid", label: "Grid" },
    { value: "board", label: "Board" },
  ];

  let view = "list";
</script>

<SegmentedControl {options} bind:value={view} />`,
  },

  select: {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "id", type: "string | undefined", default: "undefined", description: "HTML id attribute for the select." },
      { name: "value", type: "string | null", default: "null", description: "Controlled selected value." },
      { name: "defaultValue", type: "string | null", default: "null", description: "Initial value for uncontrolled mode." },
      { name: "placeholder", type: "string | null", default: "null", description: "Placeholder text when no option is selected." },
      { name: "options", type: "SelectItems", default: "[]", description: "Array of select options or grouped option objects." },
      { name: "items", type: "SelectItem[] | null", default: "null", description: "Legacy flat list of selectable items. Normalized into options internally." },
      { name: "groups", type: "SelectGroup[] | null", default: "null", description: "Legacy grouped items. Normalized into option groups internally." },
      { name: "loadItems", type: "(() => Promise<SelectItem[]>) | null", default: "null", description: "Async loader for flat options. Fetched automatically on mount." },
      { name: "loadGroups", type: "(() => Promise<SelectGroup[]>) | null", default: "null", description: "Async loader for grouped options. Fetched automatically on mount." },
      { name: "loadKey", type: "string | null", default: "null", description: "Cache-busting key for async loaders. Changing this value resets and re-fetches." },
      { name: "clearable", type: "boolean", default: "false", description: "Whether a clear option is shown to reset the value." },
      { name: "required", type: "boolean", default: "false", description: "Whether the native select has the required attribute." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the select is disabled." },
      { name: "valueLabel", type: "string | null", default: "null", description: "Display label for the current value when options have not loaded yet." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the select." },
      { name: "describedBy", type: "string | null", default: "null", description: "ID of the element describing this select." },
      { name: "name", type: "string | undefined", default: "undefined", description: "Form field name." },
      { name: "onchange", type: "((value: string) => void) | null", default: "null", description: "Direct callback fired on value change." },
      { name: "searchable", type: "boolean", default: "false", description: "Renders a custom dropdown with search/filter input instead of native <select>." },
      { name: "freeform", type: "boolean", default: "false", description: "With searchable, query text becomes the value if no option selected. Options act as suggestions." },
      { name: "native", type: "boolean | undefined", default: "undefined", description: "Explicit mode override. true = always native, false = always custom, undefined = auto." },
      { name: "emptyMessage", type: "string", default: '"No matches"', description: "Text shown when filter yields zero results (custom mode only)." },
      { name: "loadOptions", type: "SelectLoadOptions | null", default: "null", description: "Unified async loader returning flat or grouped options." },
    ],
    slots: [
      { name: "option", description: "Custom rendering for each option row. Receives: option, highlighted, selected, index. Presence forces custom mode." },
      { name: "trigger", description: "Custom trigger rendering. Receives: selectedOption, open, placeholder. Presence forces custom mode." },
      { name: "empty", description: "Custom empty-state for no matching options. Receives: query." },
    ],
    events: [
      { name: "valueChange", payload: "{ value: string }", description: "Fires when the selected value changes." },
      { name: "change", payload: "{ value: string }", description: "Alias for valueChange." },
      { name: "queryChange", payload: "{ query: string }", description: "Fires when the search query changes (searchable mode only)." },
      { name: "openChange", payload: "{ open: boolean }", description: "Fires when the dropdown opens/closes (custom mode only)." },
    ],
    usage: `<script lang="ts">
  import { Select } from "@poodle/svelte-primitives";

  const fruits = [
    { value: "apple", label: "Apple" },
    { value: "banana", label: "Banana" },
    { value: "cherry", label: "Cherry" },
  ];
</script>

<!-- Native (default) -->
<Select options={fruits} placeholder="Choose a fruit" />

<!-- Custom dropdown with rich options -->
<Select options={fruits} native={false} placeholder="Choose..." />

<!-- Searchable -->
<Select options={fruits} searchable placeholder="Search fruits..." />

<!-- Freeform autocomplete -->
<Select options={fruits} searchable freeform placeholder="Type or pick..." />

<!-- Custom option rendering -->
<Select options={countries} placeholder="Choose a country">
  <div slot="option" let:option let:selected>
    <span>{option.icon} {option.label}</span>
    {#if selected}<strong>✓</strong>{/if}
  </div>
</Select>`,
  },

  "selection-summary": {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "items", type: "Array<{ id: string; label: string }>", default: "[]", description: "Array of selected items to display." },
      { name: "selectionMode", type: '"single" | "multiple"', default: '"multiple"', description: "Selection mode." },
      { name: "maxVisibleItems", type: "number", default: "4", description: "Maximum number of items shown before truncation." },
    ],
    slots: [],
    events: [
      { name: "remove", payload: "{ id: string }", description: "Fires when an item is removed from the selection." },
      { name: "clear", payload: "void", description: "Fires when all items are cleared." },
    ],
    usage: `<script lang="ts">
  import { SelectionSummary } from "@poodle/svelte-composites";

  const items = [
    { id: "1", label: "Alice" },
    { id: "2", label: "Bob" },
    { id: "3", label: "Charlie" },
  ];
</script>

<SelectionSummary {items} maxVisibleItems={3} on:remove={(e) => deselect(e.detail.id)} on:clear={clearAll} />`,
  },

  separator: {
    props: [
      { name: "orientation", type: "Orientation", default: '"horizontal"', description: "Orientation of the separator line." },
      { name: "decorative", type: "boolean", default: "true", description: "Whether the separator is purely decorative (hides from accessibility tree)." },
      { name: "tone", type: "SeparatorTone", default: '"subtle"', description: "Visual tone of the separator." },
    ],
    slots: [],
    events: [],
    usage: `<script lang="ts">
  import { Separator } from "@poodle/svelte-primitives";
</script>

<p>Section one content</p>
<Separator />
<p>Section two content</p>`,
  },

  "sidebar-nav": {
    props: [
      { name: "groups", type: "SidebarNavGroup[]", default: "[]", description: "Array of navigation groups, each containing a list of nav items." },
      { name: "value", type: "string | null", default: "null", description: "Currently active navigation item value." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the nav landmark." },
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"chrome"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
    ],
    slots: [],
    events: [
      { name: "valueChange", payload: "{ value: string }", description: "Fires when a navigation item is activated." },
    ],
    usage: `<script lang="ts">
  import { SidebarNav } from "@poodle/svelte-composites";

  const groups = [
    {
      id: "main",
      label: "Navigation",
      items: [
        { value: "dashboard", label: "Dashboard" },
        { value: "projects", label: "Projects", href: "/projects" },
        { value: "settings", label: "Settings" },
      ],
    },
  ];

  let active = "dashboard";
</script>

<SidebarNav {groups} value={active} ariaLabel="Main navigation" on:valueChange={(e) => (active = e.detail.value)} />`,
  },

  skeleton: {
    props: [
      { name: "shape", type: "SkeletonShape", default: '"line"', description: "Shape of the skeleton placeholder." },
      { name: "preset", type: "SkeletonPreset | null", default: "null", description: "Predefined skeleton layout preset." },
      { name: "width", type: "string | null", default: "null", description: "CSS width of the skeleton." },
      { name: "height", type: "string | null", default: "null", description: "CSS height of the skeleton." },
      { name: "lines", type: "number", default: "3", description: "Number of lines when shape is 'line'." },
      { name: "animated", type: "boolean", default: "true", description: "Whether the skeleton pulses with animation." },
    ],
    slots: [],
    events: [],
    usage: `<script lang="ts">
  import { Skeleton } from "@poodle/svelte-primitives";
</script>

<Skeleton shape="circle" width="48px" height="48px" />
<Skeleton shape="line" lines={3} />`,
  },

  spinner: {
    props: [
      { name: "variant", type: "SpinnerVariant", default: '"ring"', description: "Visual style of the spinner." },
      { name: "size", type: "SpinnerSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "tone", type: "SpinnerTone", default: '"current"', description: "Color source for the spinner." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Optional announced loading label." },
      { name: "className", type: "string", default: '""', description: "Additional CSS class name applied to the spinner element." },
      { name: "style", type: "string | null", default: "null", description: "Custom inline style applied to the spinner element." },
    ],
    slots: [],
    events: [],
    usage: `<script lang="ts">
  import { Spinner } from "@poodle/svelte-primitives";
</script>

<Spinner variant="ring" sizeRole="chrome" tone="current" />
<Spinner variant="grid" size="lg" tone="accent" ariaLabel="Loading logs" />`,
  },

  slider: {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "value", type: "number", default: "0", description: "Current slider value." },
      { name: "min", type: "number", default: "0", description: "Minimum allowed value." },
      { name: "max", type: "number", default: "100", description: "Maximum allowed value." },
      { name: "step", type: "number", default: "1", description: "Step increment between values." },
      { name: "orientation", type: "Orientation", default: '"horizontal"', description: "Layout orientation of the slider." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the slider is disabled." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the slider." },
      { name: "valueText", type: "string | null", default: "null", description: "Human-readable text for the current value." },
    ],
    slots: [],
    events: [
      { name: "valueChange", payload: "{ value: number }", description: "Fires when the slider value changes during drag." },
      { name: "valueCommit", payload: "{ value: number }", description: "Fires when the slider value is committed (drag end)." },
    ],
    usage: `<script lang="ts">
  import { Slider } from "@poodle/svelte-primitives";

  let volume = 50;
</script>

<Slider bind:value={volume} min={0} max={100} ariaLabel="Volume" />`,
  },

  spacer: {
    props: [
      { name: "grow", type: "number", default: "1", description: "Flex grow factor." },
      { name: "minSize", type: "string | null", default: "null", description: "Minimum size of the spacer." },
    ],
    slots: [],
    events: [],
    usage: `<script lang="ts">
  import { Spacer } from "@poodle/svelte-primitives";
</script>

<div style="display: flex;">
  <span>Left</span>
  <Spacer />
  <span>Right</span>
</div>`,
  },

  "split-button": {
    props: [
      { name: "variant", type: "ButtonVariant", default: '"secondary"', description: "Visual variant of the button." },
      { name: "tone", type: "ButtonTone", default: '"default"', description: "Color tone of the button." },
      { name: "size", type: "ControlSize", default: '"md"', description: "Size of the button." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "type", type: '"button" | "submit" | "reset"', default: '"button"', description: "Native button type for the primary action half." },
      { name: "items", type: "MenuItem[]", default: "[]", description: "Array of menu items for the dropdown." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the button is disabled." },
      { name: "loading", type: "boolean", default: "false", description: "Whether the button shows a loading spinner." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the primary button." },
      { name: "menuAriaLabel", type: "string", default: '"More actions"', description: "Accessible label for the dropdown menu." },
    ],
    slots: [
      { name: "default", description: "Primary button label content." },
    ],
    events: [
      { name: "click", payload: "MouseEvent", description: "Fires when the primary button is clicked." },
      { name: "action", payload: "{ value: string }", description: "Fires when a dropdown menu item is selected." },
    ],
    usage: `<script lang="ts">
  import { SplitButton } from "@poodle/svelte-primitives";

  const items = [
    { value: "save-draft", label: "Save as Draft" },
    { value: "schedule", label: "Schedule" },
  ];
</script>

<SplitButton type="submit" variant="primary" {items} on:click={() => publish()} on:action={(e) => handleAction(e.detail.value)}>
  Publish
</SplitButton>`,
  },

  "split-view": {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"chrome"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "orientation", type: "SplitOrientation", default: '"horizontal"', description: "Orientation of the split layout." },
      { name: "ratio", type: "number", default: "0.5", description: "Controlled split ratio (0 to 1)." },
      { name: "defaultRatio", type: "number", default: "0.5", description: "Initial split ratio for uncontrolled mode." },
      { name: "minPrimarySize", type: "number | null", default: "null", description: "Minimum pixel size of the primary pane." },
      { name: "minSecondarySize", type: "number | null", default: "null", description: "Minimum pixel size of the secondary pane." },
      { name: "primarySize", type: "number | null", default: "null", description: "Fixed pixel size for the primary pane. When set, the primary pane uses this size and the secondary fills the remaining space." },
      { name: "secondarySize", type: "number | null", default: "null", description: "Fixed pixel size for the secondary pane. When set, the secondary pane uses this size and the primary fills the remaining space." },
      { name: "primaryCollapsed", type: "boolean", default: "false", description: "Whether the primary pane is collapsed." },
      { name: "secondaryCollapsed", type: "boolean", default: "false", description: "Whether the secondary pane is collapsed." },
      { name: "showCollapsePrimary", type: "boolean", default: "false", description: "Whether to show the primary collapse toggle." },
      { name: "showCollapseSecondary", type: "boolean", default: "false", description: "Whether to show the secondary collapse toggle." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the split view." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether resizing is disabled." },
    ],
    slots: [
      { name: "primary", description: "Content for the primary (left or top) pane." },
      { name: "secondary", description: "Content for the secondary (right or bottom) pane." },
    ],
    events: [
      { name: "ratioChange", payload: "{ ratio: number }", description: "Fires when the split ratio changes." },
      { name: "primaryCollapsedChange", payload: "{ isCollapsed: boolean }", description: "Fires when the primary pane collapse state changes." },
      { name: "secondaryCollapsedChange", payload: "{ isCollapsed: boolean }", description: "Fires when the secondary pane collapse state changes." },
    ],
    usage: `<script lang="ts">
  import { SplitView } from "@poodle/svelte-composites";
</script>

<SplitView orientation="horizontal" defaultRatio={0.3} showCollapsePrimary>
  <svelte:fragment slot="primary">
    <p>Sidebar content</p>
  </svelte:fragment>
  <svelte:fragment slot="secondary">
    <p>Main content</p>
  </svelte:fragment>
</SplitView>`,
  },

  stack: {
    props: [
      { name: "direction", type: '"column" | "row"', default: '"column"', description: "Flex direction of the stack." },
      { name: "gap", type: "SpaceScale", default: '"md"', description: "Gap between child elements." },
      { name: "align", type: "LayoutAlign", default: '"stretch"', description: "Cross-axis alignment of children." },
      { name: "justify", type: "LayoutJustify", default: '"start"', description: "Main-axis justification of children." },
      { name: "wrap", type: "boolean", default: "false", description: "Whether children wrap to new lines." },
      { name: "padding", type: "SpaceScale", default: '"none"', description: "Inner padding of the stack." },
      { name: "asRole", type: "string | null", default: "null", description: "Semantic ARIA role for the container." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the stack." },
    ],
    slots: [
      { name: "default", description: "Child elements laid out in the stack." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { Stack, Button } from "@poodle/svelte-primitives";
</script>

<Stack direction="row" gap="sm" align="center">
  <Button variant="primary">Save</Button>
  <Button variant="secondary">Cancel</Button>
</Stack>`,
  },

  "status-bar": {
    props: [
      { name: "summary", type: "string | null", default: "null", description: "Summary text displayed in the center." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the status bar." },
    ],
    slots: [
      { name: "leading", description: "Content on the left side of the status bar." },
      { name: "trailing", description: "Content on the right side of the status bar." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { StatusBar } from "@poodle/svelte-primitives";
</script>

<StatusBar summary="3 items selected">
  <svelte:fragment slot="leading">Ready</svelte:fragment>
  <svelte:fragment slot="trailing">Ln 42, Col 8</svelte:fragment>
</StatusBar>`,
  },

  "status-indicator": {
    props: [
      { name: "status", type: "StatusTone", default: '"neutral"', description: "Status tone controlling the indicator color." },
      { name: "label", type: "string | null", default: "null", description: "Label text displayed next to the indicator." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the indicator." },
    ],
    slots: [
      { name: "default", description: "Custom label content when label prop is not provided." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { StatusIndicator } from "@poodle/svelte-primitives";
</script>

<StatusIndicator status="success" label="Online" />
<StatusIndicator status="danger" label="Offline" />
<StatusIndicator status="warning">Degraded</StatusIndicator>`,
  },

  surface: {
    props: [
      { name: "tone", type: "SurfaceTone", default: '"panel"', description: "Background tone of the surface." },
      { name: "border", type: "SurfaceBorder", default: '"subtle"', description: "Border style of the surface." },
      { name: "padding", type: "SpaceScale", default: '"md"', description: "Inner padding of the surface." },
      { name: "elevated", type: "boolean", default: "false", description: "Whether the surface has a box-shadow elevation." },
      { name: "asRole", type: '"region" | "group" | null', default: "null", description: "Semantic ARIA role for the container." },
      { name: "label", type: "string | null", default: "null", description: "Accessible label for the surface." },
    ],
    slots: [
      { name: "default", description: "Content rendered inside the surface." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { Surface } from "@poodle/svelte-primitives";
</script>

<Surface tone="panel" border="subtle" padding="lg" elevated>
  <p>Content in an elevated panel surface.</p>
</Surface>`,
  },

  switch: {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "id", type: "string | undefined", default: "undefined", description: "HTML id attribute for the switch." },
      { name: "checked", type: "boolean | null", default: "null", description: "Controlled checked state." },
      { name: "defaultChecked", type: "boolean", default: "false", description: "Initial checked state for uncontrolled mode." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the switch is disabled." },
      { name: "readOnly", type: "boolean", default: "false", description: "Whether the switch is read-only." },
      { name: "label", type: "string | null", default: "null", description: "Label text displayed next to the switch." },
      { name: "leftLabel", type: "string | null", default: "null", description: "Label displayed to the left of the track for dual-label mode." },
      { name: "rightLabel", type: "string | null", default: "null", description: "Label displayed to the right of the track for dual-label mode." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label when no visible label is used." },
      { name: "describedBy", type: "string | null", default: "null", description: "ID of the element describing this switch." },
      { name: "name", type: "string | undefined", default: "undefined", description: "Form field name." },
      { name: "offColor", type: "string | null", default: "null", description: "Optional off-state accent override used for the thumb and muted track tint." },
      { name: "onColor", type: "string | null", default: "null", description: "Optional on-state accent override used for the thumb and active track tint." },
      { name: "leftTone", type: "SwitchTone", default: '"default"', description: "Color tone for the left (off) state. Supports default, primary, success, warning, and danger." },
      { name: "rightTone", type: "SwitchTone", default: '"primary"', description: "Color tone for the right (on) state. Supports default, primary, success, warning, and danger." },
    ],
    slots: [],
    events: [
      { name: "checkedChange", payload: "{ checked: boolean }", description: "Fires when the checked state changes." },
    ],
    usage: `<script lang="ts">
  import { Switch } from "@poodle/svelte-primitives";

  let notifications = true;
</script>

<Switch
  label="Enable notifications"
  bind:checked={notifications}
  onColor="#22c55e"
  offColor="#cbd5e1"
/>`,
  },

  table: {
    props: [
      { name: "columns", type: "TableColumn[]", default: "[]", description: "Column definitions for the table." },
      { name: "rows", type: "TableRow[]", default: "[]", description: "Row data to render in the table." },
      { name: "caption", type: "string | null", default: "null", description: "Visible caption for the table." },
      { name: "emptyMessage", type: "string", default: '"No rows available."', description: "Message shown when there are no rows." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the table." },
    ],
    slots: [],
    events: [],
    usage: `<script lang="ts">
  import { Table } from "@poodle/svelte-primitives";

  const columns = [
    { key: "name", label: "Name" },
    { key: "email", label: "Email" },
  ];
  const rows = [
    { name: "Alice", email: "alice@example.com" },
    { name: "Bob", email: "bob@example.com" },
  ];
</script>

<Table {columns} {rows} caption="Team members" />`,
  },

  tabs: {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"chrome"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "value", type: "string | null", default: "null", description: "Controlled active tab value." },
      { name: "defaultValue", type: "string | null", default: "null", description: "Initial active tab for uncontrolled mode." },
      { name: "items", type: "TabItem[]", default: "[]", description: "Array of tab items to render." },
      { name: "variant", type: "TabVariant", default: '"underline"', description: "Visual style variant of the tabs." },
      { name: "orientation", type: "Orientation", default: '"horizontal"', description: "Layout orientation of the tab list." },
      { name: "activationMode", type: "TabActivationMode", default: '"automatic"', description: "Whether tabs activate on focus or on click." },
      { name: "reorderable", type: "boolean", default: "false", description: "Whether tabs can be reordered via drag." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the tab list." },
      { name: "showTooltips", type: "boolean", default: "false", description: "Whether to show tooltips on tab triggers." },
      { name: "historyKey", type: "string | null", default: "null", description: "Optional URL query param key used to preserve the active tab in browser history." },
    ],
    slots: [
      { name: "default", description: "Tab panel content. Receives slot props: activeValue." },
      { name: "actions", description: "Actions rendered alongside the tab list." },
    ],
    events: [
      { name: "valueChange", payload: "{ value: string }", description: "Fires when the active tab changes." },
      { name: "reorder", payload: "{ items: string[] }", description: "Fires when tabs are reordered." },
      { name: "close", payload: "{ value: string }", description: "Fires when a tab close button is clicked." },
    ],
    usage: `<script lang="ts">
  import { Tabs } from "@poodle/svelte-primitives";

  const items = [
    { value: "overview", label: "Overview" },
    { value: "settings", label: "Settings", count: 3, separator: true },
  ];
</script>

<Tabs {items} defaultValue="overview" historyKey="tab">
  {#snippet default({ activeValue })}
    {#if activeValue === "overview"}
      <p>Overview content here.</p>
    {:else}
      <p>Settings content here.</p>
    {/if}
  {/snippet}
</Tabs>`,
  },

  "text-input": {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "id", type: "string", required: true, description: "HTML id attribute for the input." },
      { name: "value", type: "string | null", default: "null", description: "Controlled value of the input." },
      { name: "defaultValue", type: "string", default: '""', description: "Initial value for uncontrolled mode." },
      { name: "placeholder", type: "string | null", default: "null", description: "Placeholder text when empty." },
      { name: "name", type: "string | undefined", default: "undefined", description: "Form field name." },
      { name: "autocomplete", type: "string | undefined", default: "undefined", description: "Native autocomplete attribute." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the input is disabled." },
      { name: "readOnly", type: "boolean", default: "false", description: "Whether the input is read-only." },
      { name: "required", type: "boolean", default: "false", description: "Whether the native input is required." },
      { name: "pattern", type: "string | undefined", default: "undefined", description: "Native input pattern attribute." },
      { name: "spellcheck", type: "boolean | undefined", default: "undefined", description: "Native spellcheck attribute." },
      { name: "autocapitalize", type: "string | undefined", default: "undefined", description: "Native autocapitalize attribute." },
      { name: "enterKeyHint", type: '"enter" | "done" | "go" | "next" | "previous" | "search" | "send" | null', default: "null", description: "Native enterkeyhint attribute." },
      { name: "debounce", type: "number | null", default: "null", description: "Delay valueChange emission while typing." },
      { name: "validate", type: "InputValidator | undefined", default: "undefined", description: "App-owned validation function for sync or async checks." },
      { name: "validationContext", type: "unknown", default: "undefined", description: "Opaque context passed to validate." },
      { name: "validationDebounce", type: "number", default: "300", description: "Delay before validation runs while typing." },
      { name: "validateOnBlur", type: "boolean", default: "true", description: "Whether blur triggers immediate validation." },
      { name: "showValidationStatus", type: "boolean", default: "true", description: "Whether built-in validation status chrome is shown." },
      { name: "validationState", type: "ValidationState", default: '"none"', description: "Current validation state." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the input." },
      { name: "describedBy", type: "string | null", default: "null", description: "ID of the element describing this input." },
      { name: "inputMode", type: "string | null", default: "null", description: "Hint for virtual keyboard type." },
      { name: "type", type: 'HTMLInputElement["type"] | "multiline"', default: '"text"', description: "HTML input type attribute. Use \"multiline\" for multi-line textarea mode." },
      { name: "rows", type: "number | null", default: "null", description: "Number of visible text rows. When > 1 and type is not explicitly set, auto-switches to multiline mode." },
      { name: "resize", type: '"vertical" | "horizontal" | "both" | "none"', default: '"vertical"', description: "Resize behaviour for multiline mode." },
      { name: "prefix", type: "string | null", default: "null", description: "Static prefix text shown inside the input." },
      { name: "suffix", type: "string | null", default: "null", description: "Static suffix text shown inside the input." },
      { name: "maxLength", type: "number | null", default: "null", description: "Maximum character length." },
      { name: "showCharCount", type: "boolean", default: "false", description: "Whether to display a character count." },
      { name: "showClearButton", type: "boolean", default: "true", description: "When type=\"search\", whether to show the clear button when the input has a value. Only applies to search type." },
    ],
    slots: [
      { name: "leading", description: "Content rendered before the input (e.g. icon). When type=\"search\" and no leading slot is provided, a search icon is rendered automatically." },
      { name: "trailing", description: "Content rendered after the input (e.g. icon)." },
    ],
    events: [
      { name: "valueChange", payload: "{ value: string }", description: "Fires when the input value changes." },
      { name: "validationChange", payload: '{ status: InputValidationStatus; valid: boolean; message: string }', description: "Fires when built-in validation state changes." },
      { name: "submit", payload: "{ value: string }", description: "Fires on Enter key press." },
      { name: "cancel", payload: "void", description: "Fires when editing is cancelled." },
      { name: "clear", payload: "void", description: "Fires when the search clear button is clicked (type=\"search\" only)." },
      { name: "focus", payload: "FocusEvent", description: "Fires when the input receives focus." },
      { name: "blur", payload: "FocusEvent", description: "Fires when the input loses focus." },
    ],
    usage: `<script lang="ts">
  import { TextInput } from "@poodle/svelte-primitives";

  let email = "";
  async function validateEmail(value: string) {
    return value.includes("@")
      ? { valid: true }
      : { valid: false, message: "Enter a valid email address." };
  }
</script>

<TextInput
  id="email"
  bind:value={email}
  placeholder="you@example.com"
  type="email"
  autocomplete="email"
  validate={validateEmail}
/>`,
  },

  "time-ago": {
    props: [
      { name: "datetime", type: "Date | string | number", required: true, description: "The date/time to display relative to now." },
      { name: "live", type: "boolean", default: "true", description: "Whether the display updates automatically." },
      { name: "interval", type: "number", default: "30000", description: "Update interval in milliseconds when live." },
      { name: "short", type: "boolean", default: "true", description: "Use short format (e.g. 5m ago) instead of long format (e.g. 5 minutes ago)." },
      { name: "tooltipFormat", type: '"full" | "date" | "datetime"', default: '"datetime"', description: "Format for the native title tooltip showing the absolute date." },
      { name: "timezone", type: "string | null", default: "null", description: "IANA timezone override for the absolute date tooltip (e.g. America/New_York)." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label override." },
    ],
    slots: [],
    events: [],
    usage: `<script lang="ts">
  import { TimeAgo } from "@poodle/svelte-primitives";
</script>

<TimeAgo datetime={new Date("2026-03-20T12:00:00Z")} live />`,
  },

  "time-field": {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "id", type: "string | null", default: "null", description: "HTML id attribute for the field." },
      { name: "value", type: "string | null", default: "null", description: "Controlled time value (e.g. \"14:30\")." },
      { name: "defaultValue", type: "string | null", default: "null", description: "Initial time value for uncontrolled mode." },
      { name: "min", type: "string | null", default: "null", description: "Minimum allowed time." },
      { name: "max", type: "string | null", default: "null", description: "Maximum allowed time." },
      { name: "step", type: "number", default: "60", description: "Step interval in seconds." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the field is disabled." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the time field." },
      { name: "describedBy", type: "string | null", default: "null", description: "ID of the element describing this field." },
    ],
    slots: [],
    events: [
      { name: "valueChange", payload: "{ value: string | null }", description: "Fires when the time value changes." },
    ],
    usage: `<script lang="ts">
  import { TimeField } from "@poodle/svelte-primitives";

  let time = "09:00";
</script>

<TimeField id="start-time" bind:value={time} min="08:00" max="18:00" step={60} />`,
  },

  "time-zone-select": {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "id", type: "string | undefined", default: "undefined", description: "HTML id attribute for the select." },
      { name: "value", type: "string | null", default: "null", description: "Controlled selected time zone." },
      { name: "defaultValue", type: "string | null", default: "null", description: "Initial time zone for uncontrolled mode." },
      { name: "placeholder", type: "string | null", default: '"Select time zone"', description: "Placeholder text when no value is selected." },
      { name: "options", type: "TimeZoneOption[]", default: "[]", description: "Available time zone options." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the select is disabled." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the select." },
      { name: "describedBy", type: "string | null", default: "null", description: "ID of the element describing this select." },
      { name: "name", type: "string | undefined", default: "undefined", description: "Form field name." },
    ],
    slots: [],
    events: [
      { name: "valueChange", payload: "{ value: string }", description: "Fires when the selected time zone changes." },
    ],
    usage: `<script lang="ts">
  import { TimeZoneSelect } from "@poodle/svelte-primitives";

  let tz = "America/New_York";
</script>

<TimeZoneSelect id="tz" bind:value={tz} placeholder="Select time zone" />`,
  },

  "toast-stack": {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"chrome"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "items", type: "ToastItem[]", default: "[]", description: "Array of toast notification items." },
      { name: "ariaLabel", type: "string", default: '"Notifications"', description: "Accessible label for the toast region." },
    ],
    slots: [],
    events: [
      { name: "dismiss", payload: "{ id: string }", description: "Fires when a toast is dismissed." },
      { name: "action", payload: "{ id: string }", description: "Fires when a toast action button is clicked." },
    ],
    usage: `<script lang="ts">
  import { ToastStack } from "@poodle/svelte-composites";

  let toasts = [
    { id: "1", title: "Saved", description: "Your changes have been saved." },
  ];
</script>

<ToastStack items={toasts} on:dismiss={(e) => { toasts = toasts.filter(t => t.id !== e.detail.id); }} />`,
  },

  "toast-host": {
    props: [
      { name: "store", type: "ToastHostStore", description: "Store with readable toast items and dismiss(id) wiring." },
      { name: "autoDismissMs", type: "number", default: "6000", description: "Auto-dismiss delay for non-sticky toasts. Set to 0 or less to disable timers." },
      { name: "stickyTones", type: "ToastTone[]", default: '["danger"]', description: "Toast tones that should stay visible until explicitly dismissed." },
      { name: "placement", type: '"bottom-end" | "bottom-start" | "top-end" | "top-start"', default: '"bottom-end"', description: "Viewport placement for the toast host." },
      { name: "ariaLabel", type: "string", default: '"Notifications"', description: "Accessible label forwarded to the internal toast stack." },
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override forwarded to the internal toast stack." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"chrome"', description: "Semantic size offset forwarded to the internal toast stack." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override forwarded to the internal toast stack." },
      { name: "onAction", type: "((id: string) => void) | null", default: "null", description: "Optional callback when a toast action button is activated." },
    ],
    slots: [],
    events: [
      { name: "dismiss", payload: "{ id: string }", description: "Fires after the host dismisses a toast from the backing store." },
      { name: "action", payload: "{ id: string }", description: "Fires when a toast action button is clicked." },
    ],
    usage: `<script lang="ts">
  import { writable } from "svelte/store";
  import { ToastHost, type ToastHostStoreItem } from "@poodle/svelte-composites";

  const toasts = writable<ToastHostStoreItem[]>([
    { id: "1", variant: "success", title: "Saved", message: "Your settings were updated." },
    { id: "2", variant: "error", message: "Publishing failed." },
  ]);

  const store = {
    toasts,
    dismiss(id: string) {
      toasts.update((items) => items.filter((item) => item.id !== id));
    }
  };
</script>

<ToastHost {store} />`,
  },

  "toggle-group": {
    props: [
      { name: "value", type: "string | string[] | null", default: "null", description: "Controlled selected value(s)." },
      { name: "defaultValue", type: "string | string[] | null", default: "null", description: "Initial selected value(s) for uncontrolled mode." },
      { name: "options", type: "ToggleGroupOption[]", default: "[]", description: "Array of toggle options." },
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: '"compact" | "default" | "comfortable" | null', default: "null", description: "Explicit density override for group gap and item padding." },
      { name: "selectionMode", type: '"single" | "multiple"', default: '"single"', description: "Whether one or many toggles can be active." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the entire group is disabled." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the toggle group." },
    ],
    slots: [],
    events: [
      { name: "valueChange", payload: "{ value: string | string[] }", description: "Fires when the selected value(s) change." },
    ],
    usage: `<script lang="ts">
  import { ToggleGroup } from "@poodle/svelte-primitives";

  const options = [
    { value: "left", label: "Left" },
    { value: "center", label: "Center" },
    { value: "right", label: "Right" },
  ];
</script>

<ToggleGroup {options} defaultValue="left" ariaLabel="Text alignment" />`,
  },

  toolbar: {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"chrome"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "orientation", type: "Orientation", default: '"horizontal"', description: "Layout orientation of the toolbar." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the toolbar." },
    ],
    slots: [
      { name: "default", description: "Toolbar items (buttons, separators, etc.)." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { Toolbar } from "@poodle/svelte-primitives";
  import { Button } from "@poodle/svelte-primitives";
</script>

<Toolbar ariaLabel="Formatting">
  <Button variant="ghost">Bold</Button>
  <Button variant="ghost">Italic</Button>
</Toolbar>`,
  },

  tooltip: {
    props: [
      { name: "content", type: "string", required: true, description: "Text content displayed in the tooltip." },
      { name: "open", type: "boolean | null", default: "null", description: "Controlled open state." },
      { name: "defaultOpen", type: "boolean", default: "false", description: "Initial open state for uncontrolled mode." },
      { name: "delayMs", type: "number", default: "300", description: "Delay in milliseconds before the tooltip appears." },
      { name: "placement", type: "OverlayPlacement", default: '"top"', description: "Preferred placement of the tooltip." },
    ],
    slots: [
      { name: "default", description: "Trigger element the tooltip is anchored to." },
    ],
    events: [
      { name: "openChange", payload: "{ open: boolean }", description: "Fires when the tooltip open state changes." },
    ],
    usage: `<script lang="ts">
  import { Tooltip } from "@poodle/svelte-primitives";
  import { Button } from "@poodle/svelte-primitives";
</script>

<Tooltip content="Save your work" placement="top">
  <Button>Save</Button>
</Tooltip>`,
  },

  "tri-state-switch": {
    props: [
      { name: "value", type: "TriStateValue", default: '"default"', description: "Current tri-state value." },
      { name: "options", type: "Record<TriStateValue, string>", default: '{ excluded: "Exclude", default: "Default", included: "Include" }', description: "Labels for each state." },
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: '"compact" | "default" | "comfortable" | null', default: "null", description: "Explicit density override for track inset and segment padding." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the switch is disabled." },
      { name: "ariaLabel", type: "string", required: true, description: "Accessible label for the switch." },
      { name: "excludedColor", type: "string | null", default: "null", description: "Optional color override for the selected excluded state." },
      { name: "defaultColor", type: "string | null", default: "null", description: "Optional color override for the selected default state." },
      { name: "includedColor", type: "string | null", default: "null", description: "Optional color override for the selected included state." },
    ],
    slots: [],
    events: [
      { name: "valueChange", payload: "{ value: TriStateValue }", description: "Fires when the tri-state value changes." },
    ],
    usage: `<script lang="ts">
  import { TriStateSwitch } from "@poodle/svelte-primitives";

  let filter = "default";
</script>

<TriStateSwitch bind:value={filter} ariaLabel="Include archived items" excludedColor="#ef4444" includedColor="#22c55e" />`,
  },

  "ui-presentation-provider": {
    props: [
      { name: "density", type: "ControlDensity", default: '"default"', description: "Density applied to all descendant components. Supports compact, default, and comfortable." },
      { name: "sizeScale", type: "ControlSize", default: '"md"', description: "Base size scale applied to all descendant components. Supports xs, sm, md, lg, and xl." },
    ],
    slots: [
      { name: "default", description: "Child content that inherits the presentation settings." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { UiPresentationProvider, Button, Select } from "@poodle/svelte-primitives";
</script>

<UiPresentationProvider density="compact" sizeScale="sm">
  <Button>Compact small button</Button>
  <Select id="demo" options={[{ value: "a", label: "Alpha" }]} />
</UiPresentationProvider>`,
  },

  "video-player": {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "src", type: "string", required: true, description: "URL of the video file to play." },
      { name: "poster", type: "string | null", default: "null", description: "URL of the poster image shown before playback." },
      { name: "aspectRatio", type: "number", default: "16/9", description: "Aspect ratio of the video player." },
      { name: "ariaLabel", type: "string", default: '"Video player"', description: "Accessible label for the player." },
      { name: "showCaptions", type: "boolean", default: "false", description: "Whether captions are displayed by default." },
      { name: "captionsSrc", type: "string | null", default: "null", description: "URL of the captions/subtitles file." },
    ],
    slots: [],
    events: [],
    usage: `<script lang="ts">
  import { VideoPlayer } from "@poodle/svelte-composites";
</script>

<VideoPlayer src="/videos/intro.mp4" poster="/images/poster.jpg" showCaptions />`,
  },

  "zoned-date-time-picker": {
    props: [
      { name: "size", type: "ControlSize | null", default: "null", description: "Explicit size override. Supports xs, sm, md, lg, and xl." },
      { name: "sizeRole", type: '"chrome" | "control" | "prominent"', default: '"control"', description: "Semantic size offset relative to the inherited presentation scale." },
      { name: "density", type: "ControlDensity | null", default: "null", description: "Explicit density override. Supports compact, default, and comfortable." },
      { name: "value", type: "ZonedDateTimeValue | null", default: "null", description: "Controlled date/time/zone value." },
      { name: "defaultValue", type: "ZonedDateTimeValue", default: '{ date: null, time: null, timeZone: null }', description: "Initial value for uncontrolled mode." },
      { name: "open", type: "boolean | null", default: "null", description: "Controlled open state of the picker." },
      { name: "defaultOpen", type: "boolean", default: "false", description: "Initial open state for uncontrolled mode." },
      { name: "placeholder", type: "string", default: '"Select date, time, and zone"', description: "Placeholder text when no value is selected." },
      { name: "weekStartsOn", type: "CalendarWeekStart", default: '"monday"', description: "First day of the week in the calendar." },
      { name: "locale", type: "string", default: '"en-US"', description: "Locale for date formatting." },
      { name: "timeZoneOptions", type: "TimeZoneOption[]", default: "[]", description: "Available time zone options." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the picker is disabled." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the picker." },
    ],
    slots: [
      { name: "default", description: "Custom trigger content for the picker." },
    ],
    events: [
      { name: "valueChange", payload: "{ value: ZonedDateTimeValue }", description: "Fires when the date/time/zone value changes." },
      { name: "openChange", payload: "{ open: boolean }", description: "Fires when the picker open state changes." },
    ],
    usage: `<script lang="ts">
  import { ZonedDateTimePicker } from "@poodle/svelte-primitives";

  let meeting = { date: null, time: null, timeZone: "America/New_York" };
</script>

<ZonedDateTimePicker bind:value={meeting} placeholder="Pick date, time & zone" />`,
  },

  "action-discovery-panel": {
    props: [
      { name: "items", type: "CommandActionItem[]", default: "[]", description: "Array of discoverable action items." },
      { name: "state", type: "DiscoveryState", default: '"ready"', description: "Current state of the discovery panel." },
      { name: "activeId", type: "string | null", default: "null", description: "ID of the currently active/highlighted item." },
      { name: "ariaLabel", type: "string", default: '"Actions"', description: "Accessible label for the panel." },
      { name: "size", type: 'ControlSize | null', default: "null", description: "Explicit semantic size override for badge and shortcut chips." },
      { name: "sizeRole", type: 'SemanticControlSizeRole', default: '"control"', description: "Semantic size role used when inheriting from presentation context." },
      { name: "density", type: 'ControlDensity | null', default: "null", description: "Explicit density override for group, list, and skeleton spacing." },
    ],
    slots: [],
    events: [
      { name: "itemSelect", payload: "{ id: string }", description: "Fires when an action item is selected." },
      { name: "activeChange", payload: "{ id: string | null }", description: "Fires when the active item changes." },
    ],
    usage: `<script lang="ts">
  import { ActionDiscoveryPanel } from "@poodle/svelte-composites";

  const items = [
    { id: "copy", label: "Copy", shortcut: "Cmd+C" },
    { id: "paste", label: "Paste", shortcut: "Cmd+V" },
  ];
</script>

<ActionDiscoveryPanel {items} on:itemSelect={(e) => console.log(e.detail.id)} />`,
  },

  "app-header": {
    props: [
      { name: "title", type: "string | null", default: "null", description: "Primary title displayed in the header." },
      { name: "subtitle", type: "string | null", default: "null", description: "Secondary subtitle below the title." },
      { name: "dragRegion", type: "boolean", default: "false", description: "Whether the header acts as a window drag region." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the header." },
    ],
    slots: [
      { name: "identity", description: "Custom content for the identity/logo area." },
      { name: "actions", description: "Primary action buttons in the header." },
      { name: "utility", description: "Utility controls (e.g. user menu, settings)." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { AppHeader } from "@poodle/svelte-composites";
  import { Button } from "@poodle/svelte-primitives";
</script>

<AppHeader title="My App" subtitle="Dashboard">
  {#snippet actions()}
    <Button variant="primary">New</Button>
  {/snippet}
</AppHeader>`,
  },
};

// Alias: nav-card-grid shares docs with nav-card
componentDocsMap["nav-card-grid"] = componentDocsMap["nav-card"];
