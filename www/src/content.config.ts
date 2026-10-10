import { defineCollection } from "astro:content";
import { glob } from "astro/loaders";
import { docsSchema } from "@silitics/astro-docs";

const docs = defineCollection({
  loader: glob({
    pattern: "**/*.md",
    base: "./src/content/docs",
    generateId: ({ entry }) =>
      entry.replace(/\.md$/, "").replace(/\/index$/, ""),
  }),
  schema: docsSchema(),
});

export const collections = { docs };
