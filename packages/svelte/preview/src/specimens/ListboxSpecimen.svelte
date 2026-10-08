<script lang="ts">
  import { Listbox, Surface, type ListboxItem } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const libraries: ListboxItem[] = [
    { value: "north", label: "Northstar" },
    { value: "poodle", label: "Poodle" },
    { value: "underlay", label: "Underlay" },
  ];
  const cards: ListboxItem[] = [
    { value: "amber", label: "Amber Library" },
    { value: "blue", label: "Blue Library" },
    { value: "green", label: "Green Library" },
  ];
  const multiple: ListboxItem[] = [
    { value: "audio", label: "Audio projects" },
    { value: "video", label: "Video projects" },
    { value: "image", label: "Image projects" },
  ];
  const withDisabled: ListboxItem[] = [
    { value: "ready", label: "Ready to use" },
    { value: "archived", label: "Archived collection", disabled: true },
    { value: "recent", label: "Recently opened" },
  ];
</script>

<div class="poodle-specimen">
  <SpecimenGroup label="Plain rows — single selection and typeahead">
    <Listbox items={libraries} defaultValue="north" ariaLabel="Libraries" />
  </SpecimenGroup>

  <SpecimenGroup label="Rich card rows — host content with Listbox selection">
    <Listbox items={cards} defaultValue="blue" ariaLabel="Library cards">
      {#snippet children(item, selected, focused)}
        <div class="poodle-listbox-card" data-selected={selected} data-focused={focused}>
          <Surface padding="sm" border="subtle">
            <strong>{item.label}</strong>
            <div class="poodle-listbox-card-detail">Three collections · Updated today</div>
          </Surface>
        </div>
      {/snippet}
    </Listbox>
  </SpecimenGroup>

  <SpecimenGroup label="Multiple selection — Space, Shift+arrow, and Ctrl/Cmd+A">
    <Listbox items={multiple} selectionMode="multiple" defaultValues={["audio"]} ariaLabel="Project types" />
  </SpecimenGroup>

  <SpecimenGroup label="Disabled option — skipped by focus and typeahead">
    <Listbox items={withDisabled} defaultValue="ready" ariaLabel="Collections" />
  </SpecimenGroup>
</div>

<style>
  .poodle-specimen { display: flex; flex-direction: column; gap: 1rem; }
  .poodle-listbox-card-detail { margin-top: 0.25rem; color: var(--poodle-color-text-secondary); font-size: 0.8125rem; }
  .poodle-listbox-card { border-radius: var(--poodle-radius-surface); }
  .poodle-listbox-card[data-selected="true"] { outline: 2px solid var(--poodle-color-accent-base); }
  .poodle-listbox-card[data-focused="true"] { outline-offset: 2px; }
</style>
