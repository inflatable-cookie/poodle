import { useEffect, useLayoutEffect, useRef, type RefObject } from "react";

import {
  cancelWebMotion,
  motionKey,
  playClippedHeight,
  type MotionPolicy,
} from "@inflatable-cookie/poodle-core";

export function useClippedHeightMotion({
  owner,
  open,
  policy,
  ready,
  onCloseFinished,
}: {
  owner: string;
  open: boolean;
  policy: MotionPolicy;
  ready: boolean;
  onCloseFinished?: () => void;
}): RefObject<HTMLDivElement | null> {
  const elementRef = useRef<HTMLDivElement | null>(null);
  const firstMotion = useRef(true);
  const lastState = useRef<string | null>(null);
  const closeFinishedRef = useRef(onCloseFinished);
  closeFinishedRef.current = onCloseFinished;
  const key = motionKey(owner, "disclosure-height", "panel");

  useLayoutEffect(() => {
    const element = elementRef.current;
    if (!element) {
      return;
    }
    const state = `${owner}:${open ? "open" : "closed"}:${policy}`;
    if (state === lastState.current) {
      return;
    }
    lastState.current = state;
    const shouldAnimate = ready && !firstMotion.current;
    playClippedHeight(element, {
      owner,
      open,
      policy,
      initial: !shouldAnimate,
      onComplete: (status) => {
        if (status === "finish" && !open) {
          closeFinishedRef.current?.();
        }
      },
    });
    firstMotion.current = false;
  }, [owner, open, policy, ready]);

  useEffect(() => () => cancelWebMotion(key), [key]);

  return elementRef;
}
