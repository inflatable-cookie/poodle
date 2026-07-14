import { Toolbar, Button, IconButton, Separator } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

export function ToolbarSpecimen() {
  return (
    <SpecimenLayout
      bareVariants
      sizes={(size) => (
        <Toolbar ariaLabel={`Toolbar at ${size}`} size={size}>
          <IconButton icon="bold" ariaLabel="Bold" variant="ghost" size={size} />
          <IconButton icon="italic" ariaLabel="Italic" variant="ghost" size={size} />
          <Separator orientation="vertical" />
          <Button variant="secondary" size={size}>Save</Button>
        </Toolbar>
      )}
      densities={(density) => (
        <Toolbar ariaLabel={`Toolbar at ${density}`} density={density}>
          <IconButton icon="bold" ariaLabel="Bold" variant="ghost" density={density} />
          <IconButton icon="italic" ariaLabel="Italic" variant="ghost" density={density} />
          <Separator orientation="vertical" />
          <Button variant="secondary" density={density}>Save</Button>
        </Toolbar>
      )}
    >
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
    </SpecimenLayout>
  );
}
