
use crate::types::*;
use crate::errors::*;




/// Contains a list of chat lists
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatLists {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// List of chat lists
  chat_lists: Vec<ChatList>,
  
}

impl RObject for ChatLists {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatLists" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ChatLists {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatListsBuilder {
    let mut inner = ChatLists::default();
    inner.td_name = "chatLists".to_string();
    RTDChatListsBuilder { inner }
  }

  pub fn chat_lists(&self) -> &Vec<ChatList> { &self.chat_lists }

}

#[doc(hidden)]
pub struct RTDChatListsBuilder {
  inner: ChatLists
}

impl RTDChatListsBuilder {
  pub fn build(&self) -> ChatLists { self.inner.clone() }

   
  pub fn chat_lists(&mut self, chat_lists: Vec<ChatList>) -> &mut Self {
    self.inner.chat_lists = chat_lists;
    self
  }

}

impl AsRef<ChatLists> for ChatLists {
  fn as_ref(&self) -> &ChatLists { self }
}

impl AsRef<ChatLists> for RTDChatListsBuilder {
  fn as_ref(&self) -> &ChatLists { &self.inner }
}



