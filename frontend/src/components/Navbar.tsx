import { useContext, useState } from "react";
import { useNavigate } from "react-router-dom";
import { ArrowLeftIcon, ArrowRightIcon, Bars3Icon } from "@heroicons/react/24/solid";
import { Cog8ToothIcon, PowerIcon, PuzzlePieceIcon } from "@heroicons/react/24/outline";

import { Quit } from "~wails/runtime";

import { AppContext } from "~root/App";

const Navbar = () => {
  const appContext = useContext(AppContext);
  const [isDrawerOpen, setIsDrawerOpen] = useState(false);
  const navigate = useNavigate();

  if (!appContext) {
    throw new Error("Navbar must be rendered within AppProvider");
  }

  const toggleDrawer = () => {
    setIsDrawerOpen(!isDrawerOpen);
  };

  const handleNavigate = (path) => {
    setIsDrawerOpen(false);
    navigate(path);
  };

  const themeColor = appContext.darkMode ? "bg-base-100" : "bg-neutral-content";
  const iconColor = appContext.darkMode ? "text-neutral-content" : "text-base-100";

  return (
    <>
      <div id="navbar" className={`navbar ${themeColor} py-0 px-2 min-h-2 h-12 z-10`}>
        <div className="flex-1">
          <div className="flex gap-2">
            <button className="btn btn-sm btn-square btn-ghost">
              <ArrowLeftIcon className="h-5 w-5" />
            </button>
            <button className="btn btn-sm btn-square btn-ghost">
              <ArrowRightIcon className="h-5 w-5" />
            </button>
          </div>
        </div>
        <div className="flex-none">
          <div className="drawer drawer-end">
            <input id="drawer" type="checkbox" checked={isDrawerOpen} className="drawer-toggle" />
            <div className="drawer-content">
              <ul className="menu">
                <li>
                  <label
                    htmlFor="drawer"
                    className={`drawer-button btn btn-sm btn-square btn-ghost ${iconColor}`}
                    onClick={toggleDrawer}
                  >
                    <Bars3Icon className="h-5 w-5" />
                  </label>
                </li>
              </ul>
            </div>
            <div className="drawer-side">
              <label
                htmlFor="drawer"
                aria-label="close sidebar"
                onClick={toggleDrawer}
                className="drawer-overlay"
              ></label>
              <ul className="bg-base-200 flex flex-col min-h-full w-[400px] py-2">
                <div className="flex-1">
                  <li>
                    <button
                      className="btn btn-sm btn-ghost btn-block px-4 justify-start align-middle text-xs"
                      onClick={() => handleNavigate("/settings")}
                    >
                      <Cog8ToothIcon className="h-4 w-4" />
                      Settings
                    </button>
                  </li>
                  <li>
                    <button
                      className="btn btn-sm btn-ghost btn-block px-4 justify-start align-middle text-xs"
                      onClick={() => handleNavigate("/components")}
                    >
                      <PuzzlePieceIcon className="h-4 w-4" />
                      Components
                    </button>
                  </li>
                </div>
                <li className="flex-none">
                  <button
                    className="btn btn-sm btn-ghost btn-block px-4 justify-start align-middle text-xs"
                    onClick={Quit}
                  >
                    <PowerIcon className="h-4 w-4" />
                    Quit
                  </button>
                </li>
              </ul>
            </div>
          </div>
        </div>
      </div>
    </>
  );
};

export default Navbar;
