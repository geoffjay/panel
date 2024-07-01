import { useContext, useState } from "react";
import { useNavigate } from "react-router-dom";
import { ArrowLeftIcon, ArrowRightIcon, Bars3Icon } from "@heroicons/react/24/solid";

import { Quit } from "~wails/runtime";

import { AppContext } from "~root/App";

const Navbar = () => {
  const appContext = useContext(AppContext);
  const [menuOpen, setMenuOpen] = useState(false);
  const navigate = useNavigate();

  if (!appContext) {
    throw new Error("Navbar must be rendered within AppProvider");
  }

  const toggleMenu = () => {
    setMenuOpen(!menuOpen);
  };

  const handleSettings = () => {
    navigate("/settings");
    toggleMenu();
  };

  const themeColor = appContext.darkMode ? "bg-base-100" : "bg-neutral-content";
  const iconColor = appContext.darkMode ? "text-neutral-content" : "text-base-100";

  return (
    <>
      <div id="navbar" className={`navbar ${themeColor} py-0 px-2 min-h-2 h-12`}>
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
          <ul className="menu">
            <li>
              <button onClick={toggleMenu} className={`btn btn-sm btn-square btn-ghost ${iconColor}`}>
                <Bars3Icon className="h-5 w-5" />
              </button>
            </li>
          </ul>
        </div>
      </div>
      {menuOpen && (
        <div id="menu" className="bg-base-100 border border-neutral-content z-10 shadow-xl rounded-lg p-4 w-[200px] mt-1 mr-1 top-12 right-0 absolute">
          <div className="divide-y divide-neutral-content">
            <ul className="pb-2">
              <li>
                <button
                  className="btn btn-ghost btn-block px-0 text-left"
                  onClick={handleSettings}
                >
                  Settings
                </button>
              </li>
            </ul>
            <ul className="pt-2">
              <li>
                <button
                  className="btn btn-ghost btn-block px-0 text-left"
                  onClick={Quit}
                >
                  Quit
                </button>
              </li>
            </ul>
          </div>
        </div>
      )}
    </>
  );
};

export default Navbar;
