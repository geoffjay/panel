import React from "react";
import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import moment from "moment";
import { FloatingDelayGroup } from "@floating-ui/react";
import { TvIcon, SquaresPlusIcon } from "@heroicons/react/24/outline";

import { GetProjects, LoadProject } from "~wails/go/main/App";
import { main } from "~wails/go/models";
import { Tooltip } from "~components";

const DASHBOARD = "dashboard";
const BUILDER = "builder";

const ProjectList: React.FC = () => {
  const [projects, setProjects] = useState<main.Project[]>([]);
  const navigate = useNavigate();

  useEffect(() => {
    GetProjects()
      .then((response) => {
        console.log(response);
        setProjects(response);
      })
      .catch((error) => {
        console.error(error);
      });
  }, []);

  const handleLoad = (event: React.MouseEvent<HTMLButtonElement>, project: string, page: string) => {
    event.preventDefault();
    LoadProject(project)
      .then(() => {
        navigate(`/${page}`);
      })
      .catch((error) => {
        console.error(error);
      });
  };

  return (
    <table className="table table-zebra table-lg">
      <thead>
        <tr>
          <th>Path</th>
          <th>Description</th>
          <th>Modified</th>
          <th></th>
        </tr>
      </thead>
      <tbody>
        {projects.map((project) => (
          <tr key={project.path} className="hover">
            <td>
              {project.path}
            </td>
            <td>{project.description}</td>
            <td>{moment(project.modifiedTime, "YYYY-MM-DD hh:mm:ss").format("MMMM Do YYYY, h:mm:ss a")}</td>
            <td>
              <div className="flex gap-2">
                <FloatingDelayGroup delay={500}>
                  <Tooltip>
                    <Tooltip.Trigger>
                      <button className="btn btn-md btn-square btn-ghost" onClick={(event) => handleLoad(event, project.path, DASHBOARD)}>
                        <TvIcon className="h-6 w-6" />
                      </button>
                    </Tooltip.Trigger>
                    <Tooltip.Content>
                      <div className="tooltip bg-base-200 text-neutral-content shadow-lg rounded-lg p-2">
                        Dashboard
                      </div>
                    </Tooltip.Content>
                  </Tooltip>
                  <Tooltip>
                    <Tooltip.Trigger>
                      <button className="btn btn-md btn-square btn-ghost" onClick={(event) => handleLoad(event, project.path, BUILDER)}>
                        <SquaresPlusIcon className="h-6 w-6" />
                      </button>
                    </Tooltip.Trigger>
                    <Tooltip.Content>
                      <div className="tooltip bg-base-200 text-neutral-content shadow-lg rounded-lg p-2">
                        Builder
                      </div>
                    </Tooltip.Content>
                  </Tooltip>
                </FloatingDelayGroup>
              </div>
            </td>
          </tr>
        ))}
      </tbody>
    </table>
  );
};

export default ProjectList;
