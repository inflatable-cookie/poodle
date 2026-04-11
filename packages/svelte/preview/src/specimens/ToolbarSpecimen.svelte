<script lang="ts">
  import { Toolbar, Button, IconButton, Separator } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
</script>

<SpecimenLayout bareVariants>
  <SpecimenGroup bare label="Formatting toolbar">
    <Toolbar ariaLabel="Formatting toolbar">
      <IconButton icon="bold" ariaLabel="Bold" variant="ghost" />
      <IconButton icon="italic" ariaLabel="Italic" variant="ghost" />
      <IconButton icon="underline" ariaLabel="Underline" variant="ghost" />
      <Separator orientation="vertical" />
      <IconButton icon="align-left" ariaLabel="Align left" variant="ghost" />
      <IconButton icon="align-center" ariaLabel="Align center" variant="ghost" />
      <IconButton icon="align-right" ariaLabel="Align right" variant="ghost" />
    </Toolbar>
  </SpecimenGroup>

  <SpecimenGroup bare label="Actions toolbar">
    <Toolbar ariaLabel="Actions toolbar">
      <Button variant="secondary">Discard</Button>
      <Button variant="secondary">Save draft</Button>
      <Separator orientation="vertical" />
      <Button variant="primary">Publish</Button>
    </Toolbar>
  </SpecimenGroup>

  <svelte:fragment slot="sizes" let:size>
    <Toolbar ariaLabel="Toolbar at {size}" {size}>
      <IconButton icon="bold" ariaLabel="Bold" variant="ghost" {size} />
      <IconButton icon="italic" ariaLabel="Italic" variant="ghost" {size} />
      <Separator orientation="vertical" />
      <Button variant="secondary" {size}>Save</Button>
    </Toolbar>
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <Toolbar ariaLabel="Toolbar at {density}" {density}>
      <IconButton icon="bold" ariaLabel="Bold" variant="ghost" {density} />
      <IconButton icon="italic" ariaLabel="Italic" variant="ghost" {density} />
      <Separator orientation="vertical" />
      <Button variant="secondary" {density}>Save</Button>
    </Toolbar>
  </svelte:fragment>
</SpecimenLayout>
