import { create } from "zustand";

import { invoke } from "@tauri-apps/api/core";

import { Dashboard, Project } from "~lib/types";

type ProjectStore = {
  isLoading: boolean;
  error: Error | null;
  getDashboardByProject: (project: Project) => Promise<Dashboard | null>;
};

export const useProjectStore = create<ProjectStore>((set) => ({
  isLoading: false,
  error: null,
  getDashboardByProject: async (project: Project) => {
    set({ isLoading: true });
    const dashboard = await invoke("get_dashboard_by_project", { project })
      .then((dashboard) => {
        set({ isLoading: false });
        return dashboard as Dashboard;
      })
      .catch((e) => {
        set({ error: e, isLoading: false });
        return null;
      });

    return dashboard;
  },
}));
