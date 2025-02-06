export type User = {
  id?: number;
  username: string;
  email: string;
};

export type Dashboard = {
  id: number;
  title: string;
  description: string;
};

export type Project = {
  id?: number;
  path: string;
  name: string;
  description: string;
};
