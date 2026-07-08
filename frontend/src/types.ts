export type TransferStatus =
  | "Draft"
  | "Opened"
  | "Funded"
  | "PaidOut"
  | "Disputed"
  | "Released"
  | "Refunded";

export type CorridorStats = {
  totalTransfers: number;
  fundedTransfers: number;
  paidOutTransfers: number;
  disputedTransfers: number;
};

export type RemittanceTransfer = {
  transferId: string;
  sender: string;
  recipient: string;
  payoutPartner: string;
  sourceCurrency: string;
  targetCurrency: string;
  amount: string;
  status: TransferStatus;
};

export type PreparedContractAction = {
  method: string;
  summary: string;
  args: string[];
};
