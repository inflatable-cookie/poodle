<script lang="ts">
  import { Toggle, Eyebrow, Icon } from "@pug/svelte-primitives";

  let bold = false;
  let italic = false;
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Ghost variant (default)</Eyebrow>
    <div class="toggle-row">
      <Toggle
        isPressed={bold}
        ariaLabel="Bold"
        on:pressedChange={(e) => (bold = e.detail.pressed)}
      ><Icon name="bold" size="sm" /></Toggle>
      <Toggle
        isPressed={italic}
        ariaLabel="Italic"
        on:pressedChange={(e) => (italic = e.detail.pressed)}
      ><Icon name="italic" size="sm" /></Toggle>
    </div>
    <p>Bold: {bold}, Italic: {italic}</p>
  </div>

  <div class="specimen__group">
    <Eyebrow>Primary variant</Eyebrow>
    <Toggle variant="primary" defaultPressed ariaLabel="Pinned"><Icon name="pin" size="sm" /> Pinned</Toggle>
  </div>

  <div class="specimen__group">
    <Eyebrow>Secondary variant</Eyebrow>
    <Toggle variant="secondary" ariaLabel="Favorite"><Icon name="star" size="sm" /> Favorite</Toggle>
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <Toggle defaultPressed isDisabled ariaLabel="Disabled toggle">Locked</Toggle>
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
  }

  .specimen__group p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--pug-color-text-secondary);
  }
</style>
