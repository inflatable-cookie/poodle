<script lang="ts">
  import { NavigationMenu, Eyebrow, type NavigationMenuItem } from "@flint/svelte-primitives";

  const items: NavigationMenuItem[] = [
    { value: "home", label: "Home" },
    { value: "components", label: "Components" },
    { value: "tokens", label: "Tokens" },
    { value: "guides", label: "Guides" },
    { value: "changelog", label: "Changelog", isDisabled: true },
  ];

  let active = "components";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Horizontal navigation</Eyebrow>
    <NavigationMenu
      {items}
      value={active}
      ariaLabel="Main navigation"
      on:valueChange={(e) => { if (e.detail.value) active = e.detail.value; }}
    >
      <p>Active section: <strong>{active}</strong></p>
    </NavigationMenu>
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

  p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--flint-color-text-secondary);
  }
</style>
