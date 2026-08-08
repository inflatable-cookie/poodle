import type { ReactNode } from "react";

import "@inflatable-cookie/poodle-core/styles/form-actions.css";

import { IconButton } from "./IconButton";
import { Menu } from "./Menu";
import { useUiPresentation } from "./presentation";
import type { ControlDensity, FormActionAlign, FormActionDangerItem, MenuItem } from "./types";

export interface FormActionsProps {
  align?: FormActionAlign;
  density?: ControlDensity | null;
  showTopSeparation?: boolean;
  showTopBorder?: boolean;
  dangerItems?: FormActionDangerItem[];
  children?: ReactNode;
  danger?: ReactNode;
}

export function FormActions({
  align = "end",
  density = null,
  showTopSeparation = true,
  showTopBorder = false,
  dangerItems = [],
  children,
  danger,
}: FormActionsProps) {
  const uiPresentation = useUiPresentation();

  const resolvedDensity = density ?? uiPresentation.density;
  const hasDangerSlot = Boolean(danger);
  const hasDangerMenu = dangerItems.length > 0;
  const showResponsiveDangerSwap = hasDangerSlot && hasDangerMenu;
  const collapsedDangerItems: MenuItem[] = dangerItems.map((item, index) => ({
    value: item.value ?? `${index}:${item.label}`,
    label: item.label,
    disabled: item.disabled === true,
  }));

  function handleDangerAction(value: string): void {
    const item = dangerItems.find((candidate, index) => (candidate.value ?? `${index}:${candidate.label}`) === value);
    item?.onSelect();
  }

  return (
    <div
      className="poodle-form-actions"
      data-align={align}
      data-density={resolvedDensity}
      data-top-separation={showTopSeparation ? "true" : "false"}
      data-top-border={showTopBorder ? "true" : "false"}
    >
      {hasDangerSlot ? (
        <div className="poodle-form-actions__danger" data-mode={showResponsiveDangerSwap ? "responsive" : "inline"}>
          {danger}
        </div>
      ) : null}

      {hasDangerMenu ? (
        <div className="poodle-form-actions__danger-menu" data-visible={showResponsiveDangerSwap ? "responsive" : "always"}>
          <Menu
            items={collapsedDangerItems}
            ariaLabel="More actions"
            placement="top-end"
            onAction={handleDangerAction}
            trigger={
              <span>
                <IconButton icon="ellipsis" ariaLabel="More actions" variant="ghost" sizeRole="chrome" />
              </span>
            }
          />
        </div>
      ) : null}

      {children}
    </div>
  );
}
