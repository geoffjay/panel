import React from "react";

import { Panel, Range, Stack, Stat, Text } from "~components/dashboard";

import {
  DashboardComponentConfig,
  DashboardComponent,
} from "~lib/types/dashboard";

export class DashboardFactory {
  static createComponent(config: DashboardComponentConfig): DashboardComponent {
    switch (config.type) {
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

abstract class BaseComponent implements DashboardComponent {
  constructor(protected config: DashboardComponentConfig) {}

  protected getChildren(): React.ReactNode[] | undefined {
    return this.config.children?.map((child) =>
      DashboardFactory.createComponent(child).render(),
    );
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  protected getProperties(): { [key: string]: any }[] {
    console.log(this.config);

    if (!this.config.properties) return [];

    // If properties is already an array, return it transformed to objects
    if (Array.isArray(this.config.properties)) {
      return this.config.properties.map((property) => ({
        [property.name]: property.value,
      }));
    }

    // If properties is a hash/object, convert it to array of objects
    return Object.entries(this.config.properties).map(([key, value]) => ({
      [key]: value,
    }));
  }

  abstract render(): React.JSX.Element;
}

class PanelComponent extends BaseComponent {
  render() {
    return <Panel {...this.getProperties()}>{this.getChildren()}</Panel>;
  }
}

class RangeComponent extends BaseComponent {
  render() {
    const props = Object.assign({}, ...(this.getProperties() || []));
    return <Range {...props} />;
  }
}

class StackComponent extends BaseComponent {
  render() {
    return <Stack {...this.getProperties()}>{this.getChildren()}</Stack>;
  }
}

class StatComponent extends BaseComponent {
  render() {
    const props = Object.assign({}, ...(this.getProperties() || []));
    return <Stat {...props} />;
  }
}

class TextComponent extends BaseComponent {
  render() {
    const props = Object.assign({}, ...(this.getProperties() || []));
    return <Text {...props} />;
  }
}
