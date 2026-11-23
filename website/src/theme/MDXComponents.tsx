import type {ComponentProps, ReactNode} from 'react';
import MDXComponents from '@theme-original/MDXComponents';

// A Markdown table scrolls sideways on a narrow window (Infima `table {display: block; overflow: auto}`).
// tabIndex makes that scroll region reachable by keyboard (axe-core `scrollable-region-focusable`, UXF-006).
function Table(props: ComponentProps<'table'>): ReactNode {
  return <table tabIndex={0} {...props} />;
}

export default {
  ...MDXComponents,
  table: Table,
};
