<script lang="ts">
  import { RangeSlider } from "@inflatable-cookie/poodle-svelte";
  import {
    rangeSliderCases,
    rangeSliderInterface,
    projectCorpus,
  } from "@inflatable-cookie/poodle-core/conformance";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
  import type { ProjectedInstance } from "@inflatable-cookie/poodle-core/conformance";

  // g14.003: specimen page is a projection of the conformance case corpus.
  const groups = projectCorpus(rangeSliderCases, rangeSliderInterface);

  let valueLog = $state("No range change yet.");

  function propsOf(instance: ProjectedInstance): Record<string, unknown> {
    const props: Record<string, unknown> = { ...instance.props };
    for (const key of Object.keys(props)) {
      if (props[key] === null) delete props[key];
    }
    return props;
  }

  function onChange(instance: ProjectedInstance, value: [number, number]): void {
    valueLog = `${instance.caption}: [${value[0]}, ${value[1]}]`;
  }
</script>

<SpecimenLayout>
  {#each groups as group (group.label)}
    <SpecimenGroup label={group.label}>
      {#each group.instances as instance (instance.caseId + instance.caption)}
        <div class="poodle-specimen__row poodle-specimen__row--captioned">
          <span class="poodle-specimen__caption">{instance.caption}</span>
          <RangeSlider
            {...propsOf(instance)}
            onValueChange={(value) => onChange(instance, value)}
          />
        </div>
      {/each}
    </SpecimenGroup>
  {/each}

  <SpecimenGroup label="Interaction" bare>
    <div class="poodle-specimen__row">
      <span class="poodle-specimen__caption">{valueLog}</span>
    </div>
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
