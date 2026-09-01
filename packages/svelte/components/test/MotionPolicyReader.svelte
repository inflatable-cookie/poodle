<script lang="ts">
  import { getMotionPolicy } from "../src/motion-policy";

  const motion = getMotionPolicy();
</script>

<span data-testid="effective-motion">{$motion}</span>
