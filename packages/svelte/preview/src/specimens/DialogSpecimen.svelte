<script lang="ts">
  import { Dialog, Button, Eyebrow, TextInput, Select, Field } from "@poodle/svelte-primitives";

  let basicOpen = false;
  let customHeaderOpen = false;
  let wideOpen = false;
  let bareOpen = false;
  let noBackdropOpen = false;
  let closeButtonOpen = false;
  let widthOpenMap: Record<string, boolean> = {};
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Basic dialog with title and actions</Eyebrow>
    <Button variant="secondary" on:click={() => (basicOpen = true)}>Open dialog</Button>
    <Dialog
      open={basicOpen}
      title="Confirm action"
      description="Are you sure you want to proceed? This action cannot be undone."
      showCloseButton
      on:openChange={(e) => (basicOpen = e.detail.open)}
    >
      <p>This is the dialog body content. You can place any content here.</p>
      <svelte:fragment slot="actions">
        <Button variant="ghost" on:click={() => (basicOpen = false)}>Cancel</Button>
        <Button on:click={() => (basicOpen = false)}>Confirm</Button>
      </svelte:fragment>
    </Dialog>
  </div>

  <div class="specimen__group">
    <Eyebrow>Custom header slot</Eyebrow>
    <Button variant="secondary" on:click={() => (customHeaderOpen = true)}>Open with custom header</Button>
    <Dialog
      open={customHeaderOpen}
      showCloseButton
      on:openChange={(e) => (customHeaderOpen = e.detail.open)}
    >
      <svelte:fragment slot="header">
        <div class="custom-header">
          <div class="custom-header__badge">NEW</div>
          <h2 class="custom-header__title">Custom Header Layout</h2>
          <p class="custom-header__subtitle">The header slot gives you full control over the dialog header structure.</p>
        </div>
      </svelte:fragment>

      <p>When using the <code>header</code> slot, the built-in title/description are replaced entirely. This lets you add badges, icons, tabs, or any custom layout in the header area.</p>

      <svelte:fragment slot="actions">
        <Button variant="ghost" on:click={() => (customHeaderOpen = false)}>Close</Button>
        <Button on:click={() => (customHeaderOpen = false)}>Continue</Button>
      </svelte:fragment>
    </Dialog>
  </div>

  <div class="specimen__group">
    <Eyebrow>Width presets</Eyebrow>
    <div class="specimen__row">
      {#each ["sm", "md", "lg", "xl"] as w}
        <Button variant="secondary" on:click={() => (widthOpenMap[w] = true)}>{w}</Button>
        <Dialog
          open={widthOpenMap[w] ?? false}
          width={w}
          title="Width: {w}"
          showCloseButton
          on:openChange={(e) => (widthOpenMap[w] = e.detail.open)}
        >
          <p>This dialog uses <code>width="{w}"</code>. The surface scales from 24rem (sm) through 64rem (xl).</p>
          <svelte:fragment slot="actions">
            <Button on:click={() => (widthOpenMap[w] = false)}>Close</Button>
          </svelte:fragment>
        </Dialog>
      {/each}
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Wide dialog with form content</Eyebrow>
    <Button variant="secondary" on:click={() => (wideOpen = true)}>Open wide dialog</Button>
    <Dialog
      open={wideOpen}
      width="lg"
      title="Edit profile"
      description="Update your account information below."
      showCloseButton
      on:openChange={(e) => (wideOpen = e.detail.open)}
    >
      <div class="form-grid">
        <Field label="Display name">
          <TextInput id="dialog-name" placeholder="Enter your name" />
        </Field>
        <Field label="Role">
          <Select id="dialog-role" options={[
            { value: "admin", label: "Admin" },
            { value: "editor", label: "Editor" },
            { value: "viewer", label: "Viewer" },
          ]} />
        </Field>
      </div>
      <svelte:fragment slot="actions">
        <Button variant="ghost" on:click={() => (wideOpen = false)}>Cancel</Button>
        <Button on:click={() => (wideOpen = false)}>Save changes</Button>
      </svelte:fragment>
    </Dialog>
  </div>

  <div class="specimen__group">
    <Eyebrow>Bare mode (no internal padding)</Eyebrow>
    <Button variant="secondary" on:click={() => (bareOpen = true)}>Open bare dialog</Button>
    <Dialog
      open={bareOpen}
      bare
      showCloseButton
      ariaLabel="Image preview"
      on:openChange={(e) => (bareOpen = e.detail.open)}
    >
      <div class="bare-content">
        <div class="bare-content__image">
          <span class="bare-content__placeholder">Image preview area</span>
        </div>
        <div class="bare-content__footer">
          <Button variant="ghost" on:click={() => (bareOpen = false)}>Close</Button>
          <Button on:click={() => (bareOpen = false)}>Download</Button>
        </div>
      </div>
    </Dialog>
  </div>

  <div class="specimen__group">
    <Eyebrow>Custom footer slot</Eyebrow>
    <Button variant="secondary" on:click={() => (closeButtonOpen = true)}>Open with footer</Button>
    <Dialog
      open={closeButtonOpen}
      title="Terms and conditions"
      showCloseButton
      on:openChange={(e) => (closeButtonOpen = e.detail.open)}
    >
      <p>The <code>footer</code> slot replaces the standard right-aligned actions row, giving you full control over the footer layout — left-aligned links, split buttons, progress indicators, etc.</p>

      <svelte:fragment slot="footer">
        <div class="custom-footer">
          <a href="#learn-more" class="custom-footer__link">Learn more</a>
          <div class="custom-footer__actions">
            <Button variant="ghost" on:click={() => (closeButtonOpen = false)}>Decline</Button>
            <Button on:click={() => (closeButtonOpen = false)}>Accept</Button>
          </div>
        </div>
      </svelte:fragment>
    </Dialog>
  </div>

  <div class="specimen__group">
    <Eyebrow>Non-dismissible (no backdrop close)</Eyebrow>
    <Button variant="secondary" on:click={() => (noBackdropOpen = true)}>Open persistent dialog</Button>
    <Dialog
      open={noBackdropOpen}
      title="Persistent dialog"
      description="This dialog cannot be dismissed by clicking the backdrop."
      dismissOnBackdrop={false}
      on:openChange={(e) => (noBackdropOpen = e.detail.open)}
    >
      <p>Click the backdrop — nothing happens. Use the button or Escape key to close.</p>
      <svelte:fragment slot="actions">
        <Button on:click={() => (noBackdropOpen = false)}>Got it</Button>
      </svelte:fragment>
    </Dialog>
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
    align-items: flex-start;
  }

  .specimen__group p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--poodle-color-text-secondary);
  }

  .specimen__group code {
    padding: 0.125rem 0.375rem;
    border-radius: 0.25rem;
    background: color-mix(in srgb, var(--poodle-color-background-panel) 72%, var(--poodle-color-background-elevated));
    font-family: var(--poodle-typography-code-family);
    font-size: 0.8125rem;
  }

  .specimen__row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    align-items: center;
  }

  .custom-header {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }

  .custom-header__badge {
    display: inline-flex;
    align-self: flex-start;
    padding: 0.125rem 0.5rem;
    border-radius: 999px;
    background: var(--poodle-color-accent-base);
    color: var(--poodle-color-text-inverse);
    font-size: 0.625rem;
    font-weight: 700;
    letter-spacing: 0.05em;
    text-transform: uppercase;
  }

  .custom-header__title {
    margin: 0;
    font-family: var(--poodle-typography-heading-family);
    font-size: 1.125rem;
    font-weight: 600;
    line-height: 1.2;
  }

  .custom-header__subtitle {
    margin: 0;
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
  }

  .form-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
  }

  .bare-content {
    display: flex;
    flex-direction: column;
  }

  .bare-content__image {
    display: grid;
    place-items: center;
    min-height: 16rem;
    background: color-mix(in srgb, var(--poodle-color-background-canvas) 90%, black);
  }

  .bare-content__placeholder {
    color: var(--poodle-color-text-secondary);
    font-size: 0.875rem;
  }

  .bare-content__footer {
    display: flex;
    justify-content: flex-end;
    gap: 0.5rem;
    padding: 0.75rem 1rem;
    border-top: 0.0625rem solid var(--poodle-color-border-subtle);
  }

  .custom-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .custom-footer__link {
    color: var(--poodle-color-accent-base);
    font-size: 0.8125rem;
    text-decoration: none;
  }

  .custom-footer__link:hover {
    text-decoration: underline;
  }

  .custom-footer__actions {
    display: flex;
    gap: 0.5rem;
  }
</style>
