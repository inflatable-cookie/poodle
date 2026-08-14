import { useEffect, useRef, useState } from "react";
import { Icon, SidebarNav } from "@inflatable-cookie/poodle-react";
import { allComponents, findComponent } from "./registry";
import {
  componentsBySection,
  isFamilyDisclosed,
  matchesCatalogueSearch,
} from "../../../../svelte/preview/src/catalogue-nav";
import { CatalogueLanding } from "./CatalogueLanding";
import { ComponentPage } from "./ComponentPage";
import { specimenMap } from "./specimen-map";

export interface ComponentsSectionProps {
  activeComponent?: string;
  search?: string;
}

export function ComponentsSection({
  activeComponent,
  search = "",
}: ComponentsSectionProps) {
  const contentRef = useRef<HTMLDivElement | null>(null);
  const previousActiveComponent = useRef(activeComponent);
  const [userDisclosure, setUserDisclosure] = useState<Map<string, boolean>>(
    () => new Map(),
  );

  const entry = activeComponent ? findComponent(activeComponent) : undefined;
  const specimen = entry?.slug ? (specimenMap[entry.slug] ?? null) : null;

  useEffect(() => {
    if (previousActiveComponent.current !== activeComponent) {
      const activeFamily = activeComponent
        ? findComponent(activeComponent)?.family
        : undefined;
      if (activeFamily) {
        setUserDisclosure((current) => {
          if (!current.has(activeFamily)) return current;
          const next = new Map(current);
          next.delete(activeFamily);
          return next;
        });
      }
      previousActiveComponent.current = activeComponent;
    }
    if (activeComponent && contentRef.current) {
      contentRef.current.scrollTop = 0;
    }
  }, [activeComponent]);

  const searchActive = search.trim().length > 0;
  const filteredComponents = searchActive
    ? allComponents.filter((component) =>
        matchesCatalogueSearch(component, search),
      )
    : allComponents;
  const sectionGroups = componentsBySection(allComponents);

  const toggleFamily = (familyId: string, open: boolean) => {
    setUserDisclosure((current) => {
      const next = new Map(current);
      next.set(familyId, !open);
      return next;
    });
  };

  return (
    <div className="poodle-catalogue-layout">
      <div className="poodle-catalogue-sidebar">
        {searchActive ? (
          <div className="poodle-catalogue-search" data-catalogue-search="true">
            {filteredComponents.length === 0 ? (
              <p className="poodle-catalogue-search__empty">
                No matching components.
              </p>
            ) : (
              filteredComponents.map((component) => (
                <a
                  key={component.slug}
                  className="poodle-catalogue-search__item"
                  href={`#components/${component.slug}`}
                  aria-current={
                    component.slug === activeComponent ? "page" : undefined
                  }
                  data-catalogue-result={component.slug}
                >
                  <span className="poodle-catalogue-search__name">
                    {component.displayName}
                  </span>
                  <span className="poodle-catalogue-search__crumb">
                    {component.familyLabel} · {component.kindLabel}
                  </span>
                </a>
              ))
            )}
          </div>
        ) : (
          <nav className="poodle-catalogue-nav" aria-label="Components">
            {sectionGroups.map((section) => (
              <div
                key={section.id}
                className="poodle-catalogue-nav__section"
                data-catalogue-section={section.id}
              >
                <h2 className="poodle-catalogue-nav__section-title">
                  {section.label}
                </h2>
                {section.families.map((family) => {
                  const open = isFamilyDisclosed(
                    family.id,
                    activeComponent,
                    userDisclosure,
                    allComponents,
                  );
                  return (
                    <div
                      key={family.id}
                      className="poodle-catalogue-family"
                      data-catalogue-family={family.id}
                      data-open={open || undefined}
                    >
                      <button
                        type="button"
                        className="poodle-catalogue-family__trigger"
                        aria-expanded={open}
                        onClick={() => toggleFamily(family.id, open)}
                      >
                        <Icon
                          name={open ? "chevron-down" : "chevron-right"}
                          size="sm"
                        />
                        <span className="poodle-catalogue-family__label">
                          {family.label}
                        </span>
                        <span className="poodle-catalogue-family__count">
                          {family.items.length}
                        </span>
                      </button>
                      {open ? (
                        <div className="poodle-catalogue-family__items">
                          <SidebarNav
                            ariaLabel={family.label}
                            groups={[
                              {
                                id: family.id,
                                items: family.items.map((component) => ({
                                  value: component.slug,
                                  label: component.displayName,
                                  href: `#components/${component.slug}`,
                                })),
                              },
                            ]}
                            value={activeComponent ?? null}
                          />
                        </div>
                      ) : null}
                    </div>
                  );
                })}
              </div>
            ))}
          </nav>
        )}
      </div>

      <div className="poodle-catalogue-content" ref={contentRef}>
        {entry ? (
          <ComponentPage entry={entry} specimenComponent={specimen} />
        ) : (
          <CatalogueLanding components={filteredComponents} />
        )}
      </div>
    </div>
  );
}
