<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/media-picker.css";
  import { default as Dialog } from "./Dialog.svelte";
  import { default as FileUpload } from "./FileUpload.svelte";
  import { default as Tabs } from "./Tabs.svelte";
  import { default as TextInput } from "./TextInput.svelte";
  import { default as UiPresentationProvider } from "./UiPresentationProvider.svelte";
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

