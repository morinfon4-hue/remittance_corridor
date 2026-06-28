#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map, Symbol};

pub const STATUS_OPEN: u32 = 1;
pub const STATUS_FUNDED: u32 = 2;
pub const STATUS_PAID_OUT: u32 = 3;
pub const STATUS_DISPUTED: u32 = 4;
pub const STATUS_RELEASED: u32 = 5;
pub const STATUS_CANCELLED: u32 = 6;

const DEFAULT_DISPUTE_WINDOW_LEDGERS: u64 = 17_280;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    NextId,
    DisputeWindow,
    Initialized,
    Transfer(u64),
    SenderList(Address),
    Stats,
}

#[contracttype]
#[derive(Clone)]
pub struct Transfer {
    pub id: u64,
    pub sender: Address,
    pub recipient: Address,
    pub payout_partner: Address,
    pub amount: i128,
    pub source_ccy: Symbol,
    pub target_ccy: Symbol,
    pub status: u32,
    pub opened_at: u64,
    pub funded_at: u64,
    pub dispute_deadline: u64,
    pub dispute_reason: Symbol,
}

#[contracttype]
#[derive(Clone)]
pub struct CorridorStats {
    pub total_opened: u64,
    pub total_funded: u64,
    pub total_paid_out: u64,
    pub total_disputed: u64,
    pub total_released: u64,
    pub total_cancelled: u64,
}

#[contract]
pub struct RemittanceCorridor;

#[contractimpl]
impl RemittanceCorridor {
    pub fn init(env: Env, admin: Address, dispute_window_ledgers: u64) {
        if env.storage().instance().has(&DataKey::Initialized) {
            panic!("Already initialized");
        }

        if dispute_window_ledgers == 0 {
            panic!("Dispute window must be positive");
        }

        admin.require_auth();

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage()
            .instance()
            .set(&DataKey::DisputeWindow, &dispute_window_ledgers);
        env.storage().instance().set(&DataKey::NextId, &1u64);
        env.storage().instance().set(&DataKey::Initialized, &true);

        let stats = CorridorStats {
            total_opened: 0,
            total_funded: 0,
            total_paid_out: 0,
            total_disputed: 0,
            total_released: 0,
            total_cancelled: 0,
        };

        env.storage().instance().set(&DataKey::Stats, &stats);
    }

    pub fn open_transfer(
        env: Env,
        sender: Address,
        recipient: Address,
        amount: i128,
        source_ccy: Symbol,
        target_ccy: Symbol,
    ) -> u64 {
        sender.require_auth();

        if amount <= 0 {
            panic!("Amount must be positive");
        }

        if sender == recipient {
            panic!("Sender and recipient must differ");
        }

        let next_id = Self::read_next_id(&env);
        let transfer_id = next_id;

        env.storage()
            .instance()
            .set(&DataKey::NextId, &(next_id + 1));

        let dispute_window = Self::read_dispute_window(&env);
        let now = Self::current_ledger(&env);

        let transfer = Transfer {
            id: transfer_id,
            sender: sender.clone(),
            recipient,
            payout_partner: sender.clone(),
            amount,
            source_ccy,
            target_ccy,
            status: STATUS_OPEN,
            opened_at: now,
            funded_at: 0,
            dispute_deadline: now + dispute_window,
            dispute_reason: Symbol::new(&env, "none"),
        };

        env.storage()
            .instance()
            .set(&DataKey::Transfer(transfer_id), &transfer);

        let sender_key = DataKey::SenderList(sender);
        let mut sender_list: Map<u64, bool> = env
            .storage()
            .instance()
            .get(&sender_key)
            .unwrap_or(Map::new(&env));

        sender_list.set(transfer_id, true);
        env.storage().instance().set(&sender_key, &sender_list);

        Self::increase_total_opened(&env);

        transfer_id
    }

