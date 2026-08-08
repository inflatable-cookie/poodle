import "@inflatable-cookie/poodle-core/styles/meta-bar.css";

import type { ReactNode } from "react";

import { PillContext } from "./pill-context";

export interface MetaBarProps {
  ariaLabel?: string | null;
  showSeparators?: boolean;
  children?: ReactNode;
}

export function MetaBar({ ariaLabel = null, showSeparators = true, children }: MetaBarProps) {
  return (
    <PillContext.Provider value={{ size: "md", typography: "inherit" }}>
      <div className="poodle-meta-bar" data-separators={showSeparators} aria-label={ariaLabel ?? undefined}>
        {children}
      </div>
    </PillContext.Provider>
  );
}
