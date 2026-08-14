import { useState } from "react";

import "@inflatable-cookie/poodle-core/styles/message-center.css";

import { Button } from "./Button";
import { EmptyState } from "./EmptyState";
import { Icon } from "./Icon";
import { IconButton } from "./IconButton";
import { Popover } from "./Popover";
import { Progress } from "./Progress";
import { TimeAgo } from "./TimeAgo";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type {
  ControlDensity,
  ControlSize,
  IconProp,
  MessageCenterItem,
  OverlayPlacement,
  SemanticControlSizeRole,
} from "./types";

export interface MessageCenterProps {
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

export function MessageCenter({
  items = [],
  open = null,
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
}: MessageCenterProps) {
  const uiPresentation = useUiPresentation();
  const [uncontrolledOpen, setUncontrolledOpen] = useState(defaultOpen);
  const isOpen = open === null ? uncontrolledOpen : open;
  const unreadCount = items.filter((item) => !item.read).length;
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const resolvedTriggerLabel = triggerLabel ?? (unreadCount > 0 ? `${title}, ${unreadCount} unread` : title);

  function handleOpenChange(next: boolean): void {
    if (open === null) setUncontrolledOpen(next);
    onOpenChange?.(next);
  }

  function messageContent(item: MessageCenterItem) {
    return (
      <>
        <span className="poodle-message-center__leading" aria-hidden="true">
          {item.icon ? <Icon icon={item.icon} size={resolvedSize} /> : <span className="poodle-message-center__read-dot" />}
        </span>
        <span className="poodle-message-center__copy">
          <span className="poodle-message-center__title">{item.title}</span>
          {item.message ? <span className="poodle-message-center__message">{item.message}</span> : null}
          {item.meta || item.timestamp != null ? (
            <span className="poodle-message-center__meta">
              {item.meta ? <span>{item.meta}</span> : null}
              {item.meta && item.timestamp != null ? <span aria-hidden="true">·</span> : null}
              {item.timestamp != null ? <TimeAgo datetime={item.timestamp} short typography="inherit" /> : null}
            </span>
          ) : null}
          {item.progress ? (
            <Progress
              value={item.progress.value}
              max={item.progress.max ?? 100}
              indeterminate={item.progress.indeterminate ?? false}
              size="xs"
              ariaLabel={`${item.title} progress`}
            />
          ) : null}
        </span>
      </>
    );
  }

  return (
    <div className="poodle-message-center-popover">
      <Popover
        open={isOpen}
        placement={placement}
        initialFocus="content"
        triggerIsInteractive
        ariaLabel={ariaLabel ?? title}
        surfaceMinWidth="min(24rem, calc(100vw - 2rem))"
        surfaceMaxWidth="min(30rem, calc(100vw - 2rem))"
        onOpenChange={handleOpenChange}
        trigger={
          <span className="poodle-message-center__trigger" data-unread={unreadCount > 0}>
            <IconButton
              icon={triggerIcon}
              ariaLabel={resolvedTriggerLabel}
              tooltip={title}
              variant="ghost"
              size={resolvedSize}
              density={resolvedDensity}
              expanded={isOpen}
            />
            {unreadCount > 0 ? (
              <span className="poodle-message-center__indicator" aria-hidden="true">
                {unreadCount > 99 ? "99+" : unreadCount}
              </span>
            ) : null}
          </span>
        }
      >
        <section className="poodle-message-center" data-size={resolvedSize} data-density={resolvedDensity} aria-label={ariaLabel ?? title}>
          <header className="poodle-message-center__header">
            <div>
              <h2>{title}</h2>
              {unreadCount > 0 ? <p>{unreadCount} unread</p> : null}
            </div>
            {unreadCount > 0 && onMarkAllRead ? (
              <Button variant="ghost" size="xs" density={resolvedDensity} onClick={onMarkAllRead}>
                Mark all read
              </Button>
            ) : null}
          </header>

          {items.length === 0 ? (
            <div className="poodle-message-center__empty">
              <EmptyState title={emptyTitle} message={emptyMessage} size="compact" />
            </div>
          ) : (
            <ul className="poodle-message-center__list">
              {items.map((item) => {
                const canSelect = onItemSelect !== null && item.selectable !== false;
                const canRead = onReadChange !== null && item.readControl !== false;
                const canRemove = onRemove !== null && item.removable !== false;
                return (
                <li key={item.id} className="poodle-message-center__item" data-read={item.read} data-tone={item.tone ?? "info"}>
                  {canSelect ? (
                    <button
                      type="button"
                      className="poodle-message-center__content poodle-message-center__content--interactive"
                      aria-label={item.title}
                      onClick={() => onItemSelect(item.id)}
                    >
                      {messageContent(item)}
                    </button>
                  ) : (
                    <div className="poodle-message-center__content">{messageContent(item)}</div>
                  )}

                  {canRead || canRemove ? (
                    <div className="poodle-message-center__actions">
                      {canRead ? (
                        <IconButton
                          icon={item.read ? "mail" : "check"}
                          ariaLabel={item.read ? `Mark ${item.title} unread` : `Mark ${item.title} read`}
                          tooltip={item.read ? "Mark unread" : "Mark read"}
                          variant="ghost"
                          size="xs"
                          density={resolvedDensity}
                          onClick={() => onReadChange(item.id, !item.read)}
                        />
                      ) : null}
                      {canRemove ? (
                        <IconButton
                          icon="trash-2"
                          ariaLabel={`Remove ${item.title}`}
                          tooltip="Remove"
                          variant="ghost"
                          tone="danger"
                          size="xs"
                          density={resolvedDensity}
                          onClick={() => onRemove(item.id)}
                        />
                      ) : null}
                    </div>
                  ) : null}
                </li>
                );
              })}
            </ul>
          )}
        </section>
      </Popover>
    </div>
  );
}
