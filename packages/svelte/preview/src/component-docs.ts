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
      { name: "items", type: "AccordionItem[]", default: "[]", description: "Array of accordion panel items to render." },
      { name: "value", type: "string | string[] | null", default: "null", description: "Controlled open panel value(s)." },
      { name: "defaultValue", type: "string | string[] | null", default: "null", description: "Initial open panel value(s) for uncontrolled mode." },
      { name: "selectionMode", type: '"single" | "multiple"', default: '"single"', description: "Whether one or many panels can be open simultaneously." },
      { name: "isCollapsible", type: "boolean", default: "true", description: "Whether the open panel can be collapsed by clicking its trigger again." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the accordion region." },
    ],
    slots: [
      { name: "default", description: "Custom panel content. Receives slot props: item, isOpen." },
    ],
    events: [
      { name: "valueChange", payload: "{ value: string | string[] | null }", description: "Fires when the open panel(s) change." },
    ],
    usage: `<script lang="ts">
  import { Accordion } from "@pug/svelte-primitives";

  const items = [
    { value: "one", title: "Section One", content: "Content for section one." },
    { value: "two", title: "Section Two", content: "Content for section two." },
  ];
</script>

<Accordion {items} selectionMode="single" isCollapsible />`,
  },

  "alert-dialog": {
    props: [
      { name: "open", type: "boolean | null", default: "null", description: "Controlled open state of the dialog." },
      { name: "title", type: "string", required: true, description: "Title displayed at the top of the alert dialog." },
      { name: "description", type: "string | null", default: "null", description: "Descriptive text shown below the title." },
      { name: "tone", type: "AlertDialogTone", default: '"danger"', description: "Visual tone of the dialog (e.g. danger, warning)." },
      { name: "confirmLabel", type: "string", default: '"Confirm"', description: "Label for the confirm action button." },
      { name: "cancelLabel", type: "string", default: '"Cancel"', description: "Label for the cancel action button." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the dialog." },
    ],
    slots: [
      { name: "default", description: "Optional body content rendered between description and actions." },
      { name: "actions", description: "Custom actions replacing the default confirm/cancel buttons." },
    ],
    events: [
      { name: "confirm", payload: "void", description: "Fires when the confirm button is clicked." },
      { name: "cancel", payload: "void", description: "Fires when the cancel button is clicked." },
      { name: "openChange", payload: "{ open: boolean }", description: "Fires when the open state changes." },
    ],
    usage: `<script lang="ts">
  import { AlertDialog } from "@pug/svelte-primitives";

  let open = false;
</script>

<AlertDialog
  bind:open
  title="Delete item?"
  description="This action cannot be undone."
  tone="danger"
  on:confirm={() => console.log("confirmed")}
/>`,
  },

  "audio-player": {
    props: [
      { name: "src", type: "string", required: true, description: "URL of the audio file to play." },
      { name: "ariaLabel", type: "string", default: '"Audio player"', description: "Accessible label for the player." },
      { name: "showSpeedControl", type: "boolean", default: "false", description: "Whether to show playback speed controls." },
    ],
    slots: [],
    events: [],
    usage: `<script lang="ts">
  import { AudioPlayer } from "@pug/svelte-composites";
</script>

<AudioPlayer src="/audio/podcast-episode.mp3" showSpeedControl />`,
  },

  "block-editor": {
    props: [
      { name: "blocks", type: "EditorBlock[]", default: "[default block]", description: "Array of content blocks in the editor." },
      { name: "blockTypes", type: "BlockTypeDefinition[]", default: "[defaults]", description: "Available block type definitions." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the editor is disabled." },
      { name: "ariaLabel", type: "string", default: '"Block editor"', description: "Accessible label for the editor region." },
    ],
    slots: [
      { name: "block", description: "Custom block renderer. Receives slot props: block, index, isDisabled, update." },
    ],
    events: [
      { name: "change", payload: "{ blocks: EditorBlock[] }", description: "Fires when any block content changes." },
    ],
    usage: `<script lang="ts">
  import { BlockEditor } from "@pug/svelte-composites";

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
  import { Box } from "@pug/svelte-primitives";
</script>

<Box padding="md" width="100%">
  <p>Contained content with padding.</p>
</Box>`,
  },

  breadcrumbs: {
    props: [
      { name: "items", type: "BreadcrumbItem[]", default: "[]", description: "Array of breadcrumb navigation items." },
      { name: "ariaLabel", type: "string", default: '"Breadcrumb"', description: "Accessible label for the breadcrumb nav." },
      { name: "maxVisibleItems", type: "number | null", default: "null", description: "Maximum number of visible items before collapsing." },
    ],
    slots: [],
    events: [
      { name: "navigate", payload: "{ value: string }", description: "Fires when a breadcrumb item is clicked." },
    ],
    usage: `<script lang="ts">
  import { Breadcrumbs } from "@pug/svelte-primitives";

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
      { name: "selectionCount", type: "number", default: "0", description: "Number of currently selected items." },
      { name: "totalCount", type: "number | null", default: "null", description: "Total number of selectable items." },
      { name: "actions", type: "BulkAction[]", default: "[]", description: "Array of bulk action definitions." },
    ],
    slots: [],
    events: [
      { name: "action", payload: "{ id: string }", description: "Fires when a bulk action is triggered." },
      { name: "clear", payload: "void", description: "Fires when the selection is cleared." },
    ],
    usage: `<script lang="ts">
  import { BulkActionBar } from "@pug/svelte-primitives";

  const actions = [
    { id: "delete", label: "Delete", tone: "danger" },
    { id: "archive", label: "Archive" },
  ];
</script>

<BulkActionBar selectionCount={3} totalCount={50} {actions} />`,
  },

  button: {
    props: [
      { name: "variant", type: "ButtonVariant", default: '"secondary"', description: "Visual variant of the button." },
      { name: "tone", type: "ButtonTone", default: '"default"', description: "Color tone of the button." },
      { name: "size", type: "ControlSize", default: '"md"', description: "Size of the button." },
      { name: "type", type: 'HTMLButtonElement["type"]', default: '"button"', description: "HTML button type attribute." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the button is disabled." },
      { name: "isLoading", type: "boolean", default: "false", description: "Whether the button shows a loading spinner." },
      { name: "leadingIcon", type: "string | null", default: "null", description: "Icon name displayed before the label." },
      { name: "trailingIcon", type: "string | null", default: "null", description: "Icon name displayed after the label." },
      { name: "chevron", type: "boolean", default: "false", description: "Whether to show a dropdown chevron." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the button." },
      { name: "describedBy", type: "string | null", default: "null", description: "ID of the element describing this button." },
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
  import { Button } from "@pug/svelte-primitives";
</script>

<Button variant="primary" tone="default" leadingIcon="plus" on:click={() => console.log("clicked")}>
  Create Item
</Button>`,
  },

  calendar: {
    props: [
      { name: "value", type: "string | null", default: "null", description: "Controlled selected date in ISO format." },
      { name: "defaultValue", type: "string | null", default: "null", description: "Initial selected date for uncontrolled mode." },
      { name: "visibleMonth", type: "string | null", default: "null", description: "Controlled visible month in YYYY-MM format." },
      { name: "weekStartsOn", type: "CalendarWeekStart", default: '"monday"', description: "Which day of the week the calendar starts on." },
      { name: "locale", type: "string", default: '"en-US"', description: "Locale for date formatting." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the calendar is disabled." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the calendar." },
    ],
    slots: [],
    events: [
      { name: "valueChange", payload: "{ value: string }", description: "Fires when the selected date changes." },
      { name: "monthChange", payload: "{ month: string }", description: "Fires when the visible month changes." },
    ],
    usage: `<script lang="ts">
  import { Calendar } from "@pug/svelte-primitives";

  let selectedDate: string | null = null;
</script>

<Calendar bind:value={selectedDate} weekStartsOn="monday" locale="en-US" />`,
  },

  callout: {
    props: [
      { name: "tone", type: 'StatusTone | "neutral"', default: '"neutral"', description: "Visual tone indicating the callout severity." },
      { name: "title", type: "string | null", default: "null", description: "Title text for the callout." },
      { name: "message", type: "string | null", default: "null", description: "Body message text." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the callout." },
      { name: "announceMode", type: '"none" | "polite" | "assertive"', default: '"none"', description: "ARIA live region announcement mode." },
      { name: "isDismissible", type: "boolean", default: "false", description: "Whether the callout can be dismissed." },
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
  import { Callout } from "@pug/svelte-primitives";
</script>

<Callout tone="warning" title="Unsaved changes" message="You have unsaved changes that will be lost." isDismissible />`,
  },

  card: {
    props: [
      { name: "variant", type: "CardVariant", default: '"default"', description: "Visual variant of the card." },
      { name: "layout", type: '"vertical" | "horizontal" | "compact"', default: '"vertical"', description: "Layout direction of card content." },
      { name: "isInteractive", type: "boolean", default: "false", description: "Whether the card responds to hover and click." },
      { name: "isSelected", type: "boolean", default: "false", description: "Whether the card is in a selected state." },
      { name: "hasMedia", type: "boolean", default: "false", description: "Whether the card contains a media slot." },
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
  import { Card } from "@pug/svelte-primitives";
</script>

<Card variant="default" layout="vertical">
  <svelte:fragment slot="header">Project Alpha</svelte:fragment>
  <p>A brief description of the project and its current status.</p>
  <svelte:fragment slot="footer">Updated 2 hours ago</svelte:fragment>
</Card>`,
  },

  "card-radio-group": {
    props: [
      { name: "items", type: "CardRadioItem[]", default: "[]", description: "Array of card radio option items." },
      { name: "value", type: "string | null", default: "null", description: "Currently selected item value." },
      { name: "columns", type: "1 | 2 | 3 | 4", default: "2", description: "Number of columns in the grid layout." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the radio group." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the entire group is disabled." },
    ],
    slots: [
      { name: "card", description: "Custom card renderer. Receives slot props: item, checked, disabled." },
    ],
    events: [
      { name: "change", payload: "{ value: string }", description: "Fires when the selected card changes." },
    ],
    usage: `<script lang="ts">
  import { CardRadioGroup } from "@pug/svelte-composites";

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
      { name: "id", type: "string | undefined", default: "undefined", description: "HTML id attribute for the checkbox input." },
      { name: "isChecked", type: "boolean", default: "false", description: "Controlled checked state." },
      { name: "defaultChecked", type: "boolean", default: "false", description: "Initial checked state for uncontrolled mode." },
      { name: "isMixed", type: "boolean", default: "false", description: "Whether the checkbox is in an indeterminate state." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the checkbox is disabled." },
      { name: "isReadOnly", type: "boolean", default: "false", description: "Whether the checkbox is read-only." },
      { name: "label", type: "string | null", default: "null", description: "Label text displayed next to the checkbox." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label when no visible label is used." },
      { name: "describedBy", type: "string | null", default: "null", description: "ID of the element describing this checkbox." },
    ],
    slots: [],
    events: [
      { name: "checkedChange", payload: "{ checked: boolean }", description: "Fires when the checked state changes." },
    ],
    usage: `<script lang="ts">
  import { Checkbox } from "@pug/svelte-primitives";

  let agreed = false;
</script>

<Checkbox label="I agree to the terms" bind:isChecked={agreed} />`,
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
    ],
    slots: [],
    events: [],
    usage: `<script lang="ts">
  import { Code } from "@pug/svelte-primitives";

  const source = \`function greet(name: string) {
  return \\\`Hello, \\\${name}!\\\`;
}\`;
</script>

<Code {source} language="typescript" showLineNumbers />`,
  },

  "collapse-toggle": {
    props: [
      { name: "isCollapsed", type: "boolean", default: "false", description: "Whether the target region is collapsed." },
      { name: "direction", type: "CollapseDirection", default: '"left"', description: "Direction the toggle chevron points when collapsed." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the toggle is disabled." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the toggle button." },
    ],
    slots: [],
    events: [
      { name: "toggle", payload: "{ isCollapsed: boolean }", description: "Fires when the collapsed state changes." },
    ],
    usage: `<script lang="ts">
  import { CollapseToggle } from "@pug/svelte-primitives";

  let collapsed = false;
</script>

<CollapseToggle bind:isCollapsed={collapsed} direction="left" ariaLabel="Toggle sidebar" />`,
  },

  collapsible: {
    props: [
      { name: "open", type: "boolean | null", default: "null", description: "Controlled open state." },
      { name: "defaultOpen", type: "boolean", default: "false", description: "Initial open state for uncontrolled mode." },
      { name: "title", type: "string | null", default: "null", description: "Title displayed in the trigger." },
      { name: "description", type: "string | null", default: "null", description: "Description shown below the title." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the collapsible is disabled." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the collapsible region." },
    ],
    slots: [
      { name: "trigger", description: "Custom trigger content. Receives slot props: isOpen." },
      { name: "default", description: "Content revealed when the collapsible is open." },
    ],
    events: [
      { name: "openChange", payload: "{ open: boolean }", description: "Fires when the open state changes." },
    ],
    usage: `<script lang="ts">
  import { Collapsible } from "@pug/svelte-primitives";
</script>

<Collapsible title="Advanced options" description="Configure additional settings">
  <p>Advanced configuration fields go here.</p>
</Collapsible>`,
  },

  "color-picker": {
    props: [
      { name: "value", type: "string", default: '"#6366f1"', description: "Current color value in hex format." },
      { name: "swatches", type: "string[]", default: "[]", description: "Preset color swatches to display." },
      { name: "showInput", type: "boolean", default: "true", description: "Whether to show the hex input field." },
      { name: "showAlpha", type: "boolean", default: "false", description: "Whether to show the alpha channel slider." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the picker is disabled." },
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
  import { ColorPicker } from "@pug/svelte-primitives";

  let color = "#6366f1";
  const swatches = ["#ef4444", "#f59e0b", "#10b981", "#3b82f6", "#8b5cf6"];
</script>

<ColorPicker bind:value={color} {swatches} showAlpha />`,
  },

  combobox: {
    props: [
      { name: "value", type: "string | null", default: "null", description: "Controlled selected value." },
      { name: "defaultValue", type: "string | null", default: "null", description: "Initial value for uncontrolled mode." },
      { name: "options", type: "ComboboxOption[]", default: "[]", description: "Array of selectable options." },
      { name: "placeholder", type: "string | null", default: "null", description: "Placeholder text when no value is selected." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the combobox is disabled." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the combobox." },
    ],
    slots: [],
    events: [
      { name: "valueChange", payload: "{ value: string }", description: "Fires when the selected value changes." },
      { name: "queryChange", payload: "{ query: string }", description: "Fires when the search query text changes." },
      { name: "openChange", payload: "{ open: boolean }", description: "Fires when the dropdown open state changes." },
    ],
    usage: `<script lang="ts">
  import { Combobox } from "@pug/svelte-primitives";

  const options = [
    { value: "us", label: "United States" },
    { value: "ca", label: "Canada" },
    { value: "mx", label: "Mexico" },
  ];
</script>

<Combobox {options} placeholder="Search countries..." />`,
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
    ],
    slots: [],
    events: [
      { name: "queryChange", payload: "{ value: string }", description: "Fires when the search query changes." },
      { name: "commandSelect", payload: "{ id: string }", description: "Fires when a command is selected." },
      { name: "openChange", payload: "{ open: boolean }", description: "Fires when the open state changes." },
      { name: "activeChange", payload: "{ id: string | null }", description: "Fires when the active (highlighted) item changes." },
    ],
    usage: `<script lang="ts">
  import { CommandPalette } from "@pug/svelte-composites";

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
  import { ConfirmAction } from "@pug/svelte-composites";
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
  import { ContextMenu } from "@pug/svelte-primitives";

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
      { name: "ariaLabel", type: "string", default: '"Data table"', description: "Accessible label for the table." },
      { name: "columns", type: "TableColumn[]", default: "[]", description: "Column definitions for the table." },
      { name: "rows", type: "TableRow[]", default: "[]", description: "Row data to display." },
      { name: "selectedRowIds", type: "string[]", default: "[]", description: "Array of selected row IDs." },
      { name: "sortColumnId", type: "string | null", default: "null", description: "Currently sorted column ID." },
      { name: "sortDirection", type: "TableSortDirection", default: '"asc"', description: "Current sort direction." },
      { name: "rowActionLabel", type: "string", default: '"Open"', description: "Label for the row action button." },
      { name: "showRowActions", type: "boolean", default: "true", description: "Whether to show row action buttons." },
      { name: "emptyMessage", type: "string", default: '"No rows match the current view."', description: "Message shown when no rows are present." },
      { name: "hiddenColumnIds", type: "string[]", default: "[]", description: "Array of column IDs to hide." },
      { name: "showColumnVisibility", type: "boolean", default: "false", description: "Whether to show the column visibility toggle." },
      { name: "showExport", type: "boolean", default: "false", description: "Whether to show the CSV export button." },
      { name: "exportFilename", type: "string", default: '"export.csv"', description: "Default filename for CSV export." },
    ],
    slots: [],
    events: [
      { name: "sortChange", payload: "{ columnId: string; direction: TableSortDirection }", description: "Fires when the sort column or direction changes." },
      { name: "rowToggle", payload: "{ rowId: string; selected: boolean }", description: "Fires when a row selection is toggled." },
      { name: "toggleAll", payload: "{ selected: boolean }", description: "Fires when the select-all checkbox is toggled." },
      { name: "rowAction", payload: "{ rowId: string }", description: "Fires when a row action button is clicked." },
      { name: "columnVisibilityChange", payload: "{ columnId: string; visible: boolean }", description: "Fires when column visibility changes." },
      { name: "exportCsv", payload: "{ filename: string }", description: "Fires when CSV export is triggered." },
    ],
    usage: `<script lang="ts">
  import { DataTable } from "@pug/svelte-composites";

  const columns = [
    { id: "name", label: "Name", isSortable: true },
    { id: "email", label: "Email" },
    { id: "role", label: "Role" },
  ];

  const rows = [
    { id: "1", cells: { name: "Alice", email: "alice@example.com", role: "Admin" } },
    { id: "2", cells: { name: "Bob", email: "bob@example.com", role: "Editor" } },
  ];
</script>

<DataTable {columns} {rows} showColumnVisibility showExport />`,
  },

  "date-picker": {
    props: [
      { name: "value", type: "string | null", default: "null", description: "Controlled selected date in ISO format." },
      { name: "defaultValue", type: "string | null", default: "null", description: "Initial date for uncontrolled mode." },
      { name: "open", type: "boolean | null", default: "null", description: "Controlled open state of the calendar popup." },
      { name: "defaultOpen", type: "boolean", default: "false", description: "Initial open state for uncontrolled mode." },
      { name: "placeholder", type: "string", default: '"Select date"', description: "Placeholder text when no date is selected." },
      { name: "weekStartsOn", type: "CalendarWeekStart", default: '"monday"', description: "Which day the calendar week starts on." },
      { name: "locale", type: "string", default: '"en-US"', description: "Locale for date formatting." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the picker is disabled." },
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
  import { DatePicker } from "@pug/svelte-primitives";

  let date: string | null = null;
</script>

<DatePicker bind:value={date} placeholder="Pick a date" />`,
  },

  "date-range-picker": {
    props: [
      { name: "value", type: "DateRangeValue | null", default: "null", description: "Controlled date range value." },
      { name: "defaultValue", type: "DateRangeValue", default: "{ start: null, end: null }", description: "Initial date range for uncontrolled mode." },
      { name: "open", type: "boolean | null", default: "null", description: "Controlled open state." },
      { name: "defaultOpen", type: "boolean", default: "false", description: "Initial open state for uncontrolled mode." },
      { name: "placeholder", type: "string", default: '"Select date range"', description: "Placeholder text when no range is selected." },
      { name: "weekStartsOn", type: "CalendarWeekStart", default: '"monday"', description: "Which day the calendar week starts on." },
      { name: "locale", type: "string", default: '"en-US"', description: "Locale for date formatting." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the picker is disabled." },
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
  import { DateRangePicker } from "@pug/svelte-primitives";

  let range = { start: null, end: null };
</script>

<DateRangePicker bind:value={range} placeholder="Select date range" />`,
  },

  "date-time-picker": {
    props: [
      { name: "value", type: "DateTimeValue | null", default: "null", description: "Controlled date-time value." },
      { name: "defaultValue", type: "DateTimeValue", default: "{ date: null, time: null }", description: "Initial date-time for uncontrolled mode." },
      { name: "open", type: "boolean | null", default: "null", description: "Controlled open state." },
      { name: "defaultOpen", type: "boolean", default: "false", description: "Initial open state for uncontrolled mode." },
      { name: "placeholder", type: "string", default: '"Select date and time"', description: "Placeholder text when no value is selected." },
      { name: "weekStartsOn", type: "CalendarWeekStart", default: '"monday"', description: "Which day the calendar week starts on." },
      { name: "locale", type: "string", default: '"en-US"', description: "Locale for date formatting." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the picker is disabled." },
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
  import { DateTimePicker } from "@pug/svelte-primitives";

  let dateTime = { date: null, time: null };
</script>

<DateTimePicker bind:value={dateTime} placeholder="Select date and time" />`,
  },

  "date-time-range-picker": {
    props: [
      { name: "value", type: "DateTimeRangeValue | null", default: "null", description: "Controlled date-time range value." },
      { name: "defaultValue", type: "DateTimeRangeValue", default: "{ start: { date: null, time: null }, end: { date: null, time: null } }", description: "Initial date-time range for uncontrolled mode." },
      { name: "open", type: "boolean | null", default: "null", description: "Controlled open state." },
      { name: "defaultOpen", type: "boolean", default: "false", description: "Initial open state for uncontrolled mode." },
      { name: "placeholder", type: "string", default: '"Select date and time range"', description: "Placeholder text when no range is selected." },
      { name: "weekStartsOn", type: "CalendarWeekStart", default: '"monday"', description: "Which day the calendar week starts on." },
      { name: "locale", type: "string", default: '"en-US"', description: "Locale for date formatting." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the picker is disabled." },
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
  import { DateTimeRangePicker } from "@pug/svelte-primitives";

  let range = { start: { date: null, time: null }, end: { date: null, time: null } };
</script>

<DateTimeRangePicker bind:value={range} placeholder="Select date and time range" />`,
  },

  "detail-row": {
    props: [
      { name: "label", type: "string", required: true, description: "Label text for the detail row." },
      { name: "value", type: "string | null", default: "null", description: "Value text displayed next to the label." },
      { name: "emptyText", type: "string", default: '"\u2014"', description: "Text shown when value is null or empty." },
    ],
    slots: [
      { name: "default", description: "Custom value content replacing the value prop." },
      { name: "actions", description: "Action buttons rendered at the end of the row." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { DetailRow } from "@pug/svelte-primitives";
</script>

<DetailRow label="Email" value="alice@example.com" />
<DetailRow label="Role" value="Administrator" />
<DetailRow label="Notes" emptyText="No notes provided" />`,
  },

  "detail-section": {
    props: [
      { name: "title", type: "string | null", default: "null", description: "Section title." },
      { name: "description", type: "string | null", default: "null", description: "Description shown below the title." },
      { name: "isSeparated", type: "boolean", default: "true", description: "Whether a separator is shown above the section." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the section region." },
    ],
    slots: [
      { name: "actions", description: "Action buttons rendered in the section header." },
      { name: "default", description: "Body content of the section." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { DetailSection, DetailRow } from "@pug/svelte-composites";
</script>

<DetailSection title="Account Information" description="Basic account details">
  <DetailRow label="Name" value="Alice Johnson" />
  <DetailRow label="Email" value="alice@example.com" />
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
  import { DetailShell, DetailSection } from "@pug/svelte-composites";
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
      { name: "title", type: "string | null", default: "null", description: "Dialog title." },
      { name: "description", type: "string | null", default: "null", description: "Dialog description text." },
      { name: "kind", type: "DialogKind", default: '"dialog"', description: "Kind of dialog (dialog, alert, drawer)." },
      { name: "dismissOnEscape", type: "boolean", default: "true", description: "Whether pressing Escape dismisses the dialog." },
      { name: "dismissOnBackdrop", type: "boolean", default: "true", description: "Whether clicking the backdrop dismisses the dialog." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the dialog." },
    ],
    slots: [
      { name: "default", description: "Dialog body content." },
      { name: "actions", description: "Footer actions (buttons) for the dialog." },
    ],
    events: [
      { name: "openChange", payload: "{ open: boolean }", description: "Fires when the open state changes." },
      { name: "requestClose", payload: "void", description: "Fires when a close is requested (Escape or backdrop click)." },
    ],
    usage: `<script lang="ts">
  import { Dialog, Button } from "@pug/svelte-primitives";

  let open = false;
</script>

<Button on:click={() => (open = true)}>Open Dialog</Button>

<Dialog bind:open title="Edit Settings" description="Update your preferences below.">
  <p>Dialog content here.</p>
  <svelte:fragment slot="actions">
    <Button variant="secondary" on:click={() => (open = false)}>Cancel</Button>
    <Button variant="primary">Save</Button>
  </svelte:fragment>
</Dialog>`,
  },

  "dock-region": {
    props: [
      { name: "edge", type: "DockEdge", default: '"left"', description: "Which edge of the viewport the dock attaches to." },
      { name: "sizing", type: "DockSizing", default: '"flexible"', description: "Sizing behavior of the dock region." },
      { name: "isCollapsed", type: "boolean", default: "false", description: "Whether the dock is collapsed." },
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
  import { DockRegion } from "@pug/svelte-composites";

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
      { name: "open", type: "boolean | null", default: "null", description: "Controlled open state." },
      { name: "defaultOpen", type: "boolean", default: "false", description: "Initial open state for uncontrolled mode." },
      { name: "edge", type: "DrawerEdge", default: '"right"', description: "Which edge the drawer slides from." },
      { name: "isModal", type: "boolean", default: "true", description: "Whether the drawer is modal with a backdrop." },
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
  import { Drawer, Button } from "@pug/svelte-primitives";

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
      { name: "hours", type: "number", default: "0", description: "Hours value." },
      { name: "minutes", type: "number", default: "0", description: "Minutes value." },
      { name: "seconds", type: "number", default: "0", description: "Seconds value." },
      { name: "showSeconds", type: "boolean", default: "true", description: "Whether to show the seconds field." },
      { name: "maxHours", type: "number", default: "99", description: "Maximum allowed hours value." },
      { name: "minTotalSeconds", type: "number", default: "0", description: "Minimum total duration in seconds." },
      { name: "maxTotalSeconds", type: "number | null", default: "null", description: "Maximum total duration in seconds." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the input is disabled." },
      { name: "ariaLabel", type: "string", default: '"Duration"', description: "Accessible label for the duration input." },
    ],
    slots: [
      { name: "default", description: "Custom content rendered alongside the duration fields." },
    ],
    events: [
      { name: "change", payload: "{ hours: number; minutes: number; seconds: number; totalSeconds: number }", description: "Fires when the duration value changes." },
    ],
    usage: `<script lang="ts">
  import { DurationInput } from "@pug/svelte-primitives";

  let hours = 1;
  let minutes = 30;
  let seconds = 0;
</script>

<DurationInput bind:hours bind:minutes bind:seconds showSeconds={false} />`,
  },

  "editable-label": {
    props: [
      { name: "value", type: "string", required: true, description: "Current text value of the editable label." },
      { name: "ariaLabel", type: "string", default: '"Edit label"', description: "Accessible label for the editable input." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the label is disabled for editing." },
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
  import { EditableLabel } from "@pug/svelte-primitives";

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
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the list is disabled." },
      { name: "ariaLabel", type: "string", default: '"List"', description: "Accessible label for the list." },
      { name: "isReorderable", type: "boolean", default: "true", description: "Whether items can be reordered by dragging." },
    ],
    slots: [],
    events: [
      { name: "change", payload: "{ items: ReorderableItem[] }", description: "Fires when the list items change." },
    ],
    usage: `<script lang="ts">
  import { EditableList } from "@pug/svelte-composites";

  let items = [
    { id: "1", label: "First item" },
    { id: "2", label: "Second item" },
  ];
</script>

<EditableList bind:items addLabel="Add step" placeholder="New step" />`,
  },

  "embed-input": {
    props: [
      { name: "value", type: "string", default: '""', description: "Current input value." },
      { name: "parsed", type: "ParsedEmbed | null", default: "null", description: "Parsed embed result from the input value." },
      { name: "placeholder", type: "string", default: '"Paste a URL or embed code..."', description: "Placeholder text for the input." },
      { name: "parseDebounce", type: "number", default: "300", description: "Debounce delay in ms before parsing the input." },
      { name: "providers", type: "string[]", default: "[]", description: "Allowed embed provider names." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the input is disabled." },
      { name: "error", type: "string | null", default: "null", description: "Error message to display." },
    ],
    slots: [],
    events: [
      { name: "parse", payload: "{ parsed: ParsedEmbed | null; error: string | null }", description: "Fires when the input value is parsed." },
      { name: "change", payload: "{ value: string }", description: "Fires when the input value changes." },
    ],
    usage: `<script lang="ts">
  import { EmbedInput } from "@pug/svelte-composites";

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
  import { EmbedPreview } from "@pug/svelte-composites";

  export let parsed;
</script>

<EmbedPreview {parsed} aspectRatio={16/9} />`,
  },

  "empty-state": {
    props: [
      { name: "title", type: "string", required: true, description: "Title text for the empty state." },
      { name: "message", type: "string | null", default: "null", description: "Descriptive message below the title." },
      { name: "variant", type: "EmptyStateVariant", default: '"neutral"', description: "Visual variant of the empty state." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the empty state region." },
    ],
    slots: [
      { name: "actions", description: "Action buttons displayed below the message." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { EmptyState } from "@pug/svelte-composites";
  import { Button } from "@pug/svelte-primitives";
</script>

<EmptyState title="No results found" message="Try adjusting your search or filters.">
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
  import { Eyebrow } from "@pug/svelte-primitives";
</script>

<Eyebrow>Featured</Eyebrow>`,
  },

  field: {
    props: [
      { name: "id", type: "string", required: true, description: "Unique identifier for the field, used to connect label and input." },
      { name: "label", type: "string", required: true, description: "Label text for the field." },
      { name: "description", type: "string | null", default: "null", description: "Help text shown below the label." },
      { name: "error", type: "string | null", default: "null", description: "Error message displayed when validation fails." },
      { name: "pendingMessage", type: "string | null", default: "null", description: "Message shown while validation is pending." },
      { name: "validationState", type: "ValidationState", default: '"none"', description: "Current validation state of the field." },
      { name: "isRequired", type: "boolean", default: "false", description: "Whether the field is required." },
      { name: "optionalLabel", type: "string | null", default: '"Optional"', description: "Label shown for optional fields." },
      { name: "span", type: 'number | "full" | null', default: "null", description: "Column span within a form layout grid." },
      { name: "gridArea", type: "string | null", default: "null", description: "CSS grid-area for custom grid placement." },
    ],
    slots: [
      { name: "default", description: "Field input content. Receives: { describedBy, descriptionId, errorId, messageId, validationState }." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { Field } from "@pug/svelte-primitives";

  let email = "";
</script>

<Field id="email" label="Email address" description="We'll never share your email." isRequired>
  <input id="email" type="email" bind:value={email} />
</Field>`,
  },

  "file-upload": {
    props: [
      { name: "accept", type: "string | null", default: "null", description: "Accepted file types (e.g. 'image/*,.pdf')." },
      { name: "maxSize", type: "number", default: "10485760", description: "Maximum file size in bytes (default 10 MB)." },
      { name: "multiple", type: "boolean", default: "false", description: "Whether multiple files can be uploaded." },
      { name: "maxFiles", type: "number", default: "10", description: "Maximum number of files when multiple is true." },
      { name: "showPreview", type: "boolean", default: "true", description: "Whether to show file previews." },
      { name: "disabled", type: "boolean", default: "false", description: "Whether the upload input is disabled." },
      { name: "files", type: "FileUploadItem[]", default: "[]", description: "Current array of uploaded file items." },
    ],
    slots: [
      { name: "default", description: "Custom dropzone content." },
    ],
    events: [
      { name: "change", payload: "{ files: FileUploadItem[] }", description: "Fires when the file list changes." },
      { name: "error", payload: "{ file: File; message: string }", description: "Fires when a file fails validation." },
      { name: "remove", payload: "{ item: FileUploadItem }", description: "Fires when a file is removed." },
    ],
    usage: `<script lang="ts">
  import { FileUpload } from "@pug/svelte-primitives";

  let files = [];
</script>

<FileUpload accept="image/*,.pdf" multiple maxSize={5242880} bind:files />`,
  },

  "filter-toolbar": {
    props: [
      { name: "ariaLabel", type: "string", default: '"Filters"', description: "Accessible label for the toolbar." },
      { name: "summaryText", type: "string | null", default: "null", description: "Summary text describing active filters." },
      { name: "collapsible", type: "boolean", default: "false", description: "Whether the toolbar can be collapsed." },
      { name: "collapsed", type: "boolean", default: "false", description: "Whether the toolbar is currently collapsed." },
      { name: "columns", type: "number", default: "4", description: "Number of columns in the filter grid." },
      { name: "minItemWidth", type: "string", default: '"10rem"', description: "Minimum width of each filter item." },
      { name: "isSticky", type: "boolean", default: "false", description: "Whether the toolbar sticks to the top on scroll." },
    ],
    slots: [
      { name: "actions", description: "Action buttons in the toolbar header." },
      { name: "default", description: "Filter controls placed in the grid." },
      { name: "secondary", description: "Secondary content below the main filters." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { FilterToolbar } from "@pug/svelte-composites";
  import { Select } from "@pug/svelte-primitives";
</script>

<FilterToolbar ariaLabel="User filters" collapsible>
  <Select options={[{ value: "admin", label: "Admin" }, { value: "user", label: "User" }]} placeholder="Role" />
</FilterToolbar>`,
  },

  "form-actions": {
    props: [
      { name: "align", type: "FormActionAlign", default: '"end"', description: "Horizontal alignment of the action buttons." },
    ],
    slots: [
      { name: "default", description: "Action buttons (e.g. Submit, Cancel)." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { FormActions } from "@pug/svelte-primitives";
  import { Button } from "@pug/svelte-primitives";
</script>

<FormActions align="end">
  <Button variant="secondary">Cancel</Button>
  <Button variant="primary">Save</Button>
</FormActions>`,
  },

  "form-dialog": {
    props: [
      { name: "open", type: "boolean | null", default: "null", description: "Controlled open state." },
      { name: "title", type: "string", required: true, description: "Title of the form dialog." },
      { name: "description", type: "string | null", default: "null", description: "Description text below the title." },
      { name: "submitLabel", type: "string", default: '"Submit"', description: "Label for the submit button." },
      { name: "cancelLabel", type: "string", default: '"Cancel"', description: "Label for the cancel button." },
      { name: "submitting", type: "boolean", default: "false", description: "Whether the form is currently submitting." },
      { name: "error", type: "string | null", default: "null", description: "Error message shown in the dialog." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the dialog." },
    ],
    slots: [
      { name: "default", description: "Form content. Receives: { submitting }." },
    ],
    events: [
      { name: "submit", payload: "void", description: "Fires when the form is submitted." },
      { name: "cancel", payload: "void", description: "Fires when the dialog is cancelled." },
      { name: "openChange", payload: "{ open: boolean }", description: "Fires when the open state changes." },
    ],
    usage: `<script lang="ts">
  import { FormDialog } from "@pug/svelte-composites";

  let open = false;
  let submitting = false;
</script>

<FormDialog bind:open title="Create User" {submitting} on:submit={() => { submitting = true; }}>
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
  import { FormLayout } from "@pug/svelte-composites";
  import { Field, Button } from "@pug/svelte-primitives";
</script>

<FormLayout columns={2} description="Enter your details below.">
  <Field id="first" label="First Name" isRequired>
    <input id="first" type="text" />
  </Field>
  <Field id="last" label="Last Name" isRequired>
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
  import { Grid } from "@pug/svelte-primitives";
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
  import { HoverCard } from "@pug/svelte-primitives";
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
      { name: "icon", type: "IconNodes | string | null", required: true, description: "The icon to display. Pass an IconNodes array (from @pug/icons-lucide or lucide-static) for tree-shaking, or a string name to resolve from an IconProvider set or the built-in internals." },
      { name: "name", type: "string | null", default: "null", description: "Deprecated. Use icon instead. Alias kept for internal convenience." },
      { name: "size", type: "ControlSize", default: '"md"', description: "Size of the icon." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the icon. When set, the SVG receives role=\"img\"; otherwise it is role=\"presentation\"." },
    ],
    slots: [],
    events: [],
    usage: `<!-- Tree-shakeable: import individual icons from @pug/icons-lucide -->
<script lang="ts">
  import { Icon } from "@pug/svelte-primitives";
  import { search, heart, star } from "@pug/icons-lucide";
</script>

<Icon icon={search} size="md" ariaLabel="Search" />
<Icon icon={heart} size="sm" />
<Icon icon={star} size="lg" />

<!-- String name: resolves from IconProvider or built-in internals -->
<Icon icon="chevron-down" size="sm" />

<!-- Bulk icon set via provider -->
<script lang="ts">
  import { Icon, IconProvider } from "@pug/svelte-primitives";
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
  import { IconProvider, Icon } from "@pug/svelte-primitives";
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
      { name: "size", type: "ControlSize", default: '"md"', description: "Size of the button." },
      { name: "icon", type: "IconProp", required: true, description: "The icon to display. Accepts an IconNodes array or a string name." },
      { name: "ariaLabel", type: "string", required: true, description: "Accessible label for the button (required since there is no text)." },
      { name: "tooltip", type: "string | null", default: "null", description: "Tooltip text shown on hover." },
      { name: "tooltipPlacement", type: "OverlayPlacement", default: '"top"', description: "Placement of the tooltip." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the button is disabled." },
      { name: "isLoading", type: "boolean", default: "false", description: "Whether the button shows a loading indicator." },
      { name: "isPressed", type: "boolean | null", default: "null", description: "Pressed state for toggle buttons." },
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
  import { IconButton } from "@pug/svelte-primitives";
  import { trash2, plus, settings } from "@pug/icons-lucide";
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
      { name: "leadingShape", type: '"circle" | "rounded-square"', default: '"circle"', description: "Shape of the leading visual element." },
      { name: "leadingFill", type: '"tint" | "solid"', default: '"tint"', description: "Fill style of the leading visual." },
      { name: "accentColor", type: "string | null", default: "null", description: "Accent color for the card." },
      { name: "isInteractive", type: "boolean", default: "false", description: "Whether the card is clickable." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the card is disabled." },
      { name: "isNotLive", type: "boolean", default: "false", description: "Whether to show the card in a non-live state." },
      { name: "sash", type: "string | null", default: "null", description: "Sash label text." },
      { name: "sashColor", type: "string | null", default: "null", description: "Color of the sash." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the card." },
    ],
    slots: [
      { name: "leading", description: "Leading visual content (icon, avatar, etc.)." },
      { name: "badges", description: "Badge elements displayed on the card." },
      { name: "footer", description: "Footer content below the main body." },
      { name: "trailing", description: "Trailing content on the right side." },
    ],
    events: [
      { name: "click", payload: "MouseEvent", description: "Fires when the card is clicked." },
    ],
    usage: `<script lang="ts">
  import { ListCard } from "@pug/svelte-composites";
</script>

<ListCard title="Alice Johnson" subtitle="Software Engineer" meta="Active" isInteractive>
  <svelte:fragment slot="leading">
    <img src="/avatars/alice.jpg" alt="Alice" />
  </svelte:fragment>
</ListCard>`,
  },

  "log-list": {
    props: [
      { name: "entries", type: "LogEntry[]", default: "[]", description: "Array of log entries to display." },
      { name: "maxEntries", type: "number", default: "500", description: "Maximum number of entries to retain." },
      { name: "autoScroll", type: "boolean", default: "true", description: "Whether to auto-scroll to the latest entry." },
      { name: "filterLevel", type: "LogLevel | null", default: "null", description: "Filter entries by log level." },
      { name: "filterText", type: "string", default: '""', description: "Text filter applied to log entries." },
      { name: "ariaLabel", type: "string", default: '"Log output"', description: "Accessible label for the log list." },
    ],
    slots: [],
    events: [],
    usage: `<script lang="ts">
  import { LogList } from "@pug/svelte-composites";

  const entries = [
    { id: "1", level: "info", message: "Server started on port 3000", timestamp: Date.now() },
    { id: "2", level: "warn", message: "Deprecated API call detected", timestamp: Date.now() },
    { id: "3", level: "error", message: "Failed to connect to database", timestamp: Date.now() },
  ];
</script>

<LogList {entries} autoScroll />`,
  },

  "markdown-editor": {
    props: [
      { name: "value", type: "string", default: '""', description: "Current markdown content." },
      { name: "placeholder", type: "string", default: '"Write markdown..."', description: "Placeholder text when empty." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the editor is disabled." },
      { name: "ariaLabel", type: "string", default: '"Markdown editor"', description: "Accessible label for the editor." },
      { name: "minHeight", type: "string", default: '"12rem"', description: "Minimum height of the editor." },
      { name: "mode", type: '"edit" | "preview" | "split"', default: '"edit"', description: "Display mode of the editor." },
    ],
    slots: [],
    events: [
      { name: "change", payload: "{ value: string }", description: "Fires when the markdown content changes." },
    ],
    usage: `<script lang="ts">
  import { MarkdownEditor } from "@pug/svelte-composites";

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
    ],
    slots: [],
    events: [
      { name: "select", payload: "{ item: MediaPickerItem }", description: "Fires when a media item is selected." },
      { name: "upload", payload: "{ files: FileUploadItem[] }", description: "Fires when files are uploaded." },
      { name: "openChange", payload: "{ open: boolean }", description: "Fires when the open state changes." },
    ],
    usage: `<script lang="ts">
  import { MediaPicker } from "@pug/svelte-composites";

  let open = false;
  const items = [
    { id: "1", name: "hero.jpg", url: "/images/hero.jpg", type: "image" },
    { id: "2", name: "logo.png", url: "/images/logo.png", type: "image" },
  ];
</script>

<MediaPicker bind:open {items} accept="image/*" on:select={(e) => console.log(e.detail.item)} />`,
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
  import { MediaPreview } from "@pug/svelte-composites";
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
  import { MediaThumbnail } from "@pug/svelte-composites";
</script>

<MediaThumbnail kind="video" aspectRatio="landscape" badge="HD" meta="3:24">
  <img src="/thumbnails/video-preview.jpg" alt="Video preview" />
</MediaThumbnail>`,
  },

  menu: {
    props: [
      { name: "items", type: "MenuItem[]", default: "[]", description: "Array of menu items to display." },
      { name: "open", type: "boolean | null", default: "null", description: "Controlled open state." },
      { name: "defaultOpen", type: "boolean", default: "false", description: "Initial open state for uncontrolled mode." },
      { name: "placement", type: "OverlayPlacement", default: '"bottom-start"', description: "Preferred placement of the menu." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the menu." },
    ],
    slots: [
      { name: "trigger", description: "Element that opens the menu." },
    ],
    events: [
      { name: "openChange", payload: "{ open: boolean }", description: "Fires when the open state changes." },
      { name: "action", payload: "{ value: string }", description: "Fires when a menu item is selected." },
    ],
    usage: `<script lang="ts">
  import { Menu } from "@pug/svelte-primitives";
  import { Button } from "@pug/svelte-primitives";

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
  import { Menubar } from "@pug/svelte-primitives";

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
  import { Meter } from "@pug/svelte-primitives";
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
  import { MetricTile } from "@pug/svelte-composites";
</script>

<MetricTile label="Revenue" value="$48,200" trend="up" trendLabel="+12.5%" sparklineData={[20, 35, 28, 42, 48]} />`,
  },

  "nav-card": {
    props: [
      { name: "title", type: "string", required: true, description: "Title text displayed on the card." },
      { name: "description", type: "string | null", default: "null", description: "Description text below the title." },
      { name: "href", type: "string | null", default: "null", description: "Link URL for the card navigation." },
      { name: "badge", type: "string | null", default: "null", description: "Badge label shown on the card." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the card is disabled." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the nav card." },
    ],
    slots: [
      { name: "icon", description: "Icon content displayed in the card." },
    ],
    events: [
      { name: "click", payload: "MouseEvent", description: "Fires when the card is clicked." },
    ],
    usage: `<script lang="ts">
  import { NavCard } from "@pug/svelte-primitives";
</script>

<NavCard title="Dashboard" description="View your analytics" href="/dashboard" badge="New">
  <svelte:fragment slot="icon">📊</svelte:fragment>
</NavCard>`,
  },

  "navigation-menu": {
    props: [
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
  import { NavigationMenu } from "@pug/svelte-primitives";

  const items = [
    { value: "products", label: "Products" },
    { value: "pricing", label: "Pricing" },
    { value: "docs", label: "Documentation" },
  ];
</script>

<NavigationMenu {items} ariaLabel="Main navigation" />`,
  },

  "number-entry": {
    props: [
      { name: "id", type: "string", required: true, description: "HTML id attribute for the input." },
      { name: "value", type: "number | null", default: "null", description: "Controlled numeric value." },
      { name: "defaultValue", type: "number | null", default: "null", description: "Initial value for uncontrolled mode." },
      { name: "placeholder", type: "string | null", default: "null", description: "Placeholder text when empty." },
      { name: "min", type: "number | null", default: "null", description: "Minimum allowed value." },
      { name: "max", type: "number | null", default: "null", description: "Maximum allowed value." },
      { name: "step", type: "number", default: "1", description: "Step increment for value changes." },
      { name: "precision", type: "number | null", default: "null", description: "Number of decimal places to display." },
      { name: "name", type: "string | undefined", default: "undefined", description: "Form field name." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the input is disabled." },
      { name: "isReadOnly", type: "boolean", default: "false", description: "Whether the input is read-only." },
      { name: "validationState", type: "ValidationState", default: '"none"', description: "Validation state of the input." },
      { name: "showSteppers", type: "boolean", default: "false", description: "Whether to show increment/decrement stepper buttons." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the input." },
      { name: "describedBy", type: "string | null", default: "null", description: "ID of the element describing this input." },
    ],
    slots: [],
    events: [
      { name: "valueChange", payload: "{ value: number | null }", description: "Fires when the numeric value changes." },
      { name: "submit", payload: "{ value: number | null }", description: "Fires when the value is submitted (e.g. Enter key)." },
      { name: "increment", payload: "{ value: number | null }", description: "Fires when the value is incremented." },
      { name: "decrement", payload: "{ value: number | null }", description: "Fires when the value is decremented." },
      { name: "focus", payload: "FocusEvent", description: "Fires when the input receives focus." },
      { name: "blur", payload: "FocusEvent", description: "Fires when the input loses focus." },
    ],
    usage: `<script lang="ts">
  import { NumberEntry } from "@pug/svelte-primitives";

  let quantity: number | null = 1;
</script>

<NumberEntry id="quantity" bind:value={quantity} min={0} max={100} step={1} showSteppers />`,
  },

  "order-by": {
    props: [
      { name: "fields", type: "SortField[]", default: "[]", description: "Array of sortable field definitions." },
      { name: "activeSort", type: "ActiveSort | null", default: "null", description: "Currently active sort configuration." },
      { name: "ariaLabel", type: "string", default: '"Sort by"', description: "Accessible label for the sort control." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the sort control is disabled." },
    ],
    slots: [],
    events: [
      { name: "change", payload: "{ sort: ActiveSort | null }", description: "Fires when the sort configuration changes." },
    ],
    usage: `<script lang="ts">
  import { OrderBy } from "@pug/svelte-primitives";

  const fields = [
    { value: "name", label: "Name" },
    { value: "date", label: "Date" },
    { value: "priority", label: "Priority" },
  ];
</script>

<OrderBy {fields} on:change={(e) => console.log(e.detail.sort)} />`,
  },

  "page-header": {
    props: [
      { name: "title", type: "string", required: true, description: "Primary page title." },
      { name: "subtitle", type: "string | null", default: "null", description: "Subtitle text below the title." },
      { name: "eyebrow", type: "string | null", default: "null", description: "Eyebrow label above the title." },
      { name: "align", type: '"start" | "between"', default: '"between"', description: "Alignment of the header content and actions." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the header region." },
    ],
    slots: [
      { name: "breadcrumbs", description: "Breadcrumb navigation rendered above the title." },
      { name: "actions", description: "Action buttons rendered on the right side of the header." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { PageHeader } from "@pug/svelte-composites";
  import { Button } from "@pug/svelte-primitives";
</script>

<PageHeader title="All Products" eyebrow="Catalog" subtitle="Manage your product listings">
  <svelte:fragment slot="actions">
    <Button variant="primary" leadingIcon="plus">Add Product</Button>
  </svelte:fragment>
</PageHeader>`,
  },

  "page-loading": {
    props: [
      { name: "isVisible", type: "boolean", default: "true", description: "Whether the loading overlay is visible." },
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
  import { PageLoading } from "@pug/svelte-composites";
</script>

<PageLoading isVisible message="Loading your data..." canCancel on:cancel={() => abortRequest()} />`,
  },

  pagination: {
    props: [
      { name: "currentPage", type: "number", default: "1", description: "Currently active page number." },
      { name: "totalPages", type: "number", default: "1", description: "Total number of pages." },
      { name: "siblingCount", type: "number", default: "1", description: "Number of sibling pages shown around the current page." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the pagination nav." },
    ],
    slots: [],
    events: [
      { name: "pageChange", payload: "{ page: number }", description: "Fires when the current page changes." },
    ],
    usage: `<script lang="ts">
  import { Pagination } from "@pug/svelte-primitives";

  let page = 1;
</script>

<Pagination currentPage={page} totalPages={10} on:pageChange={(e) => (page = e.detail.page)} />`,
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
  import { PaginationSummary } from "@pug/svelte-primitives";
</script>

<PaginationSummary currentPage={1} totalPages={10} totalItems={50} pageSize={5} />`,
  },

  "panel-tabs": {
    props: [
      { name: "items", type: "PanelTabItem[]", default: "[]", description: "Array of panel tab items." },
      { name: "value", type: "string | null", default: "null", description: "Currently active tab value." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the panel tab bar." },
    ],
    slots: [],
    events: [
      { name: "valueChange", payload: "{ value: string }", description: "Fires when the active tab changes." },
      { name: "close", payload: "{ value: string }", description: "Fires when a tab close button is clicked." },
      { name: "reorder", payload: "{ items: string[] }", description: "Fires when tabs are reordered." },
    ],
    usage: `<script lang="ts">
  import { PanelTabs } from "@pug/svelte-composites";

  let activeTab = "explorer";

  const items = [
    { value: "explorer", label: "Explorer", icon: "folder" },
    { value: "search", label: "Search", icon: "search" },
    { value: "git", label: "Source Control", icon: "git-branch" },
  ];
</script>

<PanelTabs {items} value={activeTab} on:valueChange={(e) => (activeTab = e.detail.value)} />`,
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
  import { PickerShell } from "@pug/svelte-composites";
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
      { name: "size", type: "PillSize", default: '"xs"', description: "Size of the pill." },
      { name: "font", type: "PillFont", default: '"normal"', description: "Font style of the pill label." },
      { name: "isMuted", type: "boolean", default: "false", description: "Whether the pill uses muted styling." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the pill." },
    ],
    slots: [
      { name: "default", description: "Pill label content." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { Pill } from "@pug/svelte-primitives";
</script>

<Pill tone="success" appearance="solid">Active</Pill>
<Pill tone="warning" appearance="solid" isMuted>Pending</Pill>`,
  },

  "pin-input": {
    props: [
      { name: "value", type: "string | null", default: "null", description: "Controlled value of the pin input." },
      { name: "defaultValue", type: "string", default: '""', description: "Initial value for uncontrolled mode." },
      { name: "length", type: "number", default: "6", description: "Number of character fields." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the input is disabled." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the pin input." },
      { name: "mask", type: "boolean", default: "false", description: "Whether to mask input characters." },
    ],
    slots: [],
    events: [
      { name: "valueChange", payload: "{ value: string }", description: "Fires when the pin value changes." },
      { name: "complete", payload: "{ value: string }", description: "Fires when all character fields are filled." },
    ],
    usage: `<script lang="ts">
  import { PinInput } from "@pug/svelte-primitives";
</script>

<PinInput length={6} mask on:complete={(e) => verify(e.detail.value)} />`,
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
  import { Popover, Button } from "@pug/svelte-primitives";
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
      { name: "isIndeterminate", type: "boolean", default: "false", description: "Whether the progress is indeterminate." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the progress bar." },
      { name: "valueText", type: "string | null", default: "null", description: "Human-readable text for the current value." },
    ],
    slots: [],
    events: [],
    usage: `<script lang="ts">
  import { Progress } from "@pug/svelte-primitives";
</script>

<Progress value={65} max={100} ariaLabel="Upload progress" />
<Progress isIndeterminate ariaLabel="Loading..." />`,
  },

  "radio-group": {
    props: [
      { name: "value", type: "string | null", default: "null", description: "Controlled selected value." },
      { name: "defaultValue", type: "string | null", default: "null", description: "Initial value for uncontrolled mode." },
      { name: "options", type: "RadioGroupOption[]", default: "[]", description: "Array of radio option definitions." },
      { name: "orientation", type: "Orientation", default: '"vertical"', description: "Layout orientation of the radio options." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the group is disabled." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the radio group." },
      { name: "describedBy", type: "string | null", default: "null", description: "ID of the element describing this group." },
      { name: "name", type: "string | undefined", default: "undefined", description: "Form field name." },
    ],
    slots: [],
    events: [
      { name: "valueChange", payload: "{ value: string }", description: "Fires when the selected value changes." },
    ],
    usage: `<script lang="ts">
  import { RadioGroup } from "@pug/svelte-primitives";

  const options = [
    { value: "sm", label: "Small" },
    { value: "md", label: "Medium" },
    { value: "lg", label: "Large" },
  ];

  let size = "md";
</script>

<RadioGroup {options} bind:value={size} orientation="vertical" />`,
  },

  "range-calendar": {
    props: [
      { name: "value", type: "DateRangeValue | null", default: "null", description: "Controlled date range value." },
      { name: "defaultValue", type: "DateRangeValue", default: "{ start: null, end: null }", description: "Initial date range for uncontrolled mode." },
      { name: "visibleMonth", type: "string | null", default: "null", description: "Controlled visible month in YYYY-MM format." },
      { name: "weekStartsOn", type: "CalendarWeekStart", default: '"monday"', description: "Which day the calendar week starts on." },
      { name: "locale", type: "string", default: '"en-US"', description: "Locale for date formatting." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the calendar is disabled." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the range calendar." },
    ],
    slots: [],
    events: [
      { name: "valueChange", payload: "{ value: DateRangeValue }", description: "Fires when the selected date range changes." },
      { name: "monthChange", payload: "{ month: string }", description: "Fires when the visible month changes." },
    ],
    usage: `<script lang="ts">
  import { RangeCalendar } from "@pug/svelte-primitives";

  let range = { start: null, end: null };
</script>

<RangeCalendar bind:value={range} locale="en-US" />`,
  },

  "range-slider": {
    props: [
      { name: "value", type: "[number, number]", default: "[0, 100]", description: "Controlled range value as [min, max] tuple." },
      { name: "min", type: "number", default: "0", description: "Minimum allowed value." },
      { name: "max", type: "number", default: "100", description: "Maximum allowed value." },
      { name: "step", type: "number", default: "1", description: "Step increment between values." },
      { name: "orientation", type: "Orientation", default: '"horizontal"', description: "Layout orientation of the slider." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the slider is disabled." },
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
  import { RangeSlider } from "@pug/svelte-primitives";

  let priceRange: [number, number] = [20, 80];
</script>

<RangeSlider bind:value={priceRange} min={0} max={100} step={5} />`,
  },

  rating: {
    props: [
      { name: "value", type: "number | null", default: "null", description: "Controlled rating value." },
      { name: "defaultValue", type: "number | null", default: "null", description: "Initial value for uncontrolled mode." },
      { name: "max", type: "number", default: "5", description: "Maximum number of stars." },
      { name: "allowClear", type: "boolean", default: "false", description: "Whether clicking the current value clears the rating." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the rating input is disabled." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the rating." },
    ],
    slots: [],
    events: [
      { name: "valueChange", payload: "{ value: number | null }", description: "Fires when the rating value changes." },
    ],
    usage: `<script lang="ts">
  import { Rating } from "@pug/svelte-primitives";

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
  import { Region } from "@pug/svelte-primitives";
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
  import { RelationPicker } from "@pug/svelte-composites";

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
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether reordering is disabled." },
    ],
    slots: [
      { name: "item", description: "Custom item renderer. Receives slot props: item, index." },
    ],
    events: [
      { name: "reorder", payload: "{ items: ReorderableItem[] }", description: "Fires when items are reordered." },
    ],
    usage: `<script lang="ts">
  import { ReorderableList } from "@pug/svelte-composites";

  let items = [
    { id: "1", label: "First" },
    { id: "2", label: "Second" },
    { id: "3", label: "Third" },
  ];
</script>

<ReorderableList bind:items on:reorder={(e) => (items = e.detail.items)} />`,
  },

  "resize-handle": {
    props: [
      { name: "orientation", type: "SplitOrientation", default: '"horizontal"', description: "Orientation of the resize handle." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the handle is disabled." },
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
  import { ResizeHandle } from "@pug/svelte-primitives";
</script>

<ResizeHandle orientation="horizontal" ariaLabel="Resize sidebar" ariaValueNow={50} />`,
  },

  "scroll-shell": {
    props: [
      { name: "direction", type: "ScrollDirection", default: '"vertical"', description: "Scroll direction of the container." },
      { name: "padding", type: "SpaceScale", default: '"none"', description: "Inner padding of the scroll area." },
      { name: "asRole", type: '"region" | "group" | null', default: "null", description: "Semantic ARIA role for the container." },
      { name: "label", type: "string | null", default: "null", description: "Accessible label for the scrollable region." },
      { name: "isFocusable", type: "boolean", default: "false", description: "Whether the scroll region is keyboard-focusable." },
    ],
    slots: [
      { name: "default", description: "Scrollable content." },
    ],
    events: [
      { name: "scroll", payload: "Event", description: "Fires when the container is scrolled." },
    ],
    usage: `<script lang="ts">
  import { ScrollShell } from "@pug/svelte-primitives";
</script>

<ScrollShell direction="vertical" padding="md" style="max-height: 300px;">
  <p>Long scrollable content goes here...</p>
</ScrollShell>`,
  },

  "search-field": {
    props: [
      { name: "id", type: "string", required: true, description: "HTML id attribute for the input." },
      { name: "value", type: "string | null", default: "null", description: "Controlled search value." },
      { name: "defaultValue", type: "string", default: '""', description: "Initial value for uncontrolled mode." },
      { name: "placeholder", type: "string", default: '"Search"', description: "Placeholder text." },
      { name: "ariaLabel", type: "string", default: '"Search"', description: "Accessible label for the search field." },
      { name: "describedBy", type: "string | null", default: "null", description: "ID of the element describing this field." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the field is disabled." },
      { name: "isReadOnly", type: "boolean", default: "false", description: "Whether the field is read-only." },
      { name: "showClearButton", type: "boolean", default: "true", description: "Whether to show the clear button." },
      { name: "validationState", type: "ValidationState", default: '"none"', description: "Validation state of the field." },
    ],
    slots: [],
    events: [
      { name: "valueChange", payload: "{ value: string }", description: "Fires when the search value changes." },
      { name: "submit", payload: "{ value: string }", description: "Fires when the search is submitted." },
      { name: "clear", payload: "void", description: "Fires when the field is cleared." },
      { name: "cancel", payload: "void", description: "Fires when the search is cancelled." },
    ],
    usage: `<script lang="ts">
  import { SearchField } from "@pug/svelte-primitives";

  let query = "";
</script>

<SearchField id="search" bind:value={query} placeholder="Search products..." on:submit={(e) => search(e.detail.value)} />`,
  },

  "segmented-control": {
    props: [
      { name: "value", type: "string | null", default: "null", description: "Controlled selected value." },
      { name: "defaultValue", type: "string | null", default: "null", description: "Initial value for uncontrolled mode." },
      { name: "options", type: "SegmentedControlOption[]", default: "[]", description: "Array of segment option definitions." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the control is disabled." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the segmented control." },
      { name: "name", type: "string | undefined", default: "undefined", description: "Form field name." },
    ],
    slots: [],
    events: [
      { name: "valueChange", payload: "{ value: string }", description: "Fires when the selected segment changes." },
    ],
    usage: `<script lang="ts">
  import { SegmentedControl } from "@pug/svelte-primitives";

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
      { name: "id", type: "string | undefined", default: "undefined", description: "HTML id attribute for the select." },
      { name: "value", type: "string | null", default: "null", description: "Controlled selected value." },
      { name: "defaultValue", type: "string | null", default: "null", description: "Initial value for uncontrolled mode." },
      { name: "placeholder", type: "string | null", default: "null", description: "Placeholder text when no option is selected." },
      { name: "options", type: "SelectItems", default: "[]", description: "Array of select options or option groups." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the select is disabled." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the select." },
      { name: "describedBy", type: "string | null", default: "null", description: "ID of the element describing this select." },
      { name: "name", type: "string | undefined", default: "undefined", description: "Form field name." },
    ],
    slots: [],
    events: [
      { name: "valueChange", payload: "{ value: string }", description: "Fires when the selected value changes." },
    ],
    usage: `<script lang="ts">
  import { Select } from "@pug/svelte-primitives";

  const options = [
    { value: "draft", label: "Draft" },
    { value: "published", label: "Published" },
    { value: "archived", label: "Archived" },
  ];
</script>

<Select {options} placeholder="Select status..." />`,
  },

  "selection-summary": {
    props: [
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
  import { SelectionSummary } from "@pug/svelte-composites";

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
  import { Separator } from "@pug/svelte-primitives";
</script>

<p>Section one content</p>
<Separator />
<p>Section two content</p>`,
  },

  skeleton: {
    props: [
      { name: "shape", type: "SkeletonShape", default: '"line"', description: "Shape of the skeleton placeholder." },
      { name: "preset", type: "SkeletonPreset | null", default: "null", description: "Predefined skeleton layout preset." },
      { name: "width", type: "string | null", default: "null", description: "CSS width of the skeleton." },
      { name: "height", type: "string | null", default: "null", description: "CSS height of the skeleton." },
      { name: "lines", type: "number", default: "3", description: "Number of lines when shape is 'line'." },
      { name: "isAnimated", type: "boolean", default: "true", description: "Whether the skeleton pulses with animation." },
    ],
    slots: [],
    events: [],
    usage: `<script lang="ts">
  import { Skeleton } from "@pug/svelte-primitives";
</script>

<Skeleton shape="circle" width="48px" height="48px" />
<Skeleton shape="line" lines={3} />`,
  },

  slider: {
    props: [
      { name: "value", type: "number", default: "0", description: "Current slider value." },
      { name: "min", type: "number", default: "0", description: "Minimum allowed value." },
      { name: "max", type: "number", default: "100", description: "Maximum allowed value." },
      { name: "step", type: "number", default: "1", description: "Step increment between values." },
      { name: "orientation", type: "Orientation", default: '"horizontal"', description: "Layout orientation of the slider." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the slider is disabled." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the slider." },
      { name: "valueText", type: "string | null", default: "null", description: "Human-readable text for the current value." },
    ],
    slots: [],
    events: [
      { name: "valueChange", payload: "{ value: number }", description: "Fires when the slider value changes during drag." },
      { name: "valueCommit", payload: "{ value: number }", description: "Fires when the slider value is committed (drag end)." },
    ],
    usage: `<script lang="ts">
  import { Slider } from "@pug/svelte-primitives";

  let volume = 50;
</script>

<Slider bind:value={volume} min={0} max={100} ariaLabel="Volume" />`,
  },

  "slug-field": {
    props: [
      { name: "id", type: "string", required: true, description: "HTML id attribute for the input." },
      { name: "label", type: "string", default: '"Slug"', description: "Label text for the field." },
      { name: "source", type: "string", default: '""', description: "Source text to auto-generate the slug from." },
      { name: "value", type: "string", default: '""', description: "Current slug value." },
      { name: "placeholder", type: "string", default: '"auto-generated-slug"', description: "Placeholder text when empty." },
      { name: "isManualOverride", type: "boolean", default: "false", description: "Whether the user has manually overridden the auto-generated slug." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the field is disabled." },
      { name: "validationState", type: "ValidationState", default: '"none"', description: "Validation state of the field." },
      { name: "error", type: "string | null", default: "null", description: "Error message text." },
      { name: "description", type: "string | null", default: '"URL-safe identifier auto-generated from the title."', description: "Help text below the field." },
      { name: "maxLength", type: "number | null", default: "null", description: "Maximum character length." },
      { name: "prefix", type: "string | null", default: "null", description: "URL prefix displayed before the slug." },
    ],
    slots: [],
    events: [
      { name: "change", payload: "{ value: string; isManual: boolean }", description: "Fires when the slug value changes." },
    ],
    usage: `<script lang="ts">
  import { SlugField } from "@pug/svelte-composites";

  let title = "My New Article";
  let slug = "";
</script>

<SlugField id="slug" source={title} bind:value={slug} prefix="/blog/" />`,
  },

  spacer: {
    props: [
      { name: "grow", type: "number", default: "1", description: "Flex grow factor." },
      { name: "minSize", type: "string | null", default: "null", description: "Minimum size of the spacer." },
    ],
    slots: [],
    events: [],
    usage: `<script lang="ts">
  import { Spacer } from "@pug/svelte-primitives";
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
      { name: "items", type: "MenuItem[]", default: "[]", description: "Array of menu items for the dropdown." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the button is disabled." },
      { name: "isLoading", type: "boolean", default: "false", description: "Whether the button shows a loading spinner." },
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
  import { SplitButton } from "@pug/svelte-primitives";

  const items = [
    { value: "save-draft", label: "Save as Draft" },
    { value: "schedule", label: "Schedule" },
  ];
</script>

<SplitButton variant="primary" {items} on:click={() => publish()} on:action={(e) => handleAction(e.detail.value)}>
  Publish
</SplitButton>`,
  },

  "split-view": {
    props: [
      { name: "orientation", type: "SplitOrientation", default: '"horizontal"', description: "Orientation of the split layout." },
      { name: "ratio", type: "number", default: "0.5", description: "Controlled split ratio (0 to 1)." },
      { name: "defaultRatio", type: "number", default: "0.5", description: "Initial split ratio for uncontrolled mode." },
      { name: "minPrimarySize", type: "number | null", default: "null", description: "Minimum pixel size of the primary pane." },
      { name: "minSecondarySize", type: "number | null", default: "null", description: "Minimum pixel size of the secondary pane." },
      { name: "isPrimaryCollapsed", type: "boolean", default: "false", description: "Whether the primary pane is collapsed." },
      { name: "isSecondaryCollapsed", type: "boolean", default: "false", description: "Whether the secondary pane is collapsed." },
      { name: "showCollapsePrimary", type: "boolean", default: "false", description: "Whether to show the primary collapse toggle." },
      { name: "showCollapseSecondary", type: "boolean", default: "false", description: "Whether to show the secondary collapse toggle." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the split view." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether resizing is disabled." },
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
  import { SplitView } from "@pug/svelte-composites";
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
  import { Stack, Button } from "@pug/svelte-primitives";
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
  import { StatusBar } from "@pug/svelte-primitives";
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
  import { StatusIndicator } from "@pug/svelte-primitives";
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
      { name: "isElevated", type: "boolean", default: "false", description: "Whether the surface has a box-shadow elevation." },
      { name: "asRole", type: '"region" | "group" | null', default: "null", description: "Semantic ARIA role for the container." },
      { name: "label", type: "string | null", default: "null", description: "Accessible label for the surface." },
    ],
    slots: [
      { name: "default", description: "Content rendered inside the surface." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { Surface } from "@pug/svelte-primitives";
</script>

<Surface tone="panel" border="subtle" padding="lg" isElevated>
  <p>Content in an elevated panel surface.</p>
</Surface>`,
  },

  switch: {
    props: [
      { name: "id", type: "string | undefined", default: "undefined", description: "HTML id attribute for the switch." },
      { name: "isChecked", type: "boolean | null", default: "null", description: "Controlled checked state." },
      { name: "defaultChecked", type: "boolean", default: "false", description: "Initial checked state for uncontrolled mode." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the switch is disabled." },
      { name: "isReadOnly", type: "boolean", default: "false", description: "Whether the switch is read-only." },
      { name: "label", type: "string | null", default: "null", description: "Label text displayed next to the switch." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label when no visible label is used." },
      { name: "describedBy", type: "string | null", default: "null", description: "ID of the element describing this switch." },
      { name: "name", type: "string | undefined", default: "undefined", description: "Form field name." },
    ],
    slots: [],
    events: [
      { name: "checkedChange", payload: "{ checked: boolean }", description: "Fires when the checked state changes." },
    ],
    usage: `<script lang="ts">
  import { Switch } from "@pug/svelte-primitives";

  let notifications = true;
</script>

<Switch label="Enable notifications" bind:isChecked={notifications} />`,
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
  import { Table } from "@pug/svelte-primitives";

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
      { name: "value", type: "string | null", default: "null", description: "Controlled active tab value." },
      { name: "defaultValue", type: "string | null", default: "null", description: "Initial active tab for uncontrolled mode." },
      { name: "items", type: "TabItem[]", default: "[]", description: "Array of tab items to render." },
      { name: "variant", type: "TabVariant", default: '"underline"', description: "Visual style variant of the tabs." },
      { name: "orientation", type: "Orientation", default: '"horizontal"', description: "Layout orientation of the tab list." },
      { name: "activationMode", type: "TabActivationMode", default: '"automatic"', description: "Whether tabs activate on focus or on click." },
      { name: "isReorderable", type: "boolean", default: "false", description: "Whether tabs can be reordered via drag." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the tab list." },
      { name: "showTooltips", type: "boolean", default: "false", description: "Whether to show tooltips on tab triggers." },
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
  import { Tabs } from "@pug/svelte-primitives";

  const items = [
    { value: "overview", label: "Overview" },
    { value: "settings", label: "Settings" },
  ];
</script>

<Tabs {items} defaultValue="overview">
  {#snippet default({ activeValue })}
    {#if activeValue === "overview"}
      <p>Overview content here.</p>
    {:else}
      <p>Settings content here.</p>
    {/if}
  {/snippet}
</Tabs>`,
  },

  "text-area": {
    props: [
      { name: "id", type: "string", required: true, description: "HTML id attribute for the textarea." },
      { name: "value", type: "string | null", default: "null", description: "Controlled value of the textarea." },
      { name: "defaultValue", type: "string", default: '""', description: "Initial value for uncontrolled mode." },
      { name: "placeholder", type: "string | null", default: "null", description: "Placeholder text when empty." },
      { name: "rows", type: "number", default: "4", description: "Number of visible text rows." },
      { name: "name", type: "string | undefined", default: "undefined", description: "Form field name." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the textarea is disabled." },
      { name: "isReadOnly", type: "boolean", default: "false", description: "Whether the textarea is read-only." },
      { name: "validationState", type: "ValidationState", default: '"none"', description: "Current validation state." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the textarea." },
      { name: "describedBy", type: "string | null", default: "null", description: "ID of the element describing this textarea." },
    ],
    slots: [],
    events: [
      { name: "valueChange", payload: "{ value: string }", description: "Fires when the textarea value changes." },
      { name: "submit", payload: "{ value: string }", description: "Fires on submit (e.g. Cmd+Enter)." },
      { name: "cancel", payload: "void", description: "Fires when editing is cancelled." },
      { name: "focus", payload: "FocusEvent", description: "Fires when the textarea receives focus." },
      { name: "blur", payload: "FocusEvent", description: "Fires when the textarea loses focus." },
    ],
    usage: `<script lang="ts">
  import { TextArea } from "@pug/svelte-primitives";

  let comment = "";
</script>

<TextArea id="comment" bind:value={comment} placeholder="Write a comment…" rows={4} />`,
  },

  "text-input": {
    props: [
      { name: "id", type: "string", required: true, description: "HTML id attribute for the input." },
      { name: "value", type: "string | null", default: "null", description: "Controlled value of the input." },
      { name: "defaultValue", type: "string", default: '""', description: "Initial value for uncontrolled mode." },
      { name: "placeholder", type: "string | null", default: "null", description: "Placeholder text when empty." },
      { name: "name", type: "string | undefined", default: "undefined", description: "Form field name." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the input is disabled." },
      { name: "isReadOnly", type: "boolean", default: "false", description: "Whether the input is read-only." },
      { name: "validationState", type: "ValidationState", default: '"none"', description: "Current validation state." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the input." },
      { name: "describedBy", type: "string | null", default: "null", description: "ID of the element describing this input." },
      { name: "inputMode", type: "string | null", default: "null", description: "Hint for virtual keyboard type." },
      { name: "type", type: 'HTMLInputElement["type"]', default: '"text"', description: "HTML input type attribute." },
      { name: "prefix", type: "string | null", default: "null", description: "Static prefix text shown inside the input." },
      { name: "suffix", type: "string | null", default: "null", description: "Static suffix text shown inside the input." },
      { name: "maxLength", type: "number | null", default: "null", description: "Maximum character length." },
      { name: "showCharCount", type: "boolean", default: "false", description: "Whether to display a character count." },
    ],
    slots: [
      { name: "leading", description: "Content rendered before the input (e.g. icon)." },
      { name: "trailing", description: "Content rendered after the input (e.g. icon)." },
    ],
    events: [
      { name: "valueChange", payload: "{ value: string }", description: "Fires when the input value changes." },
      { name: "submit", payload: "{ value: string }", description: "Fires on Enter key press." },
      { name: "cancel", payload: "void", description: "Fires when editing is cancelled." },
      { name: "focus", payload: "FocusEvent", description: "Fires when the input receives focus." },
      { name: "blur", payload: "FocusEvent", description: "Fires when the input loses focus." },
    ],
    usage: `<script lang="ts">
  import { TextInput } from "@pug/svelte-primitives";

  let email = "";
</script>

<TextInput id="email" bind:value={email} placeholder="you@example.com" type="email" />`,
  },

  "time-ago": {
    props: [
      { name: "datetime", type: "Date | string | number", required: true, description: "The date/time to display relative to now." },
      { name: "live", type: "boolean", default: "true", description: "Whether the display updates automatically." },
      { name: "interval", type: "number", default: "30000", description: "Update interval in milliseconds when live." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label override." },
    ],
    slots: [],
    events: [],
    usage: `<script lang="ts">
  import { TimeAgo } from "@pug/svelte-primitives";
</script>

<TimeAgo datetime={new Date("2026-03-20T12:00:00Z")} live />`,
  },

  "time-field": {
    props: [
      { name: "id", type: "string | null", default: "null", description: "HTML id attribute for the field." },
      { name: "value", type: "string | null", default: "null", description: "Controlled time value (e.g. \"14:30\")." },
      { name: "defaultValue", type: "string | null", default: "null", description: "Initial time value for uncontrolled mode." },
      { name: "min", type: "string | null", default: "null", description: "Minimum allowed time." },
      { name: "max", type: "string | null", default: "null", description: "Maximum allowed time." },
      { name: "step", type: "number", default: "60", description: "Step interval in seconds." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the field is disabled." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the time field." },
      { name: "describedBy", type: "string | null", default: "null", description: "ID of the element describing this field." },
    ],
    slots: [],
    events: [
      { name: "valueChange", payload: "{ value: string | null }", description: "Fires when the time value changes." },
    ],
    usage: `<script lang="ts">
  import { TimeField } from "@pug/svelte-primitives";

  let time = "09:00";
</script>

<TimeField id="start-time" bind:value={time} min="08:00" max="18:00" step={60} />`,
  },

  "time-zone-select": {
    props: [
      { name: "id", type: "string | undefined", default: "undefined", description: "HTML id attribute for the select." },
      { name: "value", type: "string | null", default: "null", description: "Controlled selected time zone." },
      { name: "defaultValue", type: "string | null", default: "null", description: "Initial time zone for uncontrolled mode." },
      { name: "placeholder", type: "string | null", default: '"Select time zone"', description: "Placeholder text when no value is selected." },
      { name: "options", type: "TimeZoneOption[]", default: "[]", description: "Available time zone options." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the select is disabled." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the select." },
      { name: "describedBy", type: "string | null", default: "null", description: "ID of the element describing this select." },
      { name: "name", type: "string | undefined", default: "undefined", description: "Form field name." },
    ],
    slots: [],
    events: [
      { name: "valueChange", payload: "{ value: string }", description: "Fires when the selected time zone changes." },
    ],
    usage: `<script lang="ts">
  import { TimeZoneSelect } from "@pug/svelte-primitives";

  let tz = "America/New_York";
</script>

<TimeZoneSelect id="tz" bind:value={tz} placeholder="Select time zone" />`,
  },

  "toast-stack": {
    props: [
      { name: "items", type: "ToastItem[]", default: "[]", description: "Array of toast notification items." },
      { name: "ariaLabel", type: "string", default: '"Notifications"', description: "Accessible label for the toast region." },
    ],
    slots: [],
    events: [
      { name: "dismiss", payload: "{ id: string }", description: "Fires when a toast is dismissed." },
      { name: "action", payload: "{ id: string }", description: "Fires when a toast action button is clicked." },
    ],
    usage: `<script lang="ts">
  import { ToastStack } from "@pug/svelte-composites";

  let toasts = [
    { id: "1", title: "Saved", description: "Your changes have been saved." },
  ];
</script>

<ToastStack items={toasts} on:dismiss={(e) => { toasts = toasts.filter(t => t.id !== e.detail.id); }} />`,
  },

  toggle: {
    props: [
      { name: "isPressed", type: "boolean | null", default: "null", description: "Controlled pressed state." },
      { name: "defaultPressed", type: "boolean", default: "false", description: "Initial pressed state for uncontrolled mode." },
      { name: "variant", type: "ToggleVariant", default: '"ghost"', description: "Visual style variant." },
      { name: "size", type: "ControlSize", default: '"md"', description: "Size of the toggle button." },
      { name: "layout", type: '"inline" | "stack"', default: '"inline"', description: "Layout direction of icon and label." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the toggle is disabled." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the toggle." },
      { name: "className", type: "string", default: '""', description: "Additional CSS class name." },
    ],
    slots: [
      { name: "default", description: "Content rendered inside the toggle button." },
    ],
    events: [
      { name: "pressedChange", payload: "{ pressed: boolean }", description: "Fires when the pressed state changes." },
    ],
    usage: `<script lang="ts">
  import { Toggle } from "@pug/svelte-primitives";

  let bold = false;
</script>

<Toggle bind:isPressed={bold} ariaLabel="Bold">
  <strong>B</strong>
</Toggle>`,
  },

  "toggle-group": {
    props: [
      { name: "value", type: "string | string[] | null", default: "null", description: "Controlled selected value(s)." },
      { name: "defaultValue", type: "string | string[] | null", default: "null", description: "Initial selected value(s) for uncontrolled mode." },
      { name: "options", type: "ToggleGroupOption[]", default: "[]", description: "Array of toggle options." },
      { name: "selectionMode", type: '"single" | "multiple"', default: '"single"', description: "Whether one or many toggles can be active." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the entire group is disabled." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the toggle group." },
    ],
    slots: [],
    events: [
      { name: "valueChange", payload: "{ value: string | string[] }", description: "Fires when the selected value(s) change." },
    ],
    usage: `<script lang="ts">
  import { ToggleGroup } from "@pug/svelte-primitives";

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
      { name: "orientation", type: "Orientation", default: '"horizontal"', description: "Layout orientation of the toolbar." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the toolbar." },
    ],
    slots: [
      { name: "default", description: "Toolbar items (buttons, separators, etc.)." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { Toolbar } from "@pug/svelte-primitives";
  import { Button } from "@pug/svelte-primitives";
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
  import { Tooltip } from "@pug/svelte-primitives";
  import { Button } from "@pug/svelte-primitives";
</script>

<Tooltip content="Save your work" placement="top">
  <Button>Save</Button>
</Tooltip>`,
  },

  "tri-state-switch": {
    props: [
      { name: "value", type: "TriStateValue", default: '"default"', description: "Current tri-state value." },
      { name: "options", type: "Record<TriStateValue, string>", default: '{ excluded: "Exclude", default: "Default", included: "Include" }', description: "Labels for each state." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the switch is disabled." },
      { name: "ariaLabel", type: "string", required: true, description: "Accessible label for the switch." },
    ],
    slots: [],
    events: [
      { name: "valueChange", payload: "{ value: TriStateValue }", description: "Fires when the tri-state value changes." },
    ],
    usage: `<script lang="ts">
  import { TriStateSwitch } from "@pug/svelte-primitives";

  let filter = "default";
</script>

<TriStateSwitch bind:value={filter} ariaLabel="Include archived items" />`,
  },

  "video-player": {
    props: [
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
  import { VideoPlayer } from "@pug/svelte-composites";
</script>

<VideoPlayer src="/videos/intro.mp4" poster="/images/poster.jpg" showCaptions />`,
  },

  "zoned-date-time-picker": {
    props: [
      { name: "value", type: "ZonedDateTimeValue | null", default: "null", description: "Controlled date/time/zone value." },
      { name: "defaultValue", type: "ZonedDateTimeValue", default: '{ date: null, time: null, timeZone: null }', description: "Initial value for uncontrolled mode." },
      { name: "open", type: "boolean | null", default: "null", description: "Controlled open state of the picker." },
      { name: "defaultOpen", type: "boolean", default: "false", description: "Initial open state for uncontrolled mode." },
      { name: "placeholder", type: "string", default: '"Select date, time, and zone"', description: "Placeholder text when no value is selected." },
      { name: "weekStartsOn", type: "CalendarWeekStart", default: '"monday"', description: "First day of the week in the calendar." },
      { name: "locale", type: "string", default: '"en-US"', description: "Locale for date formatting." },
      { name: "timeZoneOptions", type: "TimeZoneOption[]", default: "[]", description: "Available time zone options." },
      { name: "isDisabled", type: "boolean", default: "false", description: "Whether the picker is disabled." },
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
  import { ZonedDateTimePicker } from "@pug/svelte-primitives";

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
    ],
    slots: [],
    events: [
      { name: "itemSelect", payload: "{ id: string }", description: "Fires when an action item is selected." },
      { name: "activeChange", payload: "{ id: string | null }", description: "Fires when the active item changes." },
    ],
    usage: `<script lang="ts">
  import { ActionDiscoveryPanel } from "@pug/svelte-composites";

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
      { name: "isDragRegion", type: "boolean", default: "false", description: "Whether the header acts as a window drag region." },
      { name: "ariaLabel", type: "string | null", default: "null", description: "Accessible label for the header." },
    ],
    slots: [
      { name: "identity", description: "Custom content for the identity/logo area." },
      { name: "actions", description: "Primary action buttons in the header." },
      { name: "utility", description: "Utility controls (e.g. user menu, settings)." },
    ],
    events: [],
    usage: `<script lang="ts">
  import { AppHeader } from "@pug/svelte-composites";
  import { Button } from "@pug/svelte-primitives";
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
