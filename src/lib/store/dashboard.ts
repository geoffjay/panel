import { create } from "zustand";

import { invoke } from "@tauri-apps/api/core";

import { Project } from "~lib/types";
import { DashboardConfig } from "~lib/types/dashboard";

type DashboardStore = {
  isLoading: boolean;
  error: Error | null;
  getDashboardByProject: (
    project: Project,
  ) => Promise<DashboardConfig | undefined>;
};

export const useDashboardStore = create<DashboardStore>((set) => ({
  isLoading: false,
  error: null,
  getDashboardByProject: async (project: Project) => {
    set({ isLoading: true });
    const dashboard = await invoke("get_dashboard_by_project", { project })
      .then((dashboard) => {
        set({ isLoading: false });
        return dashboard as DashboardConfig;
      })
      .catch((e) => {
        set({ error: e, isLoading: false });
        return undefined;
      });

    return dashboard;
  },
}));
