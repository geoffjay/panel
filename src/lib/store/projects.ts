import { create } from "zustand";

import { invoke } from "@tauri-apps/api/core";

import { Project } from "~lib/types";

type ProjectStore = {
  projects: Project[];
  isLoading: boolean;
  error: Error | null;
  fetchProjects: () => Promise<void>;
  createProject: (project: Project) => Promise<Project | null>;
  getProject: (id: string) => Promise<Project | undefined>;
  deleteProject: (id: number) => Promise<void>;
};

export const useProjectStore = create<ProjectStore>((set, get) => ({
  projects: [],
  isLoading: false,
  error: null,
  fetchProjects: async () => {
    set({ isLoading: true });
    await invoke("fetch_projects")
      .then((projects) => {
        set({ projects: projects as Project[], isLoading: false });
      })
      .catch((e) => set({ error: e, isLoading: false }));
  },
  createProject: async (project: Project) => {
    const newProject = await invoke("create_project", { project })
      .then((project) => {
        set((state) => ({ projects: [...state.projects, project as Project] }));
        return project as Project;
      })
      .catch((e) => {
        set({ error: e });
        return null;
      });

    return newProject;
  },
  getProject: async (id: string) => {
    if (!get().projects.length) {
      await get().fetchProjects();
    }
    return get().projects.find((project) => project.id === parseInt(id));
  },
  deleteProject: async (id: number) => {
    await invoke("delete_project", { id })
      .then(() => {
        set((state) => ({
          projects: state.projects.filter((project) => project.id !== id),
        }));
      })
      .catch((e) => set({ error: e }));
  },
}));
