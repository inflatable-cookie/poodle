import { useState } from "react";
import { Button } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

export function ButtonSpecimen() {
  const [clickLog, setClickLog] = useState("No button clicked yet.");
  const [intent, setIntent] = useState("save");
  const [bookmarked, setBookmarked] = useState(false);

  const log = (label: string) => setClickLog(`Clicked: ${label}`);

  return (
    <SpecimenLayout
      sizes={(size) => (
        <Button variant="primary" size={size} leadingIcon="plus" onClick={() => log(`Size ${size}`)}>
          {size.toUpperCase()}
        </Button>
      )}
      densities={(density) => (
        <Button variant="secondary" density={density} leadingIcon="download" onClick={() => log(`Density ${density}`)}>
          Action
        </Button>
      )}
    >
      <SpecimenGroup label="Variants">
        <div className="poodle-specimen__row">
          <Button variant="primary" onClick={() => log("Primary")}>Primary</Button>
          <Button variant="secondary" onClick={() => log("Secondary")}>Secondary</Button>
          <Button variant="ghost" onClick={() => log("Ghost")}>Ghost</Button>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Danger tone">
        <div className="poodle-specimen__row">
          <Button variant="primary" tone="danger" onClick={() => log("Danger primary")}>Danger primary</Button>
          <Button variant="secondary" tone="danger" onClick={() => log("Danger secondary")}>Danger secondary</Button>
          <Button variant="ghost" tone="danger" onClick={() => log("Danger ghost")}>Danger ghost</Button>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="With icons">
        <div className="poodle-specimen__row">
          <Button leadingIcon="plus" onClick={() => log("Leading icon")}>Create</Button>
          <Button trailingIcon="external-link" onClick={() => log("Trailing icon")}>Open</Button>
          <Button leadingIcon="save" trailingIcon="check" onClick={() => log("Both icons")}>Save</Button>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="With chevron">
        <div className="poodle-specimen__row">
          <Button chevron onClick={() => log("Chevron")}>Options</Button>
          <Button variant="primary" chevron onClick={() => log("Primary chevron")}>Actions</Button>
          <Button leadingIcon="filter" chevron onClick={() => log("Icon + chevron")}>Filter</Button>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="States">
        <div className="poodle-specimen__row">
          <Button variant="primary" disabled>Disabled</Button>
          <Button variant="primary" loading>Loading</Button>
          <Button variant="secondary" disabled>Disabled secondary</Button>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Toggle (pressed state)">
        <div className="poodle-specimen__row">
          <Button variant="secondary" leadingIcon="star" pressed={bookmarked} onPressedChange={setBookmarked}>
            {bookmarked ? "Bookmarked" : "Bookmark"}
          </Button>
          <Button variant="secondary" leadingIcon="heart" defaultPressed={false}>Like</Button>
          <Button variant="ghost" leadingIcon="lock-open" defaultPressed>Locked</Button>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Form overrides">
        <form
          className="poodle-specimen__form"
          onSubmit={(event) => {
            event.preventDefault();
            log(`Submitted via ${intent}`);
          }}
        >
          <input type="hidden" name="intent" value={intent} />
          <div className="poodle-specimen__row">
            <Button type="submit" variant="secondary" onClick={() => setIntent("save")}>
              Save
            </Button>
            <Button type="submit" variant="primary" formAction="/publish" formNoValidate onClick={() => setIntent("publish")}>
              Publish
            </Button>
          </div>
        </form>
      </SpecimenGroup>

      <p className="poodle-specimen__log">{clickLog}</p>
    </SpecimenLayout>
  );
}
