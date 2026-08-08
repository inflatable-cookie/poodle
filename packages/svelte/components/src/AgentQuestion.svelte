<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/agent-question.css";

  import {
    canSubmitQuestion,
    declineQuestion,
    questionProgress,
    resolveQuestionAnswer,
    showsQuestionProgress,
    submitsOnSelect,
    toggleQuestionSelection,
    type AgentQuestionAnswer,
    type AgentQuestionItem,
  } from "@inflatable-cookie/poodle-core";

  import Checkbox from "./Checkbox.svelte";
  import Eyebrow from "./Eyebrow.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  interface Props {
    questions?: AgentQuestionItem[];
    activeIndex?: number;
    selections?: string[];
    /** The composer's editor text. Passed in so the answer can be resolved here. */
    override?: string;
    dismissible?: boolean;
    dismissLabel?: string;
    progressLabel?: (current: number, total: number) => string;
    showShortcuts?: boolean;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onSelectionChange?: ((values: string[]) => void) | undefined;
    onSubmit?: ((answer: AgentQuestionAnswer) => void) | undefined;
    onDismiss?: ((id: string) => void) | undefined;
  }

  let {
    questions = [],
    activeIndex = $bindable<number>(0),
    selections = $bindable<string[]>([]),
    override = "",
    dismissible = false,
    dismissLabel = "Skip this question",
    progressLabel = (current: number, total: number) => `${current} of ${total}`,
    showShortcuts = true,
    size = null,
    sizeRole = "control",
    density = null,
    onSelectionChange = undefined,
    onSubmit = undefined,
    onDismiss = undefined,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);

  const activeQuestion = $derived(questions[activeIndex] ?? null);
  const isMultiSelect = $derived(activeQuestion?.allowMultiple === true);
  const progress = $derived(questionProgress(questions, activeIndex));
  const showsProgress = $derived(showsQuestionProgress(questions));
  const instanceId = $derived(activeQuestion?.id ?? "agent-question");

  /**
   * Entering override text clears the selection.
   *
   * The alternative — locking the editor once an option is picked — traps the
   * reader: tick a box, find that none of the options fit, and now you have to
   * untick before you can type. See agent-question.md §5.
   */
  $effect(() => {
    if (override.trim().length > 0 && selections.length > 0) {
      selections = [];
      onSelectionChange?.([]);
    }
  });

  export function submit(): void {
    if (!activeQuestion) return;
    const answer = resolveQuestionAnswer(activeQuestion, selections, override);
    if (!answer) return;
    onSubmit?.(answer);
  }

  export function canSubmit(): boolean {
    return canSubmitQuestion(activeQuestion, selections, override);
  }

  function choose(value: string): void {
    if (!activeQuestion) return;

    const next = toggleQuestionSelection(activeQuestion, selections, value);
    selections = next;
    onSelectionChange?.(next);

    // Single-select resolves on one click: the first click is also the last.
    // Multi-select cannot, because a click is indistinguishable from a
    // first-of-several.
    if (submitsOnSelect(activeQuestion)) {
      const answer = resolveQuestionAnswer(activeQuestion, next, override);
      if (answer) onSubmit?.(answer);
    }
  }

  function dismiss(): void {
    if (!activeQuestion) return;
    onDismiss?.(activeQuestion.id);
    onSubmit?.(declineQuestion(activeQuestion));
  }

  /**
   * Digit shortcuts, ignored while focus is in a text field.
   *
   * Without that guard, typing "1" into the override would select an option
   * instead of reaching the editor.
   */
  function handleKeydown(event: KeyboardEvent): void {
    if (!activeQuestion || !showShortcuts) return;

    const target = event.target as HTMLElement | null;
    if (target && /^(INPUT|TEXTAREA)$/.test(target.tagName)) return;
    if (event.metaKey || event.ctrlKey || event.altKey) return;

    const digit = Number.parseInt(event.key, 10);
    if (!Number.isFinite(digit) || digit < 1 || digit > 9) return;

    const option = activeQuestion.options[digit - 1];
    if (!option) return;

    event.preventDefault();
    choose(option.value);
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if activeQuestion}
  <div
    class="poodle-agent-question"
    data-size={resolvedSize}
    data-density={resolvedDensity}
    data-multi-select={String(isMultiSelect)}
  >
    {#if showsProgress}
      <div class="poodle-agent-question__progress">
        <!-- Decorative: the label beside them carries the same fact in words. -->
        <span class="poodle-agent-question__progress-dots" aria-hidden="true">
          {#each progress.states as state, index (index)}
            <span class="poodle-agent-question__progress-dot" data-state={state}></span>
          {/each}
        </span>
        <span class="poodle-agent-question__progress-label">
          {progressLabel(progress.current, progress.total)}
        </span>
      </div>
    {/if}

    {#if activeQuestion.header}
      <Eyebrow>{activeQuestion.header}</Eyebrow>
    {/if}

    <p class="poodle-agent-question__prompt" id={`${instanceId}-prompt`}>{activeQuestion.prompt}</p>

    <div
      class="poodle-agent-question__options"
      role={isMultiSelect ? "group" : "radiogroup"}
      aria-labelledby={`${instanceId}-prompt`}
    >
      {#each activeQuestion.options as option, index (option.value)}
        <button
          type="button"
          class="poodle-agent-question__option"
          role={isMultiSelect ? "checkbox" : "radio"}
          aria-checked={selections.includes(option.value)}
          data-selected={String(selections.includes(option.value))}
          onclick={() => choose(option.value)}
        >
          <!-- Decorative: the option itself carries the state, and announcing
               it twice is worse than announcing it once. -->
          {#if isMultiSelect}
            <span class="poodle-agent-question__option-check" aria-hidden="true">
              <Checkbox checked={selections.includes(option.value)} size={resolvedSize} />
            </span>
          {/if}

          <span class="poodle-agent-question__option-body">
            <span class="poodle-agent-question__option-label">{option.label}</span>
            {#if option.description}
              <span class="poodle-agent-question__option-description">{option.description}</span>
            {/if}
          </span>

          {#if showShortcuts && index < 9}
            <kbd class="poodle-agent-question__option-shortcut" aria-hidden="true">{index + 1}</kbd>
          {/if}
        </button>
      {/each}
    </div>

    {#if dismissible}
      <button type="button" class="poodle-agent-question__dismiss" onclick={dismiss}>
        {dismissLabel}
      </button>
    {/if}
  </div>
{/if}
