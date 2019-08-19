
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns only photo messages. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMessagesFilterPhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // searchMessagesFilterPhoto
  
}



impl Object for SearchMessagesFilterPhoto {}
impl RObject for SearchMessagesFilterPhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchMessagesFilterPhoto" }
  fn td_type(&self) -> RTDType { RTDType::SearchMessagesFilterPhoto }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl SearchMessagesFilter for SearchMessagesFilterPhoto {}


impl SearchMessagesFilterPhoto {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "searchMessagesFilterPhoto".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}


