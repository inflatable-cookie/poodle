import { describe, expect, test } from "bun:test";

import { decodeHtmlEntities, sanitizeMarkdownHtml } from "../src/markdown-html.ts";

// The safe policy is a security boundary, so the corpus is deliberately
// adversarial: malformed markup, encoded entities, namespace prefixes,
// event handlers, unsafe URL schemes, raw-text elements, and unmatched tags.
// Every executable vector must disappear from the output, and the admitted
// Markdown prose surface must survive unchanged.

describe("sanitizeMarkdownHtml executable surface", () => {
  const executableCases: Array<[label: string, input: string]> = [
    ["raw script", `<script>alert(1)</script>`],
    ["script with attributes", `<script src="evil.js" defer></script>`],
    ["uppercase script", `<SCRIPT>alert(1)</SCRIPT>`],
    ["nested script tag", `<scr<script>ipt>alert(1)</script>`],
    ["unterminated script", `before<script>alert(1)`],
    ["self-closing script", `<script src="evil.js" />`],
    ["style block", `<style>body{background:url("javascript:alert(1)")}</style>`],
    ["iframe", `<iframe src="https://evil.example/frame"></iframe>`],
    ["object", `<object data="evil.swf"></object>`],
    ["embed", `<embed src="evil.swf">`],
    ["svg onload", `<svg/onload=alert(1)>`],
    ["svg script namespace", `<svg><script>alert(1)</script></svg>`],
    ["math mtext table", `<math><mtext><table><mglyph><style><img src=x onerror=alert(1)></style></mglyph></table></mtext></math>`],
    ["template", `<template><img src=x onerror=alert(1)></template>`],
    ["noscript", `<noscript><p title="</noscript><img src=x onerror=alert(1)>">`],
    ["textarea escape", `<textarea><img src=x onerror=alert(1)></textarea>`],
    ["title escape", `<title><img src=x onerror=alert(1)></title>`],
    ["base tag", `<base href="https://evil.example/">`],
    ["meta refresh", `<meta http-equiv="refresh" content="0;url=javascript:alert(1)">`],
    ["link stylesheet", `<link rel="stylesheet" href="evil.css">`],
    ["form action", `<form action="javascript:alert(1)"><button formaction="javascript:alert(1)">go</button></form>`],
    ["event handler attribute", `<img src="/safe.png" onerror="alert(1)">`],
    ["onclick on allowed anchor", `<a href="/ok" onclick="alert(1)">x</a>`],
    ["onload on allowed image", `<img src="/ok" onload=alert(1)>`],
    ["style attribute", `<p style="background:url(javascript:alert(1))">x</p>`],
    ["javascript url", `<a href="javascript:alert(1)">x</a>`],
    ["javascript url with entity", `<a href="&#106;avascript:alert(1)">x</a>`],
    ["javascript url with tab", `<a href="java\tscript:alert(1)">x</a>`],
    ["javascript url upper case", `<a href="JaVaScRiPt:alert(1)">x</a>`],
    ["double encoded javascript url", `<a href="&amp;#106;avascript:alert(1)">x</a>`],
    ["colon entity javascript url", `<a href="javascript&colon;alert(1)">x</a>`],
    ["vbscript url", `<a href="vbscript:msgbox(1)">x</a>`],
    ["data url anchor", `<a href="data:text/html,<script>alert(1)</script>">x</a>`],
    ["data url image", `<img src="data:image/svg+xml,<svg onload=alert(1)>">`],
    ["file url", `<a href="file:///etc/passwd">x</a>`],
    ["blob url", `<a href="blob:https://evil.example/1">x</a>`],
    ["protocol-relative src attribute", `<img srcset="x 1x" src="/ok">`],
    ["xlink href", `<a xlink:href="javascript:alert(1)">x</a>`],
    ["unknown element with handler", `<details open ontoggle="alert(1)">x</details>`],
    ["comment with script", `<!-- <script>alert(1)</script> -->text`],
    ["cdata", `<![CDATA[<script>alert(1)</script>]]>text`],
    ["processing instruction", `<?xml-stylesheet href="javascript:alert(1)"?>text`],
    ["malformed close only", `</script><p>ok</p>`],
    ["unclosed tag at end", `<img src=x onerror=alert(1)`],    ["stray less-than", `<<script>alert(1)</script>>`],
    ["null byte in tag", `<img src="x\u0000" onerror="alert(1)">`],
  ];

  for (const [label, input] of executableCases) {
    test(`removes ${label}`, () => {
      const output = sanitizeMarkdownHtml(input);
      // Executable markup must be gone. Escaped *text* that merely spells a
      // handler or scheme is inert, so every assertion is markup-context aware:
      // it looks for the construct inside a tag, not inside rendered text.
      expect(output).not.toMatch(/<\s*(script|style|iframe|object|embed|svg|math|template|textarea|title|base|meta|link|form|noscript|applet|frame)\b/i);
      expect(output).not.toMatch(/<\s*[a-z][^>]*\son[a-z]+\s*=/i);
      expect(output).not.toMatch(/<\s*[a-z][^>]*\s(?:style|srcdoc|xlink:href)\s*=/i);
      expect(output).not.toMatch(/<\s*[a-z][^>]*\s(?:href|src|action|formaction)\s*=\s*["']?\s*(?:javascript|vbscript|data|file|blob):/i);
      expect(output).not.toContain("<!--");
      expect(output).not.toContain("<![CDATA[");
    });
  }

  test("keeps the executed form absent for every unsafe URL scheme", () => {
    for (const scheme of ["javascript", "vbscript", "data", "file", "blob", "about"]) {
      const output = sanitizeMarkdownHtml(`<a href="${scheme}:payload">x</a>`);
      expect(output).toBe(`<a>x</a>`);
    }
  });
});

describe("sanitizeMarkdownHtml admitted surface", () => {
  test("preserves representative marked output", () => {
    const html = [
      `<h1>Title</h1>`,
      `<p>text <strong>bold</strong> <em>em</em> <del>gone</del> <code>code</code></p>`,
      `<blockquote><p>quote</p></blockquote>`,
      `<ul><li>one</li><li>two</li></ul>`,
      `<ol start="3"><li>three</li></ol>`,
      `<pre><code class="language-js">const a = 1;</code></pre>`,
      `<table><thead><tr><th align="left">A</th></tr></thead><tbody><tr><td align="right">1</td></tr></tbody></table>`,
      `<p><a href="https://example.com/page?a=1#b" title="link">go</a></p>`,
      `<p><img src="/images/a.png" alt="alt" title="caption"></p>`,
      `<hr>`,
      `<ul><li><input checked="" disabled="" type="checkbox"> done</li></ul>`,
    ].join("");
    expect(sanitizeMarkdownHtml(html)).toBe(html);
  });

  test("keeps relative, fragment, and mailto links", () => {
    expect(sanitizeMarkdownHtml(`<a href="#section">a</a>`)).toBe(`<a href="#section">a</a>`);
    expect(sanitizeMarkdownHtml(`<a href="/docs/x">a</a>`)).toBe(`<a href="/docs/x">a</a>`);
    expect(sanitizeMarkdownHtml(`<a href="../x">a</a>`)).toBe(`<a href="../x">a</a>`);
    expect(sanitizeMarkdownHtml(`<a href="mailto:a@b.example">a</a>`)).toBe(
      `<a href="mailto:a@b.example">a</a>`,
    );
  });

  test("decodes then re-encodes entities in text and attributes", () => {
    expect(sanitizeMarkdownHtml(`<p>a &amp; b &lt; c</p>`)).toBe(`<p>a &amp; b &lt; c</p>`);
    expect(sanitizeMarkdownHtml(`<p title="&quot;quoted&quot;">x</p>`)).toBe(
      `<p title="&quot;quoted&quot;">x</p>`,
    );
    expect(sanitizeMarkdownHtml(`<p>&#65;&#x42;</p>`)).toBe(`<p>AB</p>`);
  });

  test("drops the tag but keeps the text of unrecognised elements", () => {
    expect(sanitizeMarkdownHtml(`<div class="note">kept</div>`)).toBe(`kept`);
    expect(sanitizeMarkdownHtml(`<custom-element>kept</custom-element>`)).toBe(`kept`);
  });

  test("drops only the unsafe attribute, not the element", () => {
    expect(sanitizeMarkdownHtml(`<a href="javascript:alert(1)" title="t">x</a>`)).toBe(
      `<a title="t">x</a>`,
    );
    expect(sanitizeMarkdownHtml(`<img src="/a.png" onerror="alert(1)" alt="a">`)).toBe(
      `<img src="/a.png" alt="a">`,
    );
  });

  test("refuses non-checkbox inputs and strips names from task checkboxes", () => {
    expect(sanitizeMarkdownHtml(`<input type="text" value="x">`)).toBe(``);
    expect(sanitizeMarkdownHtml(`<input name="csrf" value="secret" type="checkbox">`)).toBe(
      `<input type="checkbox">`,
    );
    expect(sanitizeMarkdownHtml(`<input type="checkbox" autofocus>`)).toBe(
      `<input type="checkbox">`,
    );
  });

  test("escapes stray angle brackets as text", () => {
    expect(sanitizeMarkdownHtml(`a < b`)).toBe(`a &lt; b`);
    expect(sanitizeMarkdownHtml(`<3`)).toBe(`&lt;3`);
  });

  test("is deterministic and independent of environment", () => {
    const input = `<p onclick="x">a<script>alert(1)</script></p><svg/onload=alert(1)>`;
    const first = sanitizeMarkdownHtml(input);
    const second = sanitizeMarkdownHtml(input);
    expect(first).toBe(second);
    expect(first).not.toContain("<script");
  });
});

describe("decodeHtmlEntities", () => {
  test("decodes numeric and named entities once", () => {
    expect(decodeHtmlEntities("&#106;")).toBe("j");
    expect(decodeHtmlEntities("&#x6a;")).toBe("j");
    expect(decodeHtmlEntities("&amp;")).toBe("&");
    expect(decodeHtmlEntities("&amp;#106;")).toBe("&#106;");
    expect(decodeHtmlEntities("&notanentity;")).toBe("&notanentity;");
  });
});
