import { useEffect, useRef } from "react";
import { SidebarNav } from "@poodle/react";
import { allComponents, componentsByTag, findComponent } from "./registry";
import { CatalogueLanding } from "./CatalogueLanding";
import { ComponentPage } from "./ComponentPage";
import { specimenMap } from "./specimen-map";

export interface ComponentsSectionProps {
  activeComponent?: string;
  search?: string;
}

export function ComponentsSection({ activeComponent, search = "" }: ComponentsSectionProps) {
  const contentRef = useRef<HTMLDivElement | null>(null);

  const entry = activeComponent ? findComponent(activeComponent) : undefined;
  const specimen = entry?.slug ? (specimenMap[entry.slug] ?? null) : null;

  useEffect(() => {
    if (activeComponent && contentRef.current) {
      contentRef.current.scrollTop = 0;
    }
  }, [activeComponent]);

  const searchLower = search.trim().toLowerCase();
  const filteredComponents = searchLower
    ? allComponents.filter(
        (c) => c.displayName.toLowerCase().includes(searchLower) || c.description.toLowerCase().includes(searchLower),
      )
    : allComponents;

  const navGroups = componentsByTag()
    .map((group) => ({
      id: group.tag,
      label: group.label,
      items: group.items
        .filter(
          (c) => !searchLower || c.displayName.toLowerCase().includes(searchLower) || c.description.toLowerCase().includes(searchLower),
        )
        .map((component) => ({
          value: component.slug,
          label: component.displayName,
          href: `#components/${component.slug}`,
        })),
    }))
    .filter((group) => group.items.length > 0);

  return (
    <div className="poodle-catalogue-layout">
      <div className="poodle-catalogue-sidebar">
        <SidebarNav ariaLabel="Components" groups={navGroups} value={activeComponent ?? null} />
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
