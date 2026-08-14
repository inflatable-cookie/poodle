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
</script>

<SpecimenLayout showSizes={false} showDensities={false}>
  {#snippet children()}
    <div class="poodle-licence-seats-specimen">
      <SpecimenGroup label="Seats">
        <LicenceSeats seats={mixed} />
        <LicenceSeats seats={unnamed} />
        <!-- This machine only: a marker, and no release action anywhere. -->
        <LicenceSeats seats={single} />
      </SpecimenGroup>

      <SpecimenGroup label="Pending and direct release">
        <LicenceSeats seats={mixed} pendingMachineId="cmd-41ee80d2" />
        <LicenceSeats seats={mixed} confirmRelease={false} />
      </SpecimenGroup>

      <SpecimenGroup label="Empty">
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
