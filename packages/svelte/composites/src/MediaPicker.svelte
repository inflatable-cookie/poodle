<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { Dialog, Button, TextInput, FileUpload, Tabs } from "@flint/svelte-primitives";

  import type { FileUploadItem, TabItem } from "@flint/svelte-primitives";
  import type { MediaPickerItem } from "./types";

  export let open: boolean | null = null;
  export let items: MediaPickerItem[] = [];
  export let accept = "image/*";
  export let maxFileSize: number = 25 * 1024 * 1024;
  export let title = "Select media";
  export let emptyMessage = "No media items found.";

  const dispatch = createEventDispatcher<{
    select: { item: MediaPickerItem };
    upload: { files: FileUploadItem[] };
    openChange: { open: boolean };
  }>();

  let activeTab = "browse";
  let searchQuery = "";
  let uploadFiles: FileUploadItem[] = [];

  const tabItems: TabItem[] = [
    { value: "browse", label: "Browse" },
    { value: "upload", label: "Upload" },
  ];

  $: filteredItems = searchQuery
    ? items.filter((item) =>
        item.label.toLowerCase().includes(searchQuery.toLowerCase()),
      )
    : items;

  function handleSelect(item: MediaPickerItem): void {
    dispatch("select", { item });
    dispatch("openChange", { open: false });
  }

  function handleUploadChange(event: CustomEvent<{ files: FileUploadItem[] }>): void {
    uploadFiles = event.detail.files;
    dispatch("upload", { files: uploadFiles });
  }

  function handleOpenChange(event: CustomEvent<{ open: boolean }>): void {
    dispatch("openChange", event.detail);
  }
</script>

<Dialog
  {open}
  {title}
  kind="dialog"
  on:openChange={handleOpenChange}
>
  <div class="media-picker">
    <Tabs
      items={tabItems}
      value={activeTab}
      on:valueChange={(e) => (activeTab = e.detail.value)}
    />

    {#if activeTab === "browse"}
      <div class="media-picker__search">
        <TextInput
          id="media-picker-search"
          bind:value={searchQuery}
          placeholder="Search media..."
        />
      </div>

      {#if filteredItems.length === 0}
        <div class="media-picker__empty">
          <p>{emptyMessage}</p>
        </div>
      {:else}
        <div class="media-picker__grid" role="listbox" aria-label="Media items">
          {#each filteredItems as item (item.id)}
            <button
              type="button"
              class="media-picker__item"
              role="option"
              aria-selected="false"
              on:click={() => handleSelect(item)}
            >
              {#if item.thumbnailUrl}
                <img class="media-picker__thumb" src={item.thumbnailUrl} alt="" />
              {:else}
                <div class="media-picker__thumb media-picker__thumb--placeholder">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                    <rect x="3" y="3" width="18" height="18" rx="2" />
                    <circle cx="8.5" cy="8.5" r="1.5" />
                    <path d="M21 15l-5-5L5 21" />
                  </svg>
                </div>
              {/if}
              <span class="media-picker__label">{item.label}</span>
            </button>
          {/each}
        </div>
      {/if}
    {:else}
      <div class="media-picker__upload">
        <FileUpload
          {accept}
          maxSize={maxFileSize}
          multiple
          files={uploadFiles}
          on:change={handleUploadChange}
        />
      </div>
    {/if}
  </div>
</Dialog>

<style>
  .media-picker {
    display: flex;
    flex-direction: column;
    gap: var(--flint-space-stack-sm, 0.5rem);
    min-height: 20rem;
  }

  .media-picker__search {
    margin-top: 0.25rem;
  }

  .media-picker__grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(5.5rem, 1fr));
    gap: 0.375rem;
    max-height: 20rem;
    overflow-y: auto;
  }

  .media-picker__item {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.25rem;
    padding: 0.375rem;
    border: 0.0625rem solid transparent;
    border-radius: var(--flint-radius-control, 0.375rem);
    background: transparent;
    color: inherit;
    cursor: pointer;
    transition: border-color 0.1s, background 0.1s;
  }

  .media-picker__item:hover,
  .media-picker__item:focus-visible {
    border-color: var(--flint-color-border-focus, #888);
    background: var(--flint-color-background-panel, #1a1a1a);
    outline: none;
  }

  .media-picker__thumb {
    width: 4.5rem;
    height: 4.5rem;
    border-radius: 0.25rem;
    object-fit: cover;
  }

  .media-picker__thumb--placeholder {
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--flint-color-background-panel, #1a1a1a);
  }

  .media-picker__thumb--placeholder svg {
    width: 1.5rem;
    height: 1.5rem;
    color: var(--flint-color-text-tertiary, #666);
  }

  .media-picker__label {
    font-size: 0.6875rem;
    color: var(--flint-color-text-secondary, #999);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 100%;
  }

  .media-picker__empty {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 10rem;
  }

  .media-picker__empty p {
    margin: 0;
    color: var(--flint-color-text-secondary, #999);
    font-size: 0.875rem;
  }

  .media-picker__upload {
    margin-top: 0.25rem;
  }
</style>
