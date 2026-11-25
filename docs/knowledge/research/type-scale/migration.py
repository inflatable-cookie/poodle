"""Assign each authored declaration to a proposed role for estimating the migration."""
import collections
import csv
import json
import re
from pathlib import Path

rows = json.loads(Path("declarations.json").read_text())
# Ordered source-specific rules. A source-specific rule wins over the fallback.
rules = {}
def by_file(files, *items):
    for file in files.split():
        rules[file + ".css"] = items

by_file("accordion collapsible", ("indicator", "glyph"), ("title", "sectionTitle"), ("description", "secondary"), ("panel", "body"))
by_file("action-discovery-panel", ("chip-font-size|badge.*kbd", "split"), ("__kbd", "keycap"), ("badge", "badge"))
by_file("agent-chat-input", ("editor|input-font", "inputText"), ("attachment|footer", "caption"))
by_file("agent-message", ("code", "codeInline"), ("heading", "documentHeading"), (".", "documentBody"))
by_file("agent-plan", ("badge", "badge"), ("action", "controlLabel"), ("font-size", "split"))
by_file("agent-plan-record agent-subagent agent-transcript tool-call tool-call-group", ("activity|description", "caption"), ("toggle|action|jump|trigger", "controlLabel"), ("font-size", "split"))
by_file("agent-question-record changed-files", ("font-family.*code|__path|__hash|__diff", "identifier"), ("count|__stat", "count"), ("font-size", "split"))
by_file("agent-question", ("shortcut", "keycap"), ("prompt", "sectionTitle"), ("option-description", "secondary"), ("progress", "caption"), ("option-label|dismiss", "controlLabel"), ("font-size", "split"))
by_file("app-header", ("subtitle", "secondary"), ("title|identity", "appTitle"))
by_file("audio-player", ("__time", "value"), ("__speed", "controlLabel"))
by_file("avatar", (".", "initials"))
by_file("block-editor", ("tool-btn", "controlLabel"), ("input", "inputText"))
by_file("breadcrumbs", (".", "controlLabel"))
by_file("button icon-button", ("tooltip", "tooltip"), ("__icon|__spinner|__glyph", "glyph"), (".", "controlLabel"))
by_file("bulk-action-bar", ("summary", "controlLabel"))
by_file("calendar", ("month|weekday", "groupLabel"), ("day", "controlLabel"))
by_file("callout", ("__icon", "glyph"), ("strong", "cardTitle"), ("content", "body"))
by_file("card-radio-group card-toggle-group", ("description", "secondary"), ("count", "count"), ("title", "cardTitle"))
by_file("checkbox radio radio-group switch tri-state-switch tabs toggle-group", ("tooltip", "tooltip"), ("label|__tab|__item|segment", "controlLabel"))
by_file("code-editor", (".", "codeBlock"))
by_file("code-input", ("slot|separator", "inputText"))
by_file("code", ("inline", "codeInline"), ("language", "overline"), ("source|line", "codeBlock"))
by_file("color-picker", ("input-label", "groupLabel"), ("input", "inputText"))
by_file("command-palette", ("h3", "dialogTitle"), ("hint|kbd", "keycap"), ("__header p|status", "secondary"))
by_file("data-table", ("__cell small", "tableSecondary"), ("thead th", "tableHeader"), ("__footer|filters-row", "split"), ("th,.*td", "split"), ("toolbar-btn|col-menu-item|pagination-page", "controlLabel"), ("pagination-summary", "caption"), ("__limit", "inputText"), ("__empty", "body"))
by_file("date-picker date-range-picker date-time-picker date-time-range-picker date-time-zone-picker", ("indicator", "glyph"), ("trigger", "inputText"), ("time-label|__label", "groupLabel"))
by_file("detail-item", ("info-content", "tooltip"), ("__label", "caption"), ("__value", "secondary"))
by_file("detail-section", ("title", "sectionTitle"), ("description", "secondary"))
by_file("detail-shell", ("__state", "body"))
by_file("dialog drawer alert-dialog form-dialog", ("strong|__title", "dialogTitle"), ("__header p|subtitle", "secondary"), ("__body|alert-dialog", "body"))
by_file("drag-drop", ("preview", "controlLabel"))
by_file("drag-number-field number-input time-input duration-input", ("steppers", "glyph"), ("duration-input-label-size|duration-input__label", "caption"), (".", "inputText"))
by_file("editable-label", (".", "inputText"))
by_file("editable-list", ("__count", "count"), ("__error", "helper"), ("item|font-size", "controlLabel"), ("window-label", "groupLabel"))
by_file("embed-input embed-preview", ("text-link", "controlLabel"), ("status|loading|error|empty", "secondary"))
by_file("empty-state", ("visual", "glyph"), ("h3", "sectionTitle"), (" p", "secondary"))
by_file("eyebrow", (".", "overline"))
by_file("field-set", ("legend", "groupLabel"), ("description", "secondary"))
by_file("field", ("info-content", "tooltip"), ("message|optional", "helper"), ("__label", "controlLabel"))
by_file("file-upload", ("hint.*size|hint-size", "split"), ("name", "secondary"), ("label", "controlLabel"))
by_file("filter-builder filter-toolbar order-by", ("drag-handle", "glyph"), ("pill|chip-font-size", "badge"), ("count", "count"), ("panel-title|__label", "groupLabel"), ("draft-field", "inputText"), ("empty", "secondary"), ("summary|item-label", "controlLabel"))
by_file("form-layout", ("errors", "helper"), ("description", "secondary"))
by_file("history-center", (" h2", "sectionTitle"), ("branch", "identifier"), ("meta", "caption"), ("badge", "badge"), ("input", "inputText"), ("entry-label|option-name", "controlLabel"), ("loading|rejection|header p", "secondary"))
by_file("inline-list-section", ("title", "sectionTitle"), ("count", "count"), ("empty", "body"))
by_file("licence", ("title", "sectionTitle"), ("machine-name", "identifier"), ("detail|marker", "caption"), (".", "secondary"))
by_file("list-card-counter", (".", "count"))
by_file("list-card", ("leading", "glyph"), ("eyebrow", "overline"), ("title(?!-)|__title", "cardTitle"), ("subtitle", "secondary"), ("context-meta", "keycap"), ("context-item", "controlLabel"), ("meta|footer", "caption"), ("sash", "badge"))
# Keep the title match from swallowing subtitle.
rules["list-card.css"] = tuple(sorted(rules["list-card.css"], key=lambda item: 0 if item[0] == "subtitle" else 1))
by_file("log-list", ("audit-icon", "glyph"), ("search", "inputText"), ("filter-btn|scroll-btn", "controlLabel"), ("entry|audit-details", "body"), ("pagination|audit-time", "caption"))
by_file("markdown-editor", ("tool-btn", "controlLabel"), ("__textarea", "codeBlock"), (" h[1-6]", "documentHeading"), (" code", "codeInline"), ("md-prose", "documentBody"))
by_file("media-browse-panel media-picker", ("label", "groupLabel"), ("meta|__state|empty", "secondary"))
by_file("media-preview", ("eyebrow", "overline"), ("title| h3", "sectionTitle"), ("description,.*caption|body-size", "split"), ("caption|meta", "caption"), ("description", "secondary"))
by_file("media-thumbnail", ("play", "glyph"), ("badge", "badge"), ("strong", "cardTitle"), (".", "secondary"))
by_file("menu-surface menubar", ("indicator", "glyph"), ("meta", "keycap"), ("item|trigger", "controlLabel"))
by_file("message-center", ("indicator", "glyph"), (" h2", "sectionTitle"), ("title", "cardTitle"), (" p,.*meta", "split"), ("meta", "caption"), (".", "secondary"))
by_file("meta-bar", ("code", "codeInline"))
by_file("meta-item", ("label", "caption"), ("value", "secondary"))
by_file("meter", ("ring-size", "geometry"), ("value", "value"))
by_file("metric-tile state-tile", ("arrow", "glyph"), ("value", "metricValue"), ("trend", "value"), ("label", "caption"))
by_file("mod-matrix-grid", ("header", "tableHeader"), (".", "tableCell"))
by_file("model-connection", ("group-title|__label", "groupLabel"), ("card__title|__provider", "cardTitle"), ("__title", "sectionTitle"), ("__count", "count"), ("__route", "identifier"), ("meta|route", "caption"), (".", "secondary"))
by_file("model-picker ref-select select", ("chevron|marker|indicator", "glyph"), ("badge", "badge"), ("description|empty|loading", "secondary"), ("group|axis-label", "groupLabel"), ("trigger|__control|__input", "inputText"), ("option", "controlLabel"))
by_file("nav-card", ("icon", "glyph"), ("__title", "cardTitle"), ("badge", "badge"), ("description", "secondary"))
by_file("navigation-menu", ("trigger", "controlLabel"))
by_file("page-header", ("section-title", "dormant"), ("back", "controlLabel"), ("eyebrow", "overline"), ("title-compact-size(?!.*subtitle)", "sectionTitle"), ("title-compact-size|has-title", "sectionTitle"), ("subtitle|body", "secondary"), ("section", "groupLabel"), ("title", "pageTitle"))
by_file("page-loading", ("cancel", "controlLabel"), ("message", "secondary"))
by_file("pagination", ("limit", "inputText"), ("button", "controlLabel"), (".", "caption"))
by_file("pagination-summary", (".", "caption"))
by_file("password-requirements", ("title", "cardTitle"), (".", "helper"))
by_file("picker-shell", ("__title", "dialogTitle"), (".", "secondary"))
by_file("pill", ("typography.*inherit", "badge"), (".", "badge"))
by_file("rating", (".", "glyph"))
by_file("region", ("__label", "groupLabel"))
by_file("relation-picker", ("__sep", "glyph"), ("level-label", "groupLabel"), ("count", "count"), ("__item-copy strong|title-size|__button|__item", "controlLabel"), ("desc-size", "split"), ("footer-note", "caption"), (".", "secondary"))
by_file("remediation-banner", ("strong", "cardTitle"), (" p", "body"))
by_file("rich-text", ("glyph", "glyph"), ("heading-option", "documentHeading"), ("heading-value", "controlLabel"), ("link-input", "inputText"), (" code", "codeInline"), (" pre", "codeBlock"), ("strong", "documentBody"), ("ProseMirror|renderer|rich-text-font-size", "documentBody"))
by_file("segmented-control", ("height", "geometry"), ("label", "controlLabel"))
by_file("selection-summary", ("chip", "badge"), ("clear", "controlLabel"), ("overflow", "count"), (".", "caption"))
by_file("sidebar-nav", ("end-label", "count"), ("group-title|title-font|title-letter-spacing", "groupLabel"), ("item", "controlLabel"))
by_file("slider-family value-readout video-player", (".", "value"))
by_file("split-button", (".", "controlLabel"))
by_file("status-bar status-indicator time-ago", (".", "caption"))
by_file("stepper", ("marker", "count"), (".", "controlLabel"))
by_file("table", ("__caption", "caption"), ("__header", "tableHeader"), (".", "tableCell"))
by_file("text-input", ("data-type=.*slug", "identifier"), ("__validation", "helper"), ("__count", "count"), ("__affix|__control|font-size|line-height", "inputText"))
by_file("text", ("caption", "caption"), ("label", "controlLabel"), (".", "body"))
by_file("theme-select", (".", "controlLabel"))
by_file("toast-stack", ("strong", "cardTitle"), ("__copy", "secondary"))
by_file("token-input", ("chip|token-input__token", "badge"), ("control|font-size|line-height", "inputText"))
by_file("tooltip", (".", "tooltip"))
by_file("tree", ("twisty|chevron", "glyph"), ("rename", "inputText"), ("row", "controlLabel"))
by_file("update-center", (" h2|title", "sectionTitle"), ("notice", "helper"), (".", "secondary"))
by_file("validation-summary", ("title", "cardTitle"), (".", "helper"))

