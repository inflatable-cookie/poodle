<script lang="ts">
  import LicenceActivation from "../src/LicenceActivation.svelte";
  import type { LicenceCredential } from "@inflatable-cookie/poodle-core";

  let {
    acquire,
    onActivate,
  }: {
    acquire: (email: string) => Promise<string | null>;
    onActivate: (detail: { credential: LicenceCredential; label: string | null }) => void;
  } = $props();

  let email = $state("");
  const accountTokenProvider = { acquire: () => acquire(email) };
</script>

<LicenceActivation
  mode="account"
  {accountTokenProvider}
  activateLabel="Activate"
  {onActivate}
>
  {#snippet accountContent(disabled)}
    <label>
      Account email
      <input type="email" bind:value={email} {disabled} />
    </label>
  {/snippet}
</LicenceActivation>
