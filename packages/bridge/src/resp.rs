use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resp<T> {
  pub code: i32,
  pub msg: String,
  pub data: Option<T>,
}

impl<T> Resp<T> {
  #[allow(unused)]
  fn new(code: i32, msg: String, data: Option<T>) -> Self {
    Resp { code, msg, data }
  }
}
