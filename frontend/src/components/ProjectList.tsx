import { useEffect, useState } from "react";
import moment from "moment";
import { RectangleGroupIcon } from "@heroicons/react/24/outline";

import { GetProjects, LoadProject } from "~wails/go/main/App";
import { main } from "~wails/go/models";

const ProjectList: React.FC = () => {
  const [projects, setProjects] = useState<main.Project[]>([]);

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

  const handleLoadProject = (event: React.MouseEvent<HTMLButtonElement>, project: string) => {
    event.preventDefault();
    LoadProject(project)
      .then((response) => {
        console.log(response);
      })
      .catch((error) => {
        console.error(error);
      });
  };

  return (
    <div className="card bg-base-100 shadow-xl rounded-lg">
      <div className="card-body">
        <h2 className="card-title my-2">Projects</h2>
        <div className="overflow-x-auto max-h-[600px] border border-base-200 rounded-lg">
          <table className="table table-zebra">
            <thead>
              <tr>
                <th></th>
                <th>Path</th>
                <th>Description</th>
                <th>Modified</th>
              </tr>
            </thead>
            <tbody>
              {projects.map((project) => (
                <tr key={project.path} className="hover">
                  <td>
                    <RectangleGroupIcon className="h-6 w-6" />
                  </td>
                  <td>
                    <button className="btn btn-sm btn-ghost" onClick={(event) => handleLoadProject(event, project.path)}>
                      {project.path}
                    </button>
                  </td>
                  <td>{project.description}</td>
                  <td>{moment(project.modifiedTime, "YYYY-MM-DD hh:mm:ss").format("MMMM Do YYYY, h:mm:ss a")}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
};

export default ProjectList;
