import fs from "fs";

// static SOME_THEME: &[u8; include_bytes!("../../themes/barf.tmTheme").len()] =include_bytes!("../../themes/barf.tmTheme");

const contents = fs.readFileSync("./themes.rs", { encoding: "utf8" });
const lines = contents.split("\n");

const new_lines = lines.map((line: string) => {
  const name = line.split(".").shift();
  const var_name = name
    .toUpperCase()
    .replaceAll("'", "")
    .replaceAll("[", "")
    .replaceAll("]", "")
    .replaceAll("(", "")
    .replaceAll(")", "")
    .replaceAll("&", "")
    .replaceAll("-", "")
    .trim()
    .replaceAll(" ", "_")
    .replaceAll("__", "_");
  // return `static ${var_name}: &[u8; include_bytes!("../themes/${line}").len()] = include_bytes!("../themes/${line}");`;
  return `theme_map.insert("${var_name}", ${var_name});`;
});

fs.writeFileSync("themes3.rs", new_lines.join("\n"));
