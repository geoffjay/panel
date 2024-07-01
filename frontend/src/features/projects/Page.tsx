import React, { useContext, useEffect } from "react";

import { AppContext } from "~root/App";

import ProjectList from "./ProjectList";

const Page: React.FC = () => {
  const appContext = useContext(AppContext);

  if (!appContext) {
    throw new Error("Page must be rendered within AppProvider");
  }

  useEffect(() => {
    appContext.setDarkMode(false);
  }, []);

  return (
    <div className="w-fit my-20">
      <div className="card bg-base-100 shadow-xl rounded-lg">
        <div className="card-body">
          <h2 className="card-title my-2">Projects</h2>
          <div className="overflow-x-auto max-h-[600px] border border-base-200 rounded-lg">
            <ProjectList />
          </div>
        </div>
      </div>
    </div>
  );
};

export default Page;
