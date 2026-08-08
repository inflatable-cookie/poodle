import "@inflatable-cookie/poodle-styles/inline-list-section.css";

import type { ReactNode } from "react";

import { Card } from "./Card";

export interface InlineListSectionProps<T> {
  title: string;
  items: T[];
  item: (entry: T) => ReactNode;
  actions?: ReactNode;
  emptyMessage?: string | null;
  count?: number | string | null;
  framed?: boolean;
}

export function InlineListSection<T>({
  title,
  items,
  item,
  actions,
  emptyMessage = "No items yet.",
  count = null,
  framed = true,
}: InlineListSectionProps<T>) {
  const section = (
    <section className="poodle-inline-list-section" aria-label={title}>
      <div className="poodle-inline-list-section__header">
        <div className="poodle-inline-list-section__heading">
          <h4 className="poodle-inline-list-section__title">{title}</h4>
          {count !== null ? <span className="poodle-inline-list-section__count">{count}</span> : null}
        </div>
        {actions ? <div className="poodle-inline-list-section__header-actions">{actions}</div> : null}
      </div>

      {items.length === 0 ? (
        emptyMessage ? <p className="poodle-inline-list-section__empty">{emptyMessage}</p> : null
      ) : (
        <ul className="poodle-inline-list-section__items">
          {items.map((entry, index) => (
            <li key={index} className="poodle-inline-list-section__item">
              {item(entry)}
            </li>
          ))}
        </ul>
      )}
    </section>
  );

  return framed ? <Card>{section}</Card> : section;
}
