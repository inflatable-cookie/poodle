<script lang="ts">
  import {
    Breadcrumbs,
    Card,
    DataTable,
    DetailRow,
    DetailSection,
    DetailShell,
    EmbedShell,
    EmptyState,
    FilterToolbar,
    GridShell,
    ListShell,
    MediaPreview,
    MediaThumbnail,
    PageHeader,
    PickerShell,
    RelationPicker,
    SelectionSummary,
    ToastStack,
  } from "@pug/svelte-composites";
  import {
    Banner,
    Button,
    Callout,
    DatePicker,
    Dialog,
    Drawer,
    Field,
    FormActions,
    Menu,
    Pill,
    Popover,
    RadioGroup,
    SearchField,
    SegmentedControl,
    Select,
    StatusIndicator,
    Switch,
    Table,
    Tabs,
    TextArea,
    TextInput,
    ToggleGroup,
    Toolbar,
    type MenuItem,
    type RadioGroupOption,
    type SelectOption,
    type SegmentedControlOption,
    type TabDefinition,
  } from "@pug/svelte-primitives";
  import {
    ActionDiscoveryPanel,
    AppHeader,
    DockRegion,
    PanelHeader,
    PanelSurface,
    PanelTabs,
    ProjectHeader,
    ShellStatusBar,
    SplitView,
    SurfaceTabs,
    WorkspaceShell,
  } from "@pug/svelte-workstation";
  import PrimitiveCoverageDeck from "./PrimitiveCoverageDeck.svelte";

  export let screenId: string;
  export let screenTabs: SegmentedControlOption[] = [];
  export let screenMeta: {
    title: string;
    eyebrow: string;
    summary: string;
  };
  export let theme = "loophole-studio";
  export let density = "compact";
  export let controlSize = "md";
  export let onScreenChange: (screenId: string) => void = () => {};
  export let demo: any;

  const handoffTargetOptions: SelectOption[] = [
    { value: "client-review", label: "Client review" },
    { value: "mix-qc", label: "Mix QC" },
    { value: "broadcast-delivery", label: "Broadcast delivery" },
  ];
  const deliveryModeOptions: RadioGroupOption[] = [
    { value: "revision", label: "Revision update" },
    { value: "approval", label: "Approval request" },
    { value: "handoff", label: "Downstream handoff" },
  ];
  const browsePresentationOptions: SegmentedControlOption[] = [
    { value: "table", label: "Table" },
    { value: "list", label: "List" },
    { value: "grid", label: "Grid" },
  ];
  const detailTabs: TabDefinition[] = [
    { value: "related", label: "Related" },
    { value: "activity", label: "Activity" },
  ];
  const shellMenuItems: MenuItem[] = [
    { value: "duplicate", label: "Duplicate review target", shortcutLabel: "⌘D" },
    { value: "share", label: "Share review link", shortcutLabel: "⌘⇧S" },
    { kind: "separator", value: "divider-1", label: "divider" },
    { value: "focus-command", label: "Focus command palette", shortcutLabel: "⌘K" },
  ];
  const workspacePanelItems = [
    { value: "overview", label: "Overview" },
    { value: "workspace", label: "Workspace" },
    { value: "snapshot", label: "Snapshot" },
  ];

  let deliveryNotes =
    "Needs one final pass on automation smoothness before client-facing approval.";
  let handoffTarget = "client-review";
  let deliveryMode = "approval";
  let notifyClient = true;
  let publishOn = "2026-03-15";
  let browsePresentation = "table";
  let detailTab = "related";
  let formDialogOpen = false;
  let pickerDrawerOpen = false;
  let activeWorkspacePanel = "overview";
  let shellMenuLog = "Shell actions remain host-owned, but menu structure is now part of the shared demo.";

  $: relationSummaryItems = (demo.relationItems ?? [])
    .filter((item: { id: string }) => (demo.selectedRelationIds ?? []).includes(item.id))
    .map((item: { id: string; label: string }) => ({ id: item.id, label: item.label }));
  $: browseStatusSelectOptions = (demo.browseStatusOptions ?? []).map(
    (option: { value: string; label: string }) => ({
      value: option.value,
      label: option.label,
    }),
  );
  $: visibleBrowseTableRows = (demo.visibleBrowseRows ?? []).slice(0, 4).map((row: any) => ({
    id: row.id,
    cells: {
      title: row.title,
      kind: row.kind,
      owner: row.owner,
    },
  }));
  $: selectedTone =
    demo.workspaceState === "disconnected"
      ? "danger"
      : demo.workspaceState === "offline"
        ? "warning"
        : demo.busy
          ? "pending"
          : "success";

  function handleShellMenuAction(event: CustomEvent<{ value: string }>): void {
    shellMenuLog = `Shell menu action "${event.detail.value}" requested.`;

    if (event.detail.value === "focus-command") {
      demo.openCommandPalette();
    }
  }

  function removeSelectedRelation(event: CustomEvent<{ id: string }>): void {
    const nextSelectedIds = (demo.selectedRelationIds ?? []).filter((id: string) => id !== event.detail.id);
    demo.handleRelationSelectionChange(new CustomEvent("selectionChange", { detail: { selectedIds: nextSelectedIds } }));
  }

  function clearSelectedRelations(): void {
    demo.handleRelationSelectionChange(new CustomEvent("selectionChange", { detail: { selectedIds: [] } }));
  }
