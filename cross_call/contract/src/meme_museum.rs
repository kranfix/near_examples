use near_sdk::ext_contract;

pub const TGAS: u64 = 1_000_000_000_000;

#[ext_contract(ext_meme_museum)]
trait MemeMuseumReader {
  fn get_museum_list(&self) -> Vec<String>;
}
