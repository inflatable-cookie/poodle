<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/validation-summary.css";
  import type { AnnouncementMode, ValidationSummaryEntry } from "./types";

  let { title = null, entries = [], announceMode = "polite", includePending = false }: {
    title?: string | null;
    entries?: ValidationSummaryEntry[];
    announceMode?: AnnouncementMode;
    includePending?: boolean;
  } = $props();

  const activeEntries = $derived(entries.filter((entry) =>
    entry.validationState === "invalid" || (includePending && entry.validationState === "pending")
  ));
  const hasBlockingEntries = $derived(activeEntries.some((entry) => entry.validationState === "invalid"));
  const role = $derived(announceMode === "assertive" ? "alert" : announceMode === "polite" ? "status" : undefined);
  const ariaLive = $derived(announceMode === "none" ? undefined : announceMode);
</script>

{#if activeEntries.length > 0}
  <div class="poodle-validation-summary" data-state={hasBlockingEntries ? "blocking" : "pending"} {role} aria-live={ariaLive}>
    {#if title}<strong class="poodle-validation-summary__title">{title}</strong>{/if}
    <ul class="poodle-validation-summary__list">
      {#each activeEntries as entry (entry.fieldId)}
        <li class="poodle-validation-summary__entry" data-state={entry.validationState}>
          <a href={`#${entry.fieldId}`}>{entry.label}</a>
          <span>{entry.message}</span>
        </li>
      {/each}
    </ul>
  </div>
{/if}
