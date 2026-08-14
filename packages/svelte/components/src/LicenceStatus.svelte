<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/licence.css";
  import {
    licenceStatusView,
    type LicenceAttention,
    type LicenceTrustBasis,
    type LicenceUsability,
  } from "@inflatable-cookie/poodle-core";

  import { default as StatusIndicator } from "./StatusIndicator.svelte";
  import { default as TimeAgo } from "./TimeAgo.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize } from "./types";

  interface Props {
    usability: LicenceUsability;
    trustBasis: LicenceTrustBasis;
    useUntil: number | null;
    updateUntil: number | null;
    usable: boolean;
    attention: LicenceAttention;
    title?: string;
    size?: ControlSize | null;
    density?: ControlDensity | null;
  }

  let {
    usability,
    trustBasis,
    useUntil,
    updateUntil,
    usable,
    attention,
    title = "Licence",
    size = null,
    density = null,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, "control"));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);

  /* Every display decision — title, tone, row terms, which timestamp goes
     where — is made once in poodle-core so this shell and its React twin
     cannot disagree about what a licence state means. */
  const view = $derived(
    licenceStatusView({ usability, trustBasis, useUntil, updateUntil, usable, attention }),
  );
</script>

<!-- `usable` and `attention` are authority reads. They reach the DOM as data
     state and nothing else: no branch below hides a row, disables a control, or
     turns a licence read into a feature permission. -->
<section
  class="poodle-licence-status"
  aria-label={title}
  data-state={view.state}
  data-tone={view.tone}
  data-attention={view.attention}
  data-usable={view.usable}
  data-size={resolvedSize}
  data-density={resolvedDensity}
>
  <div class="poodle-licence-status__head">
    <StatusIndicator status={view.indicator} size={resolvedSize} density={resolvedDensity} />
    <h3 class="poodle-licence-status__title">{view.title}</h3>
  </div>

  <p class="poodle-licence-status__body">
    {view.body.text}{#if view.body.timestamp !== null}&nbsp;<TimeAgo
        datetime={view.body.timestamp}
        typography="inherit"
      />{/if}
  </p>

  <!-- Use coverage, update coverage and trust basis are three labelled values,
       never merged. A single "expires" line is how someone with lapsed updates
       is told they have lost the software they own. -->
  <dl class="poodle-licence-status__coverage">
    {#each view.coverage as row (row.id)}
      <dt class="poodle-licence-status__term">{row.term}</dt>
      <dd class="poodle-licence-status__value" data-row={row.id}>
        {#if row.timestamp !== null}
          <TimeAgo datetime={row.timestamp} typography="inherit" />
        {:else}
          {row.text}
        {/if}
      </dd>
    {/each}
    <dt class="poodle-licence-status__term">{view.trust.term}</dt>
    <dd class="poodle-licence-status__value" data-row="trust">
      {view.trust.text}{#if view.trust.timestamp !== null}&nbsp;<TimeAgo
          datetime={view.trust.timestamp}
          typography="inherit"
        />{/if}
    </dd>
  </dl>

  {#if view.detail}
    <p class="poodle-licence-status__detail">
      {view.detail.text}{#if view.detail.timestamp !== null}&nbsp;<TimeAgo
          datetime={view.detail.timestamp}
          typography="inherit"
        />{/if}
    </p>
  {/if}
</section>
