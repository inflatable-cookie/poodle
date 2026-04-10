<script lang="ts">
  import { Switch } from "@poodle/svelte-primitives";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  let darkMode = true;
  let autoSave = false;
  let compactView = true;
</script>

<SpecimenLayout>
  <SpecimenGroup label="Default">
    <Switch
      label="Dark mode"
      checked={darkMode}
      on:checkedChange={(e) => (darkMode = e.detail.checked)}
    />
    <Switch
      label="Auto-save drafts"
      checked={autoSave}
      on:checkedChange={(e) => (autoSave = e.detail.checked)}
    />
    <Switch
      label="Compact view"
      checked={compactView}
      on:checkedChange={(e) => (compactView = e.detail.checked)}
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

  <svelte:fragment slot="sizes" let:size>
    <Switch label="Enabled" {size} />
  </svelte:fragment>

  <svelte:fragment slot="densities" let:density>
    <Switch id={"density-" + density} label="Toggle" {density} />
  </svelte:fragment>
</SpecimenLayout>
