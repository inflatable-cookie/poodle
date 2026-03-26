<script lang="ts">
  import { Toolbar, Button, Separator, Eyebrow, UiPresentationProvider } from "@poodle/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Chrome role inside a small toolbar</Eyebrow>
    <UiPresentationProvider density="compact" sizeScale="sm">
      <Toolbar ariaLabel="Formatting toolbar">
        <Button variant="ghost" sizeRole="chrome" leadingIcon="bold" ariaLabel="Bold" />
        <Button variant="ghost" sizeRole="chrome" leadingIcon="italic" ariaLabel="Italic" />
        <Button variant="ghost" sizeRole="chrome" leadingIcon="underline" ariaLabel="Underline" />
        <Separator orientation="vertical" />
        <Button variant="ghost" sizeRole="chrome" leadingIcon="text-align-start" ariaLabel="Align left" />
        <Button variant="ghost" sizeRole="chrome" leadingIcon="text-align-center" ariaLabel="Align center" />
        <Button variant="ghost" sizeRole="chrome" leadingIcon="text-align-end" ariaLabel="Align right" />
      </Toolbar>
    </UiPresentationProvider>
  </div>

  <div class="specimen__group">
    <Eyebrow>Prominent role offset in the same context</Eyebrow>
    <UiPresentationProvider density="compact" sizeScale="sm">
      <Toolbar ariaLabel="Actions toolbar">
        <Button variant="secondary" sizeRole="chrome">Discard</Button>
        <Button variant="secondary" sizeRole="chrome">Save draft</Button>
        <Separator orientation="vertical" />
        <Button sizeRole="prominent">Publish</Button>
      </Toolbar>
    </UiPresentationProvider>
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
</style>
