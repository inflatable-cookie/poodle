import type { ComponentType } from "react";
import { Pill, Separator } from "@poodle/react";
import type { ComponentEntry } from "./registry";
import { componentDocsMap } from "./component-docs";
import { UsageDocs } from "./UsageDocs";

export interface ComponentPageProps {
  entry: ComponentEntry;
  specimenComponent?: ComponentType | null;
}

export function ComponentPage({ entry, specimenComponent: Specimen = null }: ComponentPageProps) {
  const docs = componentDocsMap[entry.slug] ?? null;

  return (
    <article className="poodle-component-page">
      <header className="poodle-component-page__hero">
        <div className="poodle-component-page__hero-top">
          <Pill size="lg">{entry.packageName}</Pill>
        </div>
        <h1 className="poodle-component-page__title">{entry.displayName}</h1>
        <p className="poodle-component-page__description">{entry.description}</p>
      </header>

      <section className="poodle-component-page__section">
        {Specimen ? (
          <Specimen />
        ) : (
          <div className="poodle-component-page__placeholder">
            <p>
              Specimen not yet available for <strong>{entry.displayName}</strong>.
            </p>
            <p>Check back as we build out interactive demos for each component.</p>
          </div>
        )}
      </section>

      <section className="poodle-component-page__section">
        <h2 className="poodle-component-page__section-title">Import</h2>
        <pre className="poodle-component-page__code">
          <code>{`import { ${entry.displayName} } from "${entry.packageName}";`}</code>
        </pre>
      </section>

      {docs ? (
        <>
          <Separator />
          <UsageDocs docs={docs} />
        </>
      ) : null}
    </article>
  );
}
