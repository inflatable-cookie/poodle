<script module lang="ts">
  let nextAccordionId = 0;
</script>

<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/accordion.css";
  import { toggleGroupTransition } from "@inflatable-cookie/poodle-core";
  import { untrack, type Snippet } from "svelte";

  import { default as Icon } from "./Icon.svelte";
  import { useMotionReady } from "./motion-ready.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { AccordionItem, ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  interface Props {
    items?: AccordionItem[];
    value?: string | string[] | null;
    defaultValue?: string | string[] | null;
    selectionMode?: "single" | "multiple";
    collapsible?: boolean;
    ariaLabel?: string | null;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onValueChange?: ((value: string | string[] | null) => void) | undefined;
    children?: Snippet<[AccordionItem, boolean]>;
  }

  let {
    items = [],
    value = $bindable<string | string[] | null>(null),
    defaultValue = null,
    selectionMode = "single",
    collapsible = true,
    ariaLabel = null,
    size = null,
    sizeRole = "control",
    density = null,
    onValueChange = undefined,
    children,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const motionReady = useMotionReady();
  const accordionId = ++nextAccordionId;
  let uncontrolledValue = $state<string | string[] | null>(null);
  let seededDefaultValue = $state(false);

  $effect.pre(() => {
    if (!seededDefaultValue) {
      uncontrolledValue = defaultValue ?? (selectionMode === "multiple" ? [] : null);
      seededDefaultValue = true;
    }
  });

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const isControlled = $derived(value !== null);
  const currentValue = $derived(isControlled ? value : uncontrolledValue);
  const openValues = $derived(Array.isArray(currentValue)
    ? currentValue
    : currentValue
      ? [currentValue]
      : []);
  let closing = $state(new Set<string>());
  let previousOpen = new Set<string>();

  $effect.pre(() => {
    const next = new Set(openValues);
    const ready = motionReady.ready;
    const prev = previousOpen;
    previousOpen = next;
    const nextClosing = new Set<string>();
    if (ready) {
      const current = untrack(() => closing);
      for (const value of current) {
        if (!next.has(value)) {
          nextClosing.add(value);
        }
      }
      for (const value of prev) {
        if (!next.has(value)) {
          nextClosing.add(value);
        }
      }
    }
    const current = untrack(() => closing);
    if (
      current.size !== nextClosing.size ||
      [...nextClosing].some((value) => !current.has(value))
    ) {
      closing = nextClosing;
    }
  });

  function toggle(itemValue: string): void {
    const result = toggleGroupTransition(
      {
        value: selectionMode === "multiple" ? openValues : (typeof currentValue === "string" ? currentValue : null),
        options: items.map((item) => ({ value: item.value, disabled: item.disabled === true })),
        selectionMode,
        allowDeactivation: collapsible,
        disabled: false,
      },
      { type: "TOGGLE", value: itemValue },
    );

    for (const effect of result.effects) {
      if (effect.type === "emitValueChange") {
        if (!isControlled) {
          uncontrolledValue = effect.value;
        } else {
          value = effect.value;
        }

        onValueChange?.(effect.value);
      }
    }
  }
</script>

<div
  class="poodle-accordion"
  role={selectionMode === "multiple" ? "group" : undefined}
  aria-label={ariaLabel ?? undefined}
  data-size={resolvedSize}
  data-density={resolvedDensity}
  data-motion-ready={motionReady.ready}
>
  {#each items as item (item.value)}
    <section class="poodle-accordion__item" data-open={openValues.includes(item.value)}>
      <h3 class="poodle-accordion__heading">
        <button
          type="button"
          class="poodle-accordion__trigger"
          id={`poodle-accordion-trigger-${accordionId}-${item.value}`}
          disabled={item.disabled === true}
          aria-expanded={openValues.includes(item.value) ? "true" : "false"}
          aria-controls={`poodle-accordion-panel-${accordionId}-${item.value}`}
          onclick={() => toggle(item.value)}
        >
          <span class="poodle-accordion__summary">
            <span class="poodle-accordion__title">{item.label}</span>
            {#if item.description}
              <span class="poodle-accordion__description">{item.description}</span>
            {/if}
          </span>
          <span class="poodle-accordion__indicator" aria-hidden="true"><Icon name="chevron-down" /></span>
        </button>
      </h3>

      <div
        class="poodle-accordion__panel-clip"
        ontransitionend={() => {
          if (!openValues.includes(item.value)) {
            const nextClosing = new Set(closing);
            nextClosing.delete(item.value);
            closing = nextClosing;
          }
        }}
      >
        <div
          class="poodle-accordion__panel"
          id={`poodle-accordion-panel-${accordionId}-${item.value}`}
          role="region"
          aria-labelledby={`poodle-accordion-trigger-${accordionId}-${item.value}`}
          hidden={!openValues.includes(item.value) && !closing.has(item.value)}
          inert={!openValues.includes(item.value)}
          aria-hidden={!openValues.includes(item.value) ? "true" : undefined}
        >
          {@render children?.(item, openValues.includes(item.value))}
        </div>
      </div>
    </section>
  {/each}
</div>

