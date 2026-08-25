import { describe, expect, it, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import { listTaskIntelligence, parseTaskIntelligence } from "../src/taskIntelligence";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));

describe("M09 task intelligence IPC contract", () => {
  it("invokes the bounded parse command with the project id", async () => {
    vi.mocked(invoke).mockResolvedValueOnce({ tasks: [] });
    await parseTaskIntelligence("project-1");
    expect(invoke).toHaveBeenCalledWith("hiveai_task_intelligence_parse", { projectId: "project-1" });
  });

  it("invokes the persisted list command with the project id", async () => {
    vi.mocked(invoke).mockResolvedValueOnce({ tasks: [] });
    await listTaskIntelligence("project-2");
    expect(invoke).toHaveBeenCalledWith("hiveai_task_intelligence_list", { projectId: "project-2" });
  });
});
