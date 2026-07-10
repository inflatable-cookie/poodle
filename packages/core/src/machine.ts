/**
 * Shared machine model for the headless core.
 *
 * Machines are pure: a transition takes the current state + context and an
 * event, and returns the next state + context plus effect intents. Adapters
 * (Svelte, future React, GPUI/Jetstream via the Rust mirror) own reactivity
 * and execute effects. Callbacks are emitted as effects so transitions stay
 * side-effect free. See docs/specs/062-headless-core-and-dual-layer-strategy.md.
 */

export interface TransitionResult<S extends string, C, Eff> {
  state: S;
  context: C;
  effects: Eff[];
}

export type AttrValue = string | number | boolean | undefined;

/** Plain attribute map for one anatomy part; adapters spread it onto elements. */
export type PartAttrs = Record<string, AttrValue>;
