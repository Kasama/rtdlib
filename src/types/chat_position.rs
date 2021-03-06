
use crate::types::*;
use crate::errors::*;




/// Describes a position of a chat in a chat list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatPosition {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// The chat list
  list: ChatList,
  /// A parameter used to determine order of the chat in the chat list. Chats must be sorted by the pair (order, chat.id) in descending order
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")] order: isize,
  /// True, if the chat is pinned in the chat list
  is_pinned: bool,
  /// Source of the chat in the chat list; may be null
  source: Option<ChatSource>,
  
}

impl RObject for ChatPosition {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatPosition" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ChatPosition {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatPositionBuilder {
    let mut inner = ChatPosition::default();
    inner.td_name = "chatPosition".to_string();
    RTDChatPositionBuilder { inner }
  }

  pub fn list(&self) -> &ChatList { &self.list }

  pub fn order(&self) -> isize { self.order }

  pub fn is_pinned(&self) -> bool { self.is_pinned }

  pub fn source(&self) -> &Option<ChatSource> { &self.source }

}

#[doc(hidden)]
pub struct RTDChatPositionBuilder {
  inner: ChatPosition
}

impl RTDChatPositionBuilder {
  pub fn build(&self) -> ChatPosition { self.inner.clone() }

   
  pub fn list<T: AsRef<ChatList>>(&mut self, list: T) -> &mut Self {
    self.inner.list = list.as_ref().clone();
    self
  }

   
  pub fn order(&mut self, order: isize) -> &mut Self {
    self.inner.order = order;
    self
  }

   
  pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
    self.inner.is_pinned = is_pinned;
    self
  }

   
  pub fn source<T: AsRef<ChatSource>>(&mut self, source: T) -> &mut Self {
    self.inner.source = Some(source.as_ref().clone());
    self
  }

}

impl AsRef<ChatPosition> for ChatPosition {
  fn as_ref(&self) -> &ChatPosition { self }
}

impl AsRef<ChatPosition> for RTDChatPositionBuilder {
  fn as_ref(&self) -> &ChatPosition { &self.inner }
}



