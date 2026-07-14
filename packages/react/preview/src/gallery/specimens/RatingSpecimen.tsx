import { useState } from "react";
import { Rating } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

export function RatingSpecimen() {
  const [rating, setRating] = useState(3);
  const [fractionalRating, setFractionalRating] = useState(3.5);

  return (
    <SpecimenLayout
      sizes={(size) => <Rating value={3} size={size} ariaLabel={`Rating at ${size}`} />}
      densities={(density) => <Rating value={3} density={density} ariaLabel={`Rating at ${density} density`} />}
    >
      <SpecimenGroup label="Default (5 stars)">
        <Rating
          value={rating}
          ariaLabel="Rating"
          onValueChange={(value) => {
            if (value != null) setRating(value);
          }}
        />
        <p>
          Rating: <strong>{rating} / 5</strong>
        </p>
      </SpecimenGroup>

      <SpecimenGroup label="10-star scale">
        <Rating defaultValue={7} max={10} ariaLabel="Score out of 10" />
      </SpecimenGroup>

      <SpecimenGroup label="Half-star steps">
        <Rating
          value={fractionalRating}
          step={0.5}
          allowClear
          ariaLabel="Half-star rating"
          onValueChange={(value) => setFractionalRating(value ?? 0)}
        />
        <p>
          Rating: <strong>{fractionalRating} / 5</strong>
        </p>
      </SpecimenGroup>

      <SpecimenGroup label="Clearable">
        <Rating defaultValue={4} allowClear ariaLabel="Clearable rating" />
      </SpecimenGroup>

      <SpecimenGroup label="Disabled">
        <Rating defaultValue={2} disabled ariaLabel="Disabled rating" />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}
