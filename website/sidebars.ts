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
      items: ['plugins/beyond10x', 'plugins/aep-plan', 'plugins/aep-drive', 'plugins/ess-specify', 'plugins/workspace-hygiene', 'plugins/connectors'],
    },
    'trust-and-scope',
  ],
};

export default sidebars;
