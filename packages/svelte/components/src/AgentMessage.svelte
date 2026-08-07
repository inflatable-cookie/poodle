<script lang="ts">
  import "@poodle/styles/agent-message.css";

  import { blocksFromMarked, type MarkedToken, type MdBlock, type MdInline } from "@poodle/headless";
  import { marked } from "marked";

  import Code from "./Code.svelte";
  import Separator from "./Separator.svelte";
  import TextLink from "./TextLink.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation.ts";

  import type {
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
    TranscriptRole,
  } from "./types.ts";

  interface Props {
    markdown?: string;
    role?: TranscriptRole;
    isStreaming?: boolean;
    linkTarget?: string | null;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onLinkClick?: ((href: string) => void) | undefined;
  }

  let {
    markdown = "",
    role = "assistant",
    isStreaming = false,
    linkTarget = null,
    size = null,
    sizeRole = "control",
    density = null,
    onLinkClick = undefined,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);

  /**
   * Derived, never cached across changes.
   *
   * A streaming message reparses on every append, which is correct and cheap at
   * message scale. An incremental parser would have to reason about a half-open
   * fence, and getting that wrong renders the rest of the message as code.
   */
  const blocks = $derived(blocksFromMarked(marked.lexer(markdown) as unknown as MarkedToken[]));

  function handleLink(event: MouseEvent, href: string): void {
    if (!onLinkClick) return;
    event.preventDefault();
    onLinkClick(href);
  }
</script>

{#snippet inlines(nodes: MdInline[])}
  {#each nodes as node}
    {#if node.type === "text"}{node.value}
    {:else if node.type === "code"}<code class="poodle-agent-message__code-span">{node.value}</code>
    {:else if node.type === "strong"}<strong>{@render inlines(node.children)}</strong>
    {:else if node.type === "em"}<em>{@render inlines(node.children)}</em>
    {:else if node.type === "del"}<del>{@render inlines(node.children)}</del>
    {:else if node.type === "link"}<TextLink
        href={node.href}
        target={linkTarget}
        onClick={(event: MouseEvent) => handleLink(event, node.href)}
      >{@render inlines(node.children)}</TextLink>
    {:else if node.type === "break"}<br />
    {/if}
  {/each}
{/snippet}

{#snippet blockList(list: MdBlock[])}
  <div class="poodle-agent-message__body">
    {#each list as block}
      {#if block.type === "paragraph"}
        <p class="poodle-agent-message__paragraph">{@render inlines(block.children)}</p>
      {:else if block.type === "heading"}
        <svelte:element
          this={`h${block.level}`}
          class="poodle-agent-message__heading"
          data-level={block.level}
        >{@render inlines(block.children)}</svelte:element>
      {:else if block.type === "code"}
        <Code source={block.value} language={block.lang} size={resolvedSize} />
      {:else if block.type === "list"}
        <svelte:element
          this={block.ordered ? "ol" : "ul"}
          class="poodle-agent-message__list"
          start={block.ordered && block.start !== 1 ? block.start : undefined}
        >
          {#each block.items as item}
            <!-- Single-paragraph items render their inlines directly: the
                 marker sits on the text line with inside positioning, and a
                 redundant <p> would push the content below the marker. -->
            <li class="poodle-agent-message__list-item">
              {#if item.length === 1 && item[0].type === "paragraph"}
                {@render inlines(item[0].children)}
              {:else}
                {@render blockList(item)}
              {/if}
            </li>
          {/each}
        </svelte:element>
      {:else if block.type === "blockquote"}
        <blockquote class="poodle-agent-message__quote">{@render blockList(block.children)}</blockquote>
      {:else if block.type === "rule"}
        <Separator />
      {/if}
    {/each}
  </div>
{/snippet}

<!-- An empty message contributes no box: a turn with nothing in it should not
     reserve space in the transcript. -->
{#if blocks.length > 0 || isStreaming}
  <div
    class="poodle-agent-message"
    data-role={role}
    data-streaming={isStreaming ? "true" : undefined}
    data-size={resolvedSize}
    data-density={resolvedDensity}
  >
    {@render blockList(blocks)}
    <!-- A progress hint, not content: announcing it would read "block, cursor"
         after every partial sentence. -->
    {#if isStreaming}
      <span class="poodle-agent-message__caret" aria-hidden="true"></span>
    {/if}
  </div>
{/if}
