import "@inflatable-cookie/poodle-core/styles/licence.css";

import {
  LICENCE_RELEASE_CONFIRM_TITLE,
  LICENCE_THIS_MACHINE,
  licenceSeatRows,
  type LicenceSeat,
} from "@inflatable-cookie/poodle-core";

import { ConfirmAction } from "./ConfirmAction";
import { EditableLabel } from "./EditableLabel";
import { Icon } from "./Icon";
import { IconButton } from "./IconButton";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize } from "./types";

export interface LicenceSeatsProps {
  seats?: readonly LicenceSeat[];
  pendingMachineId?: string | null;
  title?: string;
  releaseLabel?: string;
  confirmRelease?: boolean;
  size?: ControlSize | null;
  density?: ControlDensity | null;
  onRename?: (detail: { machineId: string; label: string | null }) => void;
  onRelease?: (detail: { machineId: string }) => void;
}

export function LicenceSeats({
  seats = [],
  pendingMachineId = null,
  title = "Activated machines",
  releaseLabel = "Release",
  confirmRelease = true,
  size = null,
  density = null,
  onRename,
  onRelease,
}: LicenceSeatsProps) {
  const uiPresentation = useUiPresentation();
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, "control");
  const resolvedDensity = density ?? uiPresentation.density;
  const glyphSize = resolveSemanticControlSize(resolvedSize, "chrome");

  const rows = licenceSeatRows(seats, pendingMachineId, releaseLabel);

  // No seats renders nothing at all. A "1 seat" line would be Poodle inventing
  // an account of seats the authority did not give it.
  if (rows.length === 0) return null;

  return (
    <section
      className="poodle-licence-seats"
      aria-label={title}
      data-size={resolvedSize}
      data-density={resolvedDensity}
    >
      <h3 className="poodle-licence-seats__title">{title}</h3>
      <ul className="poodle-licence-seats__list">
        {rows.map((row) => (
          <li
            key={row.machineId}
            className="poodle-licence-seats__row"
            data-this-machine={row.thisMachine}
          >
            <div className="poodle-licence-seats__identity">
              <span className="poodle-licence-seats__machine-icon" aria-hidden="true">
                <Icon icon="monitor" size={glyphSize} />
              </span>
              {/* The supplied label or `Unnamed machine`. Never the machine ID,
                  whole or shortened: it is a random command identifier, and
                  showing it would offer identity Poodle was never given. Two
                  unnamed rows looking alike is the honest outcome. */}
              <div className="poodle-licence-seats__label">
                <EditableLabel
                  value={row.named ? row.displayLabel : ""}
                  ariaLabel={`Rename ${row.named ? row.displayLabel : "unnamed machine"}`}
                  activationMode="enterOrSpace"
                  variant="flush"
                  emptyText="Unnamed machine"
                  placeholder="Unnamed machine"
                  showEditIcon
                  size={resolvedSize}
                  density={resolvedDensity}
                  onCommit={({ value }) =>
                    onRename?.({ machineId: row.machineId, label: value || null })
                  }
                />
              </div>
              {row.thisMachine ? (
                <span className="poodle-licence-seats__marker">{LICENCE_THIS_MACHINE}</span>
              ) : null}
            </div>

            {row.releasable ? (
              <span className="poodle-licence-seats__action">
                {confirmRelease ? (
                  <ConfirmAction
                    title={LICENCE_RELEASE_CONFIRM_TITLE}
                    description={row.confirmBody}
                    tone="warning"
                    confirmLabel={releaseLabel}
                    size={resolvedSize}
                    density={resolvedDensity}
                    onConfirm={() => onRelease?.({ machineId: row.machineId })}
                    trigger={
                      <IconButton
                        icon="trash-2"
                        variant="ghost"
                        tone="danger"
                        size={resolvedSize}
                        density={resolvedDensity}
                        disabled={row.pending}
                        loading={row.pending}
                        ariaLabel={row.releaseName}
                      />
                    }
                  />
                ) : (
                  <IconButton
                    icon="trash-2"
                    variant="ghost"
                    tone="danger"
                    size={resolvedSize}
                    density={resolvedDensity}
                    disabled={row.pending}
                    loading={row.pending}
                    ariaLabel={row.releaseName}
                    onClick={() => onRelease?.({ machineId: row.machineId })}
                  />
                )}
              </span>
            ) : null}
          </li>
        ))}
      </ul>
    </section>
  );
}
