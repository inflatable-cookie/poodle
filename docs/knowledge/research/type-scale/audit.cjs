const fs = require('node:fs');
const path = require('node:path');
const postcss = require(require.resolve('postcss', {
  paths: [process.env.POODLE_DEPS || process.cwd()]
}));
const root = path.resolve(process.argv[2] || 'source');
const output = path.resolve(process.argv[3] || '.');
const dir = 'packages/core/src/styles';
const properties = ['font-size', 'font-weight', 'line-height', 'letter-spacing', 'text-transform', 'font-family'];
const rows = [];
for (const file of fs.readdirSync(path.join(root, dir), { recursive: true }).filter(file => file.endsWith('.css')).sort()) {
  const ast = postcss.parse(fs.readFileSync(path.join(root, dir, file), 'utf8'), { from: file });
  ast.walkDecls(decl => {
    const selectors = [];
    for (let node = decl.parent; node.type !== 'root'; node = node.parent) {
      if (node.type === 'rule') selectors.unshift(node.selector);
      if (node.type === 'atrule') selectors.unshift('@' + node.name + ' ' + node.params);
    }
    const selector = selectors.join(' > ').replace(/\s+/g, ' ');
    const value = decl.value.replace(/\s+/g, ' ').trim();
    rows.push({
      file, line: decl.source.start.line, selector, declaration: decl.prop,
      value, refs: [...new Set(Array.from(value.matchAll(/\bvar\(\s*(--[\w-]+)/g), m => m[1]))],
      size: /\[\s*data-(?:control-)?size\b/.test(selector),
      density: /\[\s*data-density\b/.test(selector),
    });
  });
}
const custom = new Map();
for (const row of rows.filter(row => row.declaration.startsWith('--'))) {
  if (!custom.has(row.declaration)) custom.set(row.declaration, []);
  custom.get(row.declaration).push(row);
}
const consumers = new Map();
function follow(name, prop) {
  if (!custom.has(name)) return;
  if (!consumers.has(name)) consumers.set(name, new Set());
  const kinds = consumers.get(name);
  if (kinds.has(prop)) return;
  kinds.add(prop);
  for (const definition of custom.get(name)) for (const ref of definition.refs) follow(ref, prop);
}
for (const row of rows.filter(row => properties.includes(row.declaration))) {
  for (const ref of row.refs) follow(ref, row.declaration);
}
// Explicit dormant type definitions. These names describe PageHeader's section
// title sizes and do not appear as var() reads anywhere under styles/.
const dormant = ['--poodle-page-header-section-title-size', '--poodle-page-header-section-title-compact-size'];
for (const name of dormant) follow(name, 'font-size');
function classify(value) {
  const global = /^(inherit|initial|unset|revert(?:-layer)?)$/;
  if (global.test(value)) return 'global';
  const varsOnly = value.replace(/var\(\s*--[\w-]+\s*(?:,\s*\)|\))/g, '');
  if (varsOnly === '' || /^\s*(?:var\(\s*--[\w-]+\s*,\s*)+\)+\s*$/.test(varsOnly)) return 'var only';
  if (/\b(?:calc|min|max|clamp)\(|(?:^|[^\w-])(?:\d*\.)?\d+em\b/.test(value)) return 'calculated or em';
  if (/^var\(/.test(value)) return 'literal fallback';
  return 'literal';
}
const census = rows.flatMap(row => {
  const types = consumers.get(row.declaration) || (properties.includes(row.declaration) ? [row.declaration] : []);
  return [...types].map(property => ({
    ...row, property, kind: classify(row.value),
    local: row.declaration.startsWith('--'), dormant: dormant.includes(row.declaration),
  }));
});
const aggreg = (items, fn) => Object.entries(items.reduce((out, item) => {
  const key = fn(item);
  out[key] = (out[key] || 0) + 1;
  return out;
}, Object.create(null))).sort(([a,n],[b,m]) => m-n || a.localeCompare(b));
const values = Object.fromEntries(properties.map(property => [property, Object.fromEntries(aggreg(census.filter(r => r.property === property), r => r.value).map(([value, n]) => {
  const list = census.filter(r => r.property === property && r.value === value);
  return [value, { count: n, direct: list.filter(r => !r.local).length, files: Object.fromEntries(aggreg(list, r => r.file)) }];
}))]));
const size = census.filter(r => r.property === 'font-size');
console.log('Files:', new Set(rows.map(r => r.file)).size);
for (const property of properties) for (const local of [false, true]) {
  const list = census.filter(r => r.property === property && r.local === local);
  console.log(property, local ? 'local' : 'direct', list.length, Object.fromEntries(aggreg(list, r => r.kind)));
}
for (const local of [false,true]) {
  console.log('Axes', local ? 'local' : 'direct', aggreg(size.filter(r => r.local === local), r => r.size ? (r.density ? 'both' : 'size') : (r.density ? 'density' : 'neither')));
  console.log('Largest size literal counts', local ? 'local' : 'direct', aggreg(size.filter(r => r.local === local && r.kind === 'literal'), r => r.file).slice(0,15));
}
console.log('Combined largest size literal counts', aggreg(size.filter(r => r.kind === 'literal'), r => r.file).slice(0,15));
console.log('Combined largest all property literal counts', aggreg(census.filter(r => r.kind === 'literal'), r => r.file).slice(0,15));
console.log('Shorthands', aggreg(rows.filter(r => r.declaration === 'font'), r => r.value));
console.log('Shared types', [...consumers].filter(([, kinds]) => kinds.size > 1).map(([name, kinds]) => [name, [...kinds]]));
console.log('Unclassified type-shaped variables', [...custom.keys()].filter(name => /font-size|letter-spacing|line-height|font-weight|font-family|text-transform/i.test(name) && !consumers.has(name)));
console.log('Distinct values', properties.map(prop => [prop, Object.keys(values[prop]).length]));
console.log('Density size declarations', size.filter(r => r.density));
fs.writeFileSync(path.join(output, 'declarations.json'), JSON.stringify(census, null, 2) + '\n');
fs.writeFileSync(path.join(output, 'values.json'), JSON.stringify(values, null, 2) + '\n');
fs.writeFileSync(path.join(output, 'declarations.tsv'), ['property\tsource\tselector\tdeclaration\tvalue\tclassification\taxis', ...census.map(r => [
  r.property, dir + '/' + r.file + ':' + r.line, r.selector, r.declaration, r.value, r.kind,
  r.size ? (r.density ? 'both' : 'size') : (r.density ? 'density' : 'neither'),
].join('\t'))].join('\n') + '\n');

const tokenDir = path.join(root, 'packages/core/src/tokens/generated/css');
const tokens = new Map();
for (const file of ['poodle-tokens.css', ...['xs', 'sm', 'md', 'lg', 'xl'].map(size => 'poodle-control-size-' + size + '.css')]) {
  postcss.parse(fs.readFileSync(path.join(tokenDir, file), 'utf8')).walkDecls(/^--/, decl => {
    if (!tokens.has(decl.prop)) tokens.set(decl.prop, new Set());
    tokens.get(decl.prop).add(decl.value);
  });
}
function expand(value, trail = []) {
  const match = /var\(\s*(--[\w-]+)\s*(?:,\s*([^()]*))?\)/.exec(value);
  if (!match) return [value];
  const [, name, fallback] = match;
  if (trail.includes(name)) return [value];
  const options = custom.has(name) ? new Set(custom.get(name).map(row => row.value)) : tokens.get(name) || (fallback ? [fallback.trim()] : []);
  if (!options.size && !options.length) return [value];
  return [...new Set([...options].flatMap(option => expand(option, [...trail,name]).flatMap(result => {
    if (result === match[0]) return [value];
    return expand(value.slice(0, match.index) + result + value.slice(match.index + match[0].length), [...trail,name]);
  })))];
}
const fmt = n => Number(n.toFixed(12)).toString();
function dimension(raw, property) {
  const match = /^(-?(?:\d*\.)?\d+)(rem|px|em)?$/.exec(raw);
  if (match) {
    const n = Number(match[1]);
    switch (match[2]) {
      case 'rem': return fmt(n * 16) + 'px';
      case 'px': return fmt(n) + 'px';
      case 'em': return fmt(n) + (property === 'font-size' ? ' × P' : ' × F');
      default: return property === 'line-height' ? fmt(n) + ' × F' : n === 0 ? '0px' : fmt(n);
    }
  }
  const calc = /^calc\(([^\s]+) ([*+-]) ([^\s]+)\)$/.exec(raw);
  if (calc) {
    const [, left, op, right] = calc;
    const a = /^(-?(?:\d*\.)?\d+)(rem|px|em)?$/.exec(left);
    const b = /^(-?(?:\d*\.)?\d+)(rem|px|em)?$/.exec(right);
    if (a && b && op === '*' && !b[2]) return dimension(fmt(+a[1] * +b[1]) + (a[2] || ''), property);
    if (a && b && op !== '*' && a[2] === b[2]) return dimension(fmt(+a[1] + (op === '+' ? +b[1] : -b[1])) + (a[2] || ''), property);
  }
  return raw;
}
function converted(value, property) {
  const results = expand(value).map(item => ['font-size','line-height','letter-spacing'].includes(property) ? dimension(item, property) : item);
  return [...new Set(results)].map(result => {
    const relative = /^([0-9.]+) × P$/.exec(result);
    return relative && property === 'font-size' ? result + ' (' + fmt(Number(relative[1]) * 16) + 'px at P = 16px)' : result;
  }).join(', ');
}
const pretty = property => Object.entries(values[property]).sort(([a],[b]) => a.localeCompare(b,'en',{numeric:true}));
const formatCode = s => '\x60' + s + '\x60';
let md = '# Poodle typography declaration inventory\n\n' +
  'Paths below are relative to `packages/core/src/styles/`. Counts include direct declarations and local assignments as separate authored occurrences. CSS values are grouped by spelling after whitespace normalization.\n\n' +
  'The resolved column uses a 16px root. P is the parent font size and F is the element font size. For local variables it lists possible authored values, independent of selector matching or consumer overrides. Defined tokens show their values when Poodle tokens load. Inline fallback values remain visible in the authored column.\n';
for (const property of properties) {
  md += '\n## ' + property + '\n\n| Authored | Resolved value | Direct | Local | Files (uses) |\n|---|---|---:|---:|---|\n';
  for (const [value, entry] of pretty(property)) {
    const resolved = converted(value, property).replace(/\|/g, '\\|');
    const files = Object.entries(entry.files).sort(([a],[b]) => a.localeCompare(b)).map(([file,n]) => file + (n === 1 ? '' : ' ×' + n));
    md += '| ' + formatCode(value) + ' | ' + resolved + ' | ' + entry.direct + ' | ' + (entry.count - entry.direct) + ' | ' + files.join(', ') + ' |\n';
  }
}
fs.writeFileSync(path.join(output, 'inventory.md'), md);
