import { readFileSync } from "node:fs";
import { defineConfig } from "astro/config";
import tailwindcss from "@tailwindcss/vite";
import { markdownConfig } from "@silitics/astro-theme/markdown";
import { docsSearch } from "@silitics/astro-docs/search";

const grammar = JSON.parse(
  readFileSync(
    new URL("../editors/vscode/sidex-grammar.tmGrammar.json", import.meta.url),
    "utf8",
  ),
);
const markdown = markdownConfig();
markdown.shikiConfig.langs = [{ ...grammar, name: "sidex" }];

export default defineConfig({
  site: "https://oss.silitics.com",
  base: "/sidex/",
  trailingSlash: "ignore",
  redirects: { "/docs/index": "/sidex/docs/" },
  vite: { plugins: [tailwindcss()] },
  integrations: [docsSearch()],
  markdown,
});
