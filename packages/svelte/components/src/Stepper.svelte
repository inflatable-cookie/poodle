<script lang="ts">
  import "@poodle/styles/stepper.css";

  import Icon from "./Icon.svelte";
  import Spinner from "./Spinner.svelte";
  import {
    getUiPresentation,
    resolveSemanticControlSize,
    resolveSupportingVisualSize,
  } from "./presentation";

  import type {
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
    StepperStep,
  } from "./types";

  interface Props {
    steps?: StepperStep[];
    value?: string | null | undefined;
    defaultValue?: string | null;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    disabled?: boolean;
    ariaLabel?: string | null;
    rerunLabel?: string;
    onValueChange?: ((value: string) => void) | undefined;
    onRerun?: ((value: string) => void) | undefined;
  }

  let {
    steps = [],
    value = $bindable<string | null | undefined>(undefined),
    defaultValue = null,
    size = null,
    sizeRole = "control",
    density = null,
    disabled = false,
    ariaLabel = null,
    rerunLabel = "Re-run step",
    onValueChange = undefined,
    onRerun = undefined,
  }: Props = $props();

  let uncontrolledValue = $state<string | null>(null);
  let seededDefaultValue = $state(false);
  const uiPresentation = getUiPresentation();

  $effect.pre(() => {
    if (!seededDefaultValue) {
      // Falling back to the first step matters: a stepper with no current step
      // renders every row as "not here", which is never what a wizard means.
      uncontrolledValue = defaultValue ?? steps[0]?.value ?? null;
      seededDefaultValue = true;
    }
  });

  const isControlled = $derived(value !== undefined);
  const currentValue = $derived(isControlled ? value : uncontrolledValue);
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const markerVisualSize = $derived(resolveSupportingVisualSize(resolvedSize));

  /**
   * The status word appended to a step's accessible name.
   *
   * Status is carried in colour and glyph, and neither reaches a screen reader.
   * `pending` is omitted because it is the unremarkable case — announcing
   * "step 4, Apply, pending" on every unvisited step is noise.
   */
  function statusSuffix(status: StepperStep["status"]): string {
    switch (status) {
      case "running":
        return ", running";
      case "complete":
        return ", complete";
      case "failed":
        return ", failed";
      default:
        return "";
    }
  }

  function isStepDisabled(step: StepperStep): boolean {
    return disabled || step.isDisabled === true;
  }

  function selectStep(step: StepperStep): void {
    if (isStepDisabled(step)) return;
    if (!isControlled) {
      uncontrolledValue = step.value;
    } else {
      value = step.value;
    }
    onValueChange?.(step.value);
  }

  /** Move focus between triggers without selecting — see stepper.md §6. */
  function moveFocus(event: KeyboardEvent, index: number): void {
    const enabled = steps
      .map((step, i) => ({ step, i }))
      .filter(({ step }) => !isStepDisabled(step));
    if (enabled.length === 0) return;

    const position = enabled.findIndex(({ i }) => i === index);
    let target: number | undefined;

    if (event.key === "ArrowRight") target = enabled[Math.min(position + 1, enabled.length - 1)]?.i;
    else if (event.key === "ArrowLeft") target = enabled[Math.max(position - 1, 0)]?.i;
    else if (event.key === "Home") target = enabled[0]?.i;
    else if (event.key === "End") target = enabled[enabled.length - 1]?.i;
    else return;

    event.preventDefault();
    if (target === undefined) return;
    const root = event.currentTarget as HTMLElement;
    const triggers = root
      .closest(".poodle-stepper")
      ?.querySelectorAll<HTMLButtonElement>(".poodle-stepper__trigger");
    triggers?.[target]?.focus();
  }
</script>

<nav class="poodle-stepper" data-size={resolvedSize} data-density={resolvedDensity} aria-label={ariaLabel ?? undefined}>
  <ol class="poodle-stepper__list">
    {#each steps as step, index (step.value)}
      <li class="poodle-stepper__step" data-status={step.status}>
        <button
          type="button"
          class="poodle-stepper__trigger"
          disabled={isStepDisabled(step)}
          aria-current={currentValue === step.value ? "step" : undefined}
          aria-label={`${step.label}${statusSuffix(step.status)}${step.description ? `. ${step.description}` : ""}`}
          onclick={() => selectStep(step)}
          onkeydown={(event) => moveFocus(event, index)}
        >
          <!-- Decorative: the status word is already in the accessible name, so
               announcing the glyph too would read "tick, complete". -->
          <span class="poodle-stepper__marker" aria-hidden="true">
            {#if step.status === "running"}
              <Spinner variant="ring" size={markerVisualSize} tone="current" />
            {:else if step.status === "complete"}
              <Icon name="check" size={markerVisualSize} />
            {:else if step.status === "failed"}
              <Icon name="x" size={markerVisualSize} />
            {:else}
              {index + 1}
            {/if}
          </span>
          <span class="poodle-stepper__label">{step.label}</span>
        </button>

        {#if onRerun && step.status === "complete"}
          <button
            type="button"
            class="poodle-stepper__rerun"
            disabled={isStepDisabled(step)}
            aria-label={`${rerunLabel}: ${step.label}`}
            onclick={() => onRerun?.(step.value)}
          >
            <Icon name="refresh-cw" size={markerVisualSize} />
          </button>
        {/if}
      </li>
    {/each}
  </ol>
</nav>
