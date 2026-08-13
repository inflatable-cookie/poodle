import { Eyebrow } from "@inflatable-cookie/poodle-react";
import { componentsByTag, type ComponentEntry } from "./registry";

export interface CatalogueLandingProps {
  components?: ComponentEntry[];
}

export function CatalogueLanding({ components = [] }: CatalogueLandingProps) {
  const componentSlugs = new Set(components.map((c) => c.slug));
  const groups = componentsByTag()
    .map((group) => ({ ...group, items: group.items.filter((c) => componentSlugs.has(c.slug)) }))
    .filter((group) => group.items.length > 0);

  return (
    <div className="poodle-catalogue-landing">
      <div className="poodle-catalogue-landing__header">
        <h2>Component catalogue</h2>
        <p>Browse the full Poodle component library. Each component handles accessibility, keyboard support, and theming.</p>
        <p className="poodle-catalogue-landing__count">{components.length} components</p>
      </div>

      {groups.map((group) => (
        <section key={group.tag} className="poodle-catalogue-landing__section">
          <Eyebrow>{group.label}</Eyebrow>
          <div className="poodle-catalogue-landing__grid">
            {group.items.map((component) => (
              <a key={component.slug} className="poodle-component-card" href={`#components/${component.slug}`}>
                <strong className="poodle-component-card__name">{component.displayName}</strong>
                <p className="poodle-component-card__description">{component.description}</p>
              </a>
            ))}
          </div>
        </section>
      ))}
    </div>
  );
}
