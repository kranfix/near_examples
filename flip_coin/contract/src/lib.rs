mod side;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{log, near_bindgen, AccountId};
use side::Side;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct CoinFlip {
  points: UnorderedMap<AccountId, u32>,
}

// Define the default, which automatically initializes the contract
impl Default for CoinFlip {
  fn default() -> Self {
    Self {
      points: UnorderedMap::new(b'p'),
    }
  }
}

#[near_bindgen]
impl CoinFlip {
  // Flip a coin. Pass in the side (heads or tails) and a random number will be chosen
  // indicating whether the flip was heads or tails. If you got it right, you get a point.
  pub fn flip_coin(&mut self, player_guess: String) -> String {
    let player_guess: Side = player_guess.try_into().unwrap();

    let outcome = Side::random();

    let player = near_sdk::env::signer_account_id();
    let mut player_points: u32 = self.points.get(&player).unwrap_or(0);

    if player_guess == outcome {
      log!("The result was {}, you get a point!", outcome);
      player_points += 1;
    } else {
      log!("The result was {}, you lost a point", outcome);
      if player_points > 0 {
        player_points -= 1;
      }
    }

    // Store the new points
    self.points.insert(&player, &player_points);

    outcome.to_string()
  }

  pub fn points_of(&self, player: String) -> u32 {
    let player = AccountId::new_unchecked(player);
    let points = self.points.get(&player).unwrap_or(0);
    log!("Points for {}: {}", player, points);
    points
  }
}

// https://doc.rust-lang.org/book/ch11-01-writing-tests.html
// #[cfg(test)]
// mod tests {
//   use super::*;

//   #[test]
//   fn get_default_greeting() {
//     let contract = CoinFlip::default();
//     // this test did not call set_greeting so should return the default "Hello" greeting
//     assert_eq!(contract.get_greeting(), "Hello".to_string());
//   }

//   #[test]
//   fn set_then_get_greeting() {
//     let mut contract = CoinFlip::default();
//     contract.set_greeting("howdy".to_string());
//     assert_eq!(contract.get_greeting(), "howdy".to_string());
//   }
// }
