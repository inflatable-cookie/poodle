<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/model-connection.css";
  import {
    filterModelConnectionOptions,
    groupModelConnectionOptions,
    menuListNavigate,
    modelConnectionAvailabilityTone,
    modelConnectionOptionSelectable,
    modelConnectionPickerResultAnnouncement,
    resolveModelConnectionPickerShellState,
    singleSelectTransition,
    type ModelConnectionOption,
    type ModelConnectionPickerState,
  } from "@inflatable-cookie/poodle-core";
  import type { Snippet } from "svelte";

  import { default as Icon } from "./Icon.svelte";
  import { default as PickerShell } from "./PickerShell.svelte";
  import { default as Pill } from "./Pill.svelte";
  import { default as StatusIndicator } from "./StatusIndicator.svelte";
  import { default as TextInput } from "./TextInput.svelte";
  import type { PickerVariant } from "./types";

  interface LeadingProps {
    option: ModelConnectionOption;
  }

  interface Props {
    options?: ModelConnectionOption[];
    value?: string | null | undefined;
    defaultValue?: string | null;
    query?: string | undefined;
    defaultQuery?: string;
    state?: ModelConnectionPickerState;
    title?: string;
    description?: string | null;
    searchPlaceholder?: string;
    ariaLabel?: string | null;
    isDisabled?: boolean;
    variant?: PickerVariant;
    onValueChange?: ((id: string) => void) | null;
    onQueryChange?: ((query: string) => void) | null;
    leading?: Snippet<[LeadingProps]>;
    footer?: Snippet<[]>;
  }

  let {
    options = [],
    value = undefined,
    defaultValue = null,
    query = undefined,
    defaultQuery = "",
    state: pickerState = "ready",
    title = "Choose a connection",
    description = null,
    searchPlaceholder = "Search connections",
    ariaLabel = null,
    isDisabled = false,
    variant = "inline",
    onValueChange = null,
    onQueryChange = null,
    leading,
    footer: footerSnippet,
  }: Props = $props();

  let instanceId = $state(`poodle-mcp-${Math.random().toString(36).slice(2, 9)}`);
  let uncontrolledValue = $state<string | null>(null);
  let uncontrolledQuery = $state("");
  let seeded = $state(false);
  let rootEl = $state<HTMLElement | null>(null);

  function groupDomId(group: string): string {
    return `${instanceId}-${group.replace(/[^a-zA-Z0-9_-]+/g, "-").toLowerCase()}`;
  }

  $effect.pre(() => {
    if (seeded) return;
    uncontrolledValue = defaultValue;
    uncontrolledQuery = defaultQuery;
    seeded = true;
  });

  const isValueControlled = $derived(value !== undefined);
  const isQueryControlled = $derived(query !== undefined);
  const currentValue = $derived(isValueControlled ? (value ?? null) : uncontrolledValue);
  const currentQuery = $derived(isQueryControlled ? (query ?? "") : uncontrolledQuery);
  const filtered = $derived(filterModelConnectionOptions(options, currentQuery));
  const groups = $derived(groupModelConnectionOptions(filtered));
  const flatEnabled = $derived(filtered.filter(modelConnectionOptionSelectable));
  const shellState = $derived(
    resolveModelConnectionPickerShellState(
      pickerState,
      options.length,
      filtered.length,
      currentQuery,
    ),
  );
  const statusText = $derived(
    shellState === "ready" || shellState === "no-results"
      ? modelConnectionPickerResultAnnouncement(filtered.length, currentQuery)
      : null,
  );

  function setQuery(next: string): void {
    if (!isQueryControlled) uncontrolledQuery = next;
    onQueryChange?.(next);
  }

  function select(id: string): void {
    const result = singleSelectTransition(
      {
        value: currentValue,
        options: options.map((option) => ({
          value: option.id,
          disabled: !modelConnectionOptionSelectable(option),
        })),
        disabled: isDisabled,
      },
      { type: "SELECT", value: id },
    );

    for (const effect of result.effects) {
      if (effect.type === "emitValueChange") {
        if (!isValueControlled) uncontrolledValue = effect.value;
        onValueChange?.(effect.value);
      }
    }
  }

  function focusOption(id: string): void {
    rootEl
      ?.querySelector<HTMLElement>(`[data-model-connection-option="${CSS.escape(id)}"]`)
      ?.focus();
  }

  function handleOptionKeydown(event: KeyboardEvent, optionId: string): void {
    if (isDisabled) return;

    const enabled = flatEnabled;
    const currentIndex = enabled.findIndex((option) => option.id === optionId);
    if (currentIndex < 0) return;

    const navItems = enabled.map((option) => ({ value: option.id, disabled: false }));
    let next: (typeof enabled)[number] | undefined;

    if (event.key === "ArrowDown" || event.key === "ArrowRight") {
      event.preventDefault();
      next = enabled[menuListNavigate(navItems, currentIndex, "next")];
    } else if (event.key === "ArrowUp" || event.key === "ArrowLeft") {
      event.preventDefault();
      next = enabled[menuListNavigate(navItems, currentIndex, "prev")];
    } else if (event.key === "Home") {
      event.preventDefault();
      next = enabled[0];
    } else if (event.key === "End") {
      event.preventDefault();
      next = enabled[enabled.length - 1];
    } else if (event.key === " " || event.key === "Enter") {
      event.preventDefault();
      select(optionId);
      return;
    }

    if (!next) return;
    select(next.id);
    focusOption(next.id);
  }

  function tabIndexFor(option: ModelConnectionOption): number {
    if (isDisabled || !modelConnectionOptionSelectable(option)) return -1;
    if (currentValue === option.id) return 0;
    if (currentValue === null && flatEnabled[0]?.id === option.id) return 0;
    return -1;
  }
