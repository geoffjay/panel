import React from "react";
import { createRoot } from "react-dom/client";
import { createBrowserRouter, RouterProvider } from "react-router-dom";

import { ErrorPage } from "~components";
import { Page as Builder } from "~features/builder";
import { Page as Dashboard } from "~features/dashboard";
import { Page as Settings } from "~features/settings";
import { Page as Projects } from "~features/projects";

import App from "./App";

import "./style.css";

const router = createBrowserRouter([
  {
    path: "/",
    element: <App />,
    errorElement: <ErrorPage />,
    children: [
      {
        path: "/",
        element: <Projects />,
      },
      {
        path: "/builder",
        element: <Builder />,
      },
      {
        path: "/dashboard",
        element: <Dashboard />,
      },
      {
        path: "/settings",
        element: <Settings />,
      },
    ],
  },
]);

const container = document.getElementById("root");

const root = createRoot(container!);

root.render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>,
);
