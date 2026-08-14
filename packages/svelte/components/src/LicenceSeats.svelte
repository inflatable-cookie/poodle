<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/licence.css";
  import {
    LICENCE_RELEASE_CONFIRM_TITLE,
    LICENCE_THIS_MACHINE,
    licenceSeatRows,
    type LicenceSeat,
  } from "@inflatable-cookie/poodle-core";

  import { default as ConfirmAction } from "./ConfirmAction.svelte";
  import { default as EditableLabel } from "./EditableLabel.svelte";
  import { default as Icon } from "./Icon.svelte";
  import { default as IconButton } from "./IconButton.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize } from "./types";

  interface Props {
    seats?: readonly LicenceSeat[];
    pendingMachineId?: string | null;
    title?: string;
    releaseLabel?: string;
    confirmRelease?: boolean;
    size?: ControlSize | null;
    density?: ControlDensity | null;
    onRename?: ((detail: { machineId: string; label: string | null }) => void) | undefined;
    onRelease?: ((detail: { machineId: string }) => void) | undefined;
  }

  let {
    seats = [],
    pendingMachineId = null,
    title = "Activated machines",
    releaseLabel = "Release",
    confirmRelease = true,
    size = null,
    density = null,
    onRename = undefined,
    onRelease = undefined,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, "control"));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const glyphSize = $derived(resolveSemanticControlSize(resolvedSize, "chrome"));

  const rows = $derived(licenceSeatRows(seats, pendingMachineId, releaseLabel));
</script>

<!-- No seats renders nothing at all. A "1 seat" line would be Poodle inventing
     an account of seats the authority did not give it. -->
{#if rows.length > 0}
  <section
    class="poodle-licence-seats"
    aria-label={title}
    data-size={resolvedSize}
    data-density={resolvedDensity}
  >
    <h3 class="poodle-licence-seats__title">{title}</h3>
    <ul class="poodle-licence-seats__list">
      {#each rows as row (row.machineId)}
        <li class="poodle-licence-seats__row" data-this-machine={row.thisMachine}>
          <div class="poodle-licence-seats__identity">
            <span class="poodle-licence-seats__machine-icon" aria-hidden="true">
              <Icon icon="monitor" size={glyphSize} />
            </span>
            <!-- The supplied label or `Unnamed machine`. Never the machine ID,
                 whole or shortened: it is a random command identifier, and
                 showing it would offer identity Poodle was never given. Two
                 unnamed rows looking alike is the honest outcome. -->
            <div class="poodle-licence-seats__label">
              <EditableLabel
                value={row.named ? row.displayLabel : ""}
                ariaLabel={`Rename ${row.named ? row.displayLabel : "unnamed machine"}`}
                activationMode="enterOrSpace"
                variant="flush"
                emptyText="Unnamed machine"
                placeholder="Unnamed machine"
                showEditIcon
                size={resolvedSize}
                density={resolvedDensity}
                onCommit={({ value }) =>
                  onRename?.({ machineId: row.machineId, label: value || null })}
              />
            </div>
            {#if row.thisMachine}
              <span class="poodle-licence-seats__marker">{LICENCE_THIS_MACHINE}</span>
            {/if}
          </div>

          {#if row.releasable}
            <span class="poodle-licence-seats__action">
              {#if confirmRelease}
                <ConfirmAction
                  title={LICENCE_RELEASE_CONFIRM_TITLE}
                  description={row.confirmBody}
                  tone="warning"
                  confirmLabel={releaseLabel}
                  size={resolvedSize}
                  density={resolvedDensity}
                  onConfirm={() => onRelease?.({ machineId: row.machineId })}
                >
                  {#snippet trigger()}
                    <IconButton
                      icon="trash-2"
                      variant="ghost"
                      tone="danger"
                      size={resolvedSize}
                      density={resolvedDensity}
                      disabled={row.pending}
                      loading={row.pending}
                      ariaLabel={row.releaseName}
                    />
                  {/snippet}
                </ConfirmAction>
              {:else}
                <IconButton
                  icon="trash-2"
                  variant="ghost"
                  tone="danger"
                  size={resolvedSize}
                  density={resolvedDensity}
                  disabled={row.pending}
                  loading={row.pending}
                  ariaLabel={row.releaseName}
                  onClick={() => onRelease?.({ machineId: row.machineId })}
                />
              {/if}
            </span>
          {/if}
        </li>
      {/each}
    </ul>
  </section>
{/if}
