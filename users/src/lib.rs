#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod users {
    use ink::storage::Mapping;

    #[ink::storage_item]
    #[derive(Debug)]
    pub struct DefaultMap {
        values: Mapping<AccountId, u8>,
        length: u32,
    }

    #[ink(storage)]
    pub struct Users {
        owner: AccountId,
        followers:  DefaultMap,
    }

    impl Users {
        /// Creates a new flipper smart contract initialized with the given value.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { owner: Self::env().caller(), followers: DefaultMap { values: Mapping::default(), length: 0 } }
        }

        #[ink(message)]
        pub fn verify_user(&self,id:AccountId) {
            assert!(self.owner == id, "Not the owner of the contract!");
        }

    }
   
}