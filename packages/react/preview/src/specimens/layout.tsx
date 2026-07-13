import { Box, Grid, Icon, Region, Spacer, Stack } from "@poodle/react";
import { registerSpecimen, Row, SpecimenSection } from "../harness";

registerSpecimen({
  slug: "box",
  title: "Box",
  render: () => (
    <SpecimenSection title="Box">
      <Box padding="md" minHeight="3rem">
        <Region label="box content" minHeight="2rem" />
      </Box>
    </SpecimenSection>
  ),
});

registerSpecimen({
  slug: "stack",
  title: "Stack",
  render: () => (
    <SpecimenSection title="Stack">
      <Stack direction="row" gap="sm">
        <Region label="a" minHeight="2rem" />
        <Spacer />
        <Region label="b" minHeight="2rem" />
      </Stack>
    </SpecimenSection>
  ),
});

registerSpecimen({
  slug: "grid",
  title: "Grid",
  render: () => (
    <SpecimenSection title="Grid">
      <Grid columns="1fr 1fr 1fr" gap="sm">
        <Region label="1" />
        <Region label="2" />
        <Region label="3" />
      </Grid>
    </SpecimenSection>
  ),
});

registerSpecimen({
  slug: "icon",
  title: "Icon",
  render: () => (
    <SpecimenSection title="Icon">
      <Row>
        <Icon name="check" ariaLabel="check" />
        <Icon name="chevron-down" />
        <Icon name="alert-circle" size="lg" />
        <Icon name="pencil" size="xl" />
      </Row>
    </SpecimenSection>
  ),
});
