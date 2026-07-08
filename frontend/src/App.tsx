import React, { useMemo, useState } from "react";
import "./App.css";
import { CONTRACT_ID, EXPLORER_URL, NETWORK } from "./contractConfig";
import {
  demoStats,
  demoTransfers,
  prepareDisputeTransfer,
  prepareFundTransfer,
  prepareOpenTransfer,
  statusTone
} from "./services/remittanceService";
import { connectFreighter, shortenAddress } from "./services/walletService";
import type { PreparedContractAction } from "./types";

function App() {
  const [walletAddress, setWalletAddress] = useState("");
  const [transferId, setTransferId] = useState("USA-VN-003");
  const [recipient, setRecipient] = useState("Vietnam recipient wallet");
  const [payoutPartner, setPayoutPartner] = useState("Licensed payout partner");
  const [amount, setAmount] = useState("250");
  const [preparedAction, setPreparedAction] = useState<PreparedContractAction | null>(null);
  const [statusMessage, setStatusMessage] = useState("Ready to prepare a remittance action.");

  const walletLabel = useMemo(() => {
    if (!walletAddress) {
      return "Not connected";
    }

    return shortenAddress(walletAddress);
  }, [walletAddress]);

  async function handleConnectWallet() {
    try {
      const publicKey = await connectFreighter();
      setWalletAddress(publicKey);
      setStatusMessage("Freighter wallet connected.");
    } catch (error) {
      const message = error instanceof Error ? error.message : "Wallet connection failed.";
      setStatusMessage(message);
    }
  }

  function handleOpenTransfer() {
    setPreparedAction(prepareOpenTransfer(transferId, recipient, payoutPartner, amount));
    setStatusMessage("open_transfer action prepared for review.");
  }

  function handleFundTransfer() {
    setPreparedAction(prepareFundTransfer(transferId, amount));
    setStatusMessage("fund_transfer action prepared for review.");
  }

  function handleDisputeTransfer() {
    setPreparedAction(prepareDisputeTransfer(transferId));
    setStatusMessage("dispute_transfer action prepared for review.");
  }

  return (
    <main className="app-shell">
      <section className="hero-card">
        <div>
          <p className="eyebrow">Stellar Testnet Remittance Corridor</p>
          <h1>remittance_corridor</h1>
          <p className="hero-copy">
            A Soroban-powered dashboard for opening, funding, monitoring, and
            disputing cross-border remittance transfers.
          </p>
        </div>

        <div className="contract-card">
          <span>Network</span>
          <strong>{NETWORK}</strong>
          <span>Contract</span>
          <strong>{shortenAddress(CONTRACT_ID)}</strong>
          <a href={EXPLORER_URL} target="_blank" rel="noreferrer">
            Open contract explorer
          </a>
        </div>
      </section>

      <section className="grid metrics-grid">
        <article className="metric-card">
          <span>Total Transfers</span>
          <strong>{demoStats.totalTransfers}</strong>
        </article>
        <article className="metric-card">
          <span>Funded</span>
          <strong>{demoStats.fundedTransfers}</strong>
        </article>
        <article className="metric-card">
          <span>Paid Out</span>
          <strong>{demoStats.paidOutTransfers}</strong>
        </article>
        <article className="metric-card">
          <span>Disputed</span>
          <strong>{demoStats.disputedTransfers}</strong>
        </article>
      </section>

      <section className="grid two-column">
        <article className="panel">
          <div className="panel-header">
            <div>
              <p className="eyebrow">Wallet</p>
              <h2>Freighter Connection</h2>
            </div>
            <button onClick={handleConnectWallet}>Connect Wallet</button>
          </div>

          <div className="status-box">
            <span>Wallet</span>
            <strong>{walletLabel}</strong>
          </div>

          <p className="muted">{statusMessage}</p>
        </article>

        <article className="panel">
          <p className="eyebrow">Operations</p>
          <h2>Prepare Remittance Action</h2>

          <label>
            Transfer ID
            <input value={transferId} onChange={(event) => setTransferId(event.target.value)} />
          </label>

          <label>
            Recipient
            <input value={recipient} onChange={(event) => setRecipient(event.target.value)} />
          </label>

          <label>
            Payout Partner
            <input
              value={payoutPartner}
              onChange={(event) => setPayoutPartner(event.target.value)}
            />
          </label>

          <label>
            Amount
            <input value={amount} onChange={(event) => setAmount(event.target.value)} />
          </label>

          <div className="button-row">
            <button onClick={handleOpenTransfer}>Open</button>
            <button onClick={handleFundTransfer}>Fund</button>
            <button onClick={handleDisputeTransfer}>Dispute</button>
          </div>
        </article>
      </section>

      <section className="grid two-column">
        <article className="panel">
          <p className="eyebrow">Prepared Contract Action</p>
          <h2>Transaction Plan</h2>

          {preparedAction ? (
            <div className="action-preview">
              <strong>{preparedAction.method}</strong>
              <p>{preparedAction.summary}</p>
              <pre>{JSON.stringify(preparedAction.args, null, 2)}</pre>
            </div>
          ) : (
            <p className="muted">Choose an operation to prepare a contract action.</p>
          )}
        </article>

        <article className="panel">
          <p className="eyebrow">Corridor Activity</p>
          <h2>Recent Transfers</h2>

          <div className="transfer-list">
            {demoTransfers.map((transfer) => (
              <div className="transfer-row" key={transfer.transferId}>
                <div>
                  <strong>{transfer.transferId}</strong>
                  <span>
                    {transfer.sourceCurrency} to {transfer.targetCurrency} · {transfer.amount}
                  </span>
                </div>
                <span className={"status-pill " + statusTone(transfer.status)}>
                  {transfer.status}
                </span>
              </div>
            ))}
          </div>
        </article>
      </section>
    </main>
  );
}

export default App;
