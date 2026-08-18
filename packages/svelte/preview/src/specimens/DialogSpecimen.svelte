<script lang="ts">
  import { Dialog, Button, TextInput, Select, Field, Checkbox, Pill, Eyebrow, Surface, Popover } from "@inflatable-cookie/poodle-svelte";

  // One $state rune puts the whole file in runes mode, where plain `let`
  // stops being reactive — every flag here must be $state or its dialog
  // never opens.
  let basicOpen = $state(false);
  let formOpen = $state(false);
  let contentOnlyOpen = $state(false);
  let customFooterOpen = $state(false);
  let bareOpen = $state(false);
  let wideOpen = $state(false);
  let scrollableOpen = $state(false);
  let widthOpenMap: Record<string, boolean> = $state({});
  let overlayInDialogOpen = $state(false);
</script>

<Surface tone="panel" border="subtle" padding="md">
  <div class="poodle-specimen">
    <div class="poodle-specimen__row">
      <!-- The combination nothing covered: an anchored surface opened from
           inside a modal. The popover portals to the theme root, leaving the
           dialog's stacking context, so it has to be lifted above it or it
           renders behind the thing that opened it. -->
      <Eyebrow>Popover inside a dialog</Eyebrow>
      <Button variant="secondary" onClick={() => (overlayInDialogOpen = true)}>Open dialog</Button>
    </div>

    <div class="poodle-specimen__row">
      <Eyebrow>Informational</Eyebrow>
      <Button variant="secondary" onClick={() => (basicOpen = true)}>View details</Button>
    </div>

    <div class="poodle-specimen__row">
      <Eyebrow>Form</Eyebrow>
      <Button variant="secondary" onClick={() => (formOpen = true)}>Create project</Button>
    </div>

    <div class="poodle-specimen__row">
      <Eyebrow>Custom header</Eyebrow>
      <Button variant="secondary" onClick={() => (contentOnlyOpen = true)}>View changelog</Button>
    </div>

    <div class="poodle-specimen__row">
      <Eyebrow>Custom footer</Eyebrow>
      <Button variant="secondary" onClick={() => (customFooterOpen = true)}>Terms &amp; conditions</Button>
    </div>

    <div class="poodle-specimen__row">
      <Eyebrow>Bare mode</Eyebrow>
      <Button variant="secondary" onClick={() => (bareOpen = true)}>Preview image</Button>
    </div>

    <div class="poodle-specimen__row">
      <Eyebrow>Scrollable</Eyebrow>
      <Button variant="secondary" onClick={() => (scrollableOpen = true)}>View log</Button>
    </div>

    <div class="poodle-specimen__row">
      <Eyebrow>Width presets</Eyebrow>
      {#each ["sm", "md", "lg", "xl"] as w}
        <Button variant="secondary" onClick={() => (widthOpenMap[w] = true)}>{w}</Button>
      {/each}
    </div>

    <div class="poodle-specimen__row">
      <Eyebrow>Non-dismissible</Eyebrow>
      <Button variant="secondary" onClick={() => (wideOpen = true)}>Open persistent</Button>
    </div>
  </div>
</Surface>

<!-- Dialogs (rendered outside the Surface, portaled to [data-theme]) -->

<Dialog
  open={basicOpen}
  title="Keyboard shortcuts"
  showCloseButton
  onOpenChange={(open) => (basicOpen = open)}
>
  <div class="poodle-shortcuts-list">
    <div class="poodle-shortcut"><kbd>⌘ K</kbd> <span>Command palette</span></div>
    <div class="poodle-shortcut"><kbd>⌘ S</kbd> <span>Save</span></div>
    <div class="poodle-shortcut"><kbd>⌘ /</kbd> <span>Toggle comment</span></div>
    <div class="poodle-shortcut"><kbd>⌘ ⇧ P</kbd> <span>Quick actions</span></div>
    <div class="poodle-shortcut"><kbd>Esc</kbd> <span>Close dialog</span></div>
  </div>
</Dialog>

<Dialog
  open={formOpen}
  title="New project"
  description="Set up a new project workspace."
  width="lg"
  showCloseButton
  onOpenChange={(open) => (formOpen = open)}
>
  <div class="poodle-form-grid">
    <Field label="Project name" id="dialog-project-name">
      <TextInput id="dialog-proj-name" placeholder="My project" />
    </Field>
    <Field label="Template" id="dialog-template">
      <Select id="dialog-template" placeholder="Choose a template" options={[
        { value: "blank", label: "Blank" },
        { value: "starter", label: "Starter kit" },
        { value: "advanced", label: "Advanced" },
      ]} />
    </Field>
    <div class="poodle-form-full-width">
      <Field label="Description" id="dialog-description">
        <TextInput id="dialog-desc" placeholder="What is this project for?" rows={3} />
      </Field>
    </div>
    <div class="poodle-form-full-width">
      <Checkbox id="dialog-private" label="Make this project private" />
    </div>
  </div>
  {#snippet actions()}
    <Button variant="ghost" onClick={() => (formOpen = false)}>Cancel</Button>
    <Button onClick={() => (formOpen = false)}>Create project</Button>
  {/snippet}
</Dialog>

<Dialog
  open={contentOnlyOpen}
  showCloseButton
  onOpenChange={(open) => (contentOnlyOpen = open)}
>
  {#snippet header()}
    <div class="poodle-changelog-header">
      <h2 class="poodle-changelog-header__title">What's new</h2>
      <Pill tone="info" appearance="badge">v2.4.0</Pill>
    </div>
  {/snippet}
  <div class="poodle-changelog">
    <div class="poodle-changelog__entry">
      <strong>Dialog flexibility improvements</strong>
      <p>Dialogs now support custom headers, footers, width presets, and bare mode.</p>
    </div>
    <div class="poodle-changelog__entry">
      <strong>Size propagation fixes</strong>
      <p>All parent components now correctly forward size and density to embedded children.</p>
    </div>
  </div>
</Dialog>

<Dialog
  open={customFooterOpen}
  title="Terms of service"
  showCloseButton
  onOpenChange={(open) => (customFooterOpen = open)}
>
  <div class="poodle-terms-body">
    <p>By using this service, you agree to our terms and conditions.</p>
  </div>
  {#snippet footer()}
    <div class="poodle-split-footer">
      <a href="#terms" class="poodle-split-footer__link">Read full terms</a>
      <div class="poodle-split-footer__actions">
        <Button variant="ghost" onClick={() => (customFooterOpen = false)}>Decline</Button>
        <Button onClick={() => (customFooterOpen = false)}>Accept</Button>
      </div>
    </div>
  {/snippet}
</Dialog>

<Dialog
  open={bareOpen}
  bare
  width="lg"
  ariaLabel="Image preview"
  onOpenChange={(open) => (bareOpen = open)}
>
  <div class="poodle-image-preview">
    <div class="poodle-image-preview__canvas">
      <span class="poodle-image-preview__placeholder">2400 × 1600</span>
    </div>
    <div class="poodle-image-preview__bar">
      <div class="poodle-image-preview__meta">
        <strong>landscape-hero.png</strong>
        <span>2.4 MB · Uploaded today</span>
      </div>
      <div class="poodle-image-preview__actions">
        <Button variant="ghost" onClick={() => (bareOpen = false)}>Close</Button>
        <Button leadingIcon="download" onClick={() => (bareOpen = false)}>Download</Button>
      </div>
    </div>
  </div>
</Dialog>

<Dialog
  open={scrollableOpen}
  title="Activity log"
  description="Recent activity across all projects."
  showCloseButton
  onOpenChange={(open) => (scrollableOpen = open)}
>
  <div class="poodle-log-list">
    {#each Array(20) as _, i}
      <div class="poodle-log-entry">
        <span class="poodle-log-entry__time">{String(9 + Math.floor(i / 3)).padStart(2, "0")}:{String((i * 17) % 60).padStart(2, "0")}</span>
        <span class="poodle-log-entry__message">
          {["User signed in", "Project created", "File uploaded", "Settings updated", "Comment added", "Build completed", "Deploy started", "Review requested"][i % 8]}
        </span>
      </div>
    {/each}
  </div>
  {#snippet actions()}
    <Button variant="ghost" onClick={() => (scrollableOpen = false)}>Close</Button>
    <Button onClick={() => (scrollableOpen = false)}>Export log</Button>
  {/snippet}
</Dialog>

{#each ["sm", "md", "lg", "xl"] as w}
  <Dialog
    open={widthOpenMap[w] ?? false}
    width={w as "sm" | "md" | "lg" | "xl"}
    title="Width: {w}"
    showCloseButton
    onOpenChange={(open) => (widthOpenMap[w] = open)}
  >
    <p>This dialog uses <code>width="{w}"</code>.</p>
    {#snippet actions()}
      <Button onClick={() => (widthOpenMap[w] = false)}>Close</Button>
    {/snippet}
  </Dialog>
{/each}

<Dialog
  open={wideOpen}
  title="Processing"
  dismissOnBackdrop={false}
  dismissOnEscape={false}
  onOpenChange={(open) => (wideOpen = open)}
>
  <p>This dialog cannot be dismissed by clicking the backdrop or pressing Escape.</p>
  {#snippet actions()}
    <Button onClick={() => (wideOpen = false)}>Done</Button>
  {/snippet}
</Dialog>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .poodle-specimen__row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    align-items: center;
  }

  .poodle-shortcuts-list { display: flex; flex-direction: column; gap: 0.5rem; }
  .poodle-shortcut { display: flex; align-items: center; gap: 0.75rem; }
  .poodle-shortcut kbd {
    display: inline-flex; align-items: center; justify-content: center;
    min-width: 4.5rem; padding: 0.25rem 0.5rem;
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: 0.25rem;
    background: color-mix(in srgb, var(--poodle-color-background-panel) 80%, var(--poodle-color-background-elevated));
    font-family: var(--poodle-typography-code-family); font-size: 0.75rem; font-weight: 500;
  }
  .poodle-shortcut span { color: var(--poodle-color-text-secondary); font-size: 0.8125rem; }

  .poodle-form-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; }
  .poodle-form-full-width { grid-column: 1 / -1; }

  .poodle-changelog-header { display: flex; align-items: center; gap: 0.625rem; }
  .poodle-changelog-header__title { margin: 0; font-family: var(--poodle-typography-heading-family); font-size: 1rem; font-weight: 600; line-height: 1.2; }
  .poodle-changelog { display: flex; flex-direction: column; gap: 0.875rem; }
  .poodle-changelog__entry { display: flex; flex-direction: column; gap: 0.25rem; }
  .poodle-changelog__entry strong { font-size: 0.875rem; }
  .poodle-changelog__entry p { margin: 0; color: var(--poodle-color-text-secondary); font-size: 0.8125rem; line-height: 1.5; }

  .poodle-terms-body p { margin: 0 0 0.5rem; color: var(--poodle-color-text-secondary); font-size: 0.8125rem; line-height: 1.6; }
  .poodle-split-footer { display: flex; align-items: center; justify-content: space-between; }
  .poodle-split-footer__link { color: var(--poodle-color-accent-base); font-size: 0.8125rem; text-decoration: none; }
  .poodle-split-footer__link:hover { text-decoration: underline; }
  .poodle-split-footer__actions { display: flex; gap: 0.5rem; }

  .poodle-image-preview { display: flex; flex-direction: column; }
  .poodle-image-preview__canvas { display: grid; place-items: center; min-height: 20rem; background: color-mix(in srgb, var(--poodle-color-background-canvas) 90%, black); border-radius: var(--poodle-radius-surface) var(--poodle-radius-surface) 0 0; }
  .poodle-image-preview__placeholder { color: var(--poodle-color-text-secondary); font-family: var(--poodle-typography-code-family); font-size: 0.875rem; opacity: 0.5; }
  .poodle-image-preview__bar { display: flex; align-items: center; justify-content: space-between; gap: 1rem; padding: 0.75rem 1rem; border-top: 0.0625rem solid var(--poodle-color-border-subtle); }
  .poodle-image-preview__meta { display: flex; flex-direction: column; gap: 0.125rem; }
  .poodle-image-preview__meta strong { font-size: 0.8125rem; }
  .poodle-image-preview__meta span { color: var(--poodle-color-text-secondary); font-size: 0.75rem; }
  .poodle-image-preview__actions { display: flex; gap: 0.5rem; }

  .poodle-log-list { display: flex; flex-direction: column; gap: 0.25rem; max-height: 18rem; overflow-y: auto; }
  .poodle-log-entry { display: flex; gap: 0.75rem; padding: 0.375rem 0; border-bottom: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 50%, transparent); }
  .poodle-log-entry__time { color: var(--poodle-color-text-secondary); font-family: var(--poodle-typography-code-family); font-size: 0.75rem; min-width: 3rem; }
  .poodle-log-entry__message { font-size: 0.8125rem; }
</style>

<Dialog bind:open={overlayInDialogOpen} title="Settings" showCloseButton>
  <Field id="overlay-model" label="Model">
    <Popover ariaLabel="Model settings">
      {#snippet trigger()}
        <Button variant="secondary">Opus 5 · Medium</Button>
      {/snippet}
      <div style="display:flex;flex-direction:column;gap:0.25rem;min-width:14rem;">
        <strong>Opus 5</strong>
        <span>Reasoning depth. Higher costs more time and tokens.</span>
      </div>
    </Popover>
  </Field>
</Dialog>
