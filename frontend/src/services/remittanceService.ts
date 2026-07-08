import type { CorridorStats, PreparedContractAction, RemittanceTransfer } from "../types";

export const demoStats: CorridorStats = {
  totalTransfers: 128,
  fundedTransfers: 91,
  paidOutTransfers: 84,
  disputedTransfers: 3
};

export const demoTransfers: RemittanceTransfer[] = [
  {
    transferId: "USA-VN-001",
    sender: "Sender wallet",
    recipient: "Recipient in Vietnam",
    payoutPartner: "Licensed payout partner",
    sourceCurrency: "USDC",
    targetCurrency: "VND",
    amount: "250",
    status: "Funded"
  },
  {
    transferId: "USA-VN-002",
    sender: "Sender wallet",
    recipient: "Recipient in Vietnam",
    payoutPartner: "Licensed payout partner",
    sourceCurrency: "USDC",
    targetCurrency: "VND",
    amount: "120",
    status: "PaidOut"
  }
];

export function prepareOpenTransfer(
  transferId: string,
  recipient: string,
  payoutPartner: string,
  amount: string
): PreparedContractAction {
  return {
    method: "open_transfer",
    summary: "Open a new remittance transfer in the corridor contract.",
    args: [transferId, recipient, payoutPartner, amount]
  };
}

export function prepareFundTransfer(transferId: string, amount: string): PreparedContractAction {
  return {
    method: "fund_transfer",
    summary: "Mark the sender escrow funding action for a transfer.",
    args: [transferId, amount]
  };
}

export function prepareDisputeTransfer(transferId: string): PreparedContractAction {
  return {
    method: "dispute_transfer",
    summary: "Open a dispute during the configured dispute window.",
    args: [transferId]
  };
}

export function statusTone(status: string): string {
  if (status === "PaidOut" || status === "Released") {
    return "success";
  }

  if (status === "Disputed") {
    return "warning";
  }

  return "active";
}
