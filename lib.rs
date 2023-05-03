#![cfg_attr(not(feature = "std"), no_std)]

use derive_more::From;
use ink::{
    self,
    env::{Environment, NoChainExtension},
};
use scale::*;
#[cfg(feature = "std")]
use scale_info::*;

use ink::prelude::vec::Vec;
use ink::primitives::Clear;
use core::array::TryFromSliceError;

#[cfg(feature = "std")]
use {scale_decode::DecodeAsType, scale_encode::EncodeAsType, scale_info::TypeInfo};

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Decode, Encode, From)]
#[cfg_attr(feature = "std", derive(TypeInfo, EncodeAsType, DecodeAsType, ink::storage::traits::StorageLayout))]
pub struct MyAccountId([u8; 16]);

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct CustomEnv;

impl Environment for CustomEnv {
    //Can be instantiated and executed with mismatch but fails when emitting more topics than allowed by the pallet
    const MAX_EVENT_TOPICS: usize = 4;
    //`MyAccountId` fails
    type AccountId = ink::primitives::AccountId;
    //u64 fail instantiation
    type Balance = u128;
    //Works with `MyHash` as long as the internal slice is the same size
    type Hash = MyHash;
    //Works as long as >32bytes
    type Timestamp = u64;
    //Works as long as >32bytes
    type BlockNumber = u64;
    //fails if mismatched
    type ChainExtension = NoChainExtension;
}

impl AsMut<[u8]> for MyAccountId {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0[..]
    }
}

impl AsRef<[u8]> for MyAccountId {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Ord,
    PartialOrd,
    Hash,
    Decode,
    Encode,
    From,
)]
#[cfg_attr(feature = "std", derive(TypeInfo, DecodeAsType, EncodeAsType))]
pub struct MyHash([u8; 32]);

impl<'a> TryFrom<&'a [u8]> for MyHash {
    type Error = TryFromSliceError;

    fn try_from(bytes: &'a [u8]) -> Result<Self, TryFromSliceError> {
        let hash = <[u8; 32]>::try_from(bytes)?;
        Ok(Self(hash))
    }
}

impl AsRef<[u8]> for MyHash {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl AsMut<[u8]> for MyHash {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0[..]
    }
}

impl Clear for MyHash {
    const CLEAR_HASH: Self = MyHash([0x00; 32]);

    fn is_clear(&self) -> bool {
        self == &Self::CLEAR_HASH
    }
}

#[ink::contract(env = CustomEnv)]
mod assets {

    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;
    use ink::prelude::format;

    use crate::CustomEnv;

    #[derive(Default)]
    #[ink(event)]
    pub struct MyEvent {
        #[ink(topic)]
        topic_one: u128,
        #[ink(topic)]
        topic_two: u128,
        #[ink(topic)]
        topic_three: u128,
        #[ink(topic)]
        topic_four: u128,
        // #[ink(topic)]
        topic_five: u128,
    }

    #[derive(Debug, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        E(String)
    }

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout, Debug)
    )]
    pub struct IndividualFundMe {
        account_id: AccountId,
        go_fund_id: u64,
        name: String,
        reason_for_fund: String,
        amount_needed: Balance,
        amount_gotten: Balance,
        status: bool,
        donators: Vec<AccountId>,
    }

    #[ink(storage)]
    pub struct Assets {
        go_funds: Mapping<AccountId, IndividualFundMe>,
        all_go_funds: Vec<IndividualFundMe>,
        successfull_go_funds: Vec<IndividualFundMe>,
        go_fund_id: u64,
    }

    impl Assets {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                go_funds: Mapping::new(),
                all_go_funds: Vec::new(),
                successfull_go_funds: Vec::new(),
                go_fund_id: 1,
            }
        }

        #[ink(message)]
        pub fn create_crowdfund(
            &mut self,
            _name: String,
            _reason_for_fund: String,
            _amount_needed: Balance,
        ) -> Result<(), Error> {
            let new_gofund = IndividualFundMe {
                account_id: self.env().caller(),
                go_fund_id: self.go_fund_id,
                name: _name,
                reason_for_fund: _reason_for_fund,
                amount_needed: _amount_needed,
                amount_gotten: 0,
                status: false,
                donators: Vec::new(),
            };
            self.go_funds.insert(self.env().caller(), &new_gofund);
            self.all_go_funds.push(new_gofund);
            Self::env().transfer(self.env().account_id(), 1000).map_err(|e| Error::E(format!("{:?}", e)))?;
            Self::env().emit_event(MyEvent{
                topic_one: Self::env().block_number().into(),
                topic_two: Self::env().block_timestamp().into(),
                ..Default::default()
            });
            self.go_fund_id += 1;
            Ok(())
        }
    }
}
