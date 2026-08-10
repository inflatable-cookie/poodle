# Native Architecture Documentation Reconciliation

Date: 2026-08-10

## Scope

Audit active contracts and contributor instructions for pre-g12.019 native
component-tier claims.

## Findings

- `CLAUDE.md` still instructed contributors to add components to the deleted
  Jetstream-specific implementation tier.
- Native accessibility and ToolCall contracts cited deleted source paths as
  current evidence.
- AgentChatInput still documented the deleted `js_agent_chat_input` entry
  point, and live Rust crate docs described the migration as unfinished.
- ToastHost, MediaPicker, MediaBrowsePanel, and MarkdownEditor contracts said
  GPUI was unimplemented despite their shared `poodle-render` implementations.
- Active contracts had no gate against retired native paths or bare execution
  backlog statements.
- `effigy health` ran only `docs:check` despite being documented as the full
  repository health gate.

## Changes

- Replaced the stale contributor manual with a short pointer to `AGENTS.md` and
  current architecture authorities.
- Reconciled affected contracts with the shared renderer and node-backend
  architecture.
- Added documentation lint rules for retired native-tier paths and execution
  status inside component contracts.
- Expanded `effigy health` to the web, Rust, native, package-install, license,
  and security gates. Display-dependent visual sweeps remain separate.
- The expanded gate found and closed a native RemediationBanner event gap:
  action and dismiss controls now reach typed shared-renderer handlers, with a
  focused activation test.
- Kept historical migration comments and project records intact.

## Validation

- `effigy health`
- `effigy docs:check`
- `effigy parity:check`
- `git diff --check`
