import type { ParsedEmbed } from "./types";

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
    };
  }

  if (trimmed.startsWith("<") && trimmed.includes("iframe")) {
    const src = extractAttribute(trimmed, "src");
    const width = parseOptionalDimension(extractAttribute(trimmed, "width"));
    const height = parseOptionalDimension(extractAttribute(trimmed, "height"));

    return {
      provider: "generic",
      id: src ?? trimmed,
      originalUrl: src ?? undefined,
      originalEmbed: trimmed,
      width,
      height,
    };
  }

  if (isProbablyUrl(trimmed)) {
    return {
      provider: "generic",
      id: trimmed,
      originalUrl: trimmed,
    };
  }

  return null;
}

export function resolveEmbedParseState(
  value: string,
  providers: string[],
): EmbedParseState {
  const parsed = detectParsedEmbed(value);

  if (parsed && providers.length > 0 && !providers.includes(parsed.provider)) {
    return {
      parsed: null,
      error: `Provider "${parsed.provider}" is not allowed`,
    };
  }

  return {
    parsed,
    error: null,
  };
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
