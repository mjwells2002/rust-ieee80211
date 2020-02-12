mod builder;
mod fixed_parameters;

pub use self::{builder::*, fixed_parameters::*};
use super::*;

pub struct DeauthenticationFrame {
  bytes: Bytes,
}

impl DeauthenticationFrame {
  pub fn new<T: Into<Bytes>>(bytes: T) -> Self {
    Self {
      bytes: bytes.into(),
    }
  }
}
impl FrameTrait for DeauthenticationFrame {
  fn bytes(&self) -> Bytes {
    self.bytes.clone()
  }
}
impl FragmentSequenceTrait for DeauthenticationFrame {}
impl ManagementFrameTrait for DeauthenticationFrame {}
impl DeauthenticationFixedParametersTrait for DeauthenticationFrame {}
