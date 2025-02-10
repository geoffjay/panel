import React, { useState } from "react";
import { useNavigate } from "react-router";
import { Folder } from "lucide-react";

import { open } from "@tauri-apps/plugin-dialog";

import { Input } from "~components/ui/input";
import { Button } from "~components/ui/button";
import { useProjectStore } from "~lib/store/projects";

const Page: React.FC = () => {
  const [path, setPath] = useState<string>("");
  const [name, setName] = useState<string>("");
  const { createProject } = useProjectStore();
  const navigate = useNavigate();

  const handleOpenFolder = async () => {
    const file = await open({
      multiple: false,
      directory: true,
    });

    if (file) {
      setPath(file);
    }
  };

  const handleCreateProject = async () => {
    if (path && name) {
      const newProject = await createProject({
        name: name,
        description: "New Project",
        path: path,
      });

      if (newProject !== null && newProject.id !== undefined) {
        navigate(`/project/${newProject.id}`);
      }
    }
  };

  return (
    <div className="h-screen w-[800px] m-auto flex flex-col justify-center items-center gap-4">
      <div className="flex w-full justify-between">
        <div className="flex flex-col gap-2 bg-white p-4 rounded-lg drop-shadow-md w-full">
          <Input
            className="border-none"
            placeholder="Name"
            value={name}
            onChange={(e) => setName(e.target.value)}
          />
        </div>
      </div>
      <p className="text-sm text-gray-400 italic w-full ml-8 mb-4">
        Enter the name of the project that will be created.
      </p>
      <div className="flex flex-col gap-2 bg-white p-4 rounded-lg drop-shadow-md w-full">
        <div className="flex justify-between">
          <Input
            className="border-none"
            placeholder="Project Path"
            value={path}
            onChange={(e) => setPath(e.target.value)}
          />
          <Button variant="ghost" size="icon" onClick={handleOpenFolder}>
            <Folder />
          </Button>
        </div>
      </div>
      <p className="text-sm text-gray-400 italic w-full ml-8 mb-4">
        The path where the project will be created and where project files will
        be stored.
      </p>
      <div className="flex justify-end gap-2">
        <Button onClick={handleCreateProject}>Create</Button>
        <Button onClick={() => navigate("/")}>Cancel</Button>
      </div>
    </div>
  );
};

export default Page;
