import { MotionPolicyProvider, Skeleton, Spinner } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

export function MotionPolicyProviderSpecimen() {
  return (
    <SpecimenLayout>
      <SpecimenGroup label="Full policy">
        <MotionPolicyProvider policy="full">
          <Skeleton width="12rem" />
          <Spinner />
        </MotionPolicyProvider>
      </SpecimenGroup>

      <SpecimenGroup label="Reduced policy">
        <MotionPolicyProvider policy="reduced">
          <Skeleton width="12rem" />
          <Spinner />
        </MotionPolicyProvider>
      </SpecimenGroup>

      <SpecimenGroup label="Frozen nested inside reduced">
        <MotionPolicyProvider policy="reduced">
          <MotionPolicyProvider policy="frozen">
            <Skeleton width="12rem" />
            <Spinner />
          </MotionPolicyProvider>
        </MotionPolicyProvider>
      </SpecimenGroup>
    </SpecimenLayout>
  );
}
