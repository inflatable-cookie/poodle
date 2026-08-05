import type { EmbedMeta, EmbedParseResult, ParsedEmbed } from "./types.ts";

export type EmbedParseState = {
  parsed: ParsedEmbed | null;
  error: string | null;
};

export function detectParsedEmbed(input: string): ParsedEmbed | null {
  const trimmed = input.trim();

  if (!trimmed) {
    return null;
  }

  const youtubeShortId = extractAfter(trimmed, "youtu.be/");
  if (youtubeShortId) {
    return {
      provider: "youtube",
      id: youtubeShortId,
      originalUrl: trimmed,
    };
  }

  const youtubeWatchId = extractAfter(trimmed, "youtube.com/watch?v=");
  if (youtubeWatchId) {
    return {
      provider: "youtube",
      id: youtubeWatchId,
      originalUrl: trimmed,
    };
  }

  const youtubeEmbedId = extractAfter(trimmed, "youtube.com/embed/");
  if (youtubeEmbedId) {
    return {
      provider: "youtube",
      id: youtubeEmbedId,
      originalUrl: trimmed,
    };
  }

  const vimeoId = extractDigitsAfter(trimmed, "vimeo.com/");
  if (vimeoId) {
    return {
      provider: "vimeo",
      id: vimeoId,
      originalUrl: trimmed,
      embedType: "video",
    };
  }

  const audioboomId = extractDigitsAfter(trimmed, "audioboom.com/posts/");
  if (audioboomId) {
    return {
      provider: "audioboom",
      id: audioboomId,
      originalUrl: trimmed,
      embedType: "audio",
    };
  }

  if (trimmed.startsWith("<") && trimmed.includes("iframe")) {
    const src = extractAttribute(trimmed, "src");
    const width = parseOptionalDimension(extractAttribute(trimmed, "width"));
    const height = parseOptionalDimension(extractAttribute(trimmed, "height"));
    const embedded = src ? detectParsedEmbed(src) : null;

    return {
      provider: embedded?.provider ?? "generic",
      id: embedded?.id ?? src ?? trimmed,
      originalUrl: src ?? undefined,
      originalEmbed: trimmed,
      width,
      height,
      embedType: embedded?.embedType ?? "generic",
    };
  }

  if (isProbablyUrl(trimmed)) {
    return {
      provider: "generic",
      id: trimmed,
      originalUrl: trimmed,
      embedType: "generic",
    };
  }

  return null;
}

export function resolveEmbedParseState(
  value: string,
  providers: string[],
): EmbedParseState {
  const result = parseEmbed(value, {
    allowedProviders: providers,
    allowGeneric: true,
  });
  return {
    parsed: result.parsed,
    error: result.success ? null : result.error ?? null,
  };
}

export function parseEmbed(
  input: string,
  options: {
    allowedProviders?: string[];
    allowGeneric?: boolean;
  } = {},
): EmbedParseResult {
  const trimmed = input.trim();
  if (!trimmed) {
    return {
      success: true,
      parsed: null,
    };
  }

  const parsed = detectParsedEmbed(trimmed);
  if (!parsed) {
    return {
      success: false,
      parsed: null,
      error: "Could not parse embed source",
    };
  }

  if (parsed.provider === "generic" && options.allowGeneric === false) {
    return {
      success: false,
      parsed: null,
      error: "Generic embeds are not allowed",
    };
  }

  if (
    options.allowedProviders &&
    options.allowedProviders.length > 0 &&
    !options.allowedProviders.includes(parsed.provider)
  ) {
    return {
      success: false,
      parsed: null,
      error: `Provider "${parsed.provider}" is not allowed`,
    };
  }

  return {
    success: true,
    parsed,
  };
}

export function renderEmbed(embed: Pick<ParsedEmbed, "provider" | "id" | "originalEmbed">): string | null {
  if (embed.originalEmbed) {
    return embed.originalEmbed;
  }

  switch (embed.provider) {
    case "youtube":
      return `<iframe src="https://www.youtube.com/embed/${embed.id}" loading="lazy" allowfullscreen></iframe>`;
    case "vimeo":
      return `<iframe src="https://player.vimeo.com/video/${embed.id}" loading="lazy" allowfullscreen></iframe>`;
    case "audioboom":
      return `<iframe src="https://embeds.audioboom.com/posts/${embed.id}/embed/v5" loading="lazy"></iframe>`;
    default:
      return null;
  }
}

