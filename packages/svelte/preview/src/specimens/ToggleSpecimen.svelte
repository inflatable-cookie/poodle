<script lang="ts">
  import { Toggle, Eyebrow, Icon } from "@poodle/svelte-primitives";
  import { bold as boldIcon, italic as italicIcon, pin, star } from "@poodle/icons-lucide";

  let bold = false;
  let italic = false;
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Ghost variant (default)</Eyebrow>
    <div class="toggle-row">
      <Toggle
        pressed={bold}
        ariaLabel="Bold"
        on:pressedChange={(e) => (bold = e.detail.pressed)}
      ><Icon icon={boldIcon} /></Toggle>
      <Toggle
        pressed={italic}
        ariaLabel="Italic"
        on:pressedChange={(e) => (italic = e.detail.pressed)}
      ><Icon icon={italicIcon} /></Toggle>
    </div>
    <p>Bold: {bold}, Italic: {italic}</p>
  </div>

  <div class="specimen__group">
    <Eyebrow>Primary variant</Eyebrow>
    <Toggle variant="primary" defaultPressed ariaLabel="Pinned"><Icon icon={pin} /> Pinned</Toggle>
  </div>

  <div class="specimen__group">
    <Eyebrow>Secondary variant</Eyebrow>
    <Toggle variant="secondary" ariaLabel="Favorite"><Icon icon={star} /> Favorite</Toggle>
  </div>

  <div class="specimen__group">
    <Eyebrow>Densities</Eyebrow>
    <div class="specimen__stack">
      {#each ["compact", "default", "comfortable"] as density}
        <div class="toggle-row">
          <span class="specimen__label">{density}</span>
          <Toggle {density} ariaLabel={"Toggle at " + density}><Icon icon={star} /> Option</Toggle>
        </div>
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <Toggle defaultPressed disabled ariaLabel="Disabled toggle">Locked</Toggle>
  </div>
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

  .toggle-row {
    display: flex;
    gap: 0.25rem;
    align-items: center;
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

  .specimen__group p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--poodle-color-text-secondary);
  }
</style>
