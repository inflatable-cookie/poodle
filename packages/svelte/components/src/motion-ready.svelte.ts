import { bindMotionReady, type MotionPolicy } from "@inflatable-cookie/poodle-core";

import { getMotionPolicy } from "./motion-policy";

export function useMotionReady(enabled: boolean | (() => boolean) = true): {
  get ready(): boolean;
  get policy(): MotionPolicy;
} {
  const policyStore = getMotionPolicy();
  let ready = $state(false);
  let currentPolicy = $state<MotionPolicy>("full");

  $effect(() => {
    const allow = typeof enabled === "function" ? enabled() : enabled;
    let stopReady = () => {};
    const unsubscribe = policyStore.subscribe((value) => {
      currentPolicy = value;
      stopReady();
      stopReady = bindMotionReady(value, allow, (next) => {
        ready = next;
      });
    });
    return () => {
      unsubscribe();
      stopReady();
    };
  });

  return {
    get ready() {
      return ready;
    },
    get policy() {
      return currentPolicy;
    },
  };
}
