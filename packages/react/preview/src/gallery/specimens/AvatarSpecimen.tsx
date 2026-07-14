import { Avatar, Stack } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";

export function AvatarSpecimen() {
  return (
    <div className="poodle-specimen">
      <SpecimenGroup label="Initials">
        <Stack direction="row" wrap align="center" gap="md">
          <Avatar initials="TA" size="xs" />
          <Avatar initials="TA" size="sm" />
          <Avatar initials="TA" size="md" />
          <Avatar initials="TA" size="lg" />
          <Avatar initials="TA" size="xl" />
        </Stack>
      </SpecimenGroup>

      <SpecimenGroup label="Tone and shape">
        <Stack direction="row" wrap align="center" gap="md">
          <Avatar initials="AC" tone="neutral" />
          <Avatar initials="AC" tone="accent" />
          <Avatar initials="AC" shape="rounded" tone="accent" />
        </Stack>
      </SpecimenGroup>

      <SpecimenGroup label="Image">
        <Avatar
          src="data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 80 80'%3E%3Crect width='80' height='80' fill='%232563eb'/%3E%3Ccircle cx='40' cy='30' r='16' fill='%23fff'/%3E%3Cpath d='M14 74c5-18 17-28 26-28s21 10 26 28' fill='%23fff'/%3E%3C/svg%3E"
          alt="Example user"
          size="lg"
        />
      </SpecimenGroup>
    </div>
  );
}
