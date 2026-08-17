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
  <SpecimenGroup label="A normal action row — the primary action, then the way out">
    <div class="poodle-specimen__row">
      <Button variant="primary" onClick={() => log("Save changes")}>Save changes</Button>
      <Button variant="ghost" onClick={() => log("Cancel")}>Cancel</Button>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Variants — how much weight the action carries">
    <div class="poodle-specimen__row">
      <Button variant="primary" onClick={() => log("Primary")}>Primary</Button>
      <Button variant="secondary" onClick={() => log("Secondary")}>Secondary</Button>
      <Button variant="ghost" onClick={() => log("Ghost")}>Ghost</Button>
    </div>
  </SpecimenGroup>

  <!-- One variant, every tone. Tone and variant compose freely, so showing the
       grid teaches nothing the two rows above and below do not. -->
  <SpecimenGroup label="Tones — what kind of action it is">
    <div class="poodle-specimen__row">
      <Button variant="secondary" onClick={() => log("Default tone")}>Default</Button>
      <Button variant="secondary" tone="danger" onClick={() => log("Danger tone")}>Delete</Button>
      <Button variant="secondary" tone="success" onClick={() => log("Success tone")}>Approve</Button>
      <Button variant="secondary" tone="warning" onClick={() => log("Warning tone")}>Override</Button>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Icons, disclosure, and icon-only">
    <div class="poodle-specimen__row">
      <Button leadingIcon="plus" onClick={() => log("Leading icon")}>Create</Button>
      <Button trailingIcon="external-link" onClick={() => log("Trailing icon")}>Open</Button>
      <Button leadingIcon="filter" chevron onClick={() => log("Icon + chevron")}>Filter</Button>
      <Button leadingIcon="settings" ariaLabel="Settings" onClick={() => log("Icon only")} />
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="States — unavailable, working, and held down">
    <div class="poodle-specimen__row">
      <Button variant="primary" disabled>Disabled</Button>
      <Button variant="primary" loading>Loading</Button>
      <Button
        variant="secondary"
        leadingIcon="star"
        bind:pressed={bookmarked}
      >{bookmarked ? "Bookmarked" : "Bookmark"}</Button>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Inside a form — each button can submit somewhere else">
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
