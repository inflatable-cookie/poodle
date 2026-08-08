import "@inflatable-cookie/poodle-core/styles/sidebar-nav.css";

import { useState } from "react";

import type { ControlDensity, ControlSize, SemanticControlSizeRole, SidebarNavGroup, SidebarNavItem } from "./types";

export interface SidebarNavProps {
  groups?: SidebarNavGroup[];
  value?: string | null;
  defaultValue?: string | null;
  ariaLabel?: string | null;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onValueChange?: ((value: string) => void) | undefined;
}

export function SidebarNav({
  groups = [],
  value: controlledValue,
  defaultValue = null,
  ariaLabel = null,
  size = null,
  sizeRole = "chrome",
  density = null,
  onValueChange = undefined,
}: SidebarNavProps) {
  const [uncontrolledValue, setUncontrolledValue] = useState<string | null>(defaultValue);
  const isControlled = controlledValue !== undefined && controlledValue !== null;
  const value = isControlled ? controlledValue : uncontrolledValue;

  const visibleGroups = groups.filter((group) => group.items.length > 0);

  function handleItemActivation(item: SidebarNavItem): void {
    if (item.disabled) return;
    if (!isControlled) {
      setUncontrolledValue(item.value);
    }
    onValueChange?.(item.value);
  }

  function itemClassName(item: SidebarNavItem): string {
    return [
      "poodle-sidebar-nav__item",
      item.value === value ? "poodle-sidebar-nav__item--active" : "",
    ]
      .filter(Boolean)
      .join(" ");
  }

  return (
    <nav
      className="poodle-sidebar-nav"
      data-size={size ?? undefined}
      data-density={density ?? undefined}
      data-size-role={sizeRole}
      aria-label={ariaLabel ?? undefined}
    >
      {visibleGroups.map((group) => (
        <section
          key={group.id}
          className="poodle-sidebar-nav__group"
          data-separated={visibleGroups.length > 1}
          aria-label={group.label ?? undefined}
        >
          {group.label ? <h2 className="poodle-sidebar-nav__group-title">{group.label}</h2> : null}

          <ul className="poodle-sidebar-nav__list">
            {group.items.map((item) => (
              <li key={item.value}>
                {item.href && !item.disabled ? (
                  <a
                    className={itemClassName(item)}
                    href={item.href}
                    aria-current={item.value === value ? "page" : undefined}
                    onClick={() => handleItemActivation(item)}
                  >
                    {item.label}
                  </a>
                ) : (
                  <button
                    type="button"
                    className={itemClassName(item)}
                    aria-current={item.value === value ? "page" : undefined}
                    disabled={item.disabled}
                    onClick={() => handleItemActivation(item)}
                  >
                    {item.label}
                  </button>
                )}
              </li>
            ))}
          </ul>
        </section>
      ))}
    </nav>
  );
}
