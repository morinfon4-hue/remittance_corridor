import { describe, expect, it } from "vitest";
import {
  prepareDisputeTransfer,
  prepareFundTransfer,
  prepareOpenTransfer,
  statusTone
} from "./remittanceService";

describe("remittanceService", () => {
  it("prepares an open transfer action", () => {
    const action = prepareOpenTransfer("USA-VN-001", "recipient", "partner", "250");

    expect(action.method).toBe("open_transfer");
    expect(action.args).toEqual(["USA-VN-001", "recipient", "partner", "250"]);
  });

  it("prepares a fund transfer action", () => {
    const action = prepareFundTransfer("USA-VN-001", "250");

    expect(action.method).toBe("fund_transfer");
    expect(action.args).toEqual(["USA-VN-001", "250"]);
  });

  it("prepares a dispute action", () => {
    const action = prepareDisputeTransfer("USA-VN-001");

    expect(action.method).toBe("dispute_transfer");
    expect(action.args).toEqual(["USA-VN-001"]);
  });

  it("maps status tones", () => {
    expect(statusTone("PaidOut")).toBe("success");
    expect(statusTone("Disputed")).toBe("warning");
    expect(statusTone("Funded")).toBe("active");
  });
});
