<script lang="ts">
  import { Dialog, Button, TextInput, Select, Field, Checkbox, Pill } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  let basicOpen = false;
  let formOpen = false;
  let contentOnlyOpen = false;
  let customHeaderOpen = false;
  let customFooterOpen = false;
  let bareOpen = false;
  let wideOpen = false;
  let scrollableOpen = false;
  let widthOpenMap: Record<string, boolean> = {};
</script>

<div class="specimen">
  <SpecimenGroup label="Simple informational">
    <Button variant="secondary" on:click={() => (basicOpen = true)}>View details</Button>
    <Dialog
      open={basicOpen}
      title="Keyboard shortcuts"
      showCloseButton
      on:openChange={(e) => (basicOpen = e.detail.open)}
    >
      <div class="shortcuts-list">
        <div class="shortcut"><kbd>⌘ K</kbd> <span>Command palette</span></div>
        <div class="shortcut"><kbd>⌘ S</kbd> <span>Save</span></div>
        <div class="shortcut"><kbd>⌘ /</kbd> <span>Toggle comment</span></div>
        <div class="shortcut"><kbd>⌘ ⇧ P</kbd> <span>Quick actions</span></div>
        <div class="shortcut"><kbd>Esc</kbd> <span>Close dialog</span></div>
      </div>
    </Dialog>
  </SpecimenGroup>

  <SpecimenGroup label="Form dialog">
    <Button variant="secondary" on:click={() => (formOpen = true)}>Create project</Button>
    <Dialog
      open={formOpen}
      title="New project"
      description="Set up a new project workspace."
      width="lg"
      showCloseButton
      on:openChange={(e) => (formOpen = e.detail.open)}
    >
      <div class="form-grid">
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
        <div class="form-full-width">
          <Field label="Description" id="dialog-description">
            <TextInput id="dialog-desc" placeholder="What is this project for?" rows={3} />
          </Field>
        </div>
        <div class="form-full-width">
          <Checkbox id="dialog-private" label="Make this project private" />
        </div>
      </div>
      <svelte:fragment slot="actions">
        <Button variant="ghost" on:click={() => (formOpen = false)}>Cancel</Button>
        <Button on:click={() => (formOpen = false)}>Create project</Button>
      </svelte:fragment>
    </Dialog>
  </SpecimenGroup>

  <SpecimenGroup label="Content-only (no actions)">
    <Button variant="secondary" on:click={() => (contentOnlyOpen = true)}>View changelog</Button>
    <Dialog
      open={contentOnlyOpen}
      showCloseButton
      on:openChange={(e) => (contentOnlyOpen = e.detail.open)}
    >
      <svelte:fragment slot="header">
        <div class="changelog-header">
          <h2 class="changelog-header__title">What's new</h2>
          <Pill tone="info" appearance="badge">v2.4.0</Pill>
        </div>
      </svelte:fragment>

      <div class="changelog">
        <div class="changelog__entry">
          <strong>Dialog flexibility improvements</strong>
          <p>Dialogs now support custom headers, footers, width presets, and bare mode for fully custom content.</p>
        </div>
        <div class="changelog__entry">
          <strong>Size propagation fixes</strong>
          <p>All parent components now correctly forward size and density to embedded children.</p>
        </div>
        <div class="changelog__entry">
          <strong>Calendar fixed width</strong>
          <p>Calendar components no longer stretch to fill their parent container.</p>
        </div>
      </div>
    </Dialog>
  </SpecimenGroup>

  <SpecimenGroup label="Custom footer">
    <Button variant="secondary" on:click={() => (customFooterOpen = true)}>Terms &amp; conditions</Button>
    <Dialog
      open={customFooterOpen}
      title="Terms of service"
      showCloseButton
      on:openChange={(e) => (customFooterOpen = e.detail.open)}
    >
      <div class="terms-body">
        <p>By using this service, you agree to our terms and conditions. Please review the full document before accepting.</p>
        <p>These terms govern your use of the platform, including data handling, privacy, and acceptable use policies.</p>
      </div>

      <svelte:fragment slot="footer">
        <div class="split-footer">
          <a href="#terms" class="split-footer__link">Read full terms</a>
          <div class="split-footer__actions">
            <Button variant="ghost" on:click={() => (customFooterOpen = false)}>Decline</Button>
            <Button on:click={() => (customFooterOpen = false)}>Accept</Button>
          </div>
        </div>
      </svelte:fragment>
    </Dialog>
  </SpecimenGroup>

  <SpecimenGroup label="Bare mode (image preview)">
    <Button variant="secondary" on:click={() => (bareOpen = true)}>Preview image</Button>
    <Dialog
      open={bareOpen}
      bare
      width="lg"
      ariaLabel="Image preview"
      on:openChange={(e) => (bareOpen = e.detail.open)}
    >
      <div class="image-preview">
        <div class="image-preview__canvas">
          <span class="image-preview__placeholder">2400 × 1600</span>
        </div>
        <div class="image-preview__bar">
          <div class="image-preview__meta">
            <strong>landscape-hero.png</strong>
            <span>2.4 MB · Uploaded today</span>
          </div>
          <div class="image-preview__actions">
            <Button variant="ghost" on:click={() => (bareOpen = false)}>Close</Button>
            <Button leadingIcon="download" on:click={() => (bareOpen = false)}>Download</Button>
          </div>
        </div>
      </div>
    </Dialog>
  </SpecimenGroup>

  <SpecimenGroup label="Scrollable content">
    <Button variant="secondary" on:click={() => (scrollableOpen = true)}>View log</Button>
    <Dialog
      open={scrollableOpen}
      title="Activity log"
      description="Recent activity across all projects."
      showCloseButton
      on:openChange={(e) => (scrollableOpen = e.detail.open)}
    >
      <div class="log-list">
        {#each Array(20) as _, i}
          <div class="log-entry">
            <span class="log-entry__time">{String(9 + Math.floor(i / 3)).padStart(2, "0")}:{String((i * 17) % 60).padStart(2, "0")}</span>
            <span class="log-entry__message">
              {["User signed in", "Project created", "File uploaded", "Settings updated", "Comment added", "Build completed", "Deploy started", "Review requested"][i % 8]}
            </span>
          </div>
        {/each}
      </div>
      <svelte:fragment slot="actions">
        <Button variant="ghost" on:click={() => (scrollableOpen = false)}>Close</Button>
        <Button on:click={() => (scrollableOpen = false)}>Export log</Button>
      </svelte:fragment>
    </Dialog>
  </SpecimenGroup>

  <SpecimenGroup label="Width presets">
    <div class="specimen__row">
      {#each ["sm", "md", "lg", "xl"] as w}
        <Button variant="secondary" on:click={() => (widthOpenMap[w] = true)}>{w}</Button>
        <Dialog
          open={widthOpenMap[w] ?? false}
          width={w as "sm" | "md" | "lg" | "xl"}
          title="Width: {w}"
          showCloseButton
          on:openChange={(e) => (widthOpenMap[w] = e.detail.open)}
        >
          <p>This dialog uses <code>width="{w}"</code>.</p>
          <svelte:fragment slot="actions">
            <Button on:click={() => (widthOpenMap[w] = false)}>Close</Button>
          </svelte:fragment>
        </Dialog>
      {/each}
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Non-dismissible">
    <Button variant="secondary" on:click={() => (wideOpen = true)}>Open persistent</Button>
    <Dialog
      open={wideOpen}
      title="Processing"
      dismissOnBackdrop={false}
      dismissOnEscape={false}
      on:openChange={(e) => (wideOpen = e.detail.open)}
    >
      <p>This dialog cannot be dismissed by clicking the backdrop or pressing Escape. Only the button below will close it.</p>
      <svelte:fragment slot="actions">
        <Button on:click={() => (wideOpen = false)}>Done</Button>
      </svelte:fragment>
    </Dialog>
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .specimen__row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    align-items: center;
  }

  /* Shortcuts dialog */
  .shortcuts-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .shortcut {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .shortcut kbd {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 4.5rem;
    padding: 0.25rem 0.5rem;
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: 0.25rem;
    background: color-mix(in srgb, var(--poodle-color-background-panel) 80%, var(--poodle-color-background-elevated));
    font-family: var(--poodle-typography-code-family);
    font-size: 0.75rem;
    font-weight: 500;
  }

  .shortcut span {
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
  }

  /* Form grid */
  .form-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
  }

  .form-full-width {
    grid-column: 1 / -1;
  }

  /* Changelog */
  .changelog-header {
    display: flex;
    align-items: center;
    gap: 0.625rem;
  }

  .changelog-header__title {
    margin: 0;
    font-family: var(--poodle-typography-heading-family);
    font-size: 1rem;
    font-weight: 600;
    line-height: 1.2;
  }

  .changelog {
    display: flex;
    flex-direction: column;
    gap: 0.875rem;
  }

  .changelog__entry {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .changelog__entry strong {
    font-size: 0.875rem;
  }

  .changelog__entry p {
    margin: 0;
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  /* Terms footer */
  .terms-body p {
    margin: 0 0 0.5rem;
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.6;
  }

  .split-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .split-footer__link {
    color: var(--poodle-color-accent-base);
    font-size: 0.8125rem;
    text-decoration: none;
  }

  .split-footer__link:hover {
    text-decoration: underline;
  }

  .split-footer__actions {
    display: flex;
    gap: 0.5rem;
  }

  /* Image preview (bare) */
  .image-preview {
    display: flex;
    flex-direction: column;
  }

  .image-preview__canvas {
    display: grid;
    place-items: center;
    min-height: 20rem;
    background: color-mix(in srgb, var(--poodle-color-background-canvas) 90%, black);
    border-radius: var(--poodle-radius-surface) var(--poodle-radius-surface) 0 0;
  }

  .image-preview__placeholder {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-code-family);
    font-size: 0.875rem;
    opacity: 0.5;
  }

  .image-preview__bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding: 0.75rem 1rem;
    border-top: 0.0625rem solid var(--poodle-color-border-subtle);
  }

  .image-preview__meta {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
  }

  .image-preview__meta strong {
    font-size: 0.8125rem;
  }

  .image-preview__meta span {
    color: var(--poodle-color-text-secondary);
    font-size: 0.75rem;
  }

  .image-preview__actions {
    display: flex;
    gap: 0.5rem;
  }

  /* Log list */
  .log-list {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    max-height: 18rem;
    overflow-y: auto;
    margin: 0 -0.25rem;
    padding: 0 0.25rem;
  }

  .log-entry {
    display: flex;
    gap: 0.75rem;
    padding: 0.375rem 0;
    border-bottom: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 50%, transparent);
  }

  .log-entry__time {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-code-family);
    font-size: 0.75rem;
    min-width: 3rem;
  }

  .log-entry__message {
    font-size: 0.8125rem;
  }
</style>
