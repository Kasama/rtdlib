
use crate::types::*;
use crate::errors::*;




/// Contains a value representing a number of seconds
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Seconds {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Number of seconds
  seconds: f32,
  
}

impl RObject for Seconds {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "seconds" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Seconds {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSecondsBuilder {
    let mut inner = Seconds::default();
    inner.td_name = "seconds".to_string();
    RTDSecondsBuilder { inner }
  }

  pub fn seconds(&self) -> f32 { self.seconds }

}

#[doc(hidden)]
pub struct RTDSecondsBuilder {
  inner: Seconds
}

impl RTDSecondsBuilder {
  pub fn build(&self) -> Seconds { self.inner.clone() }

   
  pub fn seconds(&mut self, seconds: f32) -> &mut Self {
    self.inner.seconds = seconds;
    self
  }

}

impl AsRef<Seconds> for Seconds {
  fn as_ref(&self) -> &Seconds { self }
}

impl AsRef<Seconds> for RTDSecondsBuilder {
  fn as_ref(&self) -> &Seconds { &self.inner }
}



