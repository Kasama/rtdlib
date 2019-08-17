
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The list of recently used stickers was updated. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRecentStickers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateRecentStickers
  /// True, if the list of stickers attached to photo or video files was updated, otherwise the list of sent stickers is updated.
  is_attached: Option<bool>,
  /// The new list of file identifiers of recently used stickers.
  sticker_ids: Option<Vec<i32>>,
  
}



impl Object for UpdateRecentStickers {}
impl RObject for UpdateRecentStickers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateRecentStickers" }
  fn td_type(&self) -> RTDType { RTDType::UpdateRecentStickers }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateRecentStickers {}


impl UpdateRecentStickers {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateRecentStickers".to_string(),
      is_attached: None,
      sticker_ids: None,
      
    }
  }
  
  pub fn is_attached(&self) -> Option<bool> { self.is_attached.clone() }
  #[doc(hidden)] pub fn _set_is_attached(&mut self, is_attached: bool) -> &mut Self { self.is_attached = Some(is_attached); self }
  
  pub fn sticker_ids(&self) -> Option<Vec<i32>> { self.sticker_ids.clone() }
  #[doc(hidden)] pub fn _set_sticker_ids(&mut self, sticker_ids: Vec<i32>) -> &mut Self { self.sticker_ids = Some(sticker_ids); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



