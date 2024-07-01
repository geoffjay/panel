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
    darkMode: true,
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
      <div className="min-h-screen bg-neutral-content grid grid-cols-1 justify-items-center mx-auto">
        <Navbar />
        <div id="content">
          <Outlet />
        </div>
      </div >
    </AppProvider>
  );
};

export default App;
