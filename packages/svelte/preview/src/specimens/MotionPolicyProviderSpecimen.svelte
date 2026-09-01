<script lang="ts">
  import { MotionPolicyProvider, Skeleton, Spinner } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
</script>

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
