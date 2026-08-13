<script lang="ts">
  import { UpdateCenter } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  const offer = {
    status: { kind: "ready" } as const,
    availability: {
      state: "offer",
      version: "1.4.0",
      reason: "staged",
      notes: "Faster renders, a rebuilt automation pass, and two crash fixes.",
    } as const,
  };

  const releaseNotes =
    "Faster renders across the board, a rebuilt automation pass, two crash fixes," +
    " and better memory use on large projects. Reboots required for the automation" +
    " changes to take effect.";
</script>

<SpecimenLayout showSizes={false} showDensities={false}>
  {#snippet children()}
    <div class="poodle-update-center-specimen">
      <SpecimenGroup label="Attention">
        <div class="poodle-update-center-specimen__anchor">
          <UpdateCenter presence="attention" {...offer} defaultOpen />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Release notes">
        <div class="poodle-update-center-specimen__anchor">
          <UpdateCenter
            presence="attention"
            {...offer}
            availability={{ ...offer.availability, notes: releaseNotes }}
            defaultOpen
          />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Quiet (postponed offer)">
        <div class="poodle-update-center-specimen__anchor">
          <UpdateCenter
            presence="quiet"
            {...offer}
            deferral={{ version: "1.4.0", cause: { cause: "userPostponed" } }}
          />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Quiet (work in flight)">
        <div class="poodle-update-center-specimen__anchor">
          <UpdateCenter presence="quiet" {...offer} progress={{ state: "downloading", fraction: 0.42 }} />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Quiet (indeterminate download)">
        <div class="poodle-update-center-specimen__anchor">
          <UpdateCenter presence="quiet" {...offer} progress={{ state: "downloading", fraction: null }} />
        </div>
      </SpecimenGroup>
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-update-center-specimen {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }

  .poodle-update-center-specimen__anchor {
    display: flex;
    justify-content: flex-end;
    width: min(42rem, 100%);
  }
</style>
