import { Fragment, type ReactNode, useMemo } from "react";

import "@inflatable-cookie/poodle-styles/agent-message.css";

import { blocksFromMarked, type MarkedToken, type MdBlock, type MdInline } from "@inflatable-cookie/poodle-headless";
import { marked } from "marked";

import { Code } from "./Code";
import { Separator } from "./Separator";
import { TextLink } from "./TextLink";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type {
  ControlDensity,
  ControlSize,
  SemanticControlSizeRole,
  TranscriptRole,
} from "./types";

export interface AgentMessageProps {
  markdown?: string;
  role?: TranscriptRole;
  isStreaming?: boolean;
  linkTarget?: string | null;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onLinkClick?: (href: string) => void;
}

export function AgentMessage({
  markdown = "",
  role = "assistant",
  isStreaming = false,
  linkTarget = null,
  size = null,
  sizeRole = "control",
  density = null,
  onLinkClick,
}: AgentMessageProps) {
  const presentation = useUiPresentation();
  const resolvedSize = size ?? resolveSemanticControlSize(presentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? presentation.density;

  /**
   * Derived from `markdown`, never cached across changes.
   *
   * A streaming message reparses on every append, which is correct and cheap at
   * message scale. An incremental parser would have to reason about a half-open
   * fence, and getting that wrong renders the rest of the message as code.
   */
  const blocks = useMemo(
    () => blocksFromMarked(marked.lexer(markdown) as unknown as MarkedToken[]),
    [markdown],
  );

  const renderInlines = (nodes: MdInline[]): ReactNode =>
    nodes.map((node, index) => {
      switch (node.type) {
        case "text":
          return <Fragment key={index}>{node.value}</Fragment>;
        case "code":
          return (
            <code key={index} className="poodle-agent-message__code-span">
              {node.value}
            </code>
          );
        case "strong":
          return <strong key={index}>{renderInlines(node.children)}</strong>;
        case "em":
          return <em key={index}>{renderInlines(node.children)}</em>;
        case "del":
          return <del key={index}>{renderInlines(node.children)}</del>;
        case "link":
          return (
            <TextLink
              key={index}
              href={node.href}
              target={linkTarget}
              onClick={(event) => {
                if (!onLinkClick) return;
                event.preventDefault();
                onLinkClick(node.href);
              }}
            >
              {renderInlines(node.children)}
            </TextLink>
          );
        case "break":
          return <br key={index} />;
      }
    });

  const renderBlocks = (list: MdBlock[]): ReactNode => (
    <div className="poodle-agent-message__body">
      {list.map((block, index) => {
        switch (block.type) {
          case "paragraph":
            return (
              <p key={index} className="poodle-agent-message__paragraph">
                {renderInlines(block.children)}
              </p>
            );
          case "heading": {
            const Heading = `h${block.level}` as "h1";
            return (
              <Heading key={index} className="poodle-agent-message__heading" data-level={block.level}>
                {renderInlines(block.children)}
              </Heading>
            );
          }
          case "code":
            return <Code key={index} source={block.value} language={block.lang} size={resolvedSize} />;
          case "list": {
            const List = block.ordered ? "ol" : "ul";
            return (
              <List
                key={index}
                className="poodle-agent-message__list"
                start={block.ordered && block.start !== 1 ? block.start : undefined}
              >
                {block.items.map((item, itemIndex) => (
                  <li key={itemIndex} className="poodle-agent-message__list-item">
                    {renderBlocks(item)}
                  </li>
                ))}
              </List>
            );
          }
          case "blockquote":
            return (
              <blockquote key={index} className="poodle-agent-message__quote">
                {renderBlocks(block.children)}
              </blockquote>
            );
          case "rule":
            return <Separator key={index} />;
        }
      })}
    </div>
  );

  // An empty message contributes no box: a turn with nothing in it should not
  // reserve space in the transcript.
  if (blocks.length === 0 && !isStreaming) return null;

  return (
    <div
      className="poodle-agent-message"
      data-role={role}
      data-streaming={isStreaming ? "true" : undefined}
      data-size={resolvedSize}
      data-density={resolvedDensity}
    >
      {renderBlocks(blocks)}
      {/* A progress hint, not content: announcing it would read "block, cursor"
          after every partial sentence. */}
      {isStreaming ? <span className="poodle-agent-message__caret" aria-hidden="true" /> : null}
    </div>
  );
}
