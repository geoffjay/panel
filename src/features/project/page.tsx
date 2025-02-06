import React, { useEffect, useState } from "react";
import { useParams } from "react-router";

import { Loader } from "~components";
import { useProjectStore } from "~/lib/store/projects";
import { Project } from "~/lib/types";

const Page: React.FC = () => {
  const { id } = useParams();
  const [project, setProject] = useState<Project | undefined>(undefined);
  const { error, isLoading, getProject } = useProjectStore();

  useEffect(() => {
    const loadDashboard = async (id: string) => {
      const project = await getProject(id);
      setProject(project);

      // const dashboard = await getDashboardByProject(project);
      // setDashboard(dashboard);
    };

    if (id) {
      loadDashboard(id);
    }
  }, [id]);

  return (
    <div className="pt-12 pb-2 px-2">
      {isLoading && (
        <div className="flex items-center justify-center w-full h-screen">
          <Loader />
        </div>
      )}
      {error && <div>Error: {error.message}</div>}
      {project && <div>Project {project.id}</div>}
    </div>
  );
};

export default Page;
