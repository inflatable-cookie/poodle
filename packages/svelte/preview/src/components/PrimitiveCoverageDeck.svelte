<script lang="ts">
  import {
    Badge,
    Box,
    Calendar,
    Combobox,
    ContextMenu,
    DateRangePicker,
    DateTimePicker,
    DateTimeRangePicker,
    EditableLabel,
    Eyebrow,
    Grid,
    HoverCard,
    IconButton,
    Inline,
    Menubar,
    Meter,
    NavigationMenu,
    NumberEntry,
    Pagination,
    PinInput,
    Progress,
    RangeCalendar,
    RangeSlider,
    Rating,
    ScrollShell,
    Separator,
    Slider,
    Spacer,
    Stack,
    Surface,
    Tabs,
    TimeField,
    TimeZoneSelect,
    Tooltip,
    TriStateSwitch,
    ZonedDateTimePicker,
    type ComboboxOption,
    type DateRangeValue,
    type DateTimeRangeValue,
    type DateTimeValue,
    type MenubarItem,
    type MenuItem,
    type NavigationMenuItem,
    type TabItem,
    type TimeZoneOption,
    type TriStateValue,
    type ZonedDateTimeValue,
  } from "@pug/svelte-primitives";

  const comboboxOptions: ComboboxOption[] = [
    { value: "approval", label: "Approval review", description: "Client-facing sign-off lane." },
    { value: "qc", label: "Quality control", description: "Internal defect review lane." },
    { value: "delivery", label: "Delivery handoff", description: "Broadcast and archive outputs." },
  ];
  const contextMenuItems: MenuItem[] = [
    { value: "pin", label: "Pin current selection", shortcutLabel: "P" },
    { value: "favorite", label: "Favorite", kind: "checkbox", isChecked: true },
    { value: "divider", label: "divider", kind: "separator" },
    { value: "archive", label: "Archive target", shortcutLabel: "A" },
  ];
  const navigationItems: NavigationMenuItem[] = [
    { value: "overview", label: "Overview", description: "Shell and current review context." },
    { value: "delivery", label: "Delivery", description: "Handoff dates and downstream state." },
    { value: "activity", label: "Activity", description: "Recent review and publish events." },
  ];
  const menubarItems: MenubarItem[] = [
    {
      value: "file",
      label: "File",
      items: [
        { value: "new-review", label: "New review", shortcutLabel: "N" },
        { value: "duplicate", label: "Duplicate target", shortcutLabel: "D" },
      ],
    },
    {
      value: "view",
      label: "View",
      items: [
        { value: "show-metadata", label: "Show metadata", kind: "checkbox", isChecked: true },
        { value: "focus-command", label: "Focus command palette", shortcutLabel: "K" },
      ],
    },
  ];
  const tabStripSeedItems: TabItem[] = [
    { value: "review", label: "Review", isClosable: true },
    { value: "mix", label: "Mix", isClosable: true },
    { value: "exports", label: "Exports", isClosable: true },
  ];
  const timeZoneOptions: TimeZoneOption[] = [
    { value: "Europe/London", label: "Europe/London" },
    { value: "America/Los_Angeles", label: "America/Los_Angeles" },
    { value: "Asia/Tokyo", label: "Asia/Tokyo" },
  ];

  let comboboxValue: string | null = "approval";
  let navigationValue: string | null = "overview";
  let activeMenubar: string | null = null;
  let tabStripValue = "review";
  let tabStripItems = [...tabStripSeedItems];
  let primitiveActionLog =
    "Primitive coverage deck is active. Every public primitive now needs a direct specimen here or elsewhere in the shared demo.";
  let editableLabelValue = "Aura final review";
  let numberEntryValue = 82;
  let sliderValue = 64;
  let rangeSliderValue: [number, number] = [18, 82];
  let ratingValue = 4;
  let pinValue = "4821";
  let currentPage = 3;
  let calendarValue = "2026-03-18";
  let rangeCalendarValue: DateRangeValue = { start: "2026-03-18", end: "2026-03-21" };
  let dateRangeValue: DateRangeValue = { start: "2026-03-22", end: "2026-03-24" };
  let dateTimeValue: DateTimeValue = { date: "2026-03-19", time: "14:30" };
  let dateTimeRangeValue: DateTimeRangeValue = {
    start: { date: "2026-03-19", time: "09:30" },
    end: { date: "2026-03-19", time: "17:00" },
  };
  let timeFieldValue = "14:30";
  let timeZoneValue = "Europe/London";
  let triStateValue: TriStateValue = "included";
  let zonedDateTimeValue: ZonedDateTimeValue = {
    date: "2026-03-21",
    time: "16:45",
    timeZone: "Europe/London",
  };

  function reorderTabStrip(event: CustomEvent<{ items: string[] }>): void {
    const itemOrder = event.detail.items;
    const itemMap = new Map(tabStripItems.map((item) => [item.value, item]));

    tabStripItems = itemOrder
      .map((value) => itemMap.get(value))
      .filter((item): item is TabItem => item !== undefined);
    primitiveActionLog = `Tab strip reordered to ${itemOrder.join(" -> ")}.`;
  }

  function closeTabItem(event: CustomEvent<{ value: string }>): void {
    const nextItems = tabStripItems.filter((item) => item.value !== event.detail.value);

    tabStripItems = nextItems;
    if (!nextItems.some((item) => item.value === tabStripValue)) {
      tabStripValue = nextItems[0]?.value ?? "";
    }
    primitiveActionLog = `Closed tab "${event.detail.value}".`;
  }
