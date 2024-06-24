import { Outlet } from "react-router-dom";

import { Navbar } from "~components";

const App: React.FC = () => {
  return (
    <div className="min-h-screen bg-neutral-content grid grid-cols-1 justify-items-center mx-auto">
      <Navbar />
      <div id="content">
        <Outlet />
      </div>
    </div >
  );
};

export default App;
