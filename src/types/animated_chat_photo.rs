
use crate::types::*;
use crate::errors::*;




/// Animated variant of a chat photo in MPEG4 format
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnimatedChatPhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Animation width and height
  length: i64,
  /// Information about the animation file
  file: File,
  /// Timestamp of the frame, used as static chat photo
  main_frame_timestamp: f32,
  
}

impl RObject for AnimatedChatPhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "animatedChatPhoto" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl AnimatedChatPhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAnimatedChatPhotoBuilder {
    let mut inner = AnimatedChatPhoto::default();
    inner.td_name = "animatedChatPhoto".to_string();
    RTDAnimatedChatPhotoBuilder { inner }
  }

  pub fn length(&self) -> i64 { self.length }

  pub fn file(&self) -> &File { &self.file }

  pub fn main_frame_timestamp(&self) -> f32 { self.main_frame_timestamp }

}

#[doc(hidden)]
pub struct RTDAnimatedChatPhotoBuilder {
  inner: AnimatedChatPhoto
}

impl RTDAnimatedChatPhotoBuilder {
  pub fn build(&self) -> AnimatedChatPhoto { self.inner.clone() }

   
  pub fn length(&mut self, length: i64) -> &mut Self {
    self.inner.length = length;
    self
  }

   
  pub fn file<T: AsRef<File>>(&mut self, file: T) -> &mut Self {
    self.inner.file = file.as_ref().clone();
    self
  }

   
  pub fn main_frame_timestamp(&mut self, main_frame_timestamp: f32) -> &mut Self {
    self.inner.main_frame_timestamp = main_frame_timestamp;
    self
  }

}

impl AsRef<AnimatedChatPhoto> for AnimatedChatPhoto {
  fn as_ref(&self) -> &AnimatedChatPhoto { self }
}

impl AsRef<AnimatedChatPhoto> for RTDAnimatedChatPhotoBuilder {
  fn as_ref(&self) -> &AnimatedChatPhoto { &self.inner }
}



