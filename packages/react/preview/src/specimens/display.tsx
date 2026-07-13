import {
  Avatar,
  Code,
  Eyebrow,
  MetaItem,
  Meter,
  Pill,
  Progress,
  Separator,
  Skeleton,
  Spinner,
  StatusIndicator,
  Text,
  TextLink,
} from "@poodle/react";
import { registerSpecimen, Row, SpecimenSection } from "../harness";

registerSpecimen({
  slug: "text",
  title: "Text",
  render: () => (
    <SpecimenSection title="Text">
      <Text>Default body copy</Text>
      <Text tone="secondary" size="sm">
        Secondary small
      </Text>
      <Text tone="danger" weight="semibold">
        Danger semibold
      </Text>
    </SpecimenSection>
  ),
});

registerSpecimen({
  slug: "eyebrow",
  title: "Eyebrow",
  render: () => (
    <SpecimenSection title="Eyebrow">
      <Eyebrow>Section label</Eyebrow>
      <Eyebrow as="h3" size="md" spacing="bottom">
        Heading eyebrow
      </Eyebrow>
    </SpecimenSection>
  ),
});

registerSpecimen({
  slug: "pill",
  title: "Pill",
  render: () => (
    <SpecimenSection title="Pill">
      <Row>
        <Pill>Neutral</Pill>
        <Pill tone="success">Success</Pill>
        <Pill tone="danger" appearance="subtle">
          Danger subtle
        </Pill>
        <Pill appearance="badge">3</Pill>
        <Pill accent="#f59e0b">Accent</Pill>
      </Row>
    </SpecimenSection>
  ),
});

registerSpecimen({
  slug: "avatar",
  title: "Avatar",
  render: () => (
    <SpecimenSection title="Avatar">
      <Row>
        <Avatar initials="TW" />
        <Avatar initials="AB" tone="accent" shape="rounded" />
        <Avatar initials="XL" size="xl" />
        <Avatar decorative initials="D" size="xs" />
      </Row>
    </SpecimenSection>
  ),
});

registerSpecimen({
  slug: "skeleton",
  title: "Skeleton",
  render: () => (
    <SpecimenSection title="Skeleton">
      <Skeleton />
      <Row>
        <Skeleton shape="circle" />
        <Skeleton shape="block" width="8rem" height="4rem" />
      </Row>
      <Skeleton preset="list-item" />
    </SpecimenSection>
  ),
});

registerSpecimen({
  slug: "spinner",
  title: "Spinner",
  render: () => (
    <SpecimenSection title="Spinner">
      <Row>
        <Spinner />
        <Spinner variant="grid" tone="accent" />
        <Spinner size="lg" ariaLabel="Loading" />
      </Row>
    </SpecimenSection>
  ),
});

registerSpecimen({
  slug: "separator",
  title: "Separator",
  render: () => (
    <SpecimenSection title="Separator">
      <Separator />
      <Separator tone="default" />
    </SpecimenSection>
  ),
});

registerSpecimen({
  slug: "meter",
  title: "Meter",
  render: () => (
    <SpecimenSection title="Meter">
      <Meter value={62} ariaLabel="Capacity" />
      <Meter value={20} max={40} ariaLabel="Quota" />
    </SpecimenSection>
  ),
});

registerSpecimen({
  slug: "progress",
  title: "Progress",
  render: () => (
    <SpecimenSection title="Progress">
      <Progress value={45} ariaLabel="Upload" />
      <Progress indeterminate ariaLabel="Working" />
    </SpecimenSection>
  ),
});

registerSpecimen({
  slug: "status-indicator",
  title: "StatusIndicator",
  render: () => (
    <SpecimenSection title="StatusIndicator">
      <Row>
        <StatusIndicator status="success" label="Online" />
        <StatusIndicator status="warning" label="Degraded" />
        <StatusIndicator status="danger" label="Down" />
        <StatusIndicator status="pending" label="Pending" />
      </Row>
    </SpecimenSection>
  ),
});

registerSpecimen({
  slug: "meta-item",
  title: "MetaItem",
  render: () => (
    <SpecimenSection title="MetaItem">
      <Row>
        <MetaItem label="Owner">tom</MetaItem>
        <MetaItem label="Updated">2d ago</MetaItem>
      </Row>
    </SpecimenSection>
  ),
});

registerSpecimen({
  slug: "text-link",
  title: "TextLink",
  render: () => (
    <SpecimenSection title="TextLink">
      <Row>
        <TextLink href="#components/text-link">Accent link</TextLink>
        <TextLink href="#components/text-link" tone="secondary">
          Secondary
        </TextLink>
        <TextLink disabled>Disabled</TextLink>
      </Row>
    </SpecimenSection>
  ),
});

registerSpecimen({
  slug: "code",
  title: "Code",
  render: () => (
    <SpecimenSection title="Code">
      <Code source={'const x = 1;\nconsole.log(x);'} language="ts" showLineNumbers highlightLines={[2]} />
      <Text>
        Inline: <Code inline source="bun test" />
      </Text>
    </SpecimenSection>
  ),
});
