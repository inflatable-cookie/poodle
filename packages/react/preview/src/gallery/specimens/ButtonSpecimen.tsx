import { useState } from "react";
import { Button } from "@inflatable-cookie/poodle-react";
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
      <SpecimenGroup label="A normal action row — the primary action, then the way out">
        <div className="poodle-specimen__row">
          <Button variant="primary" onClick={() => log("Save changes")}>Save changes</Button>
          <Button variant="ghost" onClick={() => log("Cancel")}>Cancel</Button>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Variants — how much weight the action carries">
        <div className="poodle-specimen__row">
          <Button variant="primary" onClick={() => log("Primary")}>Primary</Button>
          <Button variant="secondary" onClick={() => log("Secondary")}>Secondary</Button>
          <Button variant="ghost" onClick={() => log("Ghost")}>Ghost</Button>
        </div>
      </SpecimenGroup>

      {/* One variant, every tone. Tone and variant compose freely, so showing
          the grid teaches nothing the two rows above and below do not. */}
      <SpecimenGroup label="Tones — what kind of action it is">
        <div className="poodle-specimen__row">
          <Button variant="secondary" onClick={() => log("Default tone")}>Default</Button>
          <Button variant="secondary" tone="danger" onClick={() => log("Danger tone")}>Delete</Button>
          <Button variant="secondary" tone="success" onClick={() => log("Success tone")}>Approve</Button>
          <Button variant="secondary" tone="warning" onClick={() => log("Warning tone")}>Override</Button>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Icons, disclosure, and icon-only">
        <div className="poodle-specimen__row">
          <Button leadingIcon="plus" onClick={() => log("Leading icon")}>Create</Button>
          <Button trailingIcon="external-link" onClick={() => log("Trailing icon")}>Open</Button>
          <Button leadingIcon="filter" chevron onClick={() => log("Icon + chevron")}>Filter</Button>
          <Button leadingIcon="settings" ariaLabel="Settings" onClick={() => log("Icon only")} />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="States — unavailable, working, and held down">
        <div className="poodle-specimen__row">
          <Button variant="primary" disabled>Disabled</Button>
          <Button variant="primary" loading>Loading</Button>
          <Button variant="secondary" leadingIcon="star" pressed={bookmarked} onPressedChange={setBookmarked}>
            {bookmarked ? "Bookmarked" : "Bookmark"}
          </Button>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Inside a form — each button can submit somewhere else">
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
