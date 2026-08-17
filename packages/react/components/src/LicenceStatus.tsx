import "@inflatable-cookie/poodle-core/styles/licence.css";

import { Fragment } from "react";

import {
  formatDisplayTimeDate,
  licenceStatusView,
  type LicenceAttention,
  type LicenceTrustBasis,
  type LicenceUsability,
} from "@inflatable-cookie/poodle-core";

import { StatusIndicator } from "./StatusIndicator";
import { TimeAgo } from "./TimeAgo";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize } from "./types";

export interface LicenceStatusProps {
  usability: LicenceUsability;
  trustBasis: LicenceTrustBasis;
  useUntil: number | null;
  updateUntil: number | null;
  usable: boolean;
  attention: LicenceAttention;
  title?: string;
  size?: ControlSize | null;
  density?: ControlDensity | null;
}

export function LicenceStatus({
  usability,
  trustBasis,
  useUntil,
  updateUntil,
  usable,
  attention,
  title = "Licence",
  size = null,
  density = null,
}: LicenceStatusProps) {
  const uiPresentation = useUiPresentation();
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, "control");
  const resolvedDensity = density ?? uiPresentation.density;

  /* Every display decision — title, tone, row terms, which timestamp goes
     where — is made once in poodle-core so this shell and its Svelte twin
     cannot disagree about what a licence state means. */
  const view = licenceStatusView({ usability, trustBasis, useUntil, updateUntil, usable, attention });

  return (
    /* `usable` and `attention` are authority reads. They reach the DOM as data
       state and nothing else: no branch below hides a row, disables a control,
       or turns a licence read into a feature permission. */
    <section
      className="poodle-licence-status"
      aria-label={title}
      data-state={view.state}
      data-tone={view.tone}
      data-attention={view.attention}
      data-usable={view.usable}
      data-size={resolvedSize}
      data-density={resolvedDensity}
    >
      <div className="poodle-licence-status__head">
        <StatusIndicator status={view.indicator} size={resolvedSize} density={resolvedDensity} />
        <h3 className="poodle-licence-status__title">{view.title}</h3>
      </div>

      <p className="poodle-licence-status__body">
        {view.body.text}
        {view.body.timestamp !== null ? (
          <>
            &nbsp;
            <TimeAgo datetime={view.body.timestamp} typography="inherit" />
          </>
        ) : null}
      </p>

      {/* Use coverage, update coverage and trust basis are three labelled
          values, never merged. A single "expires" line is how someone with
          lapsed updates is told they have lost the software they own. */}
      <dl className="poodle-licence-status__coverage">
        {view.coverage.map((row) => (
          <Fragment key={row.id}>
            <dt className="poodle-licence-status__term">{row.term}</dt>
            <dd className="poodle-licence-status__value" data-row={row.id}>
              {row.timestamp !== null ? (
                <TimeAgo
                  datetime={row.timestamp}
                  futurePrefix={row.futurePrefix}
                  pastPrefix={row.pastPrefix}
                  typography="inherit"
                />
              ) : (
                row.text
              )}
            </dd>
          </Fragment>
        ))}
        <dt className="poodle-licence-status__term">{view.trust.term}</dt>
        <dd className="poodle-licence-status__value" data-row="trust">
          {view.trust.text}
          {view.trust.timestamp !== null ? (
            <>
              &nbsp;
              <TimeAgo datetime={view.trust.timestamp} typography="inherit" />
            </>
          ) : null}
        </dd>
      </dl>

      {view.detail ? (
        <p className="poodle-licence-status__detail">
          {view.detail.text}
          {view.detail.timestamp !== null ? (
            <>
              &nbsp;
              <time dateTime={new Date(view.detail.timestamp).toISOString()}>
                {formatDisplayTimeDate(view.detail.timestamp, "en-GB")}
              </time>
            </>
          ) : null}
        </p>
      ) : null}
    </section>
  );
}
