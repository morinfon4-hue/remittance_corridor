# remittance_corridor

## Project Title
remittance_corridor

## Project Description
Cross-border person-to-person remittance is still slow, expensive, and
opaque: a sender in the USA paying a recipient in Vietnam typically loses
a large slice of the value to layered intermediary fees, FX spreads, and
multi-day settlement delays. `remittance_corridor` is a Soroban smart
contract that brings the on-chain side of that flow onto Stellar: a sender
opens a transfer denominated in a source currency (e.g. `USDC`), funds an
on-chain escrow, and an off-chain licensed payout partner confirms the
local-currency delivery (e.g. `VND`) to the recipient. The contract
encodes the full lifecycle and the dispute window so that no party can
unilaterally run off with the escrowed funds.

The contract does not move real assets itself. It is the trust
substitute for the corridor: a single shared ledger of who is allowed to
do what, and when, for every transfer that flows through the USA -> VN
lane (or any other pair the admin enables).

## Project Vision
The long-term vision is a network of open, auditable remittance
corridors running on Stellar, where any licensed payout partner in the
destination country can plug in to receive traffic from any compatible
sender wallet. By moving the state machine on-chain we make dispute
resolution deterministic, fees transparent, and settlement times
collapsible from days to minutes. The `remittance_corridor` contract is
the smallest viable primitive for that network: one corridor, one state
machine, one auditable log.

## Key Features
- **Sender-controlled escrow.** Funds are only marked escrowed when the
  sender explicitly authorises the call, and the amount must match what
  was declared in `open_transfer`. No party can quietly inflate or
  reduce the locked value.
- **Configurable dispute window.** A single corridor-wide window (in
  ledgers) is set at `init` and applied to every funded transfer. The
  sender can dispute inside that window; after it elapses without a
  payout claim, the sender can reclaim the escrow.
- **Payout-partner handoff.** The admin assigns a designated
  `payout_partner` address per transfer, and only that address can mark
  the transfer `PaidOut`. This keeps the on-chain receipt honest even
  though the local-currency delivery happens off-chain.
- **Admin-mediated dispute resolution.** Disputed transfers are frozen
  until the admin calls `resolve_dispute`, choosing either to release
  the escrow back to the sender or to confirm the payout to the
  recipient.
- **Full state-machine observability.** `transfer_status`, `get_transfer`,
  `transfers_of`, and `corridor_stats` make every transfer and the
  aggregate health of the corridor queryable in a single read.

## Contract

- **Network:** Stellar Testnet (Public)
- **Scope:** finance dApp — see `contracts/remittance_corridor/src/lib.rs` for the full remittance_corridor business logic.
- **Functions exposed:** see `Key Features` above and the `pub fn` list in `lib.rs`.
- **Contract ID:** `CDEEHC2BY2O73N4OTIKKGLJ2IP442AFY3BEEOXGKRETUGV5LWFTRERTO`
- **Explorer template:** `https://stellar.expert/explorer/testnet/tx/c5c8a2a9fac7ac3f4096065eb476e0d5a986d75d887cf8deff9d1e2338f17bca`

## Future Scope
- **Real asset integration.** Wire the `fund_transfer`, `claim_payout`,
  and `release` paths to a Stellar Asset Contract so that the on-chain
  escrow actually holds the source-currency balance of the sender.
- **Multi-corridor routing.** Generalise from a single hard-coded
  USA -> VN corridor to a registry of corridors, each with its own
  supported currency pairs, fee schedule, and approved payout partners.
- **Oracle-driven FX quoting.** Integrate a price oracle so that the
  sender sees an indicative `target_ccy` amount at open time, and the
  payout partner's claim is checked against an on-chain rate.
- **On-chain KYC / sanctions hooks.** Gate `open_transfer` and
  `set_payout_partner` behind attestations from an on-chain identity
  service to satisfy real-world remittance compliance.
- **Frontend dApp.** A small React + Freighter wallet UI to open,
  fund, dispute, and release transfers, surfacing `transfer_status`
  and `corridor_stats` in a corridor dashboard.

## Profile

- **Name:** <!-- Fill github name -->
- **Project:** `remittance_corridor` (finance)
- **Built with:** Soroban SDK 25, Rust, Stellar Testnet
