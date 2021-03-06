
use crate::types::*;
use crate::errors::*;




/// Describes an action associated with a bank card number
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BankCardActionOpenUrl {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Action text
  text: String,
  /// The URL to be opened
  url: String,
  
}

impl RObject for BankCardActionOpenUrl {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "bankCardActionOpenUrl" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl BankCardActionOpenUrl {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDBankCardActionOpenUrlBuilder {
    let mut inner = BankCardActionOpenUrl::default();
    inner.td_name = "bankCardActionOpenUrl".to_string();
    RTDBankCardActionOpenUrlBuilder { inner }
  }

  pub fn text(&self) -> &String { &self.text }

  pub fn url(&self) -> &String { &self.url }

}

#[doc(hidden)]
pub struct RTDBankCardActionOpenUrlBuilder {
  inner: BankCardActionOpenUrl
}

impl RTDBankCardActionOpenUrlBuilder {
  pub fn build(&self) -> BankCardActionOpenUrl { self.inner.clone() }

   
  pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().to_string();
    self
  }

   
  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

}

impl AsRef<BankCardActionOpenUrl> for BankCardActionOpenUrl {
  fn as_ref(&self) -> &BankCardActionOpenUrl { self }
}

impl AsRef<BankCardActionOpenUrl> for RTDBankCardActionOpenUrlBuilder {
  fn as_ref(&self) -> &BankCardActionOpenUrl { &self.inner }
}