</script>

<Surface
  tone="panel"
  border="default"
  padding="md"
  isElevated={true}
  asRole="region"
  label="Primitive coverage deck"
>
  <Stack gap="md">
    <div class="primitive-deck__header">
      <div>
        <Eyebrow>Full primitive coverage</Eyebrow>
        <h3>Every public primitive is directly reviewable in the shared demo</h3>
      </div>
      <Inline gap="sm" wrap={true} align="center">
        <Badge>63/63 primitives</Badge>
        <Badge variant="muted">comparison target</Badge>
      </Inline>
    </div>

    <p class="primitive-deck__summary">
      The workflow screens stay coherent, but long-tail primitives also need a stable home so GPUI can match the full
      public surface rather than only the obvious controls.
    </p>

    <Grid columns="repeat(auto-fit, minmax(18rem, 1fr))" gap="md">
      <Surface tone="elevated" border="subtle" padding="md" asRole="group" label="Layout primitives">
        <Stack gap="sm">
          <div>
            <Eyebrow>Layout and surfaces</Eyebrow>
            <h4>Structural primitives</h4>
          </div>
          <Box padding="sm" minHeight="3.25rem">
            <div class="primitive-box-fill">
              <strong>Box</strong>
              <span>padding, sizing, and overflow posture</span>
            </div>
          </Box>
          <Grid columns="repeat(2, minmax(0, 1fr))" gap="sm">
            <Surface tone="panel" border="subtle" padding="sm" asRole="group" label="Grid cell A">
              <strong>Grid</strong>
            </Surface>
            <Surface tone="panel" border="subtle" padding="sm" asRole="group" label="Grid cell B">
              <strong>cells</strong>
            </Surface>
          </Grid>
          <div class="primitive-inline-strip">
            <Inline gap="sm" wrap={true} align="center">
              <span>Inline</span>
              <Spacer minSize="1rem" />
              <Badge variant="muted">Spacer</Badge>
              <span>alignment</span>
            </Inline>
          </div>
          <Separator decorative={false} tone="default" />
          <ScrollShell direction="horizontal" padding="sm" asRole="region" label="Horizontal scroll sample" isFocusable={true}>
            <div class="primitive-scroll-row">
              <Inline gap="sm" wrap={false}>
                <Badge>ScrollShell</Badge>
                <Badge variant="muted">token-backed overflow</Badge>
                <Badge>keyboard focusable</Badge>
                <Badge variant="muted">review strip</Badge>
                <Badge>surface chip</Badge>
              </Inline>
            </div>
          </ScrollShell>
        </Stack>
      </Surface>

      <Surface tone="elevated" border="subtle" padding="md" asRole="group" label="Navigation primitives">
        <Stack gap="sm">
          <div>
            <Eyebrow>Navigation and overlay</Eyebrow>
            <h4>Menu-driven primitives</h4>
          </div>
          <NavigationMenu
            value={navigationValue}
            items={navigationItems}
            ariaLabel="Primitive coverage navigation menu"
            on:valueChange={(event) => (navigationValue = event.detail.value)}
          />
          <Menubar
            value={activeMenubar}
            items={menubarItems}
            ariaLabel="Primitive coverage menubar"
            on:valueChange={(event) => (activeMenubar = event.detail.value)}
            on:action={(event) => (primitiveActionLog = `Menubar action "${event.detail.value}" requested.`)}
          />
          <Tabs
            value={tabStripValue}
            items={tabStripItems}
            variant="card"
            isReorderable={true}
            ariaLabel="Primitive coverage tab strip"
            on:valueChange={(event) => (tabStripValue = event.detail.value)}
            on:reorder={reorderTabStrip}
            on:close={closeTabItem}
          />
          <Inline gap="sm" wrap={true}>
            <Tooltip content="Tooltip coverage lives in the demo too." placement="top">
              <span class="primitive-trigger">Hover or focus for tooltip</span>
            </Tooltip>
            <HoverCard ariaLabel="Hover card sample" placement="bottom-start">
              <span slot="trigger" class="primitive-trigger">Hover card trigger</span>
              <Stack gap="sm">
                <strong>HoverCard</strong>
                <p class="primitive-card-copy">Delayed hover and focus disclosure without leaving the shared demo shell.</p>
              </Stack>
            </HoverCard>
          </Inline>
          <ContextMenu
            items={contextMenuItems}
            ariaLabel="Primitive coverage context menu"
            on:action={(event) => (primitiveActionLog = `Context menu action "${event.detail.value}" requested.`)}
          >
            <div class="primitive-context-target">
              Right-click or press Shift+F10 for ContextMenu coverage.
            </div>
          </ContextMenu>
        </Stack>
      </Surface>

      <Surface tone="elevated" border="subtle" padding="md" asRole="group" label="Input primitives">
        <Stack gap="sm">
          <div>
            <Eyebrow>Inputs and values</Eyebrow>
            <h4>Long-tail form primitives</h4>
          </div>
          <Combobox
            value={comboboxValue}
            options={comboboxOptions}
            placeholder="Choose workflow"
            ariaLabel="Workflow combobox"
            on:valueChange={(event) => (comboboxValue = event.detail.value)}
          />
          <EditableLabel
            value={editableLabelValue}
            activationMode="enterOrSpace"
            ariaLabel="Editable review label"
            on:commit={(event) => (editableLabelValue = event.detail.value)}
          />
          <Grid columns="repeat(2, minmax(0, 1fr))" gap="sm">
            <NumberEntry
              id="primitive-number-entry"
              value={numberEntryValue}
              min={0}
              max={100}
              step={1}
              showSteppers={true}
              ariaLabel="Number entry"
              on:valueChange={(event) => (numberEntryValue = event.detail.value ?? 0)}
            />
            <TimeField
              id="primitive-time-field"
              value={timeFieldValue}
              ariaLabel="Time field"
              on:valueChange={(event) => (timeFieldValue = event.detail.value)}
            />
          </Grid>
          <Grid columns="repeat(2, minmax(0, 1fr))" gap="sm">
            <Slider
              value={sliderValue}
              min={0}
              max={100}
              ariaLabel="Single value slider"
              on:valueChange={(event) => (sliderValue = event.detail.value)}
            />
            <RangeSlider
              value={rangeSliderValue}
              min={0}
              max={100}
              ariaLabel="Range slider"
              on:valueChange={(event) => (rangeSliderValue = event.detail.value)}
            />
          </Grid>
          <TriStateSwitch
            value={triStateValue}
            ariaLabel="Review inclusion switch"
            on:valueChange={(event) => (triStateValue = event.detail.value)}
          />
          <Inline gap="md" wrap={true} align="center">
            <Rating
              value={ratingValue}
              max={5}
              allowClear={true}
              ariaLabel="Review confidence"
              on:valueChange={(event) => (ratingValue = event.detail.value)}
            />
            <PinInput
              value={pinValue}
              length={4}
              ariaLabel="Review access code"
              on:valueChange={(event) => (pinValue = event.detail.value)}
            />
            <IconButton icon="plus" ariaLabel="Add marker" />
          </Inline>
          <TimeZoneSelect
            id="primitive-time-zone"
            value={timeZoneValue}
            options={timeZoneOptions}
            ariaLabel="Time zone select"
            on:valueChange={(event) => (timeZoneValue = event.detail.value)}
          />
        </Stack>
      </Surface>

      <Surface tone="elevated" border="subtle" padding="md" asRole="group" label="Feedback primitives">
        <Stack gap="sm">
          <div>
            <Eyebrow>Progress and feedback</Eyebrow>
            <h4>Status primitives</h4>
          </div>
          <Progress value={72} max={100} ariaLabel="Publish progress" valueText="72%" />
          <Meter value={82} min={0} max={100} low={35} high={70} optimum={90} ariaLabel="Signal meter" />
          <Pagination
            currentPage={currentPage}
            totalPages={8}
            ariaLabel="Primitive coverage pagination"
            on:pageChange={(event) => (currentPage = event.detail.page)}
          />
          <div class="primitive-log">
            <span class="token-path">primitiveActionLog</span>
            <strong>{primitiveActionLog}</strong>
          </div>
        </Stack>
      </Surface>
    </Grid>

    <Surface tone="elevated" border="subtle" padding="md" asRole="group" label="Date and scheduling primitives">
      <Stack gap="md">
        <div>
          <Eyebrow>Date and scheduling</Eyebrow>
          <h4>Calendar and time primitives</h4>
        </div>

        <Grid columns="repeat(auto-fit, minmax(18rem, 1fr))" gap="md">
          <Calendar
            value={calendarValue}
            ariaLabel="Single date calendar"
            on:valueChange={(event) => (calendarValue = event.detail.value)}
          />
          <RangeCalendar
            value={rangeCalendarValue}
            ariaLabel="Date range calendar"
            on:valueChange={(event) => (rangeCalendarValue = event.detail.value)}
          />
          <DateRangePicker
            value={dateRangeValue}
            ariaLabel="Date range picker"
            on:valueChange={(event) => (dateRangeValue = event.detail.value)}
          />
          <DateTimePicker
            value={dateTimeValue}
            ariaLabel="Date time picker"
            on:valueChange={(event) => (dateTimeValue = event.detail.value)}
          />
          <DateTimeRangePicker
            value={dateTimeRangeValue}
            ariaLabel="Date time range picker"
            on:valueChange={(event) => (dateTimeRangeValue = event.detail.value)}
          />
          <ZonedDateTimePicker
            value={zonedDateTimeValue}
            timeZoneOptions={timeZoneOptions}
            ariaLabel="Zoned date time picker"
            on:valueChange={(event) => (zonedDateTimeValue = event.detail.value)}
          />
        </Grid>
      </Stack>
    </Surface>
  </Stack>
