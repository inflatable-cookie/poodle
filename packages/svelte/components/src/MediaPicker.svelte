<script lang="ts">
  import Dialog from "./Dialog.svelte";
  import FileUpload from "./FileUpload.svelte";
  import Tabs from "./Tabs.svelte";
  import TextInput from "./TextInput.svelte";
  import UiPresentationProvider from "./UiPresentationProvider.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type {
    ControlDensity,
    ControlSize,
    FileUploadItem,
    SemanticControlSizeRole,
    TabItem,
  } from "./types";
  import type { MediaPickerItem } from "./types";

  interface Props {
    open?: boolean | null | undefined;
    items?: MediaPickerItem[];
    accept?: string;
    maxFileSize?: number;
    title?: string;
    emptyMessage?: string;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onSelect?: ((item: MediaPickerItem) => void) | undefined;
    onUpload?: ((files: FileUploadItem[]) => void) | undefined;
    onOpenChange?: ((open: boolean) => void) | undefined;
  }

  let {
    open = undefined,
    items = [],
    accept = "image/*",
    maxFileSize = 25 * 1024 * 1024,
    title = "Select media",
    emptyMessage = "No media items found.",
    size = null,
    sizeRole = "control",
    density = null,
    onSelect = undefined,
    onUpload = undefined,
    onOpenChange = undefined,
  }: Props = $props();

  let activeTab = $state("browse");
  let searchQuery = $state("");
  let uploadFiles = $state<FileUploadItem[]>([]);
  let uncontrolledOpen = $state(false);
  const uiPresentation = getUiPresentation();

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const isControlled = $derived(open !== undefined && open !== null);
  const isOpen = $derived(isControlled ? open === true : uncontrolledOpen);

  const tabItems: TabItem[] = [
    { value: "browse", label: "Browse" },
    { value: "upload", label: "Upload" },
  ];

  const filteredItems = $derived.by(() =>
    searchQuery
      ? items.filter((item) =>
          item.label.toLowerCase().includes(searchQuery.toLowerCase()),
        )
      : items,
  );

  function requestOpenChange(nextOpen: boolean): void {
    if (!isControlled) {
      uncontrolledOpen = nextOpen;
    }

    onOpenChange?.(nextOpen);
  }

  function handleSelect(item: MediaPickerItem): void {
    onSelect?.(item);
    requestOpenChange(false);
  }

  function handleUploadChange(nextFiles: FileUploadItem[]): void {
    uploadFiles = nextFiles;
    onUpload?.(uploadFiles);
  }
</script>

<Dialog
  open={isOpen}
  {title}
  kind="dialog"
  onOpenChange={requestOpenChange}
