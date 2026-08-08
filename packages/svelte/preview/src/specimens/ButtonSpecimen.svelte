<script lang="ts">
  import { Button } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let clickLog = $state("No button clicked yet.");
  let intent = $state("save");

  let bookmarked = $state(false);

  function log(label: string): void {
    clickLog = `Clicked: ${label}`;
  }
</script>

<SpecimenLayout>
  <SpecimenGroup label="Variants">
    <div class="poodle-specimen__row">
      <Button variant="primary" onClick={() => log("Primary")}>Primary</Button>
      <Button variant="secondary" onClick={() => log("Secondary")}>Secondary</Button>
      <Button variant="ghost" onClick={() => log("Ghost")}>Ghost</Button>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Danger tone">
    <div class="poodle-specimen__row">
      <Button variant="primary" tone="danger" onClick={() => log("Danger primary")}>Danger primary</Button>
      <Button variant="secondary" tone="danger" onClick={() => log("Danger secondary")}>Danger secondary</Button>
      <Button variant="ghost" tone="danger" onClick={() => log("Danger ghost")}>Danger ghost</Button>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="With icons">
    <div class="poodle-specimen__row">
      <Button leadingIcon="plus" onClick={() => log("Leading icon")}>Create</Button>
      <Button trailingIcon="external-link" onClick={() => log("Trailing icon")}>Open</Button>
      <Button leadingIcon="save" trailingIcon="check" onClick={() => log("Both icons")}>Save</Button>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="With chevron">
    <div class="poodle-specimen__row">
      <Button chevron onClick={() => log("Chevron")}>Options</Button>
      <Button variant="primary" chevron onClick={() => log("Primary chevron")}>Actions</Button>
      <Button leadingIcon="filter" chevron onClick={() => log("Icon + chevron")}>Filter</Button>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="States">
    <div class="poodle-specimen__row">
      <Button variant="primary" disabled>Disabled</Button>
      <Button variant="primary" loading>Loading</Button>
      <Button variant="secondary" disabled>Disabled secondary</Button>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Toggle (pressed state)">
    <div class="poodle-specimen__row">
      <Button
        variant="secondary"
        leadingIcon="star"
        bind:pressed={bookmarked}
      >{bookmarked ? "Bookmarked" : "Bookmark"}</Button>
      <Button variant="secondary" leadingIcon="heart" defaultPressed={false}>Like</Button>
      <Button variant="ghost" leadingIcon="lock-open" defaultPressed>Locked</Button>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Form overrides">
    <form
      class="poodle-specimen__form"
      onsubmit={(event) => {
        event.preventDefault();
        log(`Submitted via ${intent}`);
      }}
    >
      <input type="hidden" name="intent" value={intent} />
      <div class="poodle-specimen__row">
        <Button
          type="submit"
          variant="secondary"
          onClick={() => {
            intent = "save";
          }}
        >
          Save
        </Button>
        <Button
          type="submit"
          variant="primary"
          formaction="/publish"
          formnovalidate
          onClick={() => {
            intent = "publish";
          }}
        >
          Publish
        </Button>
      </div>
    </form>
  </SpecimenGroup>

  <p class="poodle-specimen__log">{clickLog}</p>

  {#snippet sizes(size)}
    <Button variant="primary" {size} leadingIcon="plus" onClick={() => log(`Size ${size}`)}>{size.toUpperCase()}</Button>
  {/snippet}

  {#snippet densities(density)}
    <Button variant="secondary" {density} leadingIcon="download" onClick={() => log(`Density ${density}`)}>Action</Button>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen__row {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.5rem;
  }

  .poodle-specimen__log {
    font-size: 0.8125rem;
    color: var(--poodle-color-text-secondary);
    margin: 0;
    padding: 0.5rem 0.75rem;
    background: color-mix(in srgb, var(--poodle-color-background-canvas) 80%, transparent);
    border-radius: var(--poodle-radius-control);
  }
</style>
