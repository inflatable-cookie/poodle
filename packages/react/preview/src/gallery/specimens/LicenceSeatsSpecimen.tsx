import { useState, type CSSProperties } from "react";
import { LicenceSeats } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

/* Machine IDs are random command identifiers, not human identity. They are
   here because the callback carries them — nothing renders them. */
const mixed = [
  { machineId: "cmd-9f3a2b7c", label: "Studio Mac", thisMachine: true },
  { machineId: "cmd-41ee80d2", label: "Tour laptop", thisMachine: false },
  { machineId: "cmd-77c1a5be", label: null, thisMachine: false },
];

/* Two unnamed rows look alike, and stay that way. Inventing a hostname to tell
   them apart would be claiming identity Poodle was never given. */
const unnamed = [
  { machineId: "cmd-2b90fe14", label: null, thisMachine: true },
  { machineId: "cmd-6d17c3aa", label: null, thisMachine: false },
  { machineId: "cmd-b04f9e51", label: null, thisMachine: false },
];

const single = [{ machineId: "cmd-9f3a2b7c", label: "Studio Mac", thisMachine: true }];

const stackStyle: CSSProperties = { display: "flex", flexDirection: "column", gap: "2rem" };

type Seat = { machineId: string; label: string | null; thisMachine: boolean };

interface InteractiveSeatsProps {
  initialSeats: readonly Seat[];
  pendingMachineId?: string;
  confirmRelease?: boolean;
}

function InteractiveSeats({
  initialSeats,
  pendingMachineId,
  confirmRelease,
}: InteractiveSeatsProps) {
  const [seats, setSeats] = useState<Seat[]>(() => initialSeats.map((seat) => ({ ...seat })));

  return (
    <LicenceSeats
      seats={seats}
      pendingMachineId={pendingMachineId}
      confirmRelease={confirmRelease}
      onRename={({ machineId, label }) =>
        setSeats((current) =>
          current.map((seat) => (seat.machineId === machineId ? { ...seat, label } : seat)),
        )
      }
    />
  );
}

export function LicenceSeatsSpecimen() {
  return (
    <SpecimenLayout
      sizes={(size) => <LicenceSeats seats={mixed} onRename={() => {}} size={size} />}
      densities={(density) => <LicenceSeats seats={mixed} onRename={() => {}} density={density} />}
    >
      <div style={stackStyle}>
        <SpecimenGroup label="Mixed labels">
          <InteractiveSeats initialSeats={mixed} />
        </SpecimenGroup>

        <SpecimenGroup label="Unnamed machines">
          <InteractiveSeats initialSeats={unnamed} />
        </SpecimenGroup>

        <SpecimenGroup label="This machine only">
          {/* This machine only: a marker, and no release action anywhere. */}
          <InteractiveSeats initialSeats={single} />
        </SpecimenGroup>

        <SpecimenGroup label="Pending release">
          <InteractiveSeats initialSeats={mixed} pendingMachineId="cmd-41ee80d2" />
        </SpecimenGroup>

        <SpecimenGroup label="Direct release">
          <InteractiveSeats initialSeats={mixed} confirmRelease={false} />
        </SpecimenGroup>

        <SpecimenGroup label="Empty authority">
          {/* Renders nothing: no heading, no list, and no invented seat count. */}
          <LicenceSeats seats={[]} />
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}
