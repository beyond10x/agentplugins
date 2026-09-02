import type {SidebarsConfig} from '@docusaurus/plugin-content-docs';

const sidebars: SidebarsConfig = {
  docs: [
    'intro',
    'choose-a-plugin',
    'install',
    'golden-path',
    {
      type: 'category',
      label: 'Plugin reference',
      items: ['plugins/beyond10x', 'plugins/aep-planning', 'plugins/adp', 'plugins/ess-schema', 'plugins/workspace-hygiene'],
    },
    'trust-and-scope',
  ],
};

export default sidebars;