    pub fn fund_transfer(env: Env, sender: Address, transfer_id: u64, amount: i128) {
        sender.require_auth();

        let transfer_key = DataKey::Transfer(transfer_id);
        let mut transfer = Self::read_transfer(&env, transfer_id);

        if transfer.sender != sender {
            panic!("Not the sender");
        }

        if transfer.status != STATUS_OPEN {
            panic!("Transfer is not open");
        }

        if amount != transfer.amount {
            panic!("Amount must match the opened amount");
        }

        transfer.status = STATUS_FUNDED;
        transfer.funded_at = Self::current_ledger(&env);

        env.storage().instance().set(&transfer_key, &transfer);

        Self::increase_total_funded(&env);
    }

    pub fn set_payout_partner(
        env: Env,
        admin: Address,
        transfer_id: u64,
        payout_partner: Address,
    ) {
        admin.require_auth();
        Self::require_admin(&env, &admin);

        let transfer_key = DataKey::Transfer(transfer_id);
        let mut transfer = Self::read_transfer(&env, transfer_id);

        if transfer.status != STATUS_FUNDED {
            panic!("Transfer is not in funded state");
        }

        transfer.payout_partner = payout_partner;

        env.storage().instance().set(&transfer_key, &transfer);
    }

    pub fn claim_payout(env: Env, payout_partner: Address, transfer_id: u64) {
        payout_partner.require_auth();

        let transfer_key = DataKey::Transfer(transfer_id);
        let mut transfer = Self::read_transfer(&env, transfer_id);

        if transfer.payout_partner != payout_partner {
            panic!("Not the designated payout partner");
        }

        if transfer.status != STATUS_FUNDED {
            panic!("Transfer is not in funded state");
        }

        if Self::current_ledger(&env) > transfer.dispute_deadline {
            panic!("Dispute window expired");
        }

        transfer.status = STATUS_PAID_OUT;

        env.storage().instance().set(&transfer_key, &transfer);

        Self::increase_total_paid_out(&env);
    }

    pub fn dispute(env: Env, sender: Address, transfer_id: u64, reason: Symbol) {
        sender.require_auth();

        let transfer_key = DataKey::Transfer(transfer_id);
        let mut transfer = Self::read_transfer(&env, transfer_id);

        if transfer.sender != sender {
            panic!("Not the sender");
        }

        if transfer.status != STATUS_FUNDED {
            panic!("Only funded transfers can be disputed");
        }

        if Self::current_ledger(&env) > transfer.dispute_deadline {
            panic!("Dispute window has expired");
        }

        transfer.status = STATUS_DISPUTED;
        transfer.dispute_reason = reason;

        env.storage().instance().set(&transfer_key, &transfer);

        Self::increase_total_disputed(&env);
    }

    pub fn resolve_dispute(
        env: Env,
        admin: Address,
        transfer_id: u64,
        release_to_sender: bool,
    ) {
        admin.require_auth();
        Self::require_admin(&env, &admin);

        let transfer_key = DataKey::Transfer(transfer_id);
        let mut transfer = Self::read_transfer(&env, transfer_id);

        if transfer.status != STATUS_DISPUTED {
            panic!("Transfer is not under dispute");
        }

        if release_to_sender {
            transfer.status = STATUS_RELEASED;
            Self::increase_total_released(&env);
        } else {
            transfer.status = STATUS_PAID_OUT;
            Self::increase_total_paid_out(&env);
        }

        env.storage().instance().set(&transfer_key, &transfer);
    }

    pub fn release(env: Env, sender: Address, transfer_id: u64) {
        sender.require_auth();

        let transfer_key = DataKey::Transfer(transfer_id);
        let mut transfer = Self::read_transfer(&env, transfer_id);

        if transfer.sender != sender {
            panic!("Not the sender");
        }

        if transfer.status != STATUS_FUNDED {
            panic!("Transfer is not in funded state");
        }

        if Self::current_ledger(&env) <= transfer.dispute_deadline {
            panic!("Dispute window has not yet expired");
        }

        transfer.status = STATUS_RELEASED;

        env.storage().instance().set(&transfer_key, &transfer);

        Self::increase_total_released(&env);
    }

