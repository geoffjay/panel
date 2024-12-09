import { render, screen, waitFor } from "@testing-library/react";
import Database from "@tauri-apps/plugin-sql";

import { DatabaseProvider, useDatabase } from "./database-provider";

// Mock the SQL Database
vi.mock("@tauri-apps/plugin-sql", () => ({
  Database: {
    load: vi.fn(),
  },
}));

// Test component that uses the database hook
function TestComponent() {
  const { execute, isLoading, error } = useDatabase();
  
  if (isLoading) return <div>Loading...</div>;
  if (error) return <div>Error: {error.message}</div>;
  
  return (
    <div>
      <div>Database Ready</div>
      <button onClick={() => execute("SELECT * FROM test")}>Execute Query</button>
    </div>
  );
}

describe("DatabaseProvider", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("should show loading state while initializing", () => {
    // Mock database loading to never resolve
    vi.mocked(Database.load).mockImplementation(
      () => new Promise(() => {}),
    );

    render(
      <DatabaseProvider>
        <TestComponent />
      </DatabaseProvider>,
    );

    expect(screen.getByText("Loading...")).toBeInTheDocument();
  });

  it("should initialize database successfully", async () => {
    const mockDb = {
      execute: vi.fn(),
    };

    vi.mocked(Database.load).mockResolvedValue(mockDb as any);

    render(
      <DatabaseProvider>
        <TestComponent />
      </DatabaseProvider>,
    );

    await waitFor(() => {
      expect(screen.getByText("Database Ready")).toBeInTheDocument();
    });

    expect(Database.load).toHaveBeenCalledWith("sqlite:panel.db");
  });

  it("should handle database initialization error", async () => {
    const error = new Error("Failed to connect");
    vi.mocked(Database.load).mockRejectedValue(error);

    render(
      <DatabaseProvider>
        <TestComponent />
      </DatabaseProvider>,
    );

    await waitFor(() => {
      expect(screen.getByText(`Error: ${error.message}`)).toBeInTheDocument();
    });
  });

  it("should throw error when useDatabase is used outside provider", () => {
    const consoleError = vi.spyOn(console, "error").mockImplementation(() => {});
    
    expect(() => {
      render(<TestComponent />);
    }).toThrow("useDatabase must be used within a DatabaseProvider");
    
    consoleError.mockRestore();
  });

  it("should execute database queries", async () => {
    const mockDb = {
      execute: vi.fn().mockResolvedValue({ rows: [] }),
    };

    vi.mocked(Database.load).mockResolvedValue(mockDb as any);

    const { user } = render(
      <DatabaseProvider>
        <TestComponent />
      </DatabaseProvider>,
    );

    await waitFor(() => {
      expect(screen.getByText("Database Ready")).toBeInTheDocument();
    });

    await user.click(screen.getByText("Execute Query"));

    expect(mockDb.execute).toHaveBeenCalledWith("SELECT * FROM test", undefined);
  });
}); 
