<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/settings-shell.css";
  import type { Snippet } from "svelte";

  import Callout from "./Callout.svelte";
  import Dialog from "./Dialog.svelte";
  import EmptyState from "./EmptyState.svelte";
  import ScrollShell from "./ScrollShell.svelte";
  import SidebarNav from "./SidebarNav.svelte";
  import Surface from "./Surface.svelte";
  import TextInput from "./TextInput.svelte";

  // Structural types, declared locally (R3): Poodle never imports Longhorn
  // and never learns what a storage profile or a keymap is. These shapes are
  // what the current host needs; a host maps its own domain onto them.
  interface SettingsNavGroup {
    id: string;
    label: string;
    items: { value: string; label: string }[];
  }

  interface SettingsSearchResult {
    pageId: string;
    pageLabel: string;
    anchorId?: string;
    anchorLabel?: string;
  }

  let nextSearchId = 0;

  interface Props {
    groups?: SettingsNavGroup[];
    activePageId?: string | null;
    pageTitle?: string | null;
    searchQuery?: string;
    searchResults?: SettingsSearchResult[] | null;
    open?: boolean | null;
    defaultOpen?: boolean;
    title?: string | null;
    closeLabel?: string;
    closeRefusedReason?: string | null;
    page?: Snippet;
    onNavigate?: ((pageId: string, anchorId?: string | null) => void) | undefined;
    onRequestClose?: (() => void) | undefined;
    onOpenChange?: ((open: boolean) => void) | undefined;
    onSearchQueryChange?: ((value: string) => void) | undefined;
  }

  let {
    groups = [],
    activePageId = null,
    pageTitle = null,
    searchQuery = $bindable(""),
    searchResults = null,
    open = $bindable<boolean | null>(null),
    defaultOpen = false,
    title = "Settings",
    closeLabel = "Close settings",
    closeRefusedReason = null,
    page,
    onNavigate = undefined,
    onRequestClose = undefined,
    onOpenChange = undefined,
    onSearchQueryChange = undefined,
  }: Props = $props();

  const searchId = `poodle-settings-search-${nextSearchId++}`;

  let uncontrolledOpen = $state(false);
  let seededDefaultOpen = $state(false);

  $effect.pre(() => {
    if (!seededDefaultOpen && open === null) {
      uncontrolledOpen = defaultOpen;
      seededDefaultOpen = true;
    }
  });

  const isOpen = $derived(open === null ? uncontrolledOpen : open);

  /* Dialog assigns to its own `open` prop (`Dialog.svelte:200`). Passed an
     unbound value it mutates its local copy, and the parent cannot veto —
     which is precisely a refused close, where our state deliberately does not
     change, so a plain `open={isOpen}` leaves Dialog stuck closed. Mirror it
     into bindable state instead: the effect syncs our decision down, and the
     refusal branch below forces it back open. */
  let dialogOpen = $state(false);

  $effect(() => {
    dialogOpen = isOpen;
  });
  const isSearching = $derived(searchResults !== null);
  const hasResults = $derived(isSearching && (searchResults?.length ?? 0) > 0);

  function handleSearchChange(value: string): void {
    searchQuery = value;
    onSearchQueryChange?.(value);
  }

  function handleOpenChange(next: boolean): void {
    // Refused close: the host's `closeRefusedReason` keeps the shell open.
    // The shell never closes itself against a refusal, and never invents a
    // reason of its own (R1.7).
    if (!next && closeRefusedReason) {
      if (open === null) uncontrolledOpen = true;
      else open = true;
      // Dialog has already set its own copy false; put it back.
      dialogOpen = true;
      return;
    }
    if (open === null) uncontrolledOpen = next;
    else open = next;
    onOpenChange?.(next);
  }

  function handleNavigate(pageId: string): void {
    onNavigate?.(pageId);
  }

  function handleResultActivation(result: SettingsSearchResult): void {
    onNavigate?.(result.pageId, result.anchorId ?? null);
  }
</script>

<Dialog
  bind:open={dialogOpen}
  title={title}
  ariaLabel={title}
  width="xl"
  showCloseButton
  closeLabel={closeLabel}
  onRequestClose={onRequestClose}
  onOpenChange={handleOpenChange}
>
  {#snippet header()}
    <!-- Title, search and the dialog's own close read as one bar. Dialog's
         `__header-row` is the flex container and its close button is our
         sibling, so this snippet owns only the left-of-close span. -->
    <div class="poodle-settings-shell__dialog-header">
      <strong class="poodle-settings-shell__dialog-title">{title}</strong>
      <div class="poodle-settings-shell__search">
        <TextInput
          id={searchId}
          type="search"
          placeholder="Search settings"
          ariaLabel="Search settings"
          value={searchQuery}
          showClearButton
          onValueChange={handleSearchChange}
        />
      </div>
    </div>
  {/snippet}
  <div class="poodle-settings-shell">
    <aside class="poodle-settings-shell__nav">
      <Surface tone="panel" border="subtle" padding="none">
        <ScrollShell direction="vertical">
          {#if groups.length === 0}
            <EmptyState
              variant="neutral"
              size="compact"
              title="No settings pages"
              message="This scope has no settings pages yet."
            />
          {:else}
            <SidebarNav
              groups={groups}
              value={activePageId}
              ariaLabel="Settings pages"
              onValueChange={handleNavigate}
            />
          {/if}
        </ScrollShell>
      </Surface>
    </aside>

    <div class="poodle-settings-shell__page">
      {#if closeRefusedReason}
        <div class="poodle-settings-shell__notice">
          <Callout tone="warning" announceMode="polite" message={closeRefusedReason} />
        </div>
      {/if}

      {#if isSearching}
        <div class="poodle-settings-shell__results" data-empty={!hasResults}>
          {#if hasResults}
            <ScrollShell direction="vertical" padding="md">
              <ul class="poodle-settings-shell__result-list" aria-label="Settings search results">
                {#each searchResults ?? [] as result}
                  <li>
                    <button
                      type="button"
                      class="poodle-settings-shell__result"
                      onclick={() => handleResultActivation(result)}
                    >
                      <span class="poodle-settings-shell__result-label">{result.pageLabel}</span>
                      {#if result.anchorLabel}
                        <span class="poodle-settings-shell__result-anchor">{result.anchorLabel}</span>
                      {/if}
                    </button>
                  </li>
                {/each}
              </ul>
            </ScrollShell>
          {:else}
            <EmptyState
              variant="search"
              size="compact"
              title="No results"
              message="No settings match your search."
            />
          {/if}
        </div>
      {:else}
        <!-- No visible page heading or description: the nav rail already names
             the current page, and the page snippet owns its own intro.
             `pageTitle` becomes this region's accessible name instead, so the
             name survives for screen readers without being drawn. -->
        <section class="poodle-settings-shell__page-stack" aria-label={pageTitle}>
          <ScrollShell direction="vertical" padding="md">
            {@render page?.()}
          </ScrollShell>
        </section>
      {/if}
    </div>
  </div>
</Dialog>
