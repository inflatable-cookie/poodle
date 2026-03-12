# EmbedShell

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `EmbedShell`
- Layer: `composites`
- Summary: a framed shell for embedded host-native or external content with explicit fallback posture
- In scope: local title/context, provider label, framed embed viewport, loading/error/empty states, and optional footer actions
- Out of scope: origin policy, authentication, sandboxing, permission prompts, or choosing the actual embedded runtime

## 2. Accessibility

- embeds must keep a visible title and context outside the viewport itself
- loading and failure states must preserve the framed region instead of removing structure
- fallback and recovery actions must remain explicit when embedded content is unavailable
- GPUI-native accessibility mapping notes: GPUI must recreate equivalent framed-region and fallback meaning even where the embedded destination is not web-based

## 3. Next Task

Use `EmbedShell` anywhere framed embedded destinations need a stable fallback contract instead of letting embeds collapse to raw iframes or blank native panels.
