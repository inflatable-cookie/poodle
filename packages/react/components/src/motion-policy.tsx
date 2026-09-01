import { createContext, useContext, useEffect, useMemo, useState, type ReactNode } from "react";

import "@inflatable-cookie/poodle-core/styles/motion-policy-provider.css";
import { bindMotionReady, restrictMotionPolicy, type MotionPolicy } from "@inflatable-cookie/poodle-core";

const DEFAULT_MOTION_POLICY: MotionPolicy = "full";
const MotionPolicyContext = createContext<MotionPolicy>(DEFAULT_MOTION_POLICY);

export function MotionPolicyProvider({
  policy = "full",
  children,
}: {
  policy?: MotionPolicy;
  children: ReactNode;
}) {
  const ancestor = useContext(MotionPolicyContext);
  const effective = useMemo(
    () => restrictMotionPolicy(ancestor, policy),
    [ancestor, policy],
  );

  return (
    <MotionPolicyContext.Provider value={effective}>
      <div className="poodle-motion-policy-provider" data-poodle-motion-policy={effective}>
        {children}
      </div>
    </MotionPolicyContext.Provider>
  );
}

export function useMotionPolicy(): MotionPolicy {
  return useContext(MotionPolicyContext);
}

export function useMotionReady(enabled = true): boolean {
  const policy = useMotionPolicy();
  const [ready, setReady] = useState(false);
  useEffect(() => bindMotionReady(policy, enabled, setReady), [policy, enabled]);
  return ready;
}
