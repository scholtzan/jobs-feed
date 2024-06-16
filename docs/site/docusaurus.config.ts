import {themes as prismThemes} from 'prism-react-renderer';
import type {Config} from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';

const config: Config = {
  title: 'Jobs Feed',
  tagline: 'Track job postings across various career pages',
  favicon: 'img/favicon.svg',

  url: 'https://scholtzan.github.io',

  baseUrl: '/jobs-feed/',

  organizationName: 'scholtzan',
  projectName: 'Jobs Feed',

  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',

  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },

  presets: [
    [
      'classic',
      {
        docs: {
          sidebarPath: './sidebars.ts',
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl:
            'https://github.com/scholtzan/jobs-feed/tree/main/docs/site',
        },
        blog: {
          showReadingTime: true,
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl:
            'https://github.com/scholtzan/jobs-feed/tree/main/docs/site',
        },
        theme: {
          customCss: './src/css/custom.css',
        },
      } satisfies Preset.Options,
    ],
  ],

  themeConfig: {
    image: 'img/favicon.svg',
    navbar: {
      title: 'Jobs Feed',
      logo: {
        alt: 'Jobs Feed logo',
        src: 'img/favicon.svg',
      },
      items: [
        {
          to: '/docs/quick-start',
          position: 'left',
          label: 'Quick Start',
        },
        {
          type: 'docSidebar',
          sidebarId: 'tutorialSidebar',
          position: 'left',
          label: 'Docs',
        },
        {
          href: 'https://github.com/scholtzan/jobs-feed',
          label: 'GitHub',
          position: 'right',
        },
      ],
    },
    footer: {
      copyright: `Copyright © ${new Date().getFullYear()} Jobs Feed`,
      links: [
        {
          label: "License",
          href: "https://github.com/scholtzan/jobs-feed/blob/main/LICENSE"
        },
        {
          label: "Github",
          href: "https://github.com/scholtzan/jobs-feed"
        }
      ]
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
