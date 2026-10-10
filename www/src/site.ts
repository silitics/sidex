import "./styles/site.css";
import grammar from "../../editors/vscode/sidex-grammar.tmGrammar.json";
import { getCollection } from "astro:content";
import { faGithub } from "@fortawesome/free-brands-svg-icons";
import type { Brand, NavItem, FooterColumn } from "@silitics/astro-theme";
import { getHighlighter } from "@silitics/astro-theme/shiki";
import type { DocsConfig } from "@silitics/astro-docs";
import { buildNav, memoizeDocsConfig } from "@silitics/astro-docs";

await getHighlighter({
  customLangs: [
    { ...grammar, name: "sidex", fileTypes: ["sidex"] },
  ] as NonNullable<Parameters<typeof getHighlighter>[0]>["customLangs"],
});

export const brand: Brand = {
  name: "Sidex",
  logo: "/img/logo.svg",
  href: "/",
  tagline: "A data modeling language for cross-stack data exchange.",
  titleTemplate: "%s | Sidex",
  docsTitleTemplate: "%s | Sidex Docs",
  locale: "en",
};

export const nav: NavItem[] = [
  { label: "Documentation", href: "/docs/" },
  { label: "Getting Started", href: "/docs/getting-started/" },
];

export const headerActions: NavItem[] = [
  {
    label: "GitHub",
    href: "https://github.com/silitics/sidex",
    external: true,
    icon: faGithub,
  },
];

export const footerColumns: FooterColumn[] = [
  {
    title: "Learn",
    links: [
      { label: "Getting Started", href: "/docs/getting-started/" },
      { label: "Language Guide", href: "/docs/language/types/" },
      { label: "CLI Reference", href: "/docs/reference/cli/" },
    ],
  },
  {
    title: "Project",
    links: [
      {
        label: "Source Code",
        href: "https://github.com/silitics/sidex",
        external: true,
      },
      {
        label: "Report an Issue",
        href: "https://github.com/silitics/sidex/issues",
        external: true,
      },
      { label: "Silitics", href: "https://silitics.com", external: true },
    ],
  },
  {
    title: "Legal",
    links: [
      {
        label: "Privacy Policy",
        href: "https://silitics.com/privacy-policy",
        external: true,
      },
      {
        label: "Imprint",
        href: "https://silitics.com/impressum",
        external: true,
      },
    ],
  },
];

export const getDocsConfig = memoizeDocsConfig(
  async (): Promise<DocsConfig> => {
    const entries = await getCollection("docs");
    return {
      versions: [
        {
          slug: "main",
          label: "Main",
          status: "current",
          default: true,
          pathPrefix: "/docs/",
          editBaseUrl:
            "https://github.com/silitics/sidex/edit/main/www/src/content/docs",
          nav: buildNav(entries, {
            rootGroupTitle: "Start Here",
            groupOrder: ["", "language", "generation", "guides", "reference"],
            groupTitles: { generation: "Code Generation" },
          }).map((group) => ({
            ...group,
            links: group.links.map((link) =>
              link.slug === "index" ? { ...link, href: "/docs/" } : link,
            ),
          })),
        },
      ],
    };
  },
);
