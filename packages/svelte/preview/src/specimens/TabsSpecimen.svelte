<script lang="ts">
  import { Tabs } from "@inflatable-cookie/poodle-svelte";
  import {
    projectCorpus,
    tabsCases,
    tabsInterface,
    type ProjectedInstance,
  } from "@inflatable-cookie/poodle-core/conformance";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  // g14.004: groups, fixtures, collection order, and axes come from the
  // executable Tabs corpus. The runtime contributes only its renderer.
  const groups = projectCorpus(tabsCases, tabsInterface);
  let valueLog = $state("No tab change yet.");

  function propsOf(instance: ProjectedInstance): Record<string, unknown> {
    const props: Record<string, unknown> = { ...instance.props };
    for (const key of Object.keys(props)) {
      if (props[key] === null) delete props[key];
    }
    return props;
  }
</script>

<SpecimenLayout>
  {#each groups as group (group.label)}
    <SpecimenGroup label={group.label}>
      {#each group.instances as instance (instance.caseId + instance.caption)}
        <div class="poodle-specimen__row poodle-specimen__row--captioned">
          <span class="poodle-specimen__caption">{instance.caption}</span>
          <Tabs
            {...propsOf(instance)}
            onValueChange={(value) => (valueLog = `${instance.caption}: ${value}`)}
          />
        </div>
      {/each}
    </SpecimenGroup>
  {/each}

  <SpecimenGroup label="Interaction" bare>
    <span class="poodle-specimen__caption">{valueLog}</span>
  </SpecimenGroup>
</SpecimenLayout>

<style>
  .poodle-specimen__row--captioned {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .poodle-specimen__caption {
    color: var(--poodle-color-text-secondary, #c9d4e0);
    font-size: 0.75rem;
    min-width: 12rem;
  }
</style>
