import { useState } from "react";
import { DurationInput } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

export function DurationInputSpecimen() {
  const [hours, setHours] = useState(1);
  const [minutes, setMinutes] = useState(30);
  const [seconds, setSeconds] = useState(0);

  return (
    <SpecimenLayout
      sizes={(size) => <DurationInput hours={1} minutes={30} seconds={0} size={size} />}
      densities={(density) => <DurationInput density={density} />}
    >
      <SpecimenGroup label="Hours, minutes, seconds">
        <DurationInput
          hours={hours}
          minutes={minutes}
          seconds={seconds}
          onChange={(detail) => {
            setHours(detail.hours);
            setMinutes(detail.minutes);
            setSeconds(detail.seconds);
          }}
        />
        <p style={{ margin: 0 }}>Total: {hours}h {minutes}m {seconds}s</p>
      </SpecimenGroup>

      <SpecimenGroup label="Hours and minutes only">
        <DurationInput hours={0} minutes={45} showSeconds={false} />
      </SpecimenGroup>

      <SpecimenGroup label="Disabled">
        <DurationInput hours={2} minutes={15} seconds={30} disabled />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}
