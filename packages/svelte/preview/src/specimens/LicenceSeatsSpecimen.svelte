<script lang="ts">
  import { LicenceSeats } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  /* Machine IDs are random command identifiers, not human identity. They are
     here because the callback carries them — nothing renders them. */
  const mixed = [
    { machineId: "cmd-9f3a2b7c", label: "Studio Mac", thisMachine: true },
    { machineId: "cmd-41ee80d2", label: "Tour laptop", thisMachine: false },
    { machineId: "cmd-77c1a5be", label: null, thisMachine: false },
  ];

  /* Two unnamed rows look alike, and stay that way. Inventing a hostname to
     tell them apart would be claiming identity Poodle was never given. */
  const unnamed = [
    { machineId: "cmd-2b90fe14", label: null, thisMachine: true },
    { machineId: "cmd-6d17c3aa", label: null, thisMachine: false },
    { machineId: "cmd-b04f9e51", label: null, thisMachine: false },
  ];

  const single = [{ machineId: "cmd-9f3a2b7c", label: "Studio Mac", thisMachine: true }];

  type Seat = { machineId: string; label: string | null; thisMachine: boolean };
  type Rename = { machineId: string; label: string | null };

  let mixedSeats = $state<Seat[]>(mixed.map((seat) => ({ ...seat })));
  let unnamedSeats = $state<Seat[]>(unnamed.map((seat) => ({ ...seat })));
  let singleSeats = $state<Seat[]>(single.map((seat) => ({ ...seat })));
  let pendingSeats = $state<Seat[]>(mixed.map((seat) => ({ ...seat })));
  let directSeats = $state<Seat[]>(mixed.map((seat) => ({ ...seat })));

  function renamed(seats: Seat[], detail: Rename): Seat[] {
    return seats.map((seat) =>
      seat.machineId === detail.machineId ? { ...seat, label: detail.label } : seat,
    );
  }
</script>

<SpecimenLayout showSizes={false} showDensities={false}>
  {#snippet children()}
    <div class="poodle-licence-seats-specimen">
      <SpecimenGroup label="Mixed labels">
        <LicenceSeats
          seats={mixedSeats}
          onRename={(detail) => (mixedSeats = renamed(mixedSeats, detail))}
        />
      </SpecimenGroup>

      <SpecimenGroup label="Unnamed machines">
        <LicenceSeats
          seats={unnamedSeats}
          onRename={(detail) => (unnamedSeats = renamed(unnamedSeats, detail))}
        />
      </SpecimenGroup>

      <SpecimenGroup label="This machine only">
        <!-- This machine only: a marker, and no release action anywhere. -->
        <LicenceSeats
          seats={singleSeats}
          onRename={(detail) => (singleSeats = renamed(singleSeats, detail))}
        />
      </SpecimenGroup>

      <SpecimenGroup label="Pending release">
        <LicenceSeats
          seats={pendingSeats}
          pendingMachineId="cmd-41ee80d2"
          onRename={(detail) => (pendingSeats = renamed(pendingSeats, detail))}
        />
      </SpecimenGroup>

      <SpecimenGroup label="Direct release">
        <LicenceSeats
          seats={directSeats}
          confirmRelease={false}
          onRename={(detail) => (directSeats = renamed(directSeats, detail))}
        />
      </SpecimenGroup>

      <SpecimenGroup label="Empty authority">
        <!-- Renders nothing: no heading, no list, and no invented seat count. -->
        <LicenceSeats seats={[]} />
      </SpecimenGroup>
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-licence-seats-specimen {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }
</style>
