import { Eyebrow } from "@inflatable-cookie/poodle-react";
import { componentsBySection } from "../../../../svelte/preview/src/catalogue-nav";
import type { ComponentEntry } from "./registry";

export interface CatalogueLandingProps {
  components?: ComponentEntry[];
}

export function CatalogueLanding({ components = [] }: CatalogueLandingProps) {
  const groups = componentsBySection(components);

  return (
    <div className="poodle-catalogue-landing">
      <div className="poodle-catalogue-landing__header">
        <h2>Component catalogue</h2>
        <p>Browse the full Poodle component library. Each component handles accessibility, keyboard support, and theming.</p>
        <p className="poodle-catalogue-landing__count">{components.length} components</p>
      </div>

      {groups.map((section) => (
        <section key={section.id} className="poodle-catalogue-landing__section" data-catalogue-section={section.id}>
          <Eyebrow>{section.label}</Eyebrow>
          {section.families.map((family) => (
            <div key={family.id} className="poodle-catalogue-landing__family" data-catalogue-family={family.id}>
              <h3 className="poodle-catalogue-landing__family-title">
                {family.label}
                <span className="poodle-catalogue-landing__family-count">{family.items.length}</span>
              </h3>
              <div className="poodle-catalogue-landing__grid">
                {family.items.map((component) => (
                  <a key={component.slug} className="poodle-component-card" href={`#components/${component.slug}`}>
                    <strong className="poodle-component-card__name">{component.displayName}</strong>
                    <p className="poodle-component-card__description">{component.description}</p>
                    <p className="poodle-component-card__crumb">
                      {component.familyLabel} · {component.kindLabel}
                    </p>
                  </a>
                ))}
              </div>
            </div>
          ))}
        </section>
      ))}
    </div>
  );
}