    pub fn cancel(env: Env, sender: Address, transfer_id: u64) {
        sender.require_auth();

        let transfer_key = DataKey::Transfer(transfer_id);
        let mut transfer = Self::read_transfer(&env, transfer_id);

        if transfer.sender != sender {
            panic!("Not the sender");
        }

        if transfer.status != STATUS_OPEN {
            panic!("Transfer is not open");
        }

        transfer.status = STATUS_CANCELLED;

        env.storage().instance().set(&transfer_key, &transfer);

        Self::increase_total_cancelled(&env);
    }

    pub fn transfer_status(env: Env, transfer_id: u64) -> u32 {
        let transfer = Self::read_transfer(&env, transfer_id);
        transfer.status
    }

    pub fn get_transfer(env: Env, transfer_id: u64) -> Transfer {
        Self::read_transfer(&env, transfer_id)
    }

    pub fn transfers_of(env: Env, sender: Address) -> Map<u64, bool> {
        env.storage()
            .instance()
            .get(&DataKey::SenderList(sender))
            .unwrap_or(Map::new(&env))
    }

    pub fn corridor_stats(env: Env) -> CorridorStats {
        Self::read_stats(&env)
    }

    pub fn get_admin(env: Env) -> Address {
        Self::read_admin(&env)
    }

    pub fn get_dispute_window(env: Env) -> u64 {
        Self::read_dispute_window(&env)
    }
}

impl RemittanceCorridor {
    fn current_ledger(env: &Env) -> u64 {
        env.ledger().sequence() as u64
    }

    fn read_admin(env: &Env) -> Address {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .expect("Contract not initialized")
    }

    fn require_admin(env: &Env, admin: &Address) {
        let stored_admin = Self::read_admin(env);

        if &stored_admin != admin {
            panic!("Caller is not the corridor admin");
        }
    }

    fn read_next_id(env: &Env) -> u64 {
        env.storage()
            .instance()
            .get(&DataKey::NextId)
            .unwrap_or(1u64)
    }

    fn read_dispute_window(env: &Env) -> u64 {
        env.storage()
            .instance()
            .get(&DataKey::DisputeWindow)
            .unwrap_or(DEFAULT_DISPUTE_WINDOW_LEDGERS)
    }

    fn read_transfer(env: &Env, transfer_id: u64) -> Transfer {
        env.storage()
            .instance()
            .get(&DataKey::Transfer(transfer_id))
            .expect("Transfer not found")
    }

    fn read_stats(env: &Env) -> CorridorStats {
        env.storage()
            .instance()
            .get(&DataKey::Stats)
            .unwrap_or(CorridorStats {
                total_opened: 0,
                total_funded: 0,
                total_paid_out: 0,
                total_disputed: 0,
                total_released: 0,
                total_cancelled: 0,
            })
    }

    fn save_stats(env: &Env, stats: &CorridorStats) {
        env.storage().instance().set(&DataKey::Stats, stats);
    }

    fn increase_total_opened(env: &Env) {
        let mut stats = Self::read_stats(env);
        stats.total_opened += 1;
        Self::save_stats(env, &stats);
    }

    fn increase_total_funded(env: &Env) {
        let mut stats = Self::read_stats(env);
        stats.total_funded += 1;
        Self::save_stats(env, &stats);
    }

    fn increase_total_paid_out(env: &Env) {
        let mut stats = Self::read_stats(env);
        stats.total_paid_out += 1;
        Self::save_stats(env, &stats);
    }

    fn increase_total_disputed(env: &Env) {
        let mut stats = Self::read_stats(env);
        stats.total_disputed += 1;
        Self::save_stats(env, &stats);
    }

    fn increase_total_released(env: &Env) {
        let mut stats = Self::read_stats(env);
        stats.total_released += 1;
        Self::save_stats(env, &stats);
    }

    fn increase_total_cancelled(env: &Env) {
        let mut stats = Self::read_stats(env);
        stats.total_cancelled += 1;
        Self::save_stats(env, &stats);
    }
}