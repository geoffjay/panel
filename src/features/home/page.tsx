import React, { useEffect } from "react";
import { useNavigate } from "react-router";
import { ChevronRight, Trash2 } from "lucide-react";

import { open } from "@tauri-apps/plugin-dialog";

import { Button } from "~components/ui/button";
import { useProjectStore } from "~lib/store/projects";

const Page: React.FC = () => {
  const { projects, fetchProjects, createProject, deleteProject } =
    useProjectStore();
  const navigate = useNavigate();

  useEffect(() => {
    fetchProjects();
  }, []);

  const handleOpenFolder = async () => {
    const file = await open({
      multiple: false,
      directory: true,
    });

    if (file) {
      const newProject = await createProject({
        name: "New Project",
        description: "New Project",
        path: file,
      });

      if (newProject !== null && newProject.id !== undefined) {
        navigate(`/project/${newProject.id}`);
      }
    }
  };

  const handleDelete = (id: number | undefined) => async () => {
    if (id) {
      await deleteProject(id);
    }
  };

  return (
    <div className="h-screen w-[640px] m-auto flex flex-col justify-center items-center gap-4">
      <div className="flex gap-2 align-start w-full">
        <h1 className="text-lg font-bold">Recent Projects</h1>
      </div>
      <div className="w-full rounded-lg bg-white p-4 drop-shadow-md max-h-[400px] overflow-y-auto">
        {projects.length === 0 ? (
          <div className="text-center text-gray-500 py-4">
            <p>No projects yet!</p>
            <p className="text-sm">
              Create a new project or open an existing folder to get started.
            </p>
          </div>
        ) : (
          projects.map((project, index) => (
            <div
              className={`flex justify-between ${index !== projects.length - 1 ? "border-b border-gray-200 pb-3 mb-3" : ""}`}
              key={project.id}
            >
              <div className="flex-col gap-2">
                <h2 className="text-md font-bold">{project.name}</h2>
                <p className="text-sm text-gray-500 italic">{project.path}</p>
              </div>
              <div className="flex">
                <Button
                  variant="ghost"
                  className="p-2"
                  onClick={handleDelete(project.id)}
                >
                  <Trash2 className="w-4 h-4 text-zinc-700 dark:text-zinc-300" />
                </Button>
                <Button
                  variant="ghost"
                  className="p-2"
                  onClick={() => navigate(`/project/${project.id}`)}
                >
                  <ChevronRight className="w-4 h-4 text-zinc-700 dark:text-zinc-300" />
                </Button>
              </div>
            </div>
          ))
        )}
      </div>
      <div className="flex gap-2">
        <Button onClick={() => navigate("/project/new")}>
          Create Project...
        </Button>
        <Button onClick={handleOpenFolder}>Open Folder...</Button>
      </div>
    </div>
  );
};

export default Page;
