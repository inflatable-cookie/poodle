<script lang="ts">
  import {
    Avatar,
    Callout,
    EmptyState,
    Pill,
    Spinner,
  } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "./SpecimenGroup.svelte";
  import SpecimenLayout from "./SpecimenLayout.svelte";
  import { specimenScenes } from "../generated/specimens/specimen-scenes";

  let { slug }: { slug?: string } = $props();

  const scene = $derived(slug ? specimenScenes[slug as keyof typeof specimenScenes] : undefined);

  /**
   * The fixture renderer (g14-b005 tranche one): renders one display-specimen
   * scene in the Svelte preview. The scene is generated data — groups,
   * instances, typed prop bindings, matrix axes. `content` bindings project
   * to children; every other binding forwards as a prop.
   */
  const componentMap: Record<string, any> = {
    callout: Callout,
    pill: Pill,
    spinner: Spinner,
    avatar: Avatar,
    "empty-state": EmptyState,
  };

  /** The matrix renders the scene's first instance at each axis value. */
  const matrixInstance = $derived(scene?.groups.flatMap((group) => group.instances)[0]);

  function withoutContent(props: Record<string, unknown>) {
    const { content, ...rest } = props;
    return rest;
  }
</script>

{#snippet renderInstance(instance: { component: string; props: Record<string, unknown> })}
  {@const Cmp = componentMap[instance.component]}
  {#if Cmp}
    {#if instance.props.content != null}
      <svelte:component this={Cmp} {...withoutContent(instance.props)}>
        {String(instance.props.content)}
      </svelte:component>
    {:else}
      <svelte:component this={Cmp} {...withoutContent(instance.props)} />
    {/if}
  {/if}
{/snippet}

{#snippet sizes(size)}
  {#if matrixInstance}
    {@render renderInstance({
      component: matrixInstance.component,
      props: { ...matrixInstance.props, size },
    })}
  {/if}
{/snippet}

{#snippet densities(density)}
  {#if matrixInstance}
    {@render renderInstance({
      component: matrixInstance.component,
      props: { ...matrixInstance.props, density },
    })}
  {/if}
{/snippet}

{#if scene}
  <SpecimenLayout
    sizeValues={scene.sizeAxis.length > 0 ? scene.sizeAxis : undefined}
    densityValues={scene.densityAxis.length > 0 ? scene.densityAxis : undefined}
    sizes={(scene.tabs as readonly string[]).includes("sizes") ? (sizes as never) : undefined}
    densities={(scene.tabs as readonly string[]).includes("densities") ? (densities as never) : undefined}
  >
    {#each scene.groups as group}
      <SpecimenGroup label={group.label}>
        <div class="poodle-specimen__row">
          {#each group.instances as instance}
            <div class="poodle-specimen__cell">
              {@render renderInstance(instance)}
            </div>
          {/each}
        </div>
      </SpecimenGroup>
    {/each}
  </SpecimenLayout>
{/if}

<style>
  .poodle-specimen__row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    align-items: center;
  }

  .poodle-specimen__cell {
    display: inline-flex;
    align-items: center;
  }
</style>
