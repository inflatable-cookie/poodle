<script lang="ts">
  import { LicenceActivation } from "@inflatable-cookie/poodle-svelte";
  import type { LicenceKeyProblem, LicenceKeyResult } from "@inflatable-cookie/poodle-core";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  /* Stand-ins for the host's behaviour. The real parser and account journey
     belong to the authority — the specimen shows that Poodle works against any
     pair that satisfies the interface, and imports neither. */
  const keyFormat = {
    parse(input: string): LicenceKeyResult {
      const stripped = input.replace(/[-\s]/g, "");
      if (/[^A-Za-z0-9]/.test(stripped)) {
        return { ok: false, problem: { kind: "unexpectedSymbol", symbol: stripped[0] ?? "?" } };
      }
      if (stripped.length < 20) {
        return { ok: false, problem: { kind: "tooShort", minimum: 20, actual: stripped.length } };
      }
      return { ok: true, key: stripped.toUpperCase(), grouped: stripped.toUpperCase() };
    },
    isProbablyATypo(problem: LicenceKeyProblem): boolean {
      return problem.kind === "checkFailed" || problem.kind === "unexpectedSymbol";
    },
  };

  const accountTokenProvider = { acquire: async () => null };
</script>

<SpecimenLayout showSizes={false} showDensities={false}>
  {#snippet children()}
    <div class="poodle-licence-activation-specimen">
      <!-- Three routes, one row, equal weight. The default selection does not
           make Key primary; the other two are peers, not fallbacks. -->
      <SpecimenGroup label="Routes">
        <LicenceActivation {keyFormat} {accountTokenProvider} />
        <LicenceActivation {keyFormat} {accountTokenProvider} defaultRoute="accountToken" />
        <LicenceActivation {keyFormat} {accountTokenProvider} defaultRoute="licenceFile" fileAccept=".licence" />
      </SpecimenGroup>

      <SpecimenGroup label="Pending and disabled">
        <!-- Pending blocks a duplicate submit. Every route stays on screen. -->
        <LicenceActivation {keyFormat} {accountTokenProvider} pending />
        <LicenceActivation {keyFormat} {accountTokenProvider} disabled />
      </SpecimenGroup>

      <SpecimenGroup label="Host copy">
        <LicenceActivation
          {keyFormat}
          {accountTokenProvider}
          title="Activate Finch"
          machineLabelLabel="Name this machine (optional)"
          activateLabel="Activate Finch"
        />
      </SpecimenGroup>
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-licence-activation-specimen {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }
</style>
