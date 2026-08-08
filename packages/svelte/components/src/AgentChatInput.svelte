<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/agent-chat-input.css";
  import type { Snippet } from "svelte";

  import { default as Icon } from "./Icon.svelte";
  import { default as IconButton } from "./IconButton.svelte";
  import { default as Meter } from "./Meter.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import {
    actionIcon,
    actionState,
    canSubmit as canSubmitGate,
    contextPercentage,
    resolveSubmitIntent,
  } from "./agent-chat-input-model";
  import type {
    AgentChatAttachment,
    AgentChatStatus,
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
  } from "./types";

  interface Props {
    value?: string;
    placeholder?: string;
    /** Editor placeholder while a question is live; the editor is the override. */
    questionPlaceholder?: string;
    /** The question region. Host composes an `AgentQuestion` into it. */
    question?: Snippet;
    /** Whether the live question could be answered from its own state alone. */
    questionCanSubmit?: boolean;
    /** Editor placeholder while a plan awaits a decision; the editor is the revise channel. */
    planPlaceholder?: string;
    /** The plan region. Host composes an `AgentPlan` into it. */
    plan?: Snippet;
    status?: AgentChatStatus;
    disabled?: boolean;
    readOnly?: boolean;
    ariaLabel?: string;
    submitLabel?: string;
    stopLabel?: string;
    submitOnEnter?: boolean;
    minRows?: number;
    maxRows?: number;
    maxLength?: number | null;
    allowEmptySubmit?: boolean;
    attachments?: AgentChatAttachment[];
    contextUsed?: number | null;
    contextLimit?: number | null;
    contextWarnAt?: number;
    contextLabel?: string;
    toolbarDividers?: boolean;
    sizeRole?: SemanticControlSizeRole;
    size?: ControlSize | null;
    density?: ControlDensity | null;
    onSubmit?: ((value: string) => void) | null;
    onStop?: (() => void) | null;
    onValueChange?: ((value: string) => void) | null;
    onRemoveAttachment?: ((id: string) => void) | null;
    toolbar?: Snippet<[]>;
    footer?: Snippet<[]>;
  }

  let {
    value = $bindable(""),
    placeholder = "Send a message",
    questionPlaceholder = "Type your own answer, or leave this blank to use the selected option",
    question = undefined,
    questionCanSubmit = false,
    planPlaceholder = "Describe what to change, or decide the plan above",
    plan = undefined,
    status = "idle",
    disabled = false,
    readOnly = false,
    ariaLabel = "Message",
    submitLabel = "Send",
    stopLabel = "Stop",
    submitOnEnter = true,
    minRows = 2,
    maxRows = 12,
    maxLength = null,
    allowEmptySubmit = false,
    attachments = [],
    contextUsed = null,
    contextLimit = null,
    contextWarnAt = 0.8,
    contextLabel = "Context used",
    toolbarDividers = true,
    sizeRole = "control",
    size = null,
    density = null,
    onSubmit = null,
    onStop = null,
    onValueChange = null,
    onRemoveAttachment = null,
    toolbar = undefined,
    footer = undefined,
  }: Props = $props();

  const uiPresentation = getUiPresentation();

  let editorElement = $state<HTMLTextAreaElement | null>(null);
  let isComposing = $state(false);

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const isBusy = $derived(status === "busy");
  const isQuestioning = $derived(status === "questioning");
  const isReviewingPlan = $derived(status === "reviewing-plan");
  /**
   * While a question is live the editor is the override, so an empty editor is
   * still submittable when an option is chosen — the question's own state
   * decides. See agent-chat-input.md §Question Region.
   *
   * A proposed plan does not get that treatment: the turn is already complete,
   * so the editor keeps its ordinary semantics and sending a message is the
   * revise channel. See agent-chat-input.md §Plan Region.
   */
  const actionEnabled = $derived(
    isQuestioning
      ? !disabled && (value.trim().length > 0 || questionCanSubmit)
      : canSubmitGate({ disabled, isBusy, value, allowEmptySubmit }),
  );
  const resolvedPlaceholder = $derived(
    isQuestioning ? questionPlaceholder : isReviewingPlan ? planPlaceholder : placeholder,
  );
  const contextPercent = $derived(contextPercentage(contextUsed, contextLimit));
  const showContext = $derived(contextLimit !== null && contextLimit > 0);
  const contextHigh = $derived(contextLimit === null ? null : contextLimit * contextWarnAt);
  const contextAriaLabel = $derived(
    contextPercent === null ? contextLabel : `${contextLabel}, ${Math.round(contextPercent)}%`,
  );

  // Auto-grow: reset to the row floor, then take the content height up to the
  // `maxRows` ceiling.
  function autosize(editor: HTMLTextAreaElement): void {
    editor.style.height = "auto";
    const lineHeight = Number.parseFloat(getComputedStyle(editor).lineHeight);
    const ceiling = Number.isFinite(lineHeight) ? lineHeight * maxRows : Number.POSITIVE_INFINITY;
    editor.style.height = `${Math.min(editor.scrollHeight, ceiling)}px`;
  }

  // Reading `value` keeps this reactive to every edit.
  $effect(() => {
    const editor = editorElement;
    void value;
    if (!editor) return;
    autosize(editor);

    // The first pass can measure before the web font is applied, which reports
    // fallback-font metrics and leaves the editor short of `minRows` — and
    // nothing re-measures it, because the value never changed. Re-run once the
    // fonts settle.
    if (typeof document === "undefined" || !document.fonts) return;
    let cancelled = false;
    void document.fonts.ready.then(() => {
      if (!cancelled) autosize(editor);
    });
    return () => {
      cancelled = true;
    };
  });

  function setValue(next: string): void {
    value = next;
    onValueChange?.(next);
  }

  function submit(): void {
    // Busy never submits — stopping is deliberate, not an accidental Enter.
    if (isBusy || !actionEnabled) return;
    onSubmit?.(value);
  }

  function handleAction(): void {
    if (!actionEnabled) return;
    if (isBusy) {
      onStop?.();
      return;
    }
    submit();
  }

  function handleInput(event: Event): void {
    setValue((event.currentTarget as HTMLTextAreaElement).value);
  }

  function handleKeydown(event: KeyboardEvent): void {
    const intent = resolveSubmitIntent(
      {
        key: event.key,
        shiftKey: event.shiftKey,
        metaKey: event.metaKey,
        ctrlKey: event.ctrlKey,
        isComposing: isComposing || event.isComposing,
      },
      { submitOnEnter, isBusy },
    );
    if (intent === "submit") {
      event.preventDefault();
      submit();
      return;
    }
    if (intent === "stop") {
      event.preventDefault();
      onStop?.();
    }
  }

  function removeAttachment(id: string): void {
    if (disabled) return;
    onRemoveAttachment?.(id);
  }
