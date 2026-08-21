import { readFileSync } from "node:fs";
import { join } from "node:path";

export const FROZEN_COMPONENT_COUNT = 175;

export type FrameworkRoster = {
  componentNames: string[];
  rootRuntimeNames: string[];
  nonComponentRootNames: string[];
  sourceMissingNames: string[];
  sourceExtraNames: string[];
};

export type WebPackageRoster = {
  frozenNames: string[];
  svelte: FrameworkRoster;
  react: FrameworkRoster;
};

function sortedUnique(names: Iterable<string>): string[] {
  return [...new Set(names)].sort();
}

function difference(left: Iterable<string>, right: Iterable<string>): string[] {
  const rightSet = new Set(right);
  return sortedUnique([...left].filter((name) => !rightSet.has(name)));
}

function parseExplicitRootExports(source: string): string[] {
  const names: string[] = [];
  const exportBlocks = /export\s*\{([\s\S]*?)\}\s*from\s+"[^"]+"/g;

  for (const match of source.matchAll(exportBlocks)) {
    for (const rawSpecifier of match[1].split(",")) {
      const specifier = rawSpecifier.trim();
      if (!specifier || specifier.startsWith("type ")) continue;

      const alias = specifier.match(
        /(?:default|[A-Za-z_$][\w$]*)\s+as\s+([A-Za-z_$][\w$]*)$/,
      );
      names.push(alias?.[1] ?? specifier.split(/\s+/)[0]);
    }
  }

  return sortedUnique(names);
}

function parseFrozenNames(rosterMarkdown: string): string[] {
  const start = rosterMarkdown.indexOf("## Svelte Denominator Surfaces");
  const end = rosterMarkdown.indexOf("## Cross-Runtime Surfaces", start);
  if (start < 0 || end < 0) {
    throw new Error(
      "Could not locate the frozen Svelte denominator tables in release-baseline-roster.md",
    );
  }

  return [
    ...rosterMarkdown
      .slice(start, end)
      .matchAll(/^\| `([^`]+)` \|/gm),
  ].map((match) => match[1]);
}

function parseSvelteComponentExports(source: string): string[] {
  return [
    ...source.matchAll(
      /export\s*\{\s*default as\s+(\w+)\s*\}\s*from\s+"\.\/[^"\n]+\.svelte"/g,
    ),
  ].map((match) => match[1]);
}

function buildFrameworkRoster(
  frozenNames: string[],
  sourceComponentNames: string[],
  rootRuntimeNames: string[],
): FrameworkRoster {
  return {
    componentNames: [...frozenNames],
    rootRuntimeNames: sortedUnique(rootRuntimeNames),
    nonComponentRootNames: difference(rootRuntimeNames, frozenNames),
    sourceMissingNames: difference(frozenNames, sourceComponentNames),
    sourceExtraNames: difference(sourceComponentNames, frozenNames),
  };
}

export function readWebPackageRoster(repoRoot: string): WebPackageRoster {
  const frozenNames = parseFrozenNames(
    readFileSync(
      join(repoRoot, "docs/roadmaps/g15/release-baseline-roster.md"),
      "utf8",
    ),
  );
  if (frozenNames.length !== FROZEN_COMPONENT_COUNT) {
    throw new Error(
      `Frozen roster denominator changed: expected ${FROZEN_COMPONENT_COUNT}, found ${frozenNames.length}`,
    );
  }

  const svelteSource = readFileSync(
    join(repoRoot, "packages/svelte/components/src/index.ts"),
    "utf8",
  );
  const reactSource = readFileSync(
    join(repoRoot, "packages/react/components/src/index.ts"),
    "utf8",
  );

  const svelteSourceNames = parseSvelteComponentExports(svelteSource);
  const reactRootNames = parseExplicitRootExports(reactSource);
  const reactSourceNames = frozenNames.filter((name) =>
    reactRootNames.includes(name),
  );

  const roster = {
    frozenNames,
    svelte: buildFrameworkRoster(
      frozenNames,
      svelteSourceNames,
      parseExplicitRootExports(svelteSource),
    ),
    react: buildFrameworkRoster(
      frozenNames,
      reactSourceNames,
      reactRootNames,
    ),
  } satisfies WebPackageRoster;

  for (const [framework, frameworkRoster] of Object.entries({
    svelte: roster.svelte,
    react: roster.react,
  })) {
    if (
      frameworkRoster.sourceMissingNames.length > 0 ||
      frameworkRoster.sourceExtraNames.length > 0
    ) {
      throw new Error(
        `${framework} source root disagrees with frozen roster: ${JSON.stringify({
          missing: frameworkRoster.sourceMissingNames,
          extra: frameworkRoster.sourceExtraNames,
        })}`,
      );
    }
  }

  return roster;
}
