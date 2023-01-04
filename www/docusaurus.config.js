/** @type {import('@docusaurus/types').Config} */
const config = {
  title: "Sidex",
  tagline: "Simplifies data exchange your entire stack.",
  url: "https://oss.silitics.com/",
  baseUrl: "/sidex/",

  onBrokenLinks: "warn",
  onBrokenMarkdownLinks: "warn",

  // We do not care about old browsers not supporting SVG.
  favicon: "img/logo.svg",

  organizationName: "silitics",
  projectName: "sidex",

  i18n: {
    defaultLocale: "en",
    locales: ["en"],
  },

  presets: [
    [
      "classic",
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          sidebarPath: require.resolve("./sidebars.js"),
          editUrl: "https://github.com/silitics/sidex/tree/main/www/",
        },
        blog: {
          showReadingTime: true,
          editUrl: "https://github.com/silitics/sidex/tree/main/www/",
        },
        theme: {
          customCss: require.resolve("./src/css/custom.css"),
        },
      }),
    ],
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      colorMode: {
        defaultMode: "dark",
        disableSwitch: true,
        respectPrefersColorScheme: false,
      },
      // Display a warning that Sidex is still under construction on every page.
      announcementBar: {
        id: "under_construction",
        content:
          "🚨 <strong>UNDER CONSTRUCTION</strong>: Sidex <strong>is still under construction</strong>. In particular, the functionality described here may not exist yet, may change considerably in the future, or may even be completely abandoned at a later point in time! 🚨",
        backgroundColor: "#FFFF00",
        textColor: "#000000",
        isCloseable: false,
      },
      navbar: {
        title: "Sidex",
        logo: {
          alt: "Sidex Logo",
          src: "img/logo.svg",
        },
        items: [
          {
            type: "doc",
            docId: "getting-started",
            position: "left",
            label: "Docs",
          },
          { to: "/blog", label: "Blog", position: "left" },
          {
            href: "https://github.com/silitics/sidex",
            label: "GitHub",
            position: "right",
          },
        ],
      },
      footer: {
        style: "dark",
        links: [
          {
            title: "Docs",
            items: [
              {
                label: "Getting Started",
                to: "/docs/getting-started",
              },
              {
                label: "User Guide",
                to: "/docs/guide",
              },
              {
                label: "Interchange Formats",
                to: "/docs/formats",
              },
              {
                label: "Supported Languages",
                to: "/docs/languages",
              },
            ],
          },
          {
            title: "Community",
            items: [
              {
                label: "GitHub",
                href: "https://github.com/siltics/sidex",
              },
              {
                label: "Discussions",
                href: "https://github.com/silitics/sidex/discussions",
              },
            ],
          },
          {
            title: "More",
            items: [
              {
                label: "Blog",
                to: "/blog",
              },
            ],
          },
          {
            title: "Legal",
            items: [
              {
                // German and EU law require us to have a privacy policy.
                label: "Privacy Policy",
                to: "/privacy-policy",
              },
              {
                // German law requires us to have an Impressum.
                label: "Impressum",
                to: "/impressum",
              },
            ],
          },
        ],
        copyright: `<div>Made with ❤️ for OSS</div><div>Copyright © ${new Date().getFullYear()} <a href="https://silitics.com">Silitics GmbH</a></div><div>Built with Docusaurus</div>`,
      },
      prism: {
        theme: require("prism-react-renderer/themes/oceanicNext"),
        additionalLanguages: ["rust", "toml"],
      },
    }),
}

module.exports = config