>
  <UiPresentationProvider sizeScale={resolvedSize} density={resolvedDensity}>
    <div class="poodle-media-picker" data-size={resolvedSize} data-density={resolvedDensity}>
      <Tabs
        items={tabItems}
        value={activeTab}
        onValueChange={(value) => (activeTab = value)}
      />

      {#if activeTab === "browse"}
        <div class="poodle-media-picker__search">
          <TextInput
            id="media-picker-search"
            bind:value={searchQuery}
            placeholder="Search media..."
          />
        </div>

        {#if filteredItems.length === 0}
          <div class="poodle-media-picker__empty">
            <p>{emptyMessage}</p>
          </div>
        {:else}
          <div class="poodle-media-picker__grid" role="listbox" aria-label="Media items">
            {#each filteredItems as item (item.id)}
              <button
                type="button"
                class="poodle-media-picker__item"
                role="option"
                aria-selected="false"
                onclick={() => handleSelect(item)}
              >
                {#if item.thumbnailUrl}
                  <img class="poodle-media-picker__thumb" src={item.thumbnailUrl} alt="" />
                {:else}
                  <div class="poodle-media-picker__thumb poodle-media-picker__thumb--placeholder">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                      <rect x="3" y="3" width="18" height="18" rx="2" />
                      <circle cx="8.5" cy="8.5" r="1.5" />
                      <path d="M21 15l-5-5L5 21" />
                    </svg>
                  </div>
                {/if}
                <span class="poodle-media-picker__label">{item.label}</span>
              </button>
            {/each}
          </div>
        {/if}
      {:else}
        <div class="poodle-media-picker__upload">
          <FileUpload
            {accept}
            maxSize={maxFileSize}
            multiple
            files={uploadFiles}
            onChange={handleUploadChange}
          />
        </div>
      {/if}
    </div>
  </UiPresentationProvider>
</Dialog>

<style>
  .poodle-media-picker {
    --poodle-media-picker-stack-gap: var(--poodle-space-stack-sm, 0.5rem);
    --poodle-media-picker-search-offset: 0.25rem;
    --poodle-media-picker-grid-gap: 0.375rem;
    --poodle-media-picker-item-pad: 0.375rem;
    --poodle-media-picker-thumb-size: 4.5rem;
    --poodle-media-picker-grid-min: 5.5rem;
    display: flex;
    flex-direction: column;
    gap: var(--poodle-media-picker-stack-gap);
    min-height: 20rem;
  }

  .poodle-media-picker[data-size="xs"] {
    --poodle-media-picker-thumb-size: 3.5rem;
    --poodle-media-picker-grid-min: 4.75rem;
  }

  .poodle-media-picker[data-size="sm"] {
    --poodle-media-picker-thumb-size: 4.25rem;
    --poodle-media-picker-grid-min: 5.25rem;
  }

  .poodle-media-picker[data-size="lg"] {
    --poodle-media-picker-thumb-size: 5rem;
    --poodle-media-picker-grid-min: 6rem;
  }

  .poodle-media-picker[data-size="xl"] {
    --poodle-media-picker-thumb-size: 5.5rem;
    --poodle-media-picker-grid-min: 6.5rem;
  }

  .poodle-media-picker[data-density="compact"] {
    --poodle-media-picker-search-offset: 0.125rem;
    --poodle-media-picker-grid-gap: 0.25rem;
    --poodle-media-picker-item-pad: 0.25rem;
  }

  .poodle-media-picker[data-density="comfortable"] {
    --poodle-media-picker-search-offset: 0.375rem;
    --poodle-media-picker-grid-gap: 0.5rem;
    --poodle-media-picker-item-pad: 0.5rem;
  }

  .poodle-media-picker__search {
    margin-top: var(--poodle-media-picker-search-offset);
  }

  .poodle-media-picker__grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(var(--poodle-media-picker-grid-min), 1fr));
    gap: var(--poodle-media-picker-grid-gap);
    max-height: 20rem;
    overflow-y: auto;
  }

  .poodle-media-picker__item {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.25rem;
    padding: var(--poodle-media-picker-item-pad);
    border: 0.0625rem solid transparent;
    border-radius: var(--poodle-radius-control, 0.375rem);
    background: transparent;
    color: inherit;
    cursor: pointer;
    transition: border-color 0.1s, background 0.1s;
  }

  .poodle-media-picker__item:hover,
  .poodle-media-picker__item:focus-visible {
    border-color: var(--poodle-color-border-focus, #888);
    background: var(--poodle-color-background-panel, #1a1a1a);
    outline: none;
  }

  .poodle-media-picker__thumb {
    width: var(--poodle-media-picker-thumb-size);
    height: var(--poodle-media-picker-thumb-size);
    border-radius: 0.25rem;
    object-fit: cover;
  }

  .poodle-media-picker__thumb--placeholder {
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--poodle-color-background-panel, #1a1a1a);
  }

  .poodle-media-picker__thumb--placeholder svg {
    width: 1.5rem;
    height: 1.5rem;
    color: var(--poodle-color-text-tertiary, #666);
  }

  .poodle-media-picker__label {
    font-size: var(--poodle-typography-label-size);
    color: var(--poodle-color-text-secondary, #999);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 100%;
  }

  .poodle-media-picker__empty {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 10rem;
  }

  .poodle-media-picker__empty p {
    margin: 0;
    color: var(--poodle-color-text-secondary, #999);
    font-size: 0.875rem;
  }

  .poodle-media-picker__upload {
    margin-top: var(--poodle-media-picker-search-offset);
  }
</style>
