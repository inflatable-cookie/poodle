import { Pill } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

export function PillSpecimen() {
  return (
    <SpecimenLayout
      sizes={(size) => <Pill size={size}>{size.toUpperCase()}</Pill>}
      densities={(density) => <Pill density={density}>Label</Pill>}
    >
      <SpecimenGroup label="Tones">
        <div className="poodle-specimen__row">
          <Pill tone="neutral" sizeRole="control">Neutral</Pill>
          <Pill tone="info" sizeRole="control">Info</Pill>
          <Pill tone="success" sizeRole="control">Success</Pill>
          <Pill tone="warning" sizeRole="control">Warning</Pill>
          <Pill tone="danger" sizeRole="control">Danger</Pill>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Code font">
        <div className="poodle-specimen__row">
          <Pill font="mono" sizeRole="control">v2.4.1</Pill>
          <Pill font="mono" tone="success" sizeRole="control">stable</Pill>
          <Pill font="mono" tone="warning" sizeRole="control">beta</Pill>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Muted">
        <div className="poodle-specimen__row">
          <Pill muted sizeRole="control">Muted neutral</Pill>
          <Pill muted tone="success" sizeRole="control">Muted success</Pill>
          <Pill muted tone="danger" sizeRole="control">Muted danger</Pill>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Badge">
        <div className="poodle-specimen__row">
          <Pill appearance="badge" sizeRole="control">3</Pill>
          <Pill appearance="badge" sizeRole="control">12</Pill>
          <Pill appearance="badge" sizeRole="control">99+</Pill>
          <Pill appearance="badge" sizeRole="control">New</Pill>
          <Pill appearance="badge" tone="neutral" sizeRole="control">Draft</Pill>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Inherited typography">
        <p className="poodle-specimen__inline-copy">
          Status: <Pill appearance="badge" tone="success" typography="inherit">Active</Pill> synced with surrounding text.
        </p>
      </SpecimenGroup>

      <SpecimenGroup label="Custom accent">
        <div className="poodle-specimen__row">
          <Pill accent="#3b82f6" sizeRole="control">Info-ish</Pill>
          <Pill accent="#22c55e" sizeRole="control">Positive-ish</Pill>
          <Pill accent="#f59e0b" sizeRole="control">Caution-ish</Pill>
          <Pill accent="#ef4444" sizeRole="control">Danger-ish</Pill>
        </div>
      </SpecimenGroup>
    </SpecimenLayout>
  );
}