</Surface>

<style>
  .primitive-deck__header {
    display: flex;
    align-items: start;
    justify-content: space-between;
    gap: 1rem;
    flex-wrap: wrap;
  }

  .primitive-deck__summary {
    margin: 0;
    color: var(--pug-color-text-secondary);
  }

  .primitive-box-fill {
    display: grid;
    gap: 0.25rem;
    align-content: center;
    border: 0.0625rem dashed color-mix(in srgb, var(--pug-color-border-default) 72%, transparent);
    border-radius: var(--pug-radius-control);
    background: color-mix(in srgb, var(--pug-color-background-surface) 94%, transparent);
  }

  .primitive-inline-strip {
    min-height: 2.75rem;
    padding: 0.5rem 0.625rem;
    border: 0.0625rem solid color-mix(in srgb, var(--pug-color-border-subtle) 76%, transparent);
    border-radius: var(--pug-radius-control);
    background: color-mix(in srgb, var(--pug-color-background-surface) 96%, transparent);
  }

  .primitive-scroll-row {
    min-width: max-content;
    padding-right: 0.25rem;
  }

  .primitive-trigger {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-height: 2rem;
    padding: 0 0.75rem;
    border: 0.0625rem solid color-mix(in srgb, var(--pug-color-border-default) 78%, transparent);
    border-radius: var(--pug-radius-control);
    background: color-mix(in srgb, var(--pug-color-background-surface) 96%, transparent);
    color: var(--pug-color-text-primary);
    font-family: var(--pug-typography-label-family);
    font-size: 0.75rem;
    font-weight: 600;
  }

  .primitive-context-target {
    display: grid;
    place-items: center;
    min-height: 5.5rem;
    padding: 1rem;
    border: 0.0625rem dashed color-mix(in srgb, var(--pug-color-border-default) 72%, transparent);
    border-radius: var(--pug-radius-surface);
    background: color-mix(in srgb, var(--pug-color-background-surface) 92%, transparent);
    color: var(--pug-color-text-secondary);
    text-align: center;
  }

  .primitive-card-copy {
    margin: 0;
    color: var(--pug-color-text-secondary);
  }

  .primitive-log {
    display: grid;
    gap: 0.25rem;
    padding: 0.75rem;
    border: 0.0625rem solid color-mix(in srgb, var(--pug-color-border-subtle) 76%, transparent);
    border-radius: var(--pug-radius-control);
    background: color-mix(in srgb, var(--pug-color-background-surface) 94%, transparent);
  }

  h3,
  h4 {
    margin: 0;
  }
</style>
