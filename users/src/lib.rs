#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod users {
    use ink::storage::Mapping;

    
    /// Followers for a user.
    #[ink::storage_item]
    #[derive(Debug)]
    pub struct Followers {
        /// Mapping of Follower, to followed.
        list: Mapping<AccountId, AccountId>,
        /// Mapping of followed account to number of followers.
        length: Mapping<AccountId, u32>,
    }

    #[ink(storage)]
    pub struct Users {
        /// Mapping of User count to index.
        users: Mapping<AccountId, u32>,
        /// Users count.
        user_count: u32,
        /// Followers.
        followers: Followers
    }

    #[ink(event)]
    pub struct CreateUser {
        id: AccountId,
        index: u32
    }

    #[ink(event)]
    pub struct FollowUser {
        follower: AccountId,
        followed: AccountId,
        follower_count: u32
    }

    impl Users {
        /// Creates a new User smart contract initialized with the given value.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { users: Mapping::default(), user_count:0,  followers: Followers { list: Mapping::default(), length: Mapping::default() } }
        }

        /// Checks if the user account is registered.
        #[ink(message)]
        pub fn verify_user(&self,id:AccountId) {
            assert!(self.users.get(id) != None, "Not the owner of the contract!");
        }

        /// Creates a user in the contract.
        #[ink(message)]
        pub fn create_user(&mut self) {
            self.users.insert(self.env().caller(), &self.user_count);
            self.followers.length.insert(self.env().caller(), &0);
            self.env().emit_event(CreateUser{ id: self.env().caller(), index: self.user_count});
            self.user_count += 1;
        }


        /// Follows a user.
        #[ink(message)]
        pub fn follow_user(&mut self, id:AccountId) {
            self.followers.list.insert(self.env().caller(), &id);
            let length: &u32 = &(self.followers.length.get(id)).expect("Followers not set!");
            self.followers.length.insert(id, &(length +1));
            self.env().emit_event(FollowUser{ follower: self.env().caller(), followed: id, follower_count: length-1 })
        }

        #[ink(message)]
        pub fn get_followers(&self, id:AccountId) -> u32 {
            return self.followers.length.get(id).expect("You have no followers!");
        }

        

    }
   
}