import React from "react"
import clsx from "clsx"
import styles from "./styles.module.css"

type FeatureItem = {
  title: string
  Svg: React.ComponentType<React.ComponentProps<"svg">>
  description: JSX.Element
}

const FeatureList: FeatureItem[] = [
  {
    title: "Ease of Use",
    Svg: require("@site/static/img/undraw_programming.svg").default,
    description: (
      <>
        Sidex was specifically designed with a focus on developer ergonomics and
        ease of use to get your project up and running in no time.
      </>
    ),
  },
  {
    title: "Powerful Type System",
    Svg: require("@site/static/img/undraw_abstract.svg").default,
    description: (
      <>
        Sidex's powerful type system enables you to specify rich data models and
        build APIs at scale. We'll take care of the chores and boilerplate.
      </>
    ),
  },
  {
    title: "Format- and Language-Agnostic",
    Svg: require("@site/static/img/undraw_choice.svg").default,
    description: (
      <>
        Sidex is format- and language-agnostic so you can choose the right tools
        for the job. It is designed to be extended and adapted to your needs.
      </>
    ),
  },
]

function Feature({ title, Svg, description }: FeatureItem) {
  return (
    <div className={clsx("col col--4")}>
      <div className="text--center">
        <Svg className={styles.featureSvg} role="img" />
      </div>
      <div className="text--center padding-horiz--md">
        <h3>{title}</h3>
        <p>{description}</p>
      </div>
    </div>
  )
}

export default function HomepageFeatures(): JSX.Element {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  )
}
