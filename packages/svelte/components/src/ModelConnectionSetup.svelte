<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/model-connection.css";
  import {
    modelConnectionSetupCanContinue,
    modelConnectionSetupCanSubmit,
    modelConnectionSetupTransition,
    type ModelConnectionOption,
    type ModelConnectionPickerState,
    type ModelConnectionSetupStage,
  } from "@inflatable-cookie/poodle-core";
  import { tick, type Snippet } from "svelte";

  import { default as Button } from "./Button.svelte";
  import { default as Callout } from "./Callout.svelte";
  import { default as Icon } from "./Icon.svelte";
  import { default as ModelConnectionPicker } from "./ModelConnectionPicker.svelte";
  import { default as Spinner } from "./Spinner.svelte";

  interface LeadingProps {
    option: ModelConnectionOption;
  }

  interface ConfigurationProps {
    option: ModelConnectionOption;
    isPending: boolean;
  }

  interface Props {
    stage?: ModelConnectionSetupStage | undefined;
    defaultStage?: ModelConnectionSetupStage;
    options?: ModelConnectionOption[];
    value?: string | null | undefined;
    defaultValue?: string | null;
    query?: string | undefined;
    pickerState?: ModelConnectionPickerState;
    title?: string;
    description?: string | null;
    canSubmit?: boolean;
    isPending?: boolean;
    pendingLabel?: string;
    error?: string | null;
    success?: string | null;
    continueLabel?: string;
    submitLabel?: string;
    backLabel?: string;
    cancelLabel?: string;
    ariaLabel?: string | null;
    onStageChange?: ((stage: ModelConnectionSetupStage) => void) | null;
    onValueChange?: ((id: string) => void) | null;
    onQueryChange?: ((query: string) => void) | null;
    onSubmit?: ((id: string) => void) | null;
    onCancel?: (() => void) | null;
    leading?: Snippet<[LeadingProps]>;
    configuration?: Snippet<[ConfigurationProps]>;
    configureAside?: Snippet<[LeadingProps]>;
  }

  let {
    stage = undefined,
    defaultStage = "choose",
    options = [],
    value = undefined,
    defaultValue = null,
    query = undefined,
    pickerState = "ready",
    title = "Add model connection",
    description = null,
    canSubmit = false,
    isPending = false,
    pendingLabel = "Checking connection",
    error = null,
    success = null,
    continueLabel = "Continue",
    submitLabel = "Add connection",
    backLabel = "Back",
    cancelLabel = "Cancel",
    ariaLabel = null,
    onStageChange = null,
    onValueChange = null,
    onQueryChange = null,
    onSubmit = null,
    onCancel = null,
    leading,
    configuration,
    configureAside,
  }: Props = $props();

  let uncontrolledStage = $state<ModelConnectionSetupStage>("choose");
  let uncontrolledValue = $state<string | null>(null);
  let uncontrolledQuery = $state("");
  let seeded = $state(false);
  let previousStage = $state<ModelConnectionSetupStage | null>(null);
  let headingEl = $state<HTMLElement | null>(null);
  let pickerRoot = $state<HTMLElement | null>(null);

  $effect.pre(() => {
    if (seeded) return;
    uncontrolledStage = defaultStage;
    uncontrolledValue = defaultValue;
    seeded = true;
  });

  const isStageControlled = $derived(stage !== undefined);
  const isValueControlled = $derived(value !== undefined);
  const isQueryControlled = $derived(query !== undefined);
  const currentStage = $derived(isStageControlled ? (stage as ModelConnectionSetupStage) : uncontrolledStage);
  const currentValue = $derived(isValueControlled ? (value ?? null) : uncontrolledValue);
  const currentQuery = $derived(isQueryControlled ? (query ?? "") : uncontrolledQuery);
  const selected = $derived(options.find((option) => option.id === currentValue));
  const requiresConfiguration = $derived(selected?.requiresConfiguration ?? true);
  const canContinue = $derived(
    modelConnectionSetupCanContinue({
      value: currentValue,
      options,
      isPending,
    }),
  );
  const canAdd = $derived(
    modelConnectionSetupCanSubmit({
      stage: currentStage,
      value: currentValue,
      options,
      canSubmit,
      isPending,
    }),
  );

  async function focusHeading(): Promise<void> {
    await tick();
    headingEl?.focus();
  }

  async function focusSelectedOption(): Promise<void> {
    await tick();
    if (!currentValue) return;
    pickerRoot
      ?.querySelector<HTMLElement>(
        `[data-model-connection-option="${CSS.escape(currentValue)}"]`,
      )
      ?.focus();
  }

  $effect(() => {
    const nextStage = currentStage;
    if (previousStage === null) {
      previousStage = nextStage;
      return;
    }
    if (previousStage === nextStage) return;
    previousStage = nextStage;
    if (nextStage === "configure") void focusHeading();
    else void focusSelectedOption();
  });

  function run(
    event:
      | { type: "SELECT"; id: string }
      | { type: "SET_QUERY"; query: string }
      | { type: "CONTINUE" }
      | { type: "BACK" }
      | { type: "SUBMIT" }
      | { type: "CANCEL" },
  ): void {
    const result = modelConnectionSetupTransition(
      {
        stage: currentStage,
        value: currentValue,
        query: currentQuery,
        options,
        canSubmit,
        isPending,
      },
      event,
    );

    if (!isStageControlled && result.context.stage !== currentStage) {
      uncontrolledStage = result.context.stage;
    }
    if (!isValueControlled && result.context.value !== currentValue) {
      uncontrolledValue = result.context.value;
    }
    if (!isQueryControlled && result.context.query !== currentQuery) {
      uncontrolledQuery = result.context.query;
    }

    for (const effect of result.effects) {
      switch (effect.type) {
        case "emitStageChange":
          onStageChange?.(effect.stage);
          break;
        case "emitValueChange":
          onValueChange?.(effect.id);
          break;
        case "emitQueryChange":
          onQueryChange?.(effect.query);
          break;
        case "emitSubmit":
          onSubmit?.(effect.id);
          break;
        case "emitCancel":
          onCancel?.();
          break;
      }
    }
  }
