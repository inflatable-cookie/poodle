<script lang="ts">
  import { Button, Eyebrow } from "@poodle/svelte-primitives";
  import type { ControlDensity } from "@poodle/svelte-primitives";

  const densities: ControlDensity[] = ["compact", "default", "comfortable"];

  let clickLog = "No button clicked yet.";
  let intent = "save";
  const controlSizes = ["xs", "sm", "md", "lg", "xl"] as const;

  let boldPressed = false;
  let italicPressed = false;
  let underlinePressed = true;
  let bookmarked = false;

  function log(label: string): void {
    clickLog = `Clicked: ${label}`;
  }
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Variants</Eyebrow>
    <div class="specimen__row">
      <Button variant="primary" on:click={() => log("Primary")}>Primary</Button>
      <Button variant="secondary" on:click={() => log("Secondary")}>Secondary</Button>
      <Button variant="ghost" on:click={() => log("Ghost")}>Ghost</Button>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Danger tone</Eyebrow>
    <div class="specimen__row">
      <Button variant="primary" tone="danger" on:click={() => log("Danger primary")}>Danger primary</Button>
      <Button variant="secondary" tone="danger" on:click={() => log("Danger secondary")}>Danger secondary</Button>
      <Button variant="ghost" tone="danger" on:click={() => log("Danger ghost")}>Danger ghost</Button>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>With icons</Eyebrow>
    <div class="specimen__row">
      <Button leadingIcon="plus" on:click={() => log("Leading icon")}>Create</Button>
      <Button trailingIcon="external-link" on:click={() => log("Trailing icon")}>Open</Button>
      <Button leadingIcon="save" trailingIcon="check" on:click={() => log("Both icons")}>Save</Button>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>With chevron</Eyebrow>
    <div class="specimen__row">
      <Button chevron on:click={() => log("Chevron")}>Options</Button>
      <Button variant="primary" chevron on:click={() => log("Primary chevron")}>Actions</Button>
      <Button leadingIcon="filter" chevron on:click={() => log("Icon + chevron")}>Filter</Button>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Sizes</Eyebrow>
    <div class="specimen__row">
      {#each controlSizes as size}
        <Button variant="primary" leadingIcon="plus" {size} on:click={() => log(`Size ${size}`)}>{size.toUpperCase()}</Button>
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Densities</Eyebrow>
    <div class="specimen__stack">
      {#each densities as density}
        <div class="specimen__row">
          <span class="specimen__label">{density}</span>
          <Button variant="secondary" leadingIcon="download" {density} on:click={() => log(`Density ${density}`)}>Action</Button>
        </div>
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>States</Eyebrow>
    <div class="specimen__row">
      <Button variant="primary" disabled>Disabled</Button>
      <Button variant="primary" loading>Loading</Button>
      <Button variant="secondary" disabled>Disabled secondary</Button>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Toggle (pressed state)</Eyebrow>
    <div class="specimen__row">
      <Button
        variant="ghost"
        leadingIcon="bold"
        pressed={boldPressed}
        ariaLabel="Bold"
        on:pressedChange={(e) => (boldPressed = e.detail.pressed)}
      >B</Button>
      <Button
        variant="ghost"
        leadingIcon="italic"
        pressed={italicPressed}
        ariaLabel="Italic"
        on:pressedChange={(e) => (italicPressed = e.detail.pressed)}
      >I</Button>
      <Button
        variant="ghost"
        leadingIcon="underline"
        pressed={underlinePressed}
        ariaLabel="Underline"
        on:pressedChange={(e) => (underlinePressed = e.detail.pressed)}
      >U</Button>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Toggle with icon (uncontrolled)</Eyebrow>
    <div class="specimen__row">
      <Button
        variant="secondary"
        leadingIcon="star"
        defaultPressed={false}
        on:pressedChange={(e) => (bookmarked = e.detail.pressed)}
      >{bookmarked ? "Bookmarked" : "Bookmark"}</Button>
      <Button variant="ghost" leadingIcon="heart" defaultPressed={false}>Like</Button>
      <Button variant="ghost" leadingIcon="lock-open" defaultPressed>Lock</Button>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Form overrides</Eyebrow>
    <form class="specimen__form" on:submit|preventDefault={() => log(`Submitted via ${intent}`)}>
      <input type="hidden" name="intent" value={intent} />
      <div class="specimen__row">
        <Button
          type="submit"
          variant="secondary"
          on:click={() => {
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
          on:click={() => {
            intent = "publish";
          }}
        >
          Publish
        </Button>
      </div>
    </form>
  </div>

  <p class="specimen__log">{clickLog}</p>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .specimen__row {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.5rem;
  }

  .specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .specimen__label {
    font-size: 0.75rem;
    font-family: var(--poodle-typography-code-family);
    color: var(--poodle-color-text-muted);
    min-width: 6rem;
  }

  .specimen__log {
    font-size: 0.8125rem;
    color: var(--poodle-color-text-secondary);
    margin: 0;
    padding: 0.5rem 0.75rem;
    background: color-mix(in srgb, var(--poodle-color-background-canvas) 80%, transparent);
    border-radius: var(--poodle-radius-control);
  }
</style>
