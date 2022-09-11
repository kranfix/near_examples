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
  pub fn create_meme(&mut self, title: String, url: String, museum_name: String) -> Meme {
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
    meme
  }

  #[result_serializer(borsh)]
  pub fn create_meme_borsh(
    &mut self,
    #[serializer(borsh)] title: String,
    #[serializer(borsh)] url: String,
    #[serializer(borsh)] museum_name: String,
  ) -> Meme {
    self.create_meme(title, url, museum_name)
  }

  pub fn get_meme(&self, id: u64) -> Option<Meme> {
    self.memes.get(&id)
  }

  #[result_serializer(borsh)]
  pub fn get_meme_borsh(&self, #[serializer(borsh)] id: u64) -> Option<Meme> {
    self.memes.get(&id)
  }

  pub fn get_meme_list(&self) -> Vec<(u64, Meme)> {
    self.memes.iter().collect()
  }

  #[result_serializer(borsh)]
  pub fn get_meme_list_borsh(&self) -> Vec<(u64, Meme)> {
    self.memes.iter().collect()
  }

  pub fn get_museum_list(&self) -> Vec<String> {
    self.museums.keys().collect()
  }

  pub fn get_museum_list_borsh(&self) -> Vec<String> {
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

  #[result_serializer(borsh)]
  pub fn get_meme_list_by_museum_borsh(
    &self,
    #[serializer(borsh)] museum_name: String,
  ) -> Vec<Meme> {
    self.get_meme_list_by_museum(museum_name)
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

  // delete a museum and its memes
  pub fn delete_museum(&mut self, museum_name: String) -> bool {
    let memes = match self.museums.get(&museum_name) {
      Some(memes) => memes,
      None => {
        env::log(format!("Musemum {} not found", museum_name).as_bytes());
        return false;
      }
    };
    for meme in memes {
      self.memes.remove(&meme);
    }
    self.museums.remove(&museum_name);
    true
  }
}
