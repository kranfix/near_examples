mod meme_museum;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen, AccountId, Gas, Promise};

use meme_museum::{ext_meme_museum, TGAS};

// Define the default message
const DEFAULT_MESSAGE: &str = "Hello";

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
  message: String,
  meme_museum_contract_id: String,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
  fn default() -> Self {
    Self {
      message: DEFAULT_MESSAGE.to_string(),
      meme_museum_contract_id: "dev-1660968562799-63169581314581".to_string(),
    }
  }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
  // Public method - returns the greeting saved, defaulting to DEFAULT_MESSAGE
  pub fn get_greeting(&self) -> String {
    return self.message.clone();
  }

  // Public method - accepts a greeting, such as "howdy", and records it
  pub fn set_greeting(&mut self, message: String) {
    // Use env::log to record logs permanently to the blockchain!
    log!("Saving greeting {}", message);
    self.message = message;
  }

  pub fn get_museum_list(&self) -> Promise {
    let account_id = self.meme_museum_contract_id.clone();
    let promise = ext_meme_museum::ext(AccountId::new_unchecked(account_id)).get_museum_list();

    promise.then(
      Self::ext(env::current_account_id())
        .with_static_gas(Gas(5 * TGAS))
        .get_museum_list_callback(),
    )
  }

  #[private]
  pub fn get_museum_list_callback(&self, #[callback_unwrap] museums: Vec<String>) -> Vec<String> {
    museums
  }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn get_default_greeting() {
    let contract = Contract::default();
    // this test did not call set_greeting so should return the default "Hello" greeting
    assert_eq!(contract.get_greeting(), "Hello".to_string());
  }

  #[test]
  fn set_then_get_greeting() {
    let mut contract = Contract::default();
    contract.set_greeting("howdy".to_string());
    assert_eq!(contract.get_greeting(), "howdy".to_string());
  }
}
