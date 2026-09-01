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
 */
export function clippedHeight(node: HTMLElement, initial: ClippedHeightActionState) {
  let current = initial;
  let firstMotion = true;
  let lastState: string | null = null;

  function update(next: ClippedHeightActionState): void {
    current = next;
    const state = `${next.owner}:${next.open ? "open" : "closed"}:${next.policy}`;
    if (state === lastState) {
      return;
    }
    lastState = state;
    const shouldAnimate = next.ready && !firstMotion;
    playClippedHeight(node, {
      owner: next.owner,
      open: next.open,
      policy: next.policy,
      initial: !shouldAnimate,
      onComplete: (status) => {
        if (status === "finish" && !next.open) {
          next.onCloseFinished?.();
        }
      },
    });
    firstMotion = false;
  }

  update(initial);

  return {
    update,
    destroy() {
      cancelWebMotion(motionKey(current.owner, "disclosure-height", "panel"));
    },
  };
}
