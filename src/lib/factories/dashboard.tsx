import React from "react";

// XXX: this file was produced from a prompt, and isn't 100% accurate yet, it needs:
// - to use the real components
// - to use the object returned by the "backend"
// - each component should have a mapping from config.properties to props

import {
  DashboardComponentConfig,
  DashboardComponent,
} from "~lib/types/dashboard";

const Layout: React.FC<{ children?: React.ReactNode }> = ({ children }) => (
  <div>{children}</div>
);
const Stack: React.FC<{ children?: React.ReactNode }> = ({ children }) => (
  <div>{children}</div>
);
const Text: React.FC = () => <div>Text</div>;
const Stat: React.FC = () => <div>Stat</div>;
const Panel: React.FC<{ children?: React.ReactNode }> = ({ children }) => (
  <div>{children}</div>
);
const Range: React.FC = () => <div>Range</div>;

export class DashboardFactory {
  static createComponent(config: DashboardComponentConfig): DashboardComponent {
    switch (config.type) {
      case "layout":
        return new LayoutComponent(config);
      case "stack":
        return new StackComponent(config);
      case "text":
        return new TextComponent(config);
      case "stat":
        return new StatComponent(config);
      case "panel":
        return new PanelComponent(config);
      case "range":
        return new RangeComponent(config);
      default:
        throw new Error(`Unknown component type: ${config.type}`);
    }
  }
}

class LayoutComponent implements DashboardComponent {
  constructor(private config: DashboardComponentConfig) {}

  render() {
    const children = this.config.children?.map((child) =>
      DashboardFactory.createComponent(child).render(),
    );
    return <Layout>{children}</Layout>;
  }
}

class StackComponent implements DashboardComponent {
  constructor(private config: DashboardComponentConfig) {}

  render() {
    const children = this.config.children?.map((child) =>
      DashboardFactory.createComponent(child).render(),
    );
    return <Stack>{children}</Stack>;
  }
}

class TextComponent implements DashboardComponent {
  constructor(private config: DashboardComponentConfig) {}

  render() {
    return <Text />;
  }
}

class StatComponent implements DashboardComponent {
  constructor(private config: DashboardComponentConfig) {}

  render() {
    return <Stat />;
  }
}

class PanelComponent implements DashboardComponent {
  constructor(private config: DashboardComponentConfig) {}

  render() {
    const children = this.config.children?.map((child) =>
      DashboardFactory.createComponent(child).render(),
    );
    return <Panel>{children}</Panel>;
  }
}

class RangeComponent implements DashboardComponent {
  constructor(private config: DashboardComponentConfig) {}

  render() {
    return <Range />;
  }
}
