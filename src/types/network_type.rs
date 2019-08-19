
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// This class is an abstract base class. Represents the type of a network. 
#[typetag::serde(tag = "@struct")]
pub trait NetworkType: Object + RObject + Debug {}






impl NetworkType {
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Box<Self>> { from_json!()(json.as_ref()) }
}
impl Clone for Box<NetworkType> {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, EnumString)]
pub enum RTDNetworkTypeType {
  NetworkTypeMobile,
  NetworkTypeMobileRoaming,
  NetworkTypeNone,
  NetworkTypeOther,
  NetworkTypeWiFi,
  
}
impl RTDNetworkTypeType {
  pub fn of<S: AsRef<str>>(text: S) -> Option<Self> { rtd_of!(RTDNetworkTypeType)(text.as_ref()) }
}


