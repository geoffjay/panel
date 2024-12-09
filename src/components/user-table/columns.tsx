import { ColumnDef } from "@tanstack/react-table";

export type User = {
  id?: number
  name: string
}

export const columns: ColumnDef<User>[] = [
  {
    accessorKey: "id",
    header: "ID",
  },
  {
    accessorKey: "name",
    header: "Name",
  },
];