def role(row):
    if row["dormant"]:
        return "dormant"
    if row["file"] == "page-header.css":
        declaration = row["declaration"] if row["local"] else row["selector"]
        if "subtitle-compact-size" in declaration or (not row["local"] and "subtitle-compact-size" in row["value"]) or "title-compact-size" in declaration or (not row["local"] and "title-compact-size" in row["value"]):
            return "sectionTitle"
    if row["file"] == "segmented-control.css" and row["property"] == "line-height" and row["kind"] == "calculated or em":
        return "geometry"
    subject = row["declaration"] if row["local"] else row["selector"]
    for needle, result in rules.get(row["file"], ()):
        if re.search(needle, subject):
            return result
    if re.search(r"icon|glyph|spinner|indicator|chevron", subject):
        return "glyph"
    if re.search(r"shortcut|__kbd", subject):
        return "keycap"
    if re.search(r"count", subject):
        return "count"
    if re.search(r"badge", subject):
        return "badge"
    if re.search(r"eyebrow", subject):
        return "overline"
    if re.search(r"caption|meta", subject):
        return "caption"
    if re.search(r"description|subtitle", subject):
        return "secondary"
    return "split"

# Size targets in px at root 16. Control-size targets list xs through xl.
target = {
    "pageTitle": 30, "sectionTitle": 19, "appTitle": 19, "cardTitle": 15, "dialogTitle": 24,
    "body": 15, "documentBody": 15, "secondary": 13, "helper": 13,
    "tableHeader": 13, "tableCell": 15, "tableSecondary": 13,
    "badge": 12, "count": 12, "caption": 12, "groupLabel": 13, "overline": 12,
    "codeBlock": 13, "codeInline": None, "identifier": 13, "keycap": 12,
    "value": 15, "metricValue": 24, "tooltip": 13,
    "controlLabel": [13,14,15,16,17], "inputText": [13,14,15,16,17],
    "documentHeading": None, "glyph": None, "geometry": None, "initials": None, "split": None, "dormant": None,
}
token_pixels = {"body-size": 14, "label-size": 13, "heading-size": 16, "caption-size": 11, "font-size-sm": 13}
variant = re.compile(r'\[data-(?:control-)?size\s*=\s*["\']?(xs|sm|md|lg|xl)\b')
def size_delta(row, assigned):
    value = row["value"]
    if assigned in ("dormant", "glyph", "geometry", "initials"):
        return "keep geometry / remove dormant after verification"
    if row["kind"] == "global":
        return "inherit"
    expected = target[assigned]
    if expected is None:
        return "review"
    if isinstance(expected, list):
        modes = set(variant.findall(row["selector"]))
        if len(modes) > 1:
            return "review"
        # An unqualified size declaration can serve the entire range. Validate the
        # old base value against md and retain an explicit follow-up for the cascade.
        mode = modes.pop() if modes else "md"
        expected = expected[["xs","sm","md","lg","xl"].index(mode)]
    absolute = re.fullmatch(r"(\d*\.\d+|\d+)rem", value)
    direct_token = re.fullmatch(r"var\(--poodle-typography-([\w-]+)(?:, [^()]+)?\)", value)
    if absolute:
        old = float(absolute[1]) * 16
    elif direct_token and direct_token[1] in token_pixels:
        old = token_pixels[direct_token[1]]
    else:
        return "review"
    if old == expected:
        return "same size"
    return "different size"

counts = collections.defaultdict(collections.Counter)
with open("migration.tsv", "w") as output:
    writer = csv.writer(output, delimiter="\t", lineterminator="\n")
    writer.writerow(["role", "property", "file", "line", "selector", "declaration", "value", "size_delta"])
    for row in rows:
        assigned = role(row)
        is_size = row["property"] == "font-size"
        delta = size_delta(row, assigned) if is_size else ""
        counts[assigned]["all"] += 1
        if is_size:
            counts[assigned]["direct" if not row["local"] else "local"] += 1
            counts[assigned][delta] += 1
        writer.writerow([assigned, row["property"], row["file"], row["line"], row["selector"], row["declaration"], row["value"], delta])

columns = ["all", "direct", "local", "same size", "different size", "review", "keep geometry / remove dormant after verification", "inherit"]
totals = collections.Counter()
for role_name, values in sorted(counts.items()):
    totals.update(values)
    print(role_name, *(values[k] for k in columns), sep="\t")
print("total", *(totals[k] for k in columns), sep="\t")
