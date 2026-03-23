<script lang="ts">
  import { Toolbar, Button, Separator, Eyebrow } from "@flint/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Horizontal (default)</Eyebrow>
    <Toolbar ariaLabel="Formatting toolbar">
      <Button variant="ghost" size="sm" leadingIcon="bold" ariaLabel="Bold" />
      <Button variant="ghost" size="sm" leadingIcon="italic" ariaLabel="Italic" />
      <Button variant="ghost" size="sm" leadingIcon="underline" ariaLabel="Underline" />
      <Separator orientation="vertical" />
      <Button variant="ghost" size="sm" leadingIcon="text-align-start" ariaLabel="Align left" />
      <Button variant="ghost" size="sm" leadingIcon="text-align-center" ariaLabel="Align center" />
      <Button variant="ghost" size="sm" leadingIcon="text-align-end" ariaLabel="Align right" />
    </Toolbar>
  </div>

  <div class="specimen__group">
    <Eyebrow>With primary action</Eyebrow>
    <Toolbar ariaLabel="Actions toolbar">
      <Button variant="secondary" size="sm">Discard</Button>
      <Button variant="secondary" size="sm">Save draft</Button>
      <Separator orientation="vertical" />
      <Button size="sm">Publish</Button>
    </Toolbar>
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
