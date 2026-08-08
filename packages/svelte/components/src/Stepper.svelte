<script lang="ts">
  import "@inflatable-cookie/poodle-styles/stepper.css";

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
    Orientation,
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
    orientation?: Orientation;
    collapsible?: boolean;
    collapsed?: boolean | undefined;
    defaultCollapsed?: boolean;
    disabled?: boolean;
    ariaLabel?: string | null;
    rerunLabel?: string;
    onValueChange?: ((value: string) => void) | undefined;
    onRerun?: ((value: string) => void) | undefined;
    onCollapsedChange?: ((collapsed: boolean) => void) | undefined;
  }

  let {
    steps = [],
    value = $bindable<string | null | undefined>(undefined),
    defaultValue = null,
    size = null,
    sizeRole = "control",
    density = null,
    orientation = "horizontal",
    collapsible = false,
    collapsed = $bindable<boolean | undefined>(undefined),
    defaultCollapsed = false,
    disabled = false,
    ariaLabel = null,
    rerunLabel = "Re-run step",
    onValueChange = undefined,
    onRerun = undefined,
    onCollapsedChange = undefined,
  }: Props = $props();

  let uncontrolledValue = $state<string | null>(null);
  let uncontrolledCollapsed = $state(false);
  let seededDefaultValue = $state(false);
  const uiPresentation = getUiPresentation();

  $effect.pre(() => {
    if (!seededDefaultValue) {
      // Falling back to the first step matters: a stepper with no current step
      // renders every row as "not here", which is never what a wizard means.
      uncontrolledValue = defaultValue ?? steps[0]?.value ?? null;
      uncontrolledCollapsed = defaultCollapsed;
      seededDefaultValue = true;
    }
  });

  const isControlled = $derived(value !== undefined);
  const currentValue = $derived(isControlled ? value : uncontrolledValue);
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const markerVisualSize = $derived(resolveSupportingVisualSize(resolvedSize));

  // Collapse is vertical-only (stepper.md §3): a horizontal stepper is already
  // one line, so folding it trades legible labels for dashes and buys back no
  // height. `collapsible` is ignored rather than half-honoured.
  const canCollapse = $derived(collapsible && orientation === "vertical");
  const isCollapsedControlled = $derived(collapsed !== undefined);
  const isCollapsed = $derived(
    canCollapse && (isCollapsedControlled ? collapsed === true : uncontrolledCollapsed),
  );
  const currentStep = $derived(steps.find((step) => step.value === currentValue) ?? steps[0]);
  const completedCount = $derived(steps.filter((step) => step.status === "complete").length);

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

  function toggleCollapsed(): void {
    if (disabled) return;
    const next = !isCollapsed;
    if (!isCollapsedControlled) {
      uncontrolledCollapsed = next;
    } else {
      collapsed = next;
    }
    onCollapsedChange?.(next);
  }

  /** Move focus between triggers without selecting — see stepper.md §6. */
  function moveFocus(event: KeyboardEvent, index: number): void {
    const enabled = steps
      .map((step, i) => ({ step, i }))
      .filter(({ step }) => !isStepDisabled(step));
    if (enabled.length === 0) return;

    const position = enabled.findIndex(({ i }) => i === index);
    let target: number | undefined;

    // Arrows follow the axis the steps flow along (stepper.md §6).
    const nextKey = orientation === "vertical" ? "ArrowDown" : "ArrowRight";
    const prevKey = orientation === "vertical" ? "ArrowUp" : "ArrowLeft";
    if (event.key === nextKey) target = enabled[Math.min(position + 1, enabled.length - 1)]?.i;
    else if (event.key === prevKey) target = enabled[Math.max(position - 1, 0)]?.i;
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

<nav
  class="poodle-stepper"
  data-size={resolvedSize}
  data-density={resolvedDensity}
  data-orientation={orientation}
  data-collapsible={canCollapse ? "true" : undefined}
  data-collapsed={canCollapse ? String(isCollapsed) : undefined}
  aria-label={ariaLabel ?? undefined}
>
  {#if canCollapse}
    <!-- The visible "5/5" is aria-hidden and restated in the name: "five slash
         five" is not a sentence. Chevron and rail say nothing the name doesn't. -->
    <button
      type="button"
      class="poodle-stepper__summary"
      {disabled}
      aria-expanded={!isCollapsed}
      aria-label={`${currentStep?.label ?? ""}, ${completedCount} of ${steps.length} steps complete`}
      onclick={toggleCollapsed}
    >
      <span class="poodle-stepper__summary-chevron" aria-hidden="true">
        <Icon name={isCollapsed ? "chevron-right" : "chevron-down"} size={markerVisualSize} />
      </span>
      <span class="poodle-stepper__rail" aria-hidden="true">
        {#each steps as step (step.value)}
          <span
            class="poodle-stepper__rail-segment"
            data-status={step.status}
            data-current={currentValue === step.value ? "true" : undefined}
          ></span>
        {/each}
      </span>
      <span class="poodle-stepper__summary-label">{currentStep?.label ?? ""}</span>
      <span class="poodle-stepper__summary-count" aria-hidden="true">
        {completedCount}/{steps.length}
      </span>
    </button>
  {/if}

  <!-- Collapsed drops the list from the tree rather than hiding it: hidden
       triggers would still be four unreachable stops in the tab order. -->
  {#if !isCollapsed}
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
            <!-- Decorative: the status word is already in the accessible name,
                 so announcing the glyph too would read "tick, complete". -->
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
  {/if}
</nav>
