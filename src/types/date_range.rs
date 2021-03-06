
use crate::types::*;
use crate::errors::*;




/// Represents a date range
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DateRange {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Point in time (Unix timestamp) at which the date range begins
  start_date: i64,
  /// Point in time (Unix timestamp) at which the date range ends
  end_date: i64,
  
}

impl RObject for DateRange {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "dateRange" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl DateRange {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDDateRangeBuilder {
    let mut inner = DateRange::default();
    inner.td_name = "dateRange".to_string();
    RTDDateRangeBuilder { inner }
  }

  pub fn start_date(&self) -> i64 { self.start_date }

  pub fn end_date(&self) -> i64 { self.end_date }

}

#[doc(hidden)]
pub struct RTDDateRangeBuilder {
  inner: DateRange
}

impl RTDDateRangeBuilder {
  pub fn build(&self) -> DateRange { self.inner.clone() }

   
  pub fn start_date(&mut self, start_date: i64) -> &mut Self {
    self.inner.start_date = start_date;
    self
  }

   
  pub fn end_date(&mut self, end_date: i64) -> &mut Self {
    self.inner.end_date = end_date;
    self
  }

}

impl AsRef<DateRange> for DateRange {
  fn as_ref(&self) -> &DateRange { self }
}

impl AsRef<DateRange> for RTDDateRangeBuilder {
  fn as_ref(&self) -> &DateRange { &self.inner }
}



