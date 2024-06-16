import clsx from 'clsx';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import HomepageFeatures from '@site/src/components/HomepageFeatures';
import Heading from '@theme/Heading';

import styles from './index.module.css';

function HomepageHeader() {
  return (
    <header className={clsx('hero hero--primary', styles.heroBanner, styles.hero, 'hero-jobs-feed')}>
      <div className="container" style={{ zIndex: 1 }}>
        <div className={clsx('row', styles.heroTitle)}>
          <h1 className={clsx('hyphens-auto', styles.heroName)}>Jobs Feed</h1>
          <p className={clsx(styles.heroDescription)}>
            Streamline your job search by tracking job postings from career pages that align with your skills and preferences.
          </p>

          <div className={styles.heroButtons}>
            <Link
              className={clsx('button button--secondary button--lg', styles.heroButton)}
              to="/docs/about"
            >
              Learn More

            </Link>
          </div>
        </div>
        <div className="row">
          <img className={clsx(styles.heroImage)} src="img/main-screen.png"></img>
        </div>
      </div>
    </header>
  );
}

export default function Home() {
  return (
    <Layout
      title={'Home'}
      description="Simplify the management of your server with Homarr - a sleek, modern dashboard that puts all of your apps and services at your fingertips. With Homarr, you can access and control everything in one convenient location. Homarr seamlessly integrates with the apps you've added, providing you with valuable information and giving you complete control. Installation is a breeze, and Homarr supports a wide range of deployment methods."
    >
      <HomepageHeader />



      <main className='mx-auto w-full md:w-2/3 ps-10 pr-10'>
      
        <div className={clsx('hyphens-auto', "container")}>
          <img src='img/favicon.svg' className={clsx( styles.descriptionIcon)}/>
          <h1 className={clsx(styles.heroName, styles.descriptionHeader)}>Jobs Feed</h1>
          <h2 className={clsx(styles.descriptionSubHeader)}>What is Jobs Feed?</h2>
          <p className={clsx(styles.descriptionText)}>
            Jobs Feed is a self-hosted web app which extracts job postings from configured career web pages.
            Filters can be configured to only get postings that match the desired skills and preferences.
          </p>
          <div>
            <Link
              className={clsx('button button--secondary button--lg', styles.heroButton, styles.descriptionButton)}
              to="/docs/about"
            >
              Learn More

            </Link>
          </div>

          <HomepageFeatures />
        </div>

      </main>
    </Layout>
  );
}
