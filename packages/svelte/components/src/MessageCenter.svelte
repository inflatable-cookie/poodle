<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/message-center.css";
  import { default as Button } from "./Button.svelte";
  import { default as EmptyState } from "./EmptyState.svelte";
  import { default as Icon } from "./Icon.svelte";
  import { default as IconButton } from "./IconButton.svelte";
  import { default as Popover } from "./Popover.svelte";
  import { default as Progress } from "./Progress.svelte";
  import { default as TimeAgo } from "./TimeAgo.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type {
    ControlDensity,
    ControlSize,
    IconProp,
    MessageCenterItem,
    OverlayPlacement,
    SemanticControlSizeRole,
  } from "./types";

  interface Props {
    items?: MessageCenterItem[];
    open?: boolean | null;
    defaultOpen?: boolean;
    title?: string;
    ariaLabel?: string | null;
    triggerLabel?: string | null;
    triggerIcon?: IconProp;
    placement?: OverlayPlacement;
    emptyTitle?: string;
    emptyMessage?: string;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onOpenChange?: ((open: boolean) => void) | null;
    onItemSelect?: ((id: string) => void) | null;
    onReadChange?: ((id: string, read: boolean) => void) | null;
    onRemove?: ((id: string) => void) | null;
    onMarkAllRead?: (() => void) | null;
  }

  let {
    items = [],
    open = $bindable<boolean | null>(null),
    defaultOpen = false,
    title = "Notifications",
    ariaLabel = null,
    triggerLabel = null,
    triggerIcon = "bell",
    placement = "bottom-end",
    emptyTitle = "No messages",
    emptyMessage = "New messages will appear here.",
    size = null,
    sizeRole = "chrome",
    density = null,
    onOpenChange = null,
    onItemSelect = null,
    onReadChange = null,
    onRemove = null,
    onMarkAllRead = null,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  let uncontrolledOpen = $state(false);
  let seededDefaultOpen = $state(false);

  $effect.pre(() => {
    if (!seededDefaultOpen) {
      uncontrolledOpen = defaultOpen;
      seededDefaultOpen = true;
    }
  });
  const isOpen = $derived(open === null ? uncontrolledOpen : open);
  const unreadCount = $derived(items.filter((item) => !item.read).length);
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const resolvedTriggerLabel = $derived(
    triggerLabel ?? (unreadCount > 0 ? `${title}, ${unreadCount} unread` : title),
  );

  function handleOpenChange(next: boolean): void {
    if (open === null) uncontrolledOpen = next;
    else open = next;
    onOpenChange?.(next);
  }
</script>

<div class="poodle-message-center-popover">
  <Popover
    open={isOpen}
    {placement}
    initialFocus="content"
    triggerIsInteractive
    ariaLabel={ariaLabel ?? title}
    surfaceMinWidth="min(24rem, calc(100vw - 2rem))"
    surfaceMaxWidth="min(30rem, calc(100vw - 2rem))"
    onOpenChange={handleOpenChange}
  >
    {#snippet trigger()}
      <span class="poodle-message-center__trigger" data-unread={unreadCount > 0}>
        <IconButton
          icon={triggerIcon}
          ariaLabel={resolvedTriggerLabel}
          tooltip={title}
          variant="ghost"
          size={resolvedSize}
          density={resolvedDensity}
          expanded={isOpen}
        />
        {#if unreadCount > 0}
          <span class="poodle-message-center__indicator" aria-hidden="true">
            {unreadCount > 99 ? "99+" : unreadCount}
          </span>
        {/if}
      </span>
    {/snippet}

    <section
      class="poodle-message-center"
      data-size={resolvedSize}
      data-density={resolvedDensity}
      aria-label={ariaLabel ?? title}
    >
      <header class="poodle-message-center__header">
        <div>
          <h2>{title}</h2>
          {#if unreadCount > 0}
            <p>{unreadCount} unread</p>
          {/if}
        </div>
        {#if unreadCount > 0 && onMarkAllRead}
          <Button variant="ghost" size="xs" density={resolvedDensity} onClick={onMarkAllRead}>
            Mark all read
          </Button>
        {/if}
      </header>

      {#if items.length === 0}
        <div class="poodle-message-center__empty">
          <EmptyState title={emptyTitle} message={emptyMessage} size="compact" />
        </div>
      {:else}
        <ul class="poodle-message-center__list">
          {#each items as item (item.id)}
            <li class="poodle-message-center__item" data-read={item.read} data-tone={item.tone ?? "info"}>
              {#if onItemSelect && item.selectable !== false}
                <button
                  type="button"
                  class="poodle-message-center__content poodle-message-center__content--interactive"
                  aria-label={item.title}
                  onclick={() => onItemSelect?.(item.id)}
                >
                  {@render messageContent(item)}
                </button>
              {:else}
                <div class="poodle-message-center__content">
                  {@render messageContent(item)}
                </div>
              {/if}

              {#if (onReadChange && item.readControl !== false) || (onRemove && item.removable !== false)}
                <div class="poodle-message-center__actions">
                  {#if onReadChange && item.readControl !== false}
                    <IconButton
                      icon={item.read ? "mail" : "check"}
                      ariaLabel={item.read ? `Mark ${item.title} unread` : `Mark ${item.title} read`}
                      tooltip={item.read ? "Mark unread" : "Mark read"}
                      variant="ghost"
                      size="xs"
                      density={resolvedDensity}
                      onClick={() => onReadChange?.(item.id, !item.read)}
                    />
                  {/if}
                  {#if onRemove && item.removable !== false}
                    <IconButton
                      icon="trash-2"
                      ariaLabel={`Remove ${item.title}`}
                      tooltip="Remove"
                      variant="ghost"
                      tone="danger"
                      size="xs"
                      density={resolvedDensity}
                      onClick={() => onRemove?.(item.id)}
                    />
                  {/if}
                </div>
              {/if}
            </li>
          {/each}
        </ul>
      {/if}
    </section>
  </Popover>
</div>

{#snippet messageContent(item: MessageCenterItem)}
  <span class="poodle-message-center__leading" aria-hidden="true">
    {#if item.icon}
      <Icon icon={item.icon} size={resolvedSize} />
    {:else}
      <span class="poodle-message-center__read-dot"></span>
    {/if}
  </span>
  <span class="poodle-message-center__copy">
    <span class="poodle-message-center__title">{item.title}</span>
    {#if item.message}
      <span class="poodle-message-center__message">{item.message}</span>
    {/if}
    {#if item.meta || item.timestamp != null}
      <span class="poodle-message-center__meta">
        {#if item.meta}<span>{item.meta}</span>{/if}
        {#if item.meta && item.timestamp != null}<span aria-hidden="true">·</span>{/if}
        {#if item.timestamp != null}
          <TimeAgo datetime={item.timestamp} short typography="inherit" />
        {/if}
      </span>
    {/if}
    {#if item.progress}
      <Progress
        value={item.progress.value}
        max={item.progress.max ?? 100}
        indeterminate={item.progress.indeterminate ?? false}
        size="xs"
        ariaLabel={`${item.title} progress`}
      />
    {/if}
  </span>
{/snippet}
