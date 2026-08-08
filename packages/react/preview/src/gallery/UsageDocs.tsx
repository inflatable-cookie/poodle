import { Separator } from "@inflatable-cookie/poodle-react";
import type { ComponentDocs } from "./component-docs";

export interface UsageDocsProps {
  docs: ComponentDocs;
}

function formatDefault(val: string | undefined): string {
  return val === undefined ? "—" : val;
}

export function UsageDocs({ docs }: UsageDocsProps) {
  return (
    <div className="poodle-usage-docs">
      {docs.usage ? (
        <>
          <section className="poodle-usage-docs__section">
            <h2 className="poodle-usage-docs__heading">Usage</h2>
            <pre className="poodle-usage-docs__code">
              <code>{docs.usage}</code>
            </pre>
          </section>
          <Separator />
        </>
      ) : null}

      {docs.props.length > 0 ? (
        <>
          <section className="poodle-usage-docs__section">
            <h2 className="poodle-usage-docs__heading">Props</h2>
            <div className="poodle-usage-docs__table-wrap">
              <table className="poodle-usage-docs__table">
                <thead>
                  <tr>
                    <th>Prop</th>
                    <th>Type</th>
                    <th>Default</th>
                    <th>Description</th>
                  </tr>
                </thead>
                <tbody>
                  {docs.props.map((prop) => (
                    <tr key={prop.name}>
                      <td className="poodle-usage-docs__prop-name">
                        {prop.name}
                        {prop.required ? <span className="poodle-usage-docs__required">*</span> : null}
                      </td>
                      <td className="poodle-usage-docs__type">
                        <code>{prop.type}</code>
                      </td>
                      <td className="poodle-usage-docs__default">
                        <code>{formatDefault(prop.default)}</code>
                      </td>
                      <td className="poodle-usage-docs__description">{prop.description}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </section>
          <Separator />
        </>
      ) : null}

      {docs.slots && docs.slots.length > 0 ? (
        <>
          <section className="poodle-usage-docs__section">
            <h2 className="poodle-usage-docs__heading">Slots</h2>
            <div className="poodle-usage-docs__table-wrap">
              <table className="poodle-usage-docs__table">
                <thead>
                  <tr>
                    <th>Slot</th>
                    <th>Description</th>
                  </tr>
                </thead>
                <tbody>
                  {docs.slots.map((slot) => (
                    <tr key={slot.name}>
                      <td className="poodle-usage-docs__prop-name">{slot.name}</td>
                      <td className="poodle-usage-docs__description">{slot.description}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </section>
          <Separator />
        </>
      ) : null}

      {docs.events && docs.events.length > 0 ? (
        <section className="poodle-usage-docs__section">
          <h2 className="poodle-usage-docs__heading">Events</h2>
          <div className="poodle-usage-docs__table-wrap">
            <table className="poodle-usage-docs__table">
              <thead>
                <tr>
                  <th>Event</th>
                  <th>Payload</th>
                  <th>Description</th>
                </tr>
              </thead>
              <tbody>
                {docs.events.map((event) => (
                  <tr key={event.name}>
                    <td className="poodle-usage-docs__prop-name">{event.name}</td>
                    <td className="poodle-usage-docs__type">
                      <code>{event.payload}</code>
                    </td>
                    <td className="poodle-usage-docs__description">{event.description}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </section>
      ) : null}
    </div>
  );
}