</script>

<section
  class="poodle-model-connection-picker"
  bind:this={rootEl}
  aria-label={ariaLabel ?? title}
  data-disabled={isDisabled ? "true" : "false"}
>
  {#snippet toolbarSnippet()}
    <TextInput
      type="search"
      value={currentQuery}
      placeholder={searchPlaceholder}
      disabled={isDisabled}
      ariaLabel={searchPlaceholder}
      describedBy={`${instanceId}-status`}
      onValueChange={(next) => setQuery(next)}
    />
  {/snippet}

  {#snippet footerContent()}
    <div class="poodle-model-connection-picker__footer">
      {@render footerSnippet?.()}
    </div>
  {/snippet}

  <PickerShell
    {title}
    {description}
    {variant}
    state={shellState}
    ariaLabel={ariaLabel ?? title}
    resultCount={shellState === "ready" ? filtered.length : null}
    selectionCount={currentValue ? 1 : 0}
    statusText={statusText}
    statusId={`${instanceId}-status`}
    toolbar={toolbarSnippet as unknown as Snippet<[]>}
    footer={footerSnippet ? (footerContent as unknown as Snippet<[]>) : undefined}
  >
    <div class="poodle-model-connection-picker__groups">
      {#each groups as group (group.group)}
        {@const groupId = groupDomId(group.group)}
        <section class="poodle-model-connection-picker__group" aria-labelledby={groupId}>
          <h3 class="poodle-model-connection-picker__group-title" id={groupId}>
            {group.group}
          </h3>
          <div
            class="poodle-model-connection-picker__options"
            role="radiogroup"
            aria-labelledby={groupId}
          >
            {#each group.options as option (option.id)}
              {@const selectable = modelConnectionOptionSelectable(option)}
              {@const optionDisabled = isDisabled || !selectable}
              <button
                type="button"
                class="poodle-model-connection-picker__option"
                role="radio"
                data-model-connection-option={option.id}
                data-availability={option.availability}
                tabindex={tabIndexFor(option)}
                aria-checked={currentValue === option.id ? "true" : "false"}
                aria-disabled={optionDisabled ? "true" : undefined}
                disabled={optionDisabled}
                onclick={() => select(option.id)}
                onkeydown={(event) => handleOptionKeydown(event, option.id)}
              >
                <span class="poodle-model-connection-picker__leading" aria-hidden="true">
                  {#if leading}
                    {@render leading({ option })}
                  {:else}
                    <Icon name="package" />
                  {/if}
                </span>
                <span class="poodle-model-connection-picker__copy">
                  <span class="poodle-model-connection-picker__title-row">
                    <span class="poodle-model-connection-picker__provider">{option.providerLabel}</span>
                    {#if option.badges.length > 0}
                      <span class="poodle-model-connection-picker__badges">
                        {#each option.badges as badge (badge.label)}
                          <Pill tone={badge.tone ?? "neutral"} appearance="subtle">{badge.label}</Pill>
                        {/each}
                      </span>
                    {/if}
                  </span>
                  {#if option.routeLabel}
                    <span class="poodle-model-connection-picker__route">{option.routeLabel}</span>
                  {/if}
                  {#if option.description}
                    <span class="poodle-model-connection-picker__description">{option.description}</span>
                  {/if}
                </span>
                <span class="poodle-model-connection-picker__availability">
                  <StatusIndicator
                    status={modelConnectionAvailabilityTone(option.availability)}
                    label={option.availabilityLabel}
                  />
                </span>
              </button>
            {/each}
          </div>
        </section>
      {/each}
    </div>
  </PickerShell>
</section>
