import { useState } from "react";
import { NumberInput } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const controlStyle = { maxWidth: "20rem" } as const;

export function NumberInputSpecimen() {
  const [quantity, setQuantity] = useState<number | string | null>(1);
  const [price, setPrice] = useState<number | string | null>(29.99);
  const [ticketCode, setTicketCode] = useState<number | string | null>("12");

  return (
    <SpecimenLayout
      sizes={(size) => (
        <div style={controlStyle}>
          <NumberInput id={`size-${size}`} value={1} size={size} ariaLabel={`Number at ${size}`} />
        </div>
      )}
      densities={(density) => (
        <div style={controlStyle}>
          <NumberInput id={`density-${density}`} value={1} density={density} ariaLabel={`Number at ${density} density`} />
        </div>
      )}
    >
      <SpecimenGroup label="Numeric Value">
        <div style={controlStyle}>
          <NumberInput id="qty" value={quantity} min={0} max={100} ariaLabel="Quantity" onValueChange={setQuantity} />
        </div>
        <p>
          Quantity: <strong>{quantity ?? "none"}</strong>
        </p>
      </SpecimenGroup>

      <SpecimenGroup label="With Steppers">
        <div style={controlStyle}>
          <NumberInput
            id="price"
            value={price}
            min={0}
            step={0.01}
            precision={2}
            showSteppers
            ariaLabel="Price"
            onValueChange={setPrice}
          />
        </div>
        <p>
          Price: <strong>{price == null ? "none" : `$${Number(price).toFixed(2)}`}</strong>
        </p>
      </SpecimenGroup>

      <SpecimenGroup label="String Form Binding">
        <div style={controlStyle}>
          <NumberInput
            id="ticket-code"
            value={ticketCode}
            prefix="A"
            min={1}
            max={999}
            ariaLabel="Ticket code"
            onValueChange={setTicketCode}
          />
        </div>
        <p>
          Ticket code: <strong>{ticketCode}</strong>
        </p>
      </SpecimenGroup>

      <SpecimenGroup label="Disabled">
        <div style={controlStyle}>
          <NumberInput id="disabled-num" value={42} ariaLabel="Disabled" disabled />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Invalid">
        <div style={controlStyle}>
          <NumberInput id="invalid-num" value={-5} min={0} ariaLabel="Invalid number" validationState="invalid" />
        </div>
      </SpecimenGroup>
    </SpecimenLayout>
  );
}
