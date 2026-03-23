import { underlayWrapperPolicies, underlayZeroLeakRules } from "./component-wrappers";
import { underlayThemeMap } from "./theme-map";
import { underlayTokenMap } from "./token-map";

export type UnderlayAdoptionSurfaceProof = {
  underlayExport: string;
  bridgeOwner: "underlay-bridge";
  appImportsFlintDirectly: false;
  appFacingImport: string;
  usesBridgeTokenAliases: boolean;
  usesBridgeThemeRegistration: boolean;
  wrapperPolicyNote: string;
};

export type UnderlayZeroLeakProof = {
  packageName: "@flint/bridge-underlay";
  proofVersion: 1;
  zeroLeakRules: readonly string[];
  directFlintImportsAllowedInApps: false;
  canonicalDependencyShape: "bridge-owned";
  proofSurfaces: UnderlayAdoptionSurfaceProof[];
  remainingFriction: string[];
};

export const underlayAdoptionSurfaceProof: UnderlayAdoptionSurfaceProof[] = underlayWrapperPolicies.map(
  (policy) => ({
    underlayExport: policy.underlayExport,
    bridgeOwner: "underlay-bridge",
    appImportsFlintDirectly: false,
    appFacingImport: `@underlay/ui/${policy.underlayExport}`,
    usesBridgeTokenAliases: true,
    usesBridgeThemeRegistration: true,
    wrapperPolicyNote: policy.notes,
  }),
);

export const underlayRemainingAdoptionFriction = [
  "Underlay-owned theme registration names are still placeholders and should be replaced by real Underlay canonical IDs during adoption.",
  "Wrapper prop translation remains policy-only until the first Underlay-owned wrapper package consumes the bridge in a downstream repo.",
  "Accessibility parity still requires downstream wrapper evidence once real Underlay surfaces adopt Flint-backed internals.",
  "Bridge-local token aliases remain intentionally narrow and should widen only in response to concrete Underlay adoption pressure.",
] as const;

export const underlayZeroLeakProof: UnderlayZeroLeakProof = {
  packageName: "@flint/bridge-underlay",
  proofVersion: 1,
  zeroLeakRules: underlayZeroLeakRules,
  directFlintImportsAllowedInApps: false,
  canonicalDependencyShape: "bridge-owned",
  proofSurfaces: underlayAdoptionSurfaceProof,
  remainingFriction: [...underlayRemainingAdoptionFriction],
};

export function validateUnderlayZeroLeakProof(): void {
  if (underlayTokenMap.length === 0) {
    throw new Error("Underlay zero-leak proof requires at least one bridge token alias.");
  }

  if (underlayThemeMap.length === 0) {
    throw new Error("Underlay zero-leak proof requires at least one bridge theme mapping.");
  }

  if (underlayAdoptionSurfaceProof.length === 0) {
    throw new Error("Underlay zero-leak proof requires at least one wrapper-backed adoption surface.");
  }

  if (underlayAdoptionSurfaceProof.some((surface) => surface.appImportsFlintDirectly)) {
    throw new Error("Underlay zero-leak proof failed: app-facing surfaces may not import Flint directly.");
  }
}
