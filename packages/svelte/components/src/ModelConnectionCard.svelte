<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/model-connection.css";
  import {
    disclosureTransition,
    modelConnectionReadinessTone,
  } from "@inflatable-cookie/poodle-core";
  import { tick, type Snippet } from "svelte";

  import { default as Icon } from "./Icon.svelte";
  import { default as IconButton } from "./IconButton.svelte";
  import { default as StatusIndicator } from "./StatusIndicator.svelte";
  import { default as Switch } from "./Switch.svelte";

  interface IdProps {
    id: string;
  }

  interface ActionsProps {
    id: string;
    isOpen: boolean;
  }

  interface DetailsProps {
    id: string;
    isEnabled: boolean;
  }

  interface Props {
    id: string;
    title: string;
    providerLabel: string;
    routeLabel?: string | null;
    version?: string | null;
    accessSummary?: string | null;
    readiness?:
      | "ready"
      | "checking"
      | "attention"
      | "unavailable"
      | "unknown"
      | "error";
    readinessLabel?: string;
    open?: boolean | undefined;
    defaultOpen?: boolean;
    isEnabled?: boolean;
    isEnableDisabled?: boolean;
    isDisabled?: boolean;
    ariaLabel?: string | null;
    onOpenChange?: ((open: boolean) => void) | null;
    onEnabledChange?: ((enabled: boolean) => void) | null;
    leading?: Snippet<[IdProps]>;
    badges?: Snippet<[IdProps]>;
    closedAccessory?: Snippet<[IdProps]>;
    actions?: Snippet<[ActionsProps]>;
    details?: Snippet<[DetailsProps]>;
  }

  let {
    id,
    title,
    providerLabel,
    routeLabel = null,
    version = null,
    accessSummary = null,
    readiness = "unknown",
    readinessLabel = "Status unknown",
    open = undefined,
    defaultOpen = false,
    isEnabled = true,
    isEnableDisabled = false,
    isDisabled = false,
    ariaLabel = null,
    onOpenChange = null,
    onEnabledChange = null,
    leading,
    badges,
    closedAccessory,
    actions,
    details,
  }: Props = $props();

  let uncontrolledOpen = $state(false);
  let seeded = $state(false);
  let summaryEl = $state<HTMLElement | null>(null);
  const detailsId = $derived(`poodle-mcc-details-${id.replace(/[^a-zA-Z0-9_-]+/g, "-")}`);

  $effect.pre(() => {
    if (seeded) return;
    uncontrolledOpen = defaultOpen;
    seeded = true;
  });

  const isOpenControlled = $derived(open !== undefined);
  const isOpen = $derived(isOpenControlled ? open === true : uncontrolledOpen);
  const meta = $derived(
    [routeLabel, version].filter((part): part is string => Boolean(part)).join(" · "),
  );

  function disclosureControl(): HTMLElement | null {
    return (
      summaryEl?.querySelector<HTMLElement>("[data-model-connection-disclosure] button") ?? null
    );
  }

  function toggleOpen(): void {
    const result = disclosureTransition(
      { open: isOpen, disabled: isDisabled },
      { type: "TOGGLE" },
    );

    for (const effect of result.effects) {
      if (effect.type !== "emitOpenChange") continue;
      if (!isOpenControlled) uncontrolledOpen = effect.open;
      onOpenChange?.(effect.open);
      if (!effect.open) {
        void tick().then(() => disclosureControl()?.focus());
      }
    }
  }

  $effect(() => {
    if (isOpen) return;
    const active = document.activeElement;
    const details = summaryEl?.parentElement?.querySelector(`#${CSS.escape(detailsId)}`);
    if (active instanceof HTMLElement && details?.contains(active)) {
      disclosureControl()?.focus();
    }
  });
</script>

<section
  class="poodle-model-connection-card"
  data-open={isOpen ? "true" : "false"}
  data-readiness={readiness}
  data-enabled={isEnabled ? "true" : "false"}
  aria-label={ariaLabel ?? title}
>
  <div class="poodle-model-connection-card__summary" bind:this={summaryEl}>
    <StatusIndicator
      status={modelConnectionReadinessTone(readiness)}
      label={readinessLabel}
    />

    <span class="poodle-model-connection-card__leading" aria-hidden="true">
      {#if leading}
        {@render leading({ id })}
      {:else}
        <Icon name="package" />
      {/if}
    </span>

    <div class="poodle-model-connection-card__identity">
      <div class="poodle-model-connection-card__title-row">
        <h3 class="poodle-model-connection-card__title">{title}</h3>
        {#if badges}
          {@render badges({ id })}
        {/if}
      </div>
      {#if meta}
        <p class="poodle-model-connection-card__meta">{meta}</p>
      {/if}
      {#if accessSummary}
        <p class="poodle-model-connection-card__access">{accessSummary}</p>
      {/if}
      <span class="poodle-sr-only">{providerLabel}</span>
    </div>

    <div class="poodle-model-connection-card__controls">
      {#if !isOpen && closedAccessory}
        {@render closedAccessory({ id })}
      {/if}
      {#if actions}
        {@render actions({ id, isOpen })}
      {/if}
      <span data-model-connection-disclosure>
        <IconButton
          icon={isOpen ? "chevron-up" : "chevron-down"}
          ariaLabel={isOpen ? `Collapse ${title}` : `Expand ${title}`}
          variant="ghost"
          sizeRole="chrome"
          disabled={isDisabled}
          expanded={isOpen}
          controls={detailsId}
          onClick={() => toggleOpen()}
        />
      </span>
      <Switch
        checked={isEnabled}
        disabled={isDisabled || isEnableDisabled}
        ariaLabel={`Enable ${title}`}
        onCheckedChange={(next) => {
          if (isDisabled || isEnableDisabled) return;
          onEnabledChange?.(next);
        }}
      />
    </div>
  </div>

  {#if isOpen}
    <div
      class="poodle-model-connection-card__details"
      id={detailsId}
      role="region"
      aria-label={`${title} details`}
    >
      {#if details}
        {@render details({ id, isEnabled })}
      {/if}
    </div>
  {/if}
</section>
