import React from "react";

export type DashboardComponentType =
  | "layout"
  | "stack"
  | "text"
  | "stat"
  | "panel"
  | "range";

export type DashboardComponentProperty = {
  name: string;
  value: string | number | boolean | null;
};

export interface DashboardComponentConfig {
  type: DashboardComponentType;
  children?: DashboardComponentConfig[];
  properties?: DashboardComponentProperty[];
}

export interface DashboardComponent {
  render: () => React.ReactNode;
}

export interface DashboardConfig {
  children: DashboardComponentConfig[];
}
