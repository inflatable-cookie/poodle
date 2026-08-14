<script lang="ts">
  import { LicenceStatus } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  const now = Date.now();
  const soon = now + 12 * 86_400_000;
  const later = now + 240 * 86_400_000;
  const past = now - 9 * 86_400_000;
  const checked = now - 3_600_000;

  const offline = { kind: "offlineSignature" } as const;
  const remote = { kind: "remoteAssertion", checked } as const;
</script>

<SpecimenLayout showSizes={false} showDensities={false}>
  {#snippet children()}
    <div class="poodle-licence-status-specimen">
      <SpecimenGroup label="Usability states">
        <LicenceStatus
          usability={{ state: "active" }}
          trustBasis={offline}
          useUntil={later}
          updateUntil={later}
          usable={true}
          attention="none"
        />
        <!-- A pending renewal is the seller's outstanding work. It stays calm. -->
        <LicenceStatus
          usability={{ state: "inGrace", until: soon }}
          trustBasis={remote}
          useUntil={soon}
          updateUntil={later}
          usable={true}
          attention="informational"
        />
        <LicenceStatus
          usability={{ state: "useWindowExpired", at: past }}
          trustBasis={offline}
          useUntil={past}
          updateUntil={later}
          usable={false}
          attention="actionable"
        />
        <LicenceStatus
          usability={{ state: "leaseLapsed", at: past }}
          trustBasis={remote}
          useUntil={later}
          updateUntil={later}
          usable={false}
          attention="actionable"
        />
        <!-- The remedy is the machine's clock, never a purchase. -->
        <LicenceStatus
          usability={{ state: "clockRefused" }}
          trustBasis={remote}
          useUntil={later}
          updateUntil={later}
          usable={false}
          attention="actionable"
        />
      </SpecimenGroup>

      <SpecimenGroup label="Coverage windows">
        <LicenceStatus
          usability={{ state: "active" }}
          trustBasis={offline}
          useUntil={null}
          updateUntil={null}
          usable={true}
          attention="none"
        />
        <!-- Perpetual use, lapsed updates. Two windows, two rows: collapsing
             them is how an owner is told they have lost what they bought. -->
        <LicenceStatus
          usability={{ state: "active" }}
          trustBasis={offline}
          useUntil={null}
          updateUntil={past}
          usable={true}
          attention="informational"
        />
        <LicenceStatus
          usability={{ state: "active" }}
          trustBasis={offline}
          useUntil={later}
          updateUntil={null}
          usable={true}
          attention="none"
        />
      </SpecimenGroup>

      <SpecimenGroup label="Trust basis">
        <LicenceStatus
          usability={{ state: "active" }}
          trustBasis={offline}
          useUntil={later}
          updateUntil={later}
          usable={true}
          attention="none"
        />
        <LicenceStatus
          usability={{ state: "active" }}
          trustBasis={remote}
          useUntil={later}
          updateUntil={later}
          usable={true}
          attention="none"
        />
      </SpecimenGroup>
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-licence-status-specimen {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }
</style>