export function getThumbnailUrl(
  embed: Pick<ParsedEmbed, "provider" | "id">,
  quality: "default" | "medium" | "high" | "max" = "high",
): string | null {
  if (embed.provider !== "youtube") {
    return null;
  }

  const file =
    quality === "max"
      ? "maxresdefault.jpg"
      : quality === "medium"
        ? "mqdefault.jpg"
        : quality === "default"
          ? "default.jpg"
          : "hqdefault.jpg";

  return `https://img.youtube.com/vi/${embed.id}/${file}`;
}

export function getProviderAccent(provider: string): string {
  switch (provider) {
    case "youtube":
      return "#ff0033";
    case "vimeo":
      return "#1ab7ea";
    case "audioboom":
      return "#f97316";
    default:
      return "#64748b";
  }
}

export async function lookupMeta(embed: Pick<ParsedEmbed, "provider" | "id" | "originalUrl">): Promise<EmbedMeta | null> {
  const url = getOEmbedUrl(embed);
  if (!url) {
    return null;
  }

  try {
    const response = await fetch(url);
    if (!response.ok) {
      return null;
    }

    const data = await response.json() as {
      title?: string;
      description?: string;
      duration?: number;
      thumbnail_url?: string;
      author_name?: string;
    };

    return {
      title: data.title,
      description: data.description,
      duration: data.duration,
      thumbnailUrl: data.thumbnail_url,
      authorName: data.author_name,
    };
  } catch {
    return null;
  }
}

function getOEmbedUrl(embed: Pick<ParsedEmbed, "provider" | "id" | "originalUrl">): string | null {
  const originalUrl = embed.originalUrl ?? renderOriginalUrl(embed);
  if (!originalUrl) {
    return null;
  }

  switch (embed.provider) {
    case "youtube":
      return `https://www.youtube.com/oembed?url=${encodeURIComponent(originalUrl)}&format=json`;
    case "vimeo":
      return `https://vimeo.com/api/oembed.json?url=${encodeURIComponent(originalUrl)}`;
    case "audioboom":
      return `https://api.audioboom.com/oembed?url=${encodeURIComponent(originalUrl)}&format=json`;
    default:
      return null;
  }
}

function renderOriginalUrl(embed: Pick<ParsedEmbed, "provider" | "id">): string | null {
  switch (embed.provider) {
    case "youtube":
      return `https://www.youtube.com/watch?v=${embed.id}`;
    case "vimeo":
      return `https://vimeo.com/${embed.id}`;
    case "audioboom":
      return `https://audioboom.com/posts/${embed.id}`;
    default:
      return null;
  }
}

function extractAfter(input: string, needle: string): string | null {
  const start = input.indexOf(needle);
  if (start === -1) {
    return null;
  }

  const suffix = input.slice(start + needle.length);
  let id = "";

  for (const char of suffix) {
    if (!(isAlphaNumeric(char) || char === "-" || char === "_")) {
      break;
    }
    id += char;
  }

  return id.length > 0 ? id : null;
}

function extractDigitsAfter(input: string, needle: string): string | null {
  const start = input.indexOf(needle);
  if (start === -1) {
    return null;
  }

  const suffix = input.slice(start + needle.length);
  let id = "";

  for (const char of suffix) {
    if (!isDigit(char)) {
      break;
    }
    id += char;
  }

  return id.length > 0 ? id : null;
}

function extractAttribute(input: string, attribute: string): string | null {
  for (const quote of [`"`, `'`]) {
    const token = `${attribute}=${quote}`;
    const start = input.indexOf(token);
    if (start === -1) {
      continue;
    }

    const valueStart = start + token.length;
    const end = input.indexOf(quote, valueStart);
    if (end === -1) {
      continue;
    }

    return input.slice(valueStart, end);
  }

  return null;
}

function parseOptionalDimension(value: string | null): number | undefined {
  if (!value) {
    return undefined;
  }

  const parsed = Number.parseInt(value, 10);
  return Number.isFinite(parsed) ? parsed : undefined;
}

function isProbablyUrl(input: string): boolean {
  return (input.startsWith("http://") || input.startsWith("https://")) && !/\s/.test(input);
}

function isAlphaNumeric(char: string): boolean {
  return /[A-Za-z0-9]/.test(char);
}

function isDigit(char: string): boolean {
  return /[0-9]/.test(char);
}
