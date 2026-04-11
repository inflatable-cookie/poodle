<script lang="ts">
  import { Eyebrow, Separator } from "@poodle/svelte";
  import type { ComponentDocs } from "../component-docs";

  export let docs: ComponentDocs;

  function formatDefault(val: string | undefined): string {
    if (val === undefined) return "—";
    return val;
  }
</script>

<div class="usage-docs">
  {#if docs.usage}
    <section class="usage-docs__section">
      <h2 class="usage-docs__heading">Usage</h2>
      <pre class="usage-docs__code"><code>{docs.usage}</code></pre>
    </section>
    <Separator />
  {/if}

  {#if docs.props.length > 0}
    <section class="usage-docs__section">
      <h2 class="usage-docs__heading">Props</h2>
      <div class="usage-docs__table-wrap">
        <table class="usage-docs__table">
          <thead>
            <tr>
              <th>Prop</th>
              <th>Type</th>
              <th>Default</th>
              <th>Description</th>
            </tr>
          </thead>
          <tbody>
            {#each docs.props as prop}
              <tr>
                <td class="usage-docs__prop-name">
                  {prop.name}{#if prop.required}<span class="usage-docs__required">*</span>{/if}
                </td>
                <td class="usage-docs__type"><code>{prop.type}</code></td>
                <td class="usage-docs__default"><code>{formatDefault(prop.default)}</code></td>
                <td class="usage-docs__description">{prop.description}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </section>
    <Separator />
  {/if}

  {#if docs.slots && docs.slots.length > 0}
    <section class="usage-docs__section">
      <h2 class="usage-docs__heading">Slots</h2>
      <div class="usage-docs__table-wrap">
        <table class="usage-docs__table">
          <thead>
            <tr>
              <th>Slot</th>
              <th>Description</th>
            </tr>
          </thead>
          <tbody>
            {#each docs.slots as slot}
              <tr>
                <td class="usage-docs__prop-name">{slot.name}</td>
                <td class="usage-docs__description">{slot.description}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </section>
    <Separator />
  {/if}

  {#if docs.events && docs.events.length > 0}
    <section class="usage-docs__section">
      <h2 class="usage-docs__heading">Events</h2>
      <div class="usage-docs__table-wrap">
        <table class="usage-docs__table">
          <thead>
            <tr>
              <th>Event</th>
              <th>Payload</th>
              <th>Description</th>
            </tr>
          </thead>
          <tbody>
            {#each docs.events as event}
              <tr>
                <td class="usage-docs__prop-name">{event.name}</td>
                <td class="usage-docs__type"><code>{event.payload}</code></td>
                <td class="usage-docs__description">{event.description}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </section>
  {/if}
</div>

<style>
  .usage-docs {
    display: flex;
    flex-direction: column;
  }

  .usage-docs__section {
    padding: 1.5rem 0;
  }

  .usage-docs__heading {
    font-size: 1.125rem;
    font-weight: 600;
    color: var(--poodle-color-text-primary);
    margin: 0 0 1rem;
  }

  .usage-docs__code {
    padding: 0.75rem 1rem;
    border-radius: var(--poodle-radius-surface);
    background: color-mix(in srgb, var(--poodle-color-background-canvas) 90%, transparent);
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 50%, transparent);
    font-family: "SF Mono", "Fira Code", monospace;
    font-size: 0.8125rem;
    line-height: 1.6;
    color: var(--poodle-color-text-primary);
    overflow-x: auto;
    margin: 0;
    white-space: pre;
  }

  .usage-docs__table-wrap {
    overflow-x: auto;
  }

  .usage-docs__table {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  .usage-docs__table th {
    text-align: left;
    font-weight: 600;
    color: var(--poodle-color-text-secondary);
    padding: 0.5rem 0.75rem;
    border-bottom: 0.0625rem solid var(--poodle-color-border-subtle);
    white-space: nowrap;
    font-size: 0.6875rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .usage-docs__table td {
    padding: 0.5rem 0.75rem;
    border-bottom: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 40%, transparent);
    color: var(--poodle-color-text-primary);
    vertical-align: top;
  }

  .usage-docs__table td.usage-docs__prop-name {
    white-space: nowrap;
    font-weight: 500;
  }

  .usage-docs__table td.usage-docs__type {
    white-space: nowrap;
    max-width: 14rem;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .usage-docs__table td.usage-docs__default {
    white-space: nowrap;
  }

  .usage-docs__table td.usage-docs__description {
    min-width: 12rem;
    word-break: break-word;
  }

  .usage-docs__table tbody tr:last-child td {
    border-bottom: none;
  }

  .usage-docs__prop-name {
    font-family: "SF Mono", "Fira Code", monospace;
    font-weight: 500;
    white-space: nowrap;
    color: var(--poodle-color-text-primary);
  }

  .usage-docs__required {
    color: var(--poodle-color-text-danger, #ef4444);
    margin-left: 0.125rem;
  }

  .usage-docs__type code,
  .usage-docs__default code {
    font-family: "SF Mono", "Fira Code", monospace;
    font-size: 0.75rem;
    padding: 0.0625rem 0.25rem;
    border-radius: 0.1875rem;
    background: color-mix(in srgb, var(--poodle-color-background-canvas) 80%, transparent);
    color: var(--poodle-color-text-secondary);
    white-space: nowrap;
  }

  .usage-docs__description {
    color: var(--poodle-color-text-secondary);
  }
</style>
