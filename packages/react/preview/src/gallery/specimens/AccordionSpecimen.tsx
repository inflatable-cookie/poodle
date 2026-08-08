import { Accordion, type AccordionItem } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const singleItems: AccordionItem[] = [
  { value: "getting-started", label: "Getting started" },
  { value: "api-reference", label: "API reference" },
  { value: "accessibility", label: "Accessibility" },
];

const singleContent: Record<string, string> = {
  "getting-started":
    "Install the package with your preferred package manager, then import individual components as needed. Each component is tree-shakeable and ships with its own styles scoped via CSS custom properties.",
  "api-reference":
    "Every component accepts an ariaLabel prop for accessible naming. Most interactive components support controlled and uncontrolled modes via value/defaultValue pairs, and emit granular events like valueChange, openChange, or requestClose.",
  accessibility:
    "All components follow WAI-ARIA authoring practices. Focus is trapped inside modal overlays, arrow keys navigate composite widgets, and Escape dismisses dismissible layers. Screen reader announcements use live regions where appropriate.",
};

const multiItems: AccordionItem[] = [
  { value: "design", label: "Design tokens" },
  { value: "keyboard", label: "Keyboard shortcuts" },
  { value: "known-issues", label: "Known issues" },
];

const specimenVariantItems: AccordionItem[] = [{ value: "getting-started", label: "Getting started" }];

const multiContent: Record<string, string> = {
  design:
    "Components consume semantic tokens like --poodle-color-text-primary and --poodle-size-control-height rather than hard-coded values. Switching themes at runtime updates every component instantly without re-rendering.",
  keyboard:
    "Arrow keys move focus between accordion headers. Enter or Space toggles the focused panel. Home and End jump to the first and last header respectively. Tab moves focus out of the accordion entirely.",
  "known-issues":
    "Animation on panel expand/collapse is not yet implemented. The component does not support nested accordions. Horizontal orientation is planned but not available in this release.",
};

export function AccordionSpecimen() {
  return (
    <SpecimenLayout
      sizes={(size) => (
        <div style={{ width: "min(100%, 28rem)" }}>
          <Accordion
            items={specimenVariantItems}
            selectionMode="single"
            defaultValue="getting-started"
            ariaLabel={`${size} accordion`}
            size={size}
          >
            {(item) => <p>{singleContent[item.value]}</p>}
          </Accordion>
        </div>
      )}
      densities={(density) => (
        <div style={{ width: "min(100%, 28rem)" }}>
          <Accordion
            items={specimenVariantItems}
            selectionMode="single"
            defaultValue="getting-started"
            ariaLabel={`${density} accordion`}
            density={density}
          >
            {(item) => <p>{singleContent[item.value]}</p>}
          </Accordion>
        </div>
      )}
    >
      <div className="poodle-specimen">
        <SpecimenGroup bare label="Single selection">
          <Accordion
            items={singleItems}
            selectionMode="single"
            defaultValue="getting-started"
            ariaLabel="Single-select accordion"
          >
            {(item) => <p>{singleContent[item.value]}</p>}
          </Accordion>
        </SpecimenGroup>

        <SpecimenGroup bare label="Multiple selection">
          <Accordion
            items={multiItems}
            selectionMode="multiple"
            defaultValue={["design", "keyboard"]}
            ariaLabel="Multi-select accordion"
          >
            {(item) => <p>{multiContent[item.value]}</p>}
          </Accordion>
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}
