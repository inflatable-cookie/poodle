<script lang="ts">
  import { Tabs, type TabItem } from "@inflatable-cookie/poodle-svelte";
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
  const residualItems: TabItem[] = [
    { value: "editor", label: "Editor", icon: "code" },
    { value: "preview", label: "Preview", icon: "eye", count: 12, separator: true },
    { value: "terminal", label: "Terminal", icon: "terminal", closable: true },
  ];
  let valueLog = $state("No tab change yet.");
  let values = $state<Record<string, string>>({});

  function propsOf(instance: ProjectedInstance): Record<string, unknown> {
    const props: Record<string, unknown> = { ...instance.props };
    for (const key of Object.keys(props)) {
      if (props[key] === null) delete props[key];
    }
    return props;
  }

  function instanceKey(instance: ProjectedInstance): string {
    return instance.caseId + instance.caption;
  }

  function valueOf(instance: ProjectedInstance): string {
    return values[instanceKey(instance)] ?? String(instance.props.value ?? instance.props.defaultValue ?? "");
  }

  function commit(instance: ProjectedInstance, value: string): void {
    values[instanceKey(instance)] = value;
    valueLog = `${instance.caption}: ${value}`;
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
            value={valueOf(instance)}
            onValueChange={(value) => commit(instance, value)}
          />
        </div>
      {/each}
    </SpecimenGroup>
  {/each}

  <SpecimenGroup label="Residual visual and operator coverage">
    <div style="resize:horizontal;overflow:auto;width:24rem;min-width:12rem;">
      <Tabs items={residualItems} overflowStrategy="shed" collapseWhenOverflow ariaLabel="Overflow shedding" />
    </div>
    <Tabs items={residualItems} variant="card" activeEdge="outline" activeFill="solid" defaultValue="editor" reorderable onClose={(value) => (valueLog = `Closed: ${value}`)} ariaLabel="Closable files" />
    <Tabs items={residualItems} variant="block" activeEdge="underline" activeFill="none" fullWidth defaultValue="editor" ariaLabel="Full-width workspace">
      {#snippet children(activeValue)}<p>Panel: {activeValue}</p>{/snippet}
    </Tabs>
    <Tabs items={residualItems} variant="pill" defaultValue="editor" size="lg" density="comfortable" ariaLabel="Large comfortable tabs" />
  </SpecimenGroup>

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
