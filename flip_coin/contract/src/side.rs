const HEADS: &str = "heads";
const TAILS: &str = "tails";

#[derive(PartialEq)]
pub enum Side {
  Heads,
  Tails,
}

impl Side {
  pub fn random() -> Side {
    let random = near_sdk::env::random_seed_array();
    if random[0] % 2 == 0 {
      Side::Heads
    } else {
      Side::Tails
    }
  }
}

impl std::fmt::Display for Side {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let val = match self {
      Side::Heads => HEADS,
      Side::Tails => TAILS,
    }
    .to_owned();
    write!(f, "{}", val)
  }
}

// impl std::string::ToString for Side {
//   fn to_string(&self) -> String {
//     match self {
//       Side::Heads => HEADS,
//       Side::Tails => TAILS,
//     }
//     .to_owned()
//   }
// }

impl TryFrom<String> for Side {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    match value.as_str() {
      HEADS => Ok(Side::Heads),
      TAILS => Ok(Side::Tails),
      _ => Err("Side: value can not be parsed"),
    }
  }
}
