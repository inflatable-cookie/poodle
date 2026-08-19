<script lang="ts">
  import { RemediationBanner } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const retry = { id: "retry", label: "Try again", variant: "primary" as const, isDisabled: false };
  const details = { id: "details", label: "View details", variant: "secondary" as const, isDisabled: false };
</script>

<div class="poodle-specimen">
  <SpecimenGroup label="Recovery actions">
    <RemediationBanner
      title="We could not save your changes"
      message="Your edits are still local. Retry the save or inspect the error details."
      tone="danger"
      primaryAction={retry}
      secondaryAction={details}
      isDismissible
    />
  </SpecimenGroup>
  <SpecimenGroup label="Recovery in progress">
    <RemediationBanner title="Reconnecting" message="This should only take a moment." tone="pending" />
  </SpecimenGroup>
  <SpecimenGroup label="Solid fills">
    <RemediationBanner
      title="Solid neutral recovery"
      message="Neutral solid surfaces keep the theme's primary foreground."
      tone="neutral"
      fill="solid"
    />
    <RemediationBanner title="Solid information" message="Tint-border colour promoted into the fill." tone="info" fill="solid" />
    <RemediationBanner title="Solid success" message="The operation completed successfully." tone="success" fill="solid" />
    <RemediationBanner title="Solid warning" message="Review this condition before continuing." tone="warning" fill="solid" />
    <RemediationBanner
      title="Solid danger recovery"
      message="Secondary actions stay readable on the solid surface."
      tone="danger"
      fill="solid"
      secondaryAction={details}
    />
    <RemediationBanner title="Solid pending" message="Recovery is still in progress." tone="pending" fill="solid" />
  </SpecimenGroup>
</div>
