mod meme;

use meme::Meme;
use near_sdk::{
  borsh::{self, BorshDeserialize, BorshSerialize},
  collections::UnorderedMap,
  env, near_bindgen, setup_alloc, Promise,
};

setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct SimpleMemeMuseum {
  museums: UnorderedMap<String, Vec<u64>>,
  memes: UnorderedMap<u64, Meme>,
}

impl Default for SimpleMemeMuseum {
  fn default() -> Self {
    Self {
      museums: UnorderedMap::new(b"u".to_vec()),
      memes: UnorderedMap::new(b"e".to_vec()),
    }
  }
}

#[near_bindgen]
impl SimpleMemeMuseum {
  pub fn create_meme(&mut self, title: String, url: String, museum_name: String) {
    let meme = Meme::new(title, museum_name.clone(), url);

    self.memes.insert(&meme.id, &meme);

    let mut m = self.museums.get(&museum_name).unwrap_or_else(|| Vec::new());

    m.push(meme.id);
    self.museums.insert(&museum_name, &m);

    env::log(
      format!(
        "Meme {} created by {} in {} museum",
        meme.id, meme.created_by, museum_name
      )
      .as_bytes(),
    );
  }

  pub fn get_meme(&self, id: u64) -> Option<Meme> {
    self.memes.get(&id)
  }

  pub fn get_meme_list(&self) -> Vec<(u64, Meme)> {
    self.memes.iter().collect()
  }

  pub fn get_museum_list(&self) -> Vec<String> {
    self.museums.keys().collect()
  }

  pub fn get_meme_list_by_museum(&self, museum_name: String) -> Vec<Meme> {
    let meme_ids = match self.museums.get(&museum_name) {
      None => return vec![],
      Some(meme_ids) => meme_ids,
    };

    meme_ids
      .iter()
      .filter_map(|id| self.memes.get(id))
      .collect()
  }

  #[payable]
  pub fn donate_meme(&mut self, id: u64) -> bool {
    assert!(
      env::attached_deposit() > 0,
      "Add NEAR to your balance to donate",
    );

    let mut meme = match self.memes.get(&id) {
      None => return false,
      Some(meme) => meme,
    };

    let amount = env::attached_deposit();
    meme.donations += amount;
    self.memes.insert(&id, &meme);
    Promise::new(meme.created_by.clone()).transfer(amount);
    true
  }
}
