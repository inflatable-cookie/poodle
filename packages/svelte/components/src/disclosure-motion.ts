import { tick } from "svelte";

import {
  cancelWebMotion,
  motionKey,
  playClippedHeight,
  type MotionPolicy,
} from "@inflatable-cookie/poodle-core";

export interface ClippedHeightActionState {
  owner: string;
  open: boolean;
  policy: MotionPolicy;
  ready: boolean;
  onCloseFinished?: () => void;
}

/**
 * Drive the real disclosure clip through the shared keyed runtime. The first
 * action update is an authored endpoint; later controlled updates can reverse
 * the live height animation without reintroducing CSS lifecycle events.
 *
 * Opens measure after the pending flush: this action's update runs before the
 * inner content's `hidden` binding clears (document order), so reading
 * `scrollHeight` synchronously would animate 0px to 0px. `tick()` waits out
 * the flush so the content is laid out before the open target is measured.
 */
export function clippedHeight(node: HTMLElement, initial: ClippedHeightActionState) {
  let current = initial;
  let firstMotion = true;
  let lastState: string | null = null;
  let destroyed = false;

  function play(state: ClippedHeightActionState, shouldAnimate: boolean): void {
    playClippedHeight(node, {
      owner: state.owner,
      open: state.open,
      policy: state.policy,
      initial: !shouldAnimate,
      onComplete: (status) => {
        if (status === "finish" && !state.open) {
          state.onCloseFinished?.();
        }
      },
    });
  }

  function update(next: ClippedHeightActionState): void {
    current = next;
    const state = `${next.owner}:${next.open ? "open" : "closed"}:${next.policy}`;
    if (state === lastState) {
      return;
    }
    lastState = state;
    const shouldAnimate = next.ready && !firstMotion;
    firstMotion = false;
    if (next.open && shouldAnimate) {
      const captured = next;
      void tick().then(() => {
        if (destroyed || current !== captured) {
          return;
        }
        play(captured, true);
      });
      return;
    }
    play(next, shouldAnimate);
  }

  update(initial);

  return {
    update,
    destroy() {
      destroyed = true;
      cancelWebMotion(motionKey(current.owner, "disclosure-height", "panel"));
    },
  };
}