</script>

<section class="panel shared-demo-shell-panel" aria-labelledby="shared-demo-heading">
  <div class="section-header">
    <div>
      <p class="eyebrow">Shared demo target</p>
      <h2 id="shared-demo-heading">Svelte parity target for the GPUI demo app</h2>
    </div>
  </div>
  <div class="section-meta-bar">
    <span class="command-shortcut-hint">shared-demo-app</span>
    <span class="token-path">packages/shared-demo-app-contract.json</span>
    <span class="token-path">g04.015 must match this shell and screen model</span>
  </div>

  <WorkspaceShell state="ready" activeSurfaceLabel={screenMeta.title} ariaLabel="Shared demo app shell">
    <AppHeader slot="appHeader" title="Pug shared demo" ariaLabel="Shared demo app header">
      <div slot="identity" class="workspace-identity">
        <span class="workspace-identity__mark" aria-hidden="true">P</span>
        <strong>Shared demo target</strong>
      </div>
      <div slot="actions" class="action-cluster">
        <Menu items={shellMenuItems} ariaLabel="Shared demo shell actions" on:action={handleShellMenuAction}>
          <div slot="trigger">
            <Button variant="secondary">Shell menu</Button>
          </div>
        </Menu>
        <Button variant="primary" on:click={demo.openCommandPalette}>Open commands</Button>
      </div>
      <div slot="utility" class="workspace-status-pill-row">
        <StatusIndicator status={selectedTone} label={demo.workspaceState} />
        <Pill appearance="subtle">{theme}</Pill>
        <Pill appearance="subtle">{density}</Pill>
        <Pill appearance="subtle">{controlSize}</Pill>
      </div>
    </AppHeader>

    <ProjectHeader
      slot="projectHeader"
      title={screenMeta.title}
      subtitle={screenMeta.summary}
      isDirty={demo.invalid || demo.busy}
    >
      <div slot="actions" class="action-cluster">
        <Button variant="secondary">Share state</Button>
        <Button variant="secondary" on:click={() => demo.enqueueToast("info")}>Push toast</Button>
      </div>
      <div slot="status" class="workspace-status-pill-row">
        <span class="command-shortcut-hint">{screenMeta.eyebrow}</span>
        <span class="command-shortcut-hint">{shellMenuLog}</span>
      </div>
    </ProjectHeader>

    <div slot="surfaceTabs" class="shared-demo-screen-tabs">
      <SegmentedControl
        value={screenId}
        options={screenTabs}
        ariaLabel="Shared demo screens"
        on:valueChange={(event) => onScreenChange(event.detail.value)}
      />
    </div>

    <div class="shared-demo-stage">
      <div class="shared-demo-toolbar">
        {#if screenId === "overview-shell"}
          <Toolbar ariaLabel="Overview controls">
            <Button variant="secondary" on:click={() => demo.enqueueToast("success")}>Checkpoint</Button>
            <Button variant="secondary" on:click={() => demo.enqueueToast("warning")}>Raise warning</Button>
            <Button variant="secondary" on:click={demo.openCommandPalette}>Command review</Button>
          </Toolbar>
        {:else if screenId === "form-and-validation"}
          <Toolbar ariaLabel="Form controls">
            <ToggleGroup
              value={demo.invalid ? "invalid" : demo.busy ? "pending" : "default"}
              options={[
                { value: "default", label: "Default" },
                { value: "invalid", label: "Invalid" },
                { value: "pending", label: "Pending" },
              ]}
              ariaLabel="Form state"
              on:valueChange={(event) => {
                const nextValue = event.detail.value as string;
                demo.handleInvalidChange(new CustomEvent("checkedChange", { detail: { checked: nextValue === "invalid" } }));
                demo.handleBusyChange(new CustomEvent("checkedChange", { detail: { checked: nextValue === "pending" } }));
              }}
            />
            <Button variant="secondary" on:click={() => (formDialogOpen = true)}>Open remediation</Button>
          </Toolbar>
        {:else if screenId === "browse-and-table"}
          <Toolbar ariaLabel="Browse controls">
            <Select
              id="browse-status-select"
              value={demo.browseStatus}
              options={browseStatusSelectOptions}
              ariaLabel="Browse status"
              on:valueChange={(event) =>
                demo.handleBrowseStatusChange(new CustomEvent("valueChange", { detail: { value: event.detail.value } }))}
            />
            <Button variant="secondary" on:click={demo.resetBrowseFilters}>Reset filters</Button>
            <Button variant="secondary" on:click={demo.retryBrowseState}>Retry state</Button>
          </Toolbar>
        {:else if screenId === "detail-and-related-data"}
          <Toolbar ariaLabel="Detail controls">
            <Button variant="secondary" on:click={() => demo.handleDetailStateChange(new CustomEvent("valueChange", { detail: { value: "ready" } }))}>Ready</Button>
            <Button variant="secondary" on:click={() => demo.handleDetailStateChange(new CustomEvent("valueChange", { detail: { value: "loading" } }))}>Loading</Button>
            <Button variant="secondary" on:click={() => demo.handleDetailStateChange(new CustomEvent("valueChange", { detail: { value: "error" } }))}>Error</Button>
          </Toolbar>
        {:else if screenId === "picker-and-media"}
          <Toolbar ariaLabel="Picker and media controls">
            <Button variant="secondary" on:click={() => (pickerDrawerOpen = true)}>Open drawer picker</Button>
            <Popover ariaLabel="Current asset summary">
              <div slot="trigger">
                <Button variant="secondary">Peek asset</Button>
              </div>
              <div class="demo-stack">
                <strong>{demo.activeMedia.title}</strong>
                <p class="detail-card-meta">{demo.activeMedia.description}</p>
                <div class="docs-tag-row">
                  {#each demo.activeMedia.meta as meta}
                    <Pill appearance="subtle">{meta}</Pill>
                  {/each}
                </div>
              </div>
            </Popover>
          </Toolbar>
        {:else}
          <Toolbar ariaLabel="Command and workspace controls">
            <ToggleGroup
              value={demo.commandScope}
              options={demo.commandScopeOptions}
              ariaLabel="Command scope"
              on:valueChange={demo.handleCommandScopeChange}
            />
            <ToggleGroup
              value={demo.workspaceState}
              options={demo.workspaceStateOptions}
              ariaLabel="Workspace state"
              on:valueChange={demo.handleWorkspaceStateChange}
            />
          </Toolbar>
        {/if}
      </div>

      <div class="shared-demo-grid">
        <div class="shared-demo-primary">
          {#if screenId === "overview-shell"}
            <div class="shared-demo-overview-stack">
              {#if demo.showPersistentBanner}
                <Banner
                  tone={demo.bannerTone}
                  title="Shared shell posture is visible"
                  message="The rebuilt target keeps remediation and shell status attached to the same story instead of splitting them across unrelated docs sections."
                >
                  <div slot="actions" class="action-cluster">
                    <Button variant="secondary">Inspect</Button>
                    <Button variant="primary" on:click={() => demo.enqueueToast("success")}>Resolve</Button>
                  </div>
                </Banner>
              {/if}

              <div class="shared-demo-summary-grid">
                {#each demo.detailCards as card}
                  <Card variant="elevated">
                    <div slot="header">
                      <p class="eyebrow">{card.title}</p>
                    </div>
                    <strong class="detail-card-value">{card.value}</strong>
                    <p class="detail-card-meta">{card.meta}</p>
                  </Card>
                {/each}
              </div>

              <Callout tone="info" title="Parity target">
                <p>
                  This screen now exists to anchor a believable shell, not just to
                  prove individual exports in isolation.
                </p>
              </Callout>

              <PanelSurface title="Transient notification stack" isElevated={true} bodyPadding="md">
                <ToastStack items={demo.toastItems} on:dismiss={demo.dismissToast} on:action={demo.handleToastAction} />
              </PanelSurface>

              <PrimitiveCoverageDeck />
            </div>
          {:else if screenId === "form-and-validation"}
            <div class="shared-demo-screen-stack">
              {#if demo.invalid || demo.busy}
                <Banner
                  tone={demo.invalid ? "danger" : "info"}
                  title={demo.invalid ? "Validation is blocking publish" : "Validation is running"}
                  message={demo.validationLog}
                />
              {/if}

              <form class="demo-form" on:submit={demo.handleSubmit}>
                <Field
                  id="project-title-demo"
                  label="Project title"
                  description="Used for handoff labels, activity logs, and downstream validation review."
                  error={demo.titleError}
                  pendingMessage={demo.titlePendingMessage}
                  validationState={demo.titleValidationState}
                  isRequired={true}
                  let:describedBy
                  let:validationState
                >
                  <TextInput
                    id="project-title-demo"
                    value={demo.projectTitle}
                    placeholder="Enter project title"
                    isDisabled={demo.disabled}
                    validationState={validationState}
                    describedBy={describedBy}
                    on:valueChange={demo.handleTitleChange}
                  />
                </Field>

                <Field
                  id="delivery-notes-demo"
                  label="Delivery notes"
                  description="Long-form workflow notes now use the exported text-area primitive instead of generic text boxes."
                  let:describedBy
                >
                  <TextArea
                    id="delivery-notes-demo"
                    value={deliveryNotes}
                    isDisabled={demo.disabled}
                    describedBy={describedBy}
                    on:valueChange={(event) => (deliveryNotes = event.detail.value)}
                  />
                </Field>

                <div class="shared-demo-form-grid">
                  <Field id="handoff-target-demo" label="Handoff target">
                    <Select
                      id="handoff-target-demo"
                      value={handoffTarget}
                      options={handoffTargetOptions}
                      ariaLabel="Handoff target"
                      on:valueChange={(event) => (handoffTarget = event.detail.value)}
                    />
                  </Field>

                  <Field id="publish-date-demo" label="Publish date">
                    <DatePicker
                      value={publishOn}
                      ariaLabel="Publish date"
                      on:valueChange={(event) => (publishOn = event.detail.value)}
                    />
                  </Field>
                </div>

                <Field id="asset-search-demo" label="Asset search" description="Search remains native while clear and pending semantics stay explicit." let:describedBy let:validationState>
                  <SearchField
                    id="asset-search-demo"
                    value={demo.assetSearch}
                    isDisabled={demo.disabled}
                    validationState={validationState}
                    describedBy={describedBy}
                    on:valueChange={demo.handleSearchChange}
                    on:clear={demo.handleSearchClear}
                  />
                </Field>

                <Field id="delivery-mode-demo" label="Delivery mode">
                  <RadioGroup
                    value={deliveryMode}
                    options={deliveryModeOptions}
                    orientation="horizontal"
                    ariaLabel="Delivery mode"
                    on:valueChange={(event) => (deliveryMode = event.detail.value)}
                  />
                </Field>

                <div class="shared-demo-inline-controls">
                  <Switch
                    isChecked={notifyClient}
                    label="Notify client on publish"
                    on:checkedChange={(event) => (notifyClient = event.detail.checked)}
                  />
                  <StatusIndicator
                    status={demo.invalid ? "danger" : demo.busy ? "pending" : "success"}
                    label={demo.invalid ? "blocking issue" : demo.busy ? "validation pending" : "ready to publish"}
                  />
                </div>

                <FormActions align="between">
                  <p class="demo-status">{demo.validationLog}</p>
                  <div class="action-cluster">
                    <Button variant="secondary" type="button" on:click={() => (formDialogOpen = true)}>Review issue</Button>
                    <Button variant="primary" type="submit" isDisabled={demo.disabled} isLoading={demo.busy}>
                      {demo.busy ? "Validating..." : "Save changes"}
                    </Button>
                  </div>
                </FormActions>
              </form>
            </div>
          {:else if screenId === "browse-and-table"}
            <div class="shared-demo-screen-stack">
              <FilterToolbar summaryText={demo.browseSummary} ariaLabel="Browse filters">
                <SearchField
                  id="browse-query-demo"
                  value={demo.browseQuery}
                  ariaLabel="Browse search"
                  on:valueChange={demo.handleBrowseSearchChange}
                  on:clear={demo.handleBrowseSearchClear}
                />
                <Select
                  id="browse-status-toolbar"
                  value={demo.browseStatus}
                  options={browseStatusSelectOptions}
                  ariaLabel="Browse status"
                  on:valueChange={(event) =>
                    demo.handleBrowseStatusChange(new CustomEvent("valueChange", { detail: { value: event.detail.value } }))}
                />
                <SegmentedControl
                  value={browsePresentation}
                  options={browsePresentationOptions}
                  ariaLabel="Browse presentation"
                  on:valueChange={(event) => (browsePresentation = event.detail.value)}
                />
                <div slot="secondary" class="action-cluster">
                  <Button variant="secondary" on:click={demo.resetBrowseFilters}>Reset</Button>
                  <Button variant="secondary" on:click={demo.retryBrowseState}>Retry</Button>
                </div>
              </FilterToolbar>

              {#if browsePresentation === "table"}
                {#if (demo.selectedRowIds ?? []).length > 0}
                  <div class="shared-demo-inline-stack">
                    <p class="demo-status">{demo.tableStatus}</p>
                  </div>
                {/if}
                <DataTable
                  ariaLabel="Mix tasks"
                  columns={demo.tableColumns}
                  rows={demo.visibleRows}
                  selectedRowIds={demo.selectedRowIds}
                  sortColumnId={demo.sortColumnId}
                  sortDirection={demo.sortDirection}
                  on:sortChange={demo.handleSortChange}
                  on:rowToggle={demo.handleRowToggle}
                  on:toggleAll={demo.handleToggleAll}
                  on:rowAction={demo.handleRowAction}
                />
              {:else if browsePresentation === "list"}
                <ListShell
                  state={demo.listShellState}
                  ariaLabel="Browse list"
                  itemCount={demo.filteredBrowseRows.length}
                  stateTitle="Browse list unavailable"
                  stateMessage={demo.browseSummary}
                >
                  {#if demo.listShellState === "ready"}
                    {#each demo.visibleBrowseRows as row}
                      <li class="browse-list-item">
                        <div>
                          <strong>{row.title}</strong>
                          <p>{row.kind} · {row.owner}</p>
                        </div>
                        <StatusIndicator
                          status={row.status === "Blocked" ? "danger" : row.status === "Review" ? "warning" : "success"}
                          label={row.status}
                        />
                      </li>
                    {/each}
                  {/if}
                  <div slot="footer" class="action-cluster">
                    <Button
                      variant="secondary"
                      isDisabled={demo.visibleBrowseRows.length >= demo.filteredBrowseRows.length}
                      on:click={demo.loadMoreBrowseRows}
                    >
                      Load more
                    </Button>
                  </div>
                </ListShell>
              {:else}
                <GridShell
                  state={demo.gridShellState}
                  ariaLabel="Browse grid"
                  itemCount={demo.filteredBrowseCards.length}
                  stateTitle="Browse grid unavailable"
                  stateMessage={demo.browseSummary}
                >
                  {#if demo.gridShellState === "ready"}
                    {#each demo.visibleBrowseCards as card}
                      <Card variant="outlined">
                        <div slot="header">
                          <p class="eyebrow">{card.category}</p>
                        </div>
                        <strong class="detail-card-value">{card.title}</strong>
                        <p class="detail-card-meta">{card.meta}</p>
                      </Card>
                    {/each}
                  {/if}
                </GridShell>
              {/if}

              <p class="demo-status">{demo.tableStatus}</p>
            </div>
          {:else if screenId === "detail-and-related-data"}
            <div class="shared-demo-screen-stack">
              <PageHeader
                title="Aura review delivery"
                eyebrow="Detail and related data"
                subtitle="Headers, metadata, summaries, and supporting context now read as one coherent detail workflow."
              >
                <div slot="breadcrumbs">
                  <Breadcrumbs items={demo.detailBreadcrumbs} on:navigate={demo.handleBreadcrumbNavigate} />
                </div>
                <div slot="actions" class="action-cluster">
                  <Button variant="secondary">Reveal export</Button>
                  <Button variant="primary">Approve handoff</Button>
                </div>
              </PageHeader>

              <div class="shared-demo-summary-grid">
                {#each demo.detailCards as card}
                  <Card variant="elevated">
                    <div slot="header">
                      <p class="eyebrow">{card.title}</p>
                    </div>
                    <strong class="detail-card-value">{card.value}</strong>
                    <p class="detail-card-meta">{card.meta}</p>
                  </Card>
                {/each}
              </div>

              <DetailShell
                title="Delivery brief"
                state={demo.detailState}
                stateTitle="Detail surface unavailable"
                stateMessage="The detail shell still owns state posture rather than forcing host pages to fake it."
              >
                <DetailSection
                  title="Core metadata"
                  description="The rows below should line up and remain readable across themes and densities."
                >
                  <DetailRow label="Sample rate" value="48 kHz" />
                  <DetailRow label="Loudness target" value="-16 LUFS integrated" />
                  <DetailRow label="Destination" value="/clients/aura/review/v4/final-deliverables" />
                </DetailSection>

                <DetailSection
                  title="Readiness"
                  description="Selection and review summary now sit inside the same detail story."
                >
                  <Callout tone="success" title="Ready for downstream review">
                    <p>
                      Metadata, ownership, and explicit recovery posture now remain
                      adjacent instead of being split across unrelated preview sections.
                    </p>
                  </Callout>
                </DetailSection>
              </DetailShell>
            </div>
          {:else if screenId === "picker-and-media"}
            <div class="shared-demo-screen-stack">
              <PickerShell
                title="Reference asset picker"
                description="Selection workflow, preview framing, and fallback posture now live on one screen."
                variant={demo.pickerVariant}
                state={demo.pickerState}
                resultCount={demo.filteredRelationItems.length}
                selectionCount={demo.selectedRelationIds.length}
                statusText={demo.pickerStatus}
              >
                <div slot="toolbar" class="shared-demo-picker-toolbar">
                  <SearchField
                    id="picker-query-demo"
                    value={demo.pickerQuery}
                    ariaLabel="Picker search"
                    on:valueChange={demo.handlePickerQueryChange}
                    on:clear={demo.resetPickerState}
                  />
                  <ToggleGroup
                    value={demo.pickerMode}
                    options={demo.pickerModeOptions}
                    ariaLabel="Selection mode"
                    on:valueChange={demo.handlePickerModeChange}
                  />
                </div>

                <div slot="selection">
                  <SelectionSummary
                    items={relationSummaryItems}
                    selectionMode={demo.pickerMode}
                    on:remove={removeSelectedRelation}
                    on:clear={clearSelectedRelations}
                  />
                </div>

                <RelationPicker
                  title="Attach reference assets"
                  description="Searchable attach-or-choose flow for the current delivery surface."
                  items={demo.filteredRelationItems}
                  selectedIds={demo.selectedRelationIds}
                  selectionMode={demo.pickerMode}
                  variant={demo.pickerVariant}
                  on:selectionChange={demo.handleRelationSelectionChange}
                  on:confirm={demo.handlePickerConfirm}
                  on:cancel={demo.handlePickerCancel}
                />

                <div slot="footer" class="action-cluster">
                  <Button variant="secondary" on:click={() => (pickerDrawerOpen = true)}>Open drawer mode</Button>
                  <Button variant="primary" on:click={() => demo.handlePickerConfirm(new CustomEvent("confirm", { detail: { selectedIds: demo.selectedRelationIds } }))}>
                    Confirm selection
                  </Button>
                </div>
              </PickerShell>

              <PanelSurface title="Active media preview" isElevated={true} bodyPadding="md">
                <MediaPreview
                  title={demo.activeMedia.title}
                  assetId={demo.activeMedia.assetId}
                  state={demo.mediaState}
                  kind={demo.activeMedia.kind}
                  aspectRatio={demo.activeMedia.aspectRatio}
                  meta={demo.activeMedia.meta}
                  stateTitle={demo.getMediaStateTitle(demo.mediaState, demo.activeMedia.kind)}
                  stateMessage={demo.getMediaStateMessage(demo.mediaState, demo.activeMedia.kind)}
                >
                  <div slot="thumbnail">
                    <MediaThumbnail
                      title={demo.activeMedia.title}
                      assetId={demo.activeMedia.assetId}
                      state={demo.mediaState}
                      kind={demo.activeMedia.kind}
                      aspectRatio={demo.activeMedia.aspectRatio}
                      badge={demo.activeMedia.badge}
                      meta={demo.activeMedia.thumbnailMeta}
                    />
                  </div>
                </MediaPreview>
              </PanelSurface>
            </div>
          {:else}
            <div class="shared-demo-screen-stack">
              <div class="shared-demo-command-grid">
                <PanelSurface isActive={true} isElevated={true} bodyPadding="none" ariaLabel="Command and workspace shell">
                  <div slot="header">
                    <PanelHeader title="Command and workspace shell" isActive={true}>
                      <div slot="tabs">
                        <PanelTabs
                          items={workspacePanelItems}
                          value={activeWorkspacePanel}
                          isReorderable={false}
                          ariaLabel="Command and workspace views"
                          on:valueChange={(event) => (activeWorkspacePanel = event.detail.value)}
                        />
                      </div>
                    </PanelHeader>
                  </div>

                  {#if activeWorkspacePanel === "overview"}
                    <div class="workspace-surface-stack">
                      <Callout tone="info" title="Launcher and inline discovery belong together">
                        <p>
                          The shared demo now keeps command launch, grouped discovery,
                          and shell state in one believable workstation scene.
                        </p>
                      </Callout>
                      <div class="action-cluster">
                        <Button variant="primary" on:click={demo.openCommandPalette}>Open command palette</Button>
                        <Button variant="secondary" on:click={demo.clearCommandDiscovery}>Reset command filters</Button>
                      </div>
                      <div class="workspace-signal-grid">
                        <div class="state-tile">
                          <span class="token-path">commandScope</span>
                          <strong>{demo.commandScope}</strong>
                        </div>
                        <div class="state-tile">
                          <span class="token-path">workspaceState</span>
                          <strong>{demo.workspaceState}</strong>
                        </div>
                        <div class="state-tile">
                          <span class="token-path">lastCommandId</span>
                          <strong>{demo.lastCommandId ?? "∅"}</strong>
                        </div>
                      </div>
                    </div>
                  {:else if activeWorkspacePanel === "workspace"}
                    <SplitView
                      orientation="horizontal"
                      ratio={demo.primarySplitRatio}
                      minPrimarySize={260}
                      minSecondarySize={480}
                      ariaLabel="Shared demo workspace split"
                      on:ratioChange={demo.handlePrimarySplitChange}
                    >
                      <div slot="primary" class="workspace-pane">
                        <DockRegion
                          edge="left"
                          items={demo.leftDockItems}
                          value={demo.leftDockValue}
                          isCollapsed={demo.leftDockCollapsed}
                          ariaLabel="Left dock"
                          on:valueChange={(event) => demo.handleDockValueChange("left", event)}
                          on:collapsedChange={(event) => demo.handleDockCollapsedChange("left", event)}
                          on:reorder={(event) => demo.handleDockReorder("left", event)}
                          on:requestClose={(event) => demo.handleDockClose("left", event)}
                          on:requestContextMenu={(event) => demo.handleDockContextMenu("left", event)}
                          let:activeItem
                        >
                          <div class="workspace-dock-content">
                            <p class="eyebrow">{demo.activeLeftDockMeta?.title ?? activeItem?.label ?? "Left dock"}</p>
                            <p class="detail-card-meta">{demo.activeLeftDockMeta?.summary}</p>
                            <ul class="workspace-list">
                              {#each demo.activeLeftDockMeta?.items ?? [] as item}
                                <li>{item}</li>
                              {/each}
                            </ul>
                          </div>
                        </DockRegion>
                      </div>

                      <div slot="secondary" class="workspace-pane">
                        <PanelSurface title={demo.activeWorkspaceSurface?.label ?? "Workspace surface"} isActive={true} isElevated={true} bodyPadding="md">
                          <div slot="header">
                            <PanelHeader title={demo.activeWorkspaceSurface?.label ?? "Workspace surface"} isActive={true}>
                              <div slot="tabs">
                                <SurfaceTabs
                                  items={demo.workspaceSurfaceItems}
                                  value={demo.workspaceSurfaceValue}
                                  ariaLabel="Workspace surfaces"
                                  on:valueChange={demo.handleSurfaceTabChange}
                                  on:reorder={demo.handleSurfaceReorder}
                                  on:requestRename={demo.handleSurfaceRename}
                                  on:requestMove={demo.handleSurfaceMove}
                                  on:requestClose={demo.handleSurfaceClose}
                                  on:requestAdd={demo.handleSurfaceAdd}
                                />
                              </div>
                            </PanelHeader>
                          </div>

                          <div class="workspace-surface-stack">
                            <div class="workspace-surface-copy">
                              <p class="eyebrow">{demo.activeWorkspaceSurfaceMeta.eyebrow}</p>
                              <h3 class="workspace-surface-title">{demo.activeWorkspaceSurface?.label ?? "Workspace surface"}</h3>
                              <p class="detail-card-meta">{demo.activeWorkspaceSurfaceMeta.description}</p>
                            </div>
                            <div class="workspace-signal-grid">
                              {#each demo.activeWorkspaceSurfaceMeta.highlights as highlight}
                                <div class="state-tile">
                                  <span class="token-path">surface signal</span>
                                  <strong>{highlight}</strong>
                                </div>
                              {/each}
                            </div>
                          </div>
                        </PanelSurface>
                      </div>
                    </SplitView>
                  {:else}
                    <div class="workspace-persistence-panel">
                      <pre>{demo.serializedWorkspaceLayout}</pre>
                    </div>
                  {/if}
                </PanelSurface>

                <div class="shared-demo-command-companion">
                  <ActionDiscoveryPanel
                    title="Inline action discovery"
                    description="Grouped action rediscovery now sits beside workspace context, not on a separate docs page."
                    sections={demo.commandSections}
                    invocationHint="Open the full launcher with ⌘K"
                    on:actionSelect={demo.handleActionDiscoverySelect}
                  />
                </div>
              </div>
            </div>
          {/if}
        </div>

        <aside class="shared-demo-companion">
          {#if screenId === "overview-shell"}
            <PanelSurface title="Shell signals" bodyPadding="md">
              <div class="demo-stack">
                <StatusIndicator status="success" label="GPUI target should mirror this shell layout" />
                <StatusIndicator status={selectedTone} label={`Current shell posture: ${demo.workspaceState}`} />
                <Callout tone="pending" title="Docs shell boundary is preserved">
                  <p>
                    Catalog navigation and token inspection stay outside the demo
                    target, but this shell still lives inside the docs host for now.
                  </p>
                </Callout>
              </div>
            </PanelSurface>
          {:else if screenId === "form-and-validation"}
            <div class="shared-demo-screen-stack">
              <Callout tone={demo.invalid ? "danger" : demo.busy ? "pending" : "success"} title="Current form posture">
                <p>{demo.validationLog}</p>
              </Callout>
              <div class="demo-stack">
                <div class="state-tile">
                  <span class="token-path">deliveryMode</span>
                  <strong>{deliveryMode}</strong>
                </div>
                <div class="state-tile">
                  <span class="token-path">handoffTarget</span>
                  <strong>{handoffTarget}</strong>
                </div>
                <div class="state-tile">
                  <span class="token-path">publishOn</span>
                  <strong>{publishOn}</strong>
                </div>
              </div>
            </div>
          {:else if screenId === "browse-and-table"}
            <div class="shared-demo-screen-stack">
              <Table
                ariaLabel="Visible browse summary"
                caption="Plain table primitive reference"
                columns={[
                  { id: "title", label: "Title", isRowHeader: true },
                  { id: "kind", label: "Kind" },
                  { id: "owner", label: "Owner" },
                ]}
                rows={visibleBrowseTableRows}
              />
              <Callout tone="info" title="Browse workflow stays coherent">
                <p>
                  The table, list, and grid views now share the same search and
                  filter posture instead of behaving like disconnected preview tiles.
                </p>
              </Callout>
            </div>
          {:else if screenId === "detail-and-related-data"}
            <Tabs value={detailTab} tabs={detailTabs} ariaLabel="Detail companion" on:valueChange={(event) => (detailTab = event.detail.value)} let:activeValue>
              {#if activeValue === "related"}
                <SelectionSummary
                  items={relationSummaryItems}
                  selectionMode="multiple"
                  on:remove={removeSelectedRelation}
                  on:clear={clearSelectedRelations}
                />
              {:else}
                <div class="demo-stack">
                  <div class="state-tile">
                    <span class="token-path">lastCommandId</span>
                    <strong>{demo.lastCommandId ?? "∅"}</strong>
                  </div>
                  <div class="state-tile">
                    <span class="token-path">workspaceEventLog</span>
                    <strong>{demo.workspaceEventLog}</strong>
                  </div>
                </div>
              {/if}
            </Tabs>
          {:else if screenId === "picker-and-media"}
            <div class="shared-demo-screen-stack">
              <div class="media-strip">
                {#each demo.secondaryMediaAssets as asset}
                  <Button variant="ghost" className="media-strip__item" on:click={() => demo.handleActiveMediaChange(asset.id)}>
                    <MediaThumbnail
                      title={asset.title}
                      assetId={asset.assetId}
                      state={demo.mediaState}
                      kind={asset.kind}
                      aspectRatio={asset.aspectRatio}
                      badge={asset.badge}
                      meta={asset.thumbnailMeta}
                      presentation="compact"
                    />
                  </Button>
                {/each}
              </div>
              <EmbedShell
                title="External review embed"
                description="Embed shells frame host-native or external surfaces while preserving fallback copy and recovery actions."
                state={demo.embedState}
                stateTitle={demo.embedState === "ready" ? "External review embed" : "Embed shell fallback"}
                stateMessage={demo.getEmbedStateMessage(demo.embedState)}
              >
                <div slot="actions" class="action-cluster">
                  <Button variant="secondary">Open external</Button>
                  <Button variant="primary">Focus embed</Button>
                </div>
              </EmbedShell>
            </div>
          {:else}
            <div class="shared-demo-screen-stack">
              <PanelSurface title="Workstation notes" bodyPadding="md">
                <div class="demo-stack">
                  <div class="state-tile">
                    <span class="token-path">commandStatus</span>
                    <strong>{demo.commandStatus}</strong>
                  </div>
                  <div class="state-tile">
                    <span class="token-path">workspacePersistence</span>
                    <strong>{demo.workspacePersistenceSummary}</strong>
                  </div>
                </div>
              </PanelSurface>
              <Callout tone="warning" title="Native runtime deltas still need proof">
                <p>
                  The GPUI build should match this shell semantics closely, but
                  it can still diverge where native windowing or focus behavior
                  truly demands it.
                </p>
              </Callout>
            </div>
          {/if}
        </aside>
      </div>
    </div>

    <ShellStatusBar slot="statusBar" summary="Shared demo app status">
      <div slot="leading" class="workspace-status-pill-row">
        <span class="command-shortcut-hint">screen: {screenMeta.title}</span>
        <span class="command-shortcut-hint">theme: {theme}</span>
        <span class="command-shortcut-hint">density: {density}</span>
      </div>
      <div slot="trailing" class="workspace-status-pill-row">
        <span class="command-shortcut-hint">commands: {(demo.filteredCommandActions ?? []).length}</span>
        <span class="command-shortcut-hint">selection: {(demo.selectedRelationIds ?? []).length}</span>
        <span class="command-shortcut-hint">rows: {(demo.visibleRows ?? []).length}</span>
      </div>
    </ShellStatusBar>
  </WorkspaceShell>
</section>

<Dialog
  open={formDialogOpen}
  title="Validation remediation"
  description="Dialog posture is now part of the same form story rather than a separate primitive specimen."
  on:requestClose={() => (formDialogOpen = false)}
>
  <Callout tone="danger" title="Blocking issue">
    <p>{demo.titleError ?? "No blocking issue is active."}</p>
  </Callout>
  <div slot="actions" class="action-cluster">
    <Button variant="secondary" on:click={() => (formDialogOpen = false)}>Dismiss</Button>
    <Button variant="primary" on:click={() => (formDialogOpen = false)}>Acknowledge</Button>
  </div>
</Dialog>

<Drawer
  open={pickerDrawerOpen}
  edge="right"
  title="Drawer picker"
  description="Overlay picker posture should preserve the same selection semantics as the inline flow."
  on:requestClose={() => (pickerDrawerOpen = false)}
>
  <SelectionSummary
    items={relationSummaryItems}
    selectionMode={demo.pickerMode}
    on:remove={removeSelectedRelation}
    on:clear={clearSelectedRelations}
  />
  <div slot="actions" class="action-cluster">
    <Button variant="secondary" on:click={() => (pickerDrawerOpen = false)}>Cancel</Button>
    <Button variant="primary" on:click={() => (pickerDrawerOpen = false)}>Apply selection</Button>
  </div>
</Drawer>
