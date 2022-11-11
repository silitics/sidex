import React from "react"
import clsx from "clsx"
import Link from "@docusaurus/Link"
import useDocusaurusContext from "@docusaurus/useDocusaurusContext"
import Layout from "@theme/Layout"
import HomepageFeatures from "@site/src/components/HomepageFeatures"
import Admonition from "@theme/Admonition"

import styles from "./index.module.css"

function HomepageHeader() {
  const { siteConfig } = useDocusaurusContext()
  return (
    <header className={clsx("hero hero--primary", styles.heroBanner)}>
      <div className="container">
        <h1 className="hero__title">{siteConfig.title}</h1>
        <p className="hero__subtitle">{siteConfig.tagline}</p>
        <p style={{ maxWidth: "80ch", margin: "1.5em auto" }}>
          Sidex is a format- and language-agnostic data modeling and API
          definition framework with a focus on simplicity, extensibility, and
          developer ergonomics. Sidex aims to simplify data exchange between
          different programming languages and platforms via potentially multiple
          interchange formats and storage backends.
        </p>
        <div className={styles.buttons}>
          <Link
            className="button button--secondary button--lg"
            to="/docs/getting-started"
          >
            Get Started 🚀
          </Link>
        </div>
      </div>
    </header>
  )
}

export default function Home(): JSX.Element {
  const { siteConfig } = useDocusaurusContext()
  return (
    <Layout title="Home" description={siteConfig.tagline}>
      <HomepageHeader />
      <main>
        <div style={{ maxWidth: "80ch", padding: "2rem 0", margin: "0 auto" }}>
          <Admonition type="caution" title="🚧 Under Construction 🚧">
            <p>
              Sidex <strong>is still under construction</strong>. In particular,
              the functionality described here may not exist yet, may change
              considerably in the future, or may even be completely abandoned at
              a later point in time. We are actively working on the design,
              features, and vision of Sidex. ⚠️
            </p>
            <p>
              If you have any ideas, suggestions, feedback regarding the early
              prototype, or anything else you like to discuss, please reach out
              to us by starting a{" "}
              <a
                href="https://github.com/silitics/sidex/discussions"
                target="_blank"
              >
                discussion on GitHub
              </a>
              .
            </p>
          </Admonition>
        </div>
        <HomepageFeatures />
      </main>
    </Layout>
  )
}