</script>

<div
  class="poodle-agent-chat-input"
  data-size={resolvedSize}
  data-density={resolvedDensity}
  data-status={status}
  data-disabled={disabled}
>
  <div class="poodle-agent-chat-input__field">
    <!-- The question sits above the editor because that editor is its
         free-text override. Rendering it anywhere else would put a second text
         input on screen with different submit semantics. -->
    {#if isQuestioning && question}
      <div class="poodle-agent-chat-input__question">
        {@render question()}
      </div>
    {/if}

    <!-- The plan sits above the editor for the same reason the question does:
         it is input requiring the operator's attention. Unlike a question it
         does not block the turn — the editor keeps its ordinary semantics and
         a sent message is the revise channel. -->
    {#if isReviewingPlan && plan}
      <div class="poodle-agent-chat-input__plan">
        {@render plan()}
      </div>
    {/if}

    {#if attachments.length > 0}
      <ul class="poodle-agent-chat-input__attachments" aria-label="Attachments">
        {#each attachments as attachment (attachment.id)}
          <li
            class="poodle-agent-chat-input__attachment"
            data-kind={attachment.kind}
            data-variant={attachment.thumbnailUrl ? "thumbnail" : "chip"}
            title={attachment.thumbnailUrl ? attachment.label : undefined}
          >
            {#if attachment.thumbnailUrl}
              <img
                class="poodle-agent-chat-input__attachment-thumb"
                src={attachment.thumbnailUrl}
                alt={attachment.label}
              />
            {:else}
              {#if attachment.icon}
                <span class="poodle-agent-chat-input__attachment-icon">
                  <Icon name={attachment.icon} size="xs" />
                </span>
              {/if}
              <span class="poodle-agent-chat-input__attachment-label">{attachment.label}</span>
            {/if}
            <IconButton
              icon="x"
              ariaLabel={`Remove ${attachment.label}`}
              variant="ghost"
              size="xs"
              disabled={disabled || attachment.disabled}
              onClick={() => removeAttachment(attachment.id)}
            />
          </li>
        {/each}
      </ul>
    {/if}

    <textarea
      bind:this={editorElement}
      class="poodle-agent-chat-input__editor"
      aria-label={ariaLabel}
      placeholder={resolvedPlaceholder}
      rows={minRows}
      maxlength={maxLength ?? undefined}
      disabled={disabled}
      readonly={readOnly}
      value={value}
      oninput={handleInput}
      onkeydown={handleKeydown}
      oncompositionstart={() => (isComposing = true)}
      oncompositionend={() => (isComposing = false)}
    ></textarea>

    <div class="poodle-agent-chat-input__toolbar">
      <div class="poodle-agent-chat-input__leading" data-dividers={toolbarDividers}>
        {@render toolbar?.()}
      </div>
      <div class="poodle-agent-chat-input__trailing">
        {#if showContext}
          <span class="poodle-agent-chat-input__context">
            <Meter
              shape="ring"
              value={contextUsed ?? 0}
              max={contextLimit ?? 100}
              high={contextHigh}
              ariaLabel={contextAriaLabel}
              size={resolvedSize}
            />
          </span>
        {/if}
        <button
          type="button"
          class="poodle-agent-chat-input__action"
          data-state={actionState(isBusy)}
          aria-label={isBusy ? stopLabel : submitLabel}
          disabled={!actionEnabled}
          onclick={handleAction}
        >
          <Icon name={actionIcon(isBusy)} size="sm" />
        </button>
      </div>
    </div>
  </div>

  {#if footer}
    <div class="poodle-agent-chat-input__footer">
      {@render footer()}
    </div>
  {/if}
</div>
