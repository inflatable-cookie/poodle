<script lang="ts">
  import "@poodle/styles/agent-question-record.css";

  import {
    answeredQuestionSummary,
    isChosenOption,
    type AgentQuestionAnswer,
    type AgentQuestionItem,
  } from "@poodle/headless";

  import Eyebrow from "./Eyebrow.svelte";
  import Icon from "./Icon.svelte";
  import {
    getUiPresentation,
    resolveSemanticControlSize,
    resolveSupportingVisualSize,
  } from "./presentation";

  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  interface Props {
    question: AgentQuestionItem;
    answer: AgentQuestionAnswer;
    showOptions?: boolean;
    declinedLabel?: string;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
  }

  let {
    question,
    answer,
    showOptions = true,
    declinedLabel = "Declined",
    size = null,
    sizeRole = "control",
    density = null,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const glyphSize = $derived(resolveSupportingVisualSize(resolvedSize));

  const record = $derived({ question, answer });
  const showsOptions = $derived(showOptions && answer.outcome === "selected");
  const summary = $derived(answeredQuestionSummary(record));
</script>

<div
  class="poodle-agent-question-record"
  data-outcome={answer.outcome}
  data-size={resolvedSize}
  data-density={resolvedDensity}
>
  {#if question.header}
    <Eyebrow>{question.header}</Eyebrow>
  {/if}

  <p class="poodle-agent-question-record__prompt">{question.prompt}</p>

  {#if showsOptions}
    <ul class="poodle-agent-question-record__options">
      {#each question.options as option (option.value)}
        {@const chosen = isChosenOption(record, option.value)}
        <!-- The tick alone is not the signal: the chosen option says so in its
             accessible name too. -->
        <li
          class="poodle-agent-question-record__option"
          data-chosen={String(chosen)}
          aria-label={chosen ? `chosen: ${option.label}` : option.label}
        >
          <span class="poodle-agent-question-record__option-mark" aria-hidden="true">
            <Icon name="check" size={glyphSize} />
          </span>
          <span class="poodle-agent-question-record__option-label">{option.label}</span>
        </li>
      {/each}
    </ul>
  {:else}
    <p class="poodle-agent-question-record__answer">
      {answer.outcome === "declined" ? declinedLabel : summary}
    </p>
  {/if}
</div>
