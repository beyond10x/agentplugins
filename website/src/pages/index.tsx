import type {ReactNode} from 'react';
import Link from '@docusaurus/Link';
import Layout from '@theme/Layout';
import Heading from '@theme/Heading';
import styles from './index.module.css';

const plugins = [
  ['AEP Planning', 'Plan, decompose, review, and reverse-engineer governed work.', '/docs/plugins/aep-planning'],
  ['ADP', 'Scope stories and coordinate implementation waves with adversarial review.', '/docs/plugins/adp'],
  ['ESS Schema', 'Validate typed system specifications and guide deterministic projections.', '/docs/plugins/ess-schema'],
];

export default function Home(): ReactNode {
  return (
    <Layout title="Focused engineering guidance" description="The curated beyond10x agent plugin marketplace.">
      <main>
        <header className={styles.hero}>
          <p className={styles.eyebrow}>Curated marketplace · beyond10x</p>
          <Heading as="h1">Give each agent only the engineering guidance it needs.</Heading>
          <p className={styles.lead}>
            Three focused plugins keep planning, development coordination, and executable-system
            modeling separate—without bundling credentials or hidden authority.
          </p>
          <div className={styles.actions}>
            <Link className="button button--primary button--lg" to="/docs/choose-a-plugin">Choose a plugin</Link>
            <Link className="button button--secondary button--lg" to="/docs/install">Install</Link>
          </div>
        </header>
        <section className={styles.grid} aria-label="Available plugins">
          {plugins.map(([name, description, href]) => (
            <article className={styles.card} key={name}>
              <Heading as="h2">{name}</Heading>
              <p>{description}</p>
              <Link to={href}>Read the reference →</Link>
            </article>
          ))}
        </section>
      </main>
    </Layout>
  );
}
