/**
 * g15.047 — capture-only Button fixture host. The gallery is preview chrome;
 * this page is the exact scene the Playwright harness
 * (test/visual/button-comparison/capture-web.ts) photographs: one 240×80
 * canvas div, one Button, every input resolved from the query string.
 *
 * It is deliberately dumb: it does not read the test inventory, it does not
 * know fixture rosters, and any invalid or missing param renders a
 * full-viewport `[data-fixture-error]` the harness fails on. It mirrors the
 * Svelte host (`packages/svelte/preview/src/fixture-host/FixtureHost.svelte`)
 * param for param.
 */
import { useLayoutEffect, useState } from "react";

import { defaultLucideIconSet } from "@inflatable-cookie/poodle-core/icons";
import {
  applyThemeAttributes,
  controlSizes,
  densityModes,
  themes,
} from "@inflatable-cookie/poodle-core/tokens";
import {
  Button,
  type ButtonTone,
  type ButtonVariant,
  type ControlDensity,
  type ControlSize,
} from "@inflatable-cookie/poodle-react";

// The web captures must rasterise text with the same font bytes the GPUI
// capture loads; Inter is not installed on the host, so the TTFs are bundled
// from the GPUI preview's asset directory as vite asset URLs.
const interRegularUrl = new URL(
  "../../../../gpui/preview/assets/fonts/Inter-Regular.ttf",
  import.meta.url,
).href;
const interMediumUrl = new URL(
  "../../../../gpui/preview/assets/fonts/Inter-Medium.ttf",
  import.meta.url,
).href;

const HOST_CSS = `
@font-face {
  font-family: "Inter";
  src: url("${interRegularUrl}") format("truetype");
  font-weight: 400;
  font-style: normal;
  font-display: block;
}
@font-face {
  font-family: "Inter";
  src: url("${interMediumUrl}") format("truetype");
  font-weight: 500;
  font-style: normal;
  font-display: block;
}
html,
body {
  margin: 0;
  overflow: hidden;
}

/*
 * The scene is exactly the viewport. The Button's border-box must land at
 * logical (16, 16): flex placement avoids the inline-strut offset a plain
 * block child would get from the host's line box.
 */
.poodle-fixture-host {
  width: 240px;
  height: 80px;
  background: var(--poodle-color-background-canvas);
  padding: 16px;
  box-sizing: border-box;
  display: flex;
  align-items: flex-start;
  justify-content: flex-start;
}

.poodle-fixture-error {
  position: fixed;
  inset: 0;
  padding: 8px;
  background: #7f1d1d;
  color: #ffffff;
  font: 12px/1.4 monospace;
  white-space: pre-wrap;
}
`;

const VARIANTS = ["primary", "secondary", "ghost"] as const;
const TONES = ["default", "danger", "success", "warning"] as const;
const STATES = ["rest", "disabled", "loading", "pressed"] as const;
const CONTENT_KINDS = ["label", "leading-icon", "icon-only"] as const;

type ThemeId = keyof typeof themes;
type DensityId = keyof typeof densityModes;
type SizeId = keyof typeof controlSizes;
type VisualState = (typeof STATES)[number];
type ContentKind = (typeof CONTENT_KINDS)[number];

type FixtureConfig = {
  theme: ThemeId;
  size: SizeId;
  density: DensityId;
  variant: ButtonVariant;
  tone: ButtonTone;
  state: VisualState;
  contentKind: ContentKind;
  label: string | null;
  icon: string | null;
  ariaLabel: string | null;
};

type ParsedParams = { ok: true; config: FixtureConfig } | { ok: false; problems: string[] };

