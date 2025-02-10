import React, { useEffect, useState } from "react";
import { useParams } from "react-router";

import { Loader } from "~components";
import { DashboardFactory } from "~lib/factories/dashboard";
import { useProjectStore } from "~lib/store/projects";
import { useDashboardStore } from "~lib/store/dashboard";
import { DashboardConfig } from "~lib/types/dashboard";

const Page: React.FC = () => {
  const { id } = useParams();
  const [dashboard, setDashboard] = useState<DashboardConfig | undefined>(
    undefined,
  );
  const { error, isLoading, getProject } = useProjectStore();
  const { getDashboardByProject } = useDashboardStore();

  useEffect(() => {
    const loadDashboard = async (id: string) => {
      const project = await getProject(id);
      if (project) {
        const dashboard = await getDashboardByProject(project);
        setDashboard(dashboard);
      }
    };

    if (id) {
      loadDashboard(id);
    }
  }, [id]);

  useEffect(() => {
    console.log(dashboard);
  }, [dashboard]);

  return (
    <div className="pt-12 pb-2 px-2">
      {isLoading && (
        <div className="flex items-center justify-center w-full h-screen">
          <Loader />
        </div>
      )}
      {error && <div>Error: {error.message}</div>}
      {dashboard &&
        dashboard.children.map((child) =>
          DashboardFactory.createComponent(child).render(),
        )}
    </div>
  );
};

export default Page;
