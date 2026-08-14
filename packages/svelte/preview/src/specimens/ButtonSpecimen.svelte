<script lang="ts">
  import { Button } from "@inflatable-cookie/poodle-svelte";
  import { buttonCases, buttonInterface, projectCorpus } from "@inflatable-cookie/poodle-core/conformance";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
  import type { ProjectedInstance } from "@inflatable-cookie/poodle-core/conformance";

  // g14.001: the specimen page is a projection of the conformance case
  // corpus — groups, captions, axes, and fixtures come from
  // packages/core/src/conformance/button-cases.ts, never restated here.
  const groups = projectCorpus(buttonCases, buttonInterface);

  let clickLog = $state("No button clicked yet.");

  function log(instance: ProjectedInstance): void {
    clickLog = `Clicked: ${instance.caption}`;
  }

  function propsOf(instance: ProjectedInstance): Record<string, unknown> {
    const props: Record<string, unknown> = { ...instance.props };
    for (const key of Object.keys(props)) {
      if (props[key] === null) delete props[key];
    }
    props.leadingIcon = instance.leadingIcon ?? null;
    props.trailingIcon = instance.trailingIcon ?? null;
    return props;
  }
</script>

<SpecimenLayout>
  {#each groups as group (group.label)}
    <SpecimenGroup label={group.label}>
      {#each group.instances as instance (instance.caseId + instance.caption)}
        <div class="poodle-specimen__row poodle-specimen__row--captioned">
          <span class="poodle-specimen__caption">{instance.caption}</span>
          <Button {...propsOf(instance)} onClick={() => log(instance)}>
            {instance.label}
          </Button>
        </div>
      {/each}
    </SpecimenGroup>
  {/each}

  <SpecimenGroup label="Interaction" bare>
    <div class="poodle-specimen__row">
      <span class="poodle-specimen__caption">{clickLog}</span>
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
