<script lang="ts">
  import { Switch } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let darkMode = $state(true);
  let autoSave = $state(false);
  let compactView = $state(true);
</script>

<SpecimenLayout>
  <SpecimenGroup label="Default">
    <Switch
      label="Dark mode"
      checked={darkMode}
      onCheckedChange={(checked) => (darkMode = checked)}
    />
    <Switch
      label="Auto-save drafts"
      checked={autoSave}
      onCheckedChange={(checked) => (autoSave = checked)}
    />
    <Switch
      label="Compact view"
      checked={compactView}
      onCheckedChange={(checked) => (compactView = checked)}
    />
  </SpecimenGroup>

  <SpecimenGroup label="States">
    <Switch label="Disabled off" disabled />
    <Switch label="Disabled on" checked={true} disabled />
    <Switch label="Read-only on" checked={true} readOnly />
  </SpecimenGroup>

  <SpecimenGroup label="Custom colors">
    <Switch
      label="Billing alerts"
      checked={true}
      onColor="#22c55e"
      offColor="#cbd5e1"
    />
    <Switch
      label="Quiet mode"
      checked={false}
      onColor="#f59e0b"
      offColor="#94a3b8"
    />
  </SpecimenGroup>

  <SpecimenGroup label="Dual labels and tones">
    <Switch
      defaultChecked={true}
      leftLabel="Draft"
      rightLabel="Live"
      leftTone="danger"
      rightTone="success"
      ariaLabel="Publication status"
    />
    <Switch
      defaultChecked={false}
      leftLabel="Restricted"
      rightLabel="Free"
      leftTone="warning"
      rightTone="success"
      ariaLabel="Access status"
    />
  </SpecimenGroup>

  {#snippet sizes(size)}
    <Switch label="Enabled" {size} />
  {/snippet}

  {#snippet densities(density)}
    <Switch id={"density-" + density} label="Toggle" {density} />
  {/snippet}
</SpecimenLayout>