function parseFixtureParams(params: URLSearchParams): ParsedParams {
  const problems: string[] = [];

  function readEnum(name: string, domain: readonly string[]): string | null {
    const value = params.get(name);
    if (value === null || value === "") {
      problems.push(`missing required param '${name}'`);
      return null;
    }
    if (!domain.includes(value)) {
      problems.push(`param '${name}' value '${value}' is outside [${domain.join(", ")}]`);
      return null;
    }
    return value;
  }

  const fixture = params.get("fixture");
  if (fixture === null || fixture === "") {
    problems.push("missing required param 'fixture'");
  } else if (!/^button\/[a-z0-9-]+$/.test(fixture)) {
    problems.push(`param 'fixture' value '${fixture}' is not a button fixture name`);
  }

  const theme = readEnum("theme", Object.keys(themes));
  const size = readEnum("size", Object.keys(controlSizes));
  const density = readEnum("density", Object.keys(densityModes));
  const variant = readEnum("variant", VARIANTS);
  const tone = readEnum("tone", TONES);
  const state = readEnum("state", STATES);
  const contentKind = readEnum("contentKind", CONTENT_KINDS);

  let label: string | null = null;
  let icon: string | null = null;
  let ariaLabel: string | null = null;

  if (contentKind !== null) {
    const rawLabel = params.get("label");
    const rawIcon = params.get("icon");
    const rawAriaLabel = params.get("ariaLabel");

    const needsLabel = contentKind === "label" || contentKind === "leading-icon";
    const needsIcon = contentKind === "leading-icon" || contentKind === "icon-only";

    if (needsLabel && (rawLabel === null || rawLabel === "")) {
      problems.push(`content kind '${contentKind}' requires a non-empty 'label' param`);
    }
    if (needsIcon) {
      if (rawIcon === null || rawIcon === "") {
        problems.push(`content kind '${contentKind}' requires an 'icon' param`);
      } else if (!(rawIcon in defaultLucideIconSet)) {
        problems.push(`param 'icon' value '${rawIcon}' is not a default icon registry name`);
      }
    }
    if (contentKind === "icon-only" && (rawAriaLabel === null || rawAriaLabel === "")) {
      problems.push("content kind 'icon-only' requires a non-empty 'ariaLabel' param");
    }
    if (!needsLabel && rawLabel !== null) {
      problems.push(`param 'label' is not used by content kind '${contentKind}'`);
    }
    if (!needsIcon && rawIcon !== null) {
      problems.push(`param 'icon' is not used by content kind '${contentKind}'`);
    }
    if (contentKind !== "icon-only" && rawAriaLabel !== null) {
      problems.push(`param 'ariaLabel' is not used by content kind '${contentKind}'`);
    }

    label = needsLabel ? rawLabel : null;
    icon = needsIcon ? rawIcon : null;
    ariaLabel = contentKind === "icon-only" ? rawAriaLabel : null;
  }

  if (problems.length > 0 || !theme || !size || !density || !variant || !tone || !state || !contentKind) {
    return { ok: false, problems };
  }

  return {
    ok: true,
    config: {
      theme: theme as ThemeId,
      size: size as SizeId,
      density: density as DensityId,
      variant: variant as ButtonVariant,
      tone: tone as ButtonTone,
      state: state as VisualState,
      contentKind: contentKind as ContentKind,
      label,
      icon,
      ariaLabel,
    },
  };
}

export function FixtureHost() {
  const [parsed] = useState(() => parseFixtureParams(new URLSearchParams(window.location.search)));
  const [ready, setReady] = useState(false);

  // useLayoutEffect, not useEffect: theme attributes must land before the
  // first paint so the Button's first computed style is already themed and no
  // unthemed→themed transition can start. (A post-paint effect measurably
  // raced the harness — getComputedStyle caught mid-transition colors while
  // the screenshot caught the settled frame.)
  useLayoutEffect(() => {
    if (!parsed.ok) return;
    let cancelled = false;
    const style = document.createElement("style");
    style.textContent = HOST_CSS;
    document.head.appendChild(style);
    applyThemeAttributes(document.documentElement, {
      theme: parsed.config.theme,
      density: parsed.config.density,
      controlSize: parsed.config.size,
    });
    // Pin the font bytes before the ready flag lands: the harness waits on
    // `[data-fixture-ready]` before it screenshots.
    void Promise.all([
      document.fonts.load('400 16px "Inter"'),
      document.fonts.load('500 16px "Inter"'),
    ]).then(() => {
      if (!cancelled) setReady(true);
    });
    return () => {
      cancelled = true;
      style.remove();
    };
  }, [parsed]);

  if (!parsed.ok) {
    return (
      <>
        <style>{HOST_CSS}</style>
        <div className="poodle-fixture-error" data-fixture-error>
          {parsed.problems.join("; ")}
        </div>
      </>
    );
  }

  const { config } = parsed;
  const stateProps = {
    disabled: config.state === "disabled",
    loading: config.state === "loading",
    pressed: config.state === "pressed" ? true : null,
  };

  return (
    <>
      <style>{HOST_CSS}</style>
      <div
        className="poodle-fixture-host"
        data-fixture-host
        data-fixture-ready={ready ? "" : undefined}
      >
        {config.contentKind === "icon-only" ? (
          <Button
            variant={config.variant}
            tone={config.tone}
            size={config.size as ControlSize}
            density={config.density as ControlDensity}
            {...stateProps}
            leadingIcon={config.icon}
            ariaLabel={config.ariaLabel}
          />
        ) : (
          <Button
            variant={config.variant}
            tone={config.tone}
            size={config.size as ControlSize}
            density={config.density as ControlDensity}
            {...stateProps}
            leadingIcon={config.contentKind === "leading-icon" ? config.icon : null}
          >
            {config.label}
          </Button>
        )}
      </div>
    </>
  );
}
