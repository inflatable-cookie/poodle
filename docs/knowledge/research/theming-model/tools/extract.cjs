// Load the engine (and the preset data) straight out of index.html, so the tests run the shipped
// code. During development HTML and PRESETS may point at plain .js files instead.
const fs = require("fs");
const path = require("path");
const read = (file, id) => {
  const text = fs.readFileSync(file, "utf8");
  const m = text.match(new RegExp('<script id="' + id + '">([\\s\\S]*?)</script>'));
  return m ? m[1] : text;
};
const html = process.env.HTML || path.join(__dirname, "..", "index.html");
const engine = new Function(read(html, "resolve-theme") + "\nreturn PoodleTheme;")();
engine.Presets = new Function(read(process.env.PRESETS || html, "presets") + "\nreturn PoodlePresets;")();
module.exports = engine;
