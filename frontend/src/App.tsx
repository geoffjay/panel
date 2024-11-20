import React, { createContext, useState } from "react";
import { Outlet } from "react-router-dom";

import { Navbar } from "~components";

interface AppContextType {
  darkMode: boolean;
  setDarkMode: (darkMode: boolean) => void; // eslint-disable-line no-unused-vars
};

export const AppContext = createContext<AppContextType | null>(null);

interface AppProviderProps {
  children: React.ReactNode;
}

const AppProvider: React.FC<AppProviderProps> = ({ children }) => {
  const [context, setContext] = useState<AppContextType>({
    darkMode: false,
    setDarkMode: (darkMode: boolean) => setContext({ ...context, darkMode }),
  });

  return (
    <AppContext.Provider value={context}>
      {children}
    </AppContext.Provider>
  );
};

const App: React.FC = () => {
  return (
    <AppProvider>
      <div className="min-h-screen bg-neutral-content flex flex-col">
        <Navbar className="flex-1" />
        <div id="content" className="flex-auto mx-auto">
          <Outlet />
        </div>
      </div >
    </AppProvider>
  );
};

export default App;
