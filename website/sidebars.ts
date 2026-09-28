import type {SidebarsConfig} from '@docusaurus/plugin-content-docs';

const sidebars: SidebarsConfig = {
  docs: [
    'intro',
    'choose-a-plugin',
    'install',
    {
      type: 'category',
      label: 'Tutorials',
      items: ['tutorials/first-ess-specification'],
    },
    'golden-path',
    'structure',
    {
      type: 'category',
      label: 'Plugin reference',
      items: ['plugins/b10x', 'plugins/aep', 'plugins/ess', 'plugins/worktree', 'plugins/connectors'],
    },
    'trust-and-scope',
  ],
};

export default sidebars;
