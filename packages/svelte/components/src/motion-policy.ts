import { getContext, setContext } from "svelte";
import { readable, writable, type Readable, type Writable } from "svelte/store";

import type { MotionPolicy } from "@inflatable-cookie/poodle-core";

const POODLE_MOTION_POLICY = Symbol("poodle-motion-policy");
const DEFAULT_MOTION_POLICY: MotionPolicy = "full";
const DEFAULT_MOTION_POLICY_STORE = readable(DEFAULT_MOTION_POLICY);

export function setMotionPolicy(value: MotionPolicy): Writable<MotionPolicy> {
  const store = writable(value);
  setContext(POODLE_MOTION_POLICY, store);
  return store;
}

export function getMotionPolicy(): Readable<MotionPolicy> {
  return getContext<Readable<MotionPolicy>>(POODLE_MOTION_POLICY) ?? DEFAULT_MOTION_POLICY_STORE;
}
