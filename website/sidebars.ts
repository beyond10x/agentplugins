import type {SidebarsConfig} from '@docusaurus/plugin-content-docs';

const sidebars: SidebarsConfig = {
  docs: [
    'intro',
    'choose-a-plugin',
    'install',
    {
      type: 'category',
      label: 'Plugin reference',
      items: ['plugins/aep-planning', 'plugins/adp', 'plugins/ess-schema'],
    },
    'trust-and-scope',
  ],
};

export default sidebars;
