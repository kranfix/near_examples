use near_sdk::{
  borsh::{self, BorshDeserialize, BorshSerialize},
  env,
  serde::{Deserialize, Serialize},
};

#[derive(Deserialize, Serialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Meme {
  pub id: u64,
  pub title: String,
  pub created_by: String,
  pub museum: String,
  pub url: String,
  pub donations: u128,
}

impl Default for Meme {
  fn default() -> Self {
    Self {
      id: env::block_height(),
      title: "".to_string(),
      created_by: env::signer_account_id().to_string(),
      museum: "".to_string(),
      url: "".to_string(),
      donations: 0,
    }
  }
}

impl Meme {
  pub fn new(title: String, museum: String, url: String) -> Self {
    Self {
      id: env::block_height(),
      title,
      created_by: env::signer_account_id().to_string(),
      museum,
      url,
      donations: 0,
    }
  }
}
