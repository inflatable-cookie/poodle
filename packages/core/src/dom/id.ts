/**
 * Instance-scoped id wiring for part relationships (trigger/surface,
 * tab/panel). Deterministic per module instance, matching the previous
 * per-component module counters.
 */

const counters = new Map<string, number>();

export function createInstanceId(scope: string): string {
  const next = (counters.get(scope) ?? 0) + 1;
  counters.set(scope, next);
  return `poodle-${scope}-${next}`;
}
