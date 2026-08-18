<script lang="ts">
  import { Stack, Text } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
</script>

<SpecimenLayout>
  {#snippet sizes(size)}
    {#if size === "xs" || size === "sm" || size === "md"}
      <Text size={size}>Default body text for admin and product surfaces.</Text>
    {/if}
  {/snippet}

  <SpecimenGroup label="Tones">
    <Stack gap="sm">
      <Text>Default body text for admin and product surfaces.</Text>
      <Text tone="secondary">Secondary text for supporting copy.</Text>
      <Text tone="muted">Muted text for low-priority hints.</Text>
      <Text tone="success">Success text for positive confirmation.</Text>
      <Text tone="danger">Danger text for failure or destructive context.</Text>
      <Text tone="warning">Warning text for cautionary context.</Text>
    </Stack>
  </SpecimenGroup>

  <SpecimenGroup label="Sizes and leading">
    <Stack gap="sm">
      <Text size="xs">Extra-small supporting label text.</Text>
      <Text size="sm">Small hint or caption text.</Text>
      <Text size="md" leading="relaxed">Relaxed body copy for longer readable paragraphs.</Text>
      <Text weight="medium">Medium-weight label text.</Text>
      <Text weight="semibold">Semibold emphasis text.</Text>
      <Text weight="bold">Bold metric text.</Text>
    </Stack>
  </SpecimenGroup>

  <SpecimenGroup label="Inline">
    <Text as="span" tone="secondary" size="sm">Inline secondary text</Text>
  </SpecimenGroup>

  <SpecimenGroup label="Clamp">
    <Text clamp={2}>
      This text is intentionally long enough to wrap across multiple lines in a narrow container, then clamp after two visible lines.
    </Text>
  </SpecimenGroup>
</SpecimenLayout>
