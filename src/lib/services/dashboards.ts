import { createApi, fetchBaseQuery } from "@reduxjs/toolkit/query/react";

import { Dashboard } from "~lib/types";

export const dashboardApi = createApi({
  keepUnusedDataFor: process.env.NODE_ENV === "test" ? 0 : 60,
  baseQuery: fetchBaseQuery({ baseUrl: "http://localhost:3000/" }),
  tagTypes: ["Dashboards"],
  endpoints: (build) => ({
    getDashboard: build.query<Dashboard, number>({
      query: (id) => `dashboards/${id}`,
      providesTags: (_result, _error, id) => [{ type: "Dashboards", id }],
    }),
    getDashboards: build.query<Dashboard[], void>({
      query: () => "dashboards",
      providesTags: (result) =>
        result
          ? [
              ...result.map(({ id }) => ({ type: "Dashboards" as const, id })),
              { type: "Dashboards", id: "LIST" },
            ]
          : [{ type: "Dashboards", id: "LIST" }],
    }),
    createDashboard: build.mutation<Dashboard, Dashboard>({
      query: (dashboard) => ({
        url: "dashboards",
        method: "POST",
        body: dashboard,
      }),
      invalidatesTags: () => [
        { type: "Dashboards", id: "LIST" },
      ],
    }),
    updateDashboard: build.mutation<Dashboard, Dashboard>({
      query: (dashboard) => ({
        url: `dashboards/${dashboard.id}`,
        method: "PUT",
        body: dashboard,
      }),
      invalidatesTags: (_result, _error, dashboard) => [
        { type: "Dashboards", id: dashboard.id },
      ],
    }),
    deleteDashboard: build.mutation<Dashboard, number>({
      query: (id) => ({
        url: `dashboards/${id}`,
        method: "DELETE",
      }),
      invalidatesTags: (_result, _error, id) => [
        { type: "Dashboards", id },
      ],
    }),
  }),
});

export const {
  useGetDashboardQuery,
  useGetDashboardsQuery,
  useCreateDashboardMutation,
  useUpdateDashboardMutation,
  useDeleteDashboardMutation,
} = dashboardApi;
