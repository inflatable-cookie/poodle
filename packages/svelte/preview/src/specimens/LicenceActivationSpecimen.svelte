<script lang="ts">
  import { Field, LicenceActivation, TextInput } from "@inflatable-cookie/poodle-svelte";
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
  let email = $state("");
  let password = $state("");
</script>

<SpecimenLayout showSizes={false} showDensities={false}>
  {#snippet children()}
    <div class="poodle-licence-activation-specimen">
      <SpecimenGroup label="Embedded account activation">
        <LicenceActivation
          mode="account"
          {accountTokenProvider}
          activateLabel="Activate"
          fileAccept=".licence"
          machineLabel="Studio Mac"
        >
          {#snippet accountContent(disabled)}
            <Field id="licence-account-email" label="Email address">
              <TextInput
                id="licence-account-email"
                type="email"
                value={email}
                {disabled}
                onValueChange={(value) => (email = value)}
              />
            </Field>
            <Field id="licence-account-password" label="Password">
              <TextInput
                id="licence-account-password"
                type="password"
                value={password}
                {disabled}
                onValueChange={(value) => (password = value)}
              />
            </Field>
          {/snippet}
        </LicenceActivation>
      </SpecimenGroup>

      <SpecimenGroup label="External account activation">
        <LicenceActivation mode="account" {accountTokenProvider} fileAccept=".licence" />
      </SpecimenGroup>

      <SpecimenGroup label="Key activation">
        <LicenceActivation
          mode="key"
          {keyFormat}
          keyCodeInput={{ length: 20, groups: [5, 5, 5, 5], separator: "-" }}
          size="xs"
        />
      </SpecimenGroup>

      <SpecimenGroup label="Pending and disabled">
        <LicenceActivation mode="account" {accountTokenProvider} pending />
        <LicenceActivation
          mode="key"
          {keyFormat}
          keyCodeInput={{ length: 20, groups: [5, 5, 5, 5], separator: "-" }}
          disabled
        />
      </SpecimenGroup>

      <SpecimenGroup label="Host copy">
        <LicenceActivation
          mode="account"
          {accountTokenProvider}
          title="Activate Finch"
          machineLabel={null}
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
