import {themes as prismThemes} from 'prism-react-renderer';
import type {Config} from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';
import docsSystemPlugin, {ecosystemFooterGroup, ecosystemNavbarItems} from '@beyond10x/docs-system/docusaurus';

const config: Config = {
  title: 'Beyond10x Agent Plugins',
  tagline: 'Focused guidance for governed engineering agents.',
  favicon: 'img/mark.svg',
  future: {v4: true},
  url: 'https://beyond10x.github.io',
  baseUrl: '/agentplugins/',
  organizationName: 'beyond10x',
  projectName: 'agentplugins',
  trailingSlash: false,
  onBrokenLinks: 'throw',
  onBrokenAnchors: 'throw',
  markdown: {format: 'detect', hooks: {onBrokenMarkdownLinks: 'throw'}},
  i18n: {defaultLocale: 'en', locales: ['en']},
  presets: [
    [
      'classic',
      {
        docs: {
          path: 'docs',
          routeBasePath: 'docs',
          sidebarPath: './sidebars.ts',
          editUrl: 'https://github.com/beyond10x/agentplugins/edit/main/website/docs/',
          showLastUpdateTime: true,
        },
        blog: false,
        theme: {customCss: './src/css/custom.css'},
      } satisfies Preset.Options,
    ],
  ],
  plugins: [docsSystemPlugin],
  themeConfig: {
    image: 'img/social-card.svg',
    metadata: [{name: 'keywords', content: 'agent plugins, AEP, ESS, planning, schema validation'}],
    colorMode: {defaultMode: 'dark', respectPrefersColorScheme: true},
    navbar: {
      title: 'Agent Plugins',
      hideOnScroll: true,
      logo: {alt: 'Agent Plugins mark', src: 'img/mark.svg'},
      items: [
        ...ecosystemNavbarItems(),
        {to: '/docs/choose-a-plugin', label: 'Choose', position: 'left'},
        {to: '/docs/install', label: 'Install', position: 'left'},
        {to: '/docs/plugins/beyond10x', label: 'Reference', position: 'left'},
        {href: 'https://github.com/beyond10x/agentplugins', label: 'GitHub', position: 'right'},
      ],
    },
    footer: {
      style: 'dark',
      links: [
        ecosystemFooterGroup(),
        {title: 'Use it', items: [
          {label: 'Choose a plugin', to: '/docs/choose-a-plugin'},
          {label: 'Install the marketplace', to: '/docs/install'},
        ]},
        {title: 'Plugins', items: [
          {label: 'Beyond10x', to: '/docs/plugins/beyond10x'},
          {label: 'AEP Plan', to: '/docs/plugins/aep-plan'},
          {label: 'AEP Drive', to: '/docs/plugins/aep-drive'},
          {label: 'ESS Specify', to: '/docs/plugins/ess-specify'},
        ]},
        {title: 'Project', items: [
          {label: 'GitHub repository', href: 'https://github.com/beyond10x/agentplugins'},
          {label: 'Releases', href: 'https://github.com/beyond10x/agentplugins/releases'},
          {label: 'Apache-2.0 license', href: 'https://github.com/beyond10x/agentplugins/blob/main/LICENSE'},
        ]},
      ],
      copyright: `© ${new Date().getFullYear()} beyond10x · Focused guidance, explicit scope.`,
    },
    prism: {theme: prismThemes.github, darkTheme: prismThemes.dracula, additionalLanguages: ['yaml', 'json', 'bash']},
  } satisfies Preset.ThemeConfig,
};

export default config;
