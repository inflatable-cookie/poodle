import { ListCardCounter, TimeAgo } from "@poodle/react";
import { registerSpecimen, Row, SpecimenSection } from "../harness";

registerSpecimen({
  slug: "time-ago",
  title: "TimeAgo",
  render: () => (
    <SpecimenSection title="TimeAgo">
      <Row>
        <TimeAgo datetime={Date.now() - 90_000} />
        <TimeAgo datetime={Date.now() - 3 * 86_400_000} short={false} />
      </Row>
    </SpecimenSection>
  ),
});

registerSpecimen({
  slug: "list-card-counter",
  title: "ListCardCounter",
  render: () => (
    <SpecimenSection title="ListCardCounter">
      <Row>
        <ListCardCounter icon="star" count={12} tooltip="Stars" />
        <ListCardCounter icon="eye" count={340} href="#components/list-card-counter" />
      </Row>
    </SpecimenSection>
  ),
});
