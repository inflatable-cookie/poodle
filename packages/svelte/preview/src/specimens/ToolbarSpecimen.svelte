<script lang="ts">
  import { Toolbar, Button, Separator, UiPresentationProvider } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
</script>

<div class="specimen">
  <SpecimenGroup label="Chrome role inside a small toolbar">
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
  </SpecimenGroup>

  <SpecimenGroup label="Prominent role offset in the same context">
    <UiPresentationProvider density="compact" sizeScale="sm">
      <Toolbar ariaLabel="Actions toolbar">
        <Button variant="secondary" sizeRole="chrome">Discard</Button>
        <Button variant="secondary" sizeRole="chrome">Save draft</Button>
        <Separator orientation="vertical" />
        <Button sizeRole="prominent">Publish</Button>
      </Toolbar>
    </UiPresentationProvider>
  </SpecimenGroup>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

</style>
