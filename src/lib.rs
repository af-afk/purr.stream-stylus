#![cfg_attr(target_arch = "wasm32", no_std)]

extern crate alloc;

use stylus_sdk::{
    alloy_primitives::{aliases::*, *},
    alloy_sol_types::{SolError, sol},
    prelude::*,
    storage::*,
    stylus_core,
};

use alloc::{vec, vec::Vec};

pub type CatIdentifier = FixedBytes<8>;

pub type R<T> = Result<T, Vec<u8>>;

#[storage]
#[entrypoint]
pub struct Storage {
    pub version: StorageU32,

    // User that can claim the donations made to the contract.
    pub operator: StorageAddress,

    // Epoch count that's taking place for the leaderboard.
    pub epoch_count: StorageU256,

    // Donations made by different wallet addresses.
    pub user_donations: StorageMap<Address, StorageU256>,

    // Cat donations made per epoch.
    pub cat_donations: StorageMap<U256, StorageMap<CatIdentifier, StorageU256>>,
}

macro_rules! require {
    ($r:expr, $err:expr) => {{
        if !$r {
            return Err($err.abi_encode());
        }
    }};
}

macro_rules! add_to_slot {
    ($t:expr, $v:expr) => {{
        let d = $t.get();
        $t.set(
            d.checked_add($v)
                .ok_or(ErrCheckedOverflow { x: d, y: $v }.abi_encode())?,
        );
        $t.get()
    }};
    ($t:expr, $f:expr, $v:expr) => {{
        let d = $t.get($f);
        $t.setter($f).set(
            d.checked_add($v)
                .ok_or(ErrCheckedOverflow { x: d, y: $v }.abi_encode())?,
        );
        $t.get($f)
    }};
    ($t:expr, $f:expr, $k:expr, $v:expr) => {{
        let d = $t.get($f).get($k);
        $t.setter($f).setter($k).set(
            d.checked_add($v)
                .ok_or(ErrCheckedOverflow { x: d, y: $v }.abi_encode())?,
        );
        $t.getter($f).get($k)
    }};
}

sol! {
    error ErrCheckedOverflow(uint256 x, uint256 y);
    error ErrNotOperator(address operator);

    event EventDonated(
        bytes8 indexed cat,
        uint256 indexed amount,
        address indexed recipient
    );

    event EventTaken(uint256 indexed amount, address indexed recipient);

    event EventEpochBumped(uint256 indexed epoch);
}

#[public]
impl Storage {
    #[payable]
    pub fn make_donation(&mut self, cat: CatIdentifier, recipient: Address) -> R<U256> {
        let e = self.epoch_count.get();
        let v = self.vm().msg_value();
        add_to_slot!(self.user_donations, recipient, v);
        add_to_slot!(self.cat_donations, e, cat, v);
        stylus_core::log(
            self.vm(),
            EventDonated {
                cat,
                amount: v,
                recipient,
            },
        );
        Ok(v)
    }

    pub fn take(&mut self, recipient: Address) -> R<U256> {
        require!(
            self.vm().msg_sender() == self.operator.get(),
            ErrNotOperator {
                operator: self.operator.get()
            }
        );
        let bal = self.vm().balance(self.vm().contract_address());
        self.vm().transfer_eth(recipient, bal)?;
        stylus_core::log(
            self.vm(),
            EventTaken {
                amount: bal,
                recipient,
            },
        );
        Ok(bal)
    }

    pub fn reset(&mut self) -> R<U256> {
        require!(
            self.vm().msg_sender() == self.operator.get(),
            ErrNotOperator {
                operator: self.operator.get()
            }
        );
        let epoch = add_to_slot!(self.epoch_count, U256::from(1));
        stylus_core::log(self.vm(), EventEpochBumped { epoch });
        Ok(epoch)
    }

    pub fn get(&self, cat: CatIdentifier) -> U256 {
        self.cat_donations.getter(self.epoch_count.get()).get(cat)
    }
}

#[no_mangle]
#[cfg(not(target_arch = "wasm32"))]
pub unsafe extern "C" fn native_keccak256(bytes: *const u8, len: usize, output: *mut u8) {
    use core::slice;
    use tiny_keccak::{Hasher, Keccak};
    let mut hasher = Keccak::v256();
    let data = unsafe { slice::from_raw_parts(bytes, len) };
    hasher.update(data);
    let output = unsafe { slice::from_raw_parts_mut(output, 32) };
    hasher.finalize(output);
}