</script>

<section
  class="poodle-model-connection-setup"
  data-stage={currentStage}
  data-pending={isPending ? "true" : "false"}
  aria-label={ariaLabel ?? title}
  aria-busy={isPending ? "true" : undefined}
>
  <div class="poodle-model-connection-setup__header">
    <h2
      class="poodle-model-connection-setup__title"
      bind:this={headingEl}
      tabindex="-1"
    >
      {title}
    </h2>
    {#if description}
      <p class="poodle-model-connection-setup__description">{description}</p>
    {/if}
  </div>

  <div class="poodle-model-connection-setup__body">
    {#if currentStage === "choose"}
      <div bind:this={pickerRoot}>
        <ModelConnectionPicker
          {options}
          value={currentValue}
          query={currentQuery}
          state={pickerState}
          isDisabled={isPending}
          onValueChange={(id) => run({ type: "SELECT", id })}
          onQueryChange={(next) => run({ type: "SET_QUERY", query: next })}
          {leading}
        />
      </div>
    {:else if selected}
      <div class="poodle-model-connection-setup__selected">
        <span class="poodle-model-connection-picker__leading" aria-hidden="true">
          {#if leading}
            {@render leading({ option: selected })}
          {:else}
            <Icon name="package" />
          {/if}
        </span>
        <div>
          <p class="poodle-model-connection-picker__provider">{selected.providerLabel}</p>
          {#if selected.routeLabel}
            <p class="poodle-model-connection-setup__route">{selected.routeLabel}</p>
          {/if}
        </div>
      </div>

      {#if error}
        <Callout tone="danger" message={error} announceMode="assertive" />
      {/if}
      {#if success}
        <Callout tone="success" message={success} announceMode="polite" />
      {/if}

      {#if configuration || configureAside}
        <div class="poodle-model-connection-setup__configuration">
          {#if configuration}
            {@render configuration({ option: selected, isPending })}
          {/if}
          {#if configureAside}
            <div class="poodle-model-connection-setup__aside">
              {@render configureAside({ option: selected })}
            </div>
          {/if}
        </div>
      {/if}

      {#if isPending}
        <p class="poodle-model-connection-setup__pending" role="status" aria-live="polite">
          <Spinner variant="grid" tone="accent" />
          {pendingLabel}
        </p>
      {/if}
    {/if}
  </div>

  <div class="poodle-model-connection-setup__actions">
    {#if currentStage === "choose"}
      <Button variant="ghost" disabled={isPending} onClick={() => run({ type: "CANCEL" })}>
        {cancelLabel}
      </Button>
      <Button
        disabled={requiresConfiguration ? !canContinue : !canAdd}
        onClick={() => run({ type: requiresConfiguration ? "CONTINUE" : "SUBMIT" })}
      >
        {requiresConfiguration ? continueLabel : submitLabel}
      </Button>
    {:else}
      <Button variant="ghost" disabled={isPending} onClick={() => run({ type: "BACK" })}>
        {backLabel}
      </Button>
      <Button variant="ghost" disabled={isPending} onClick={() => run({ type: "CANCEL" })}>
        {cancelLabel}
      </Button>
      <Button disabled={!canAdd} onClick={() => run({ type: "SUBMIT" })}>
        {submitLabel}
      </Button>
    {/if}
  </div>
</section>
