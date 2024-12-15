import { createApi, fetchBaseQuery } from "@reduxjs/toolkit/query/react";

import { User } from "~lib/types";

export const userApi = createApi({
  keepUnusedDataFor: process.env.NODE_ENV === "test" ? 0 : 60,
  baseQuery: fetchBaseQuery({ baseUrl: "http://localhost:3000/" }),
  tagTypes: ["Users"],
  endpoints: (build) => ({
    getUsers: build.query<User[], void>({
      query: () => "users",
      providesTags: (result) =>
        result
          ? [
              ...result.map(({ id }) => ({ type: "Users" as const, id })),
              { type: "Users", id: "LIST" },
            ]
          : [{ type: "User", id: "LIST" }],
    }),
  }),
});

export const { useGetUsersQuery } = userApi;
