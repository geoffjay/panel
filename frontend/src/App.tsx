import { ProjectList } from "./components";

import "./App.css";

function App() {
  return (
    <div className="min-h-screen bg-neutral-content grid grid-cols-1 place-items-center justify-items-center mx-auto py-8">
      <div className="w-fit my-20">
        <ProjectList />
      </div>
    </div>
  );
}

export default App;
