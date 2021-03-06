
use crate::types::*;
use crate::errors::*;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Contains information about a Telegram Passport element to be saved
pub trait TDInputPassportElement: Debug + RObject {}

/// Contains information about a Telegram Passport element to be saved
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InputPassportElement {
  #[doc(hidden)] _Default(()),
  /// A Telegram Passport element to be saved containing the user's address
  Address(InputPassportElementAddress),
  /// A Telegram Passport element to be saved containing the user's bank statement
  BankStatement(InputPassportElementBankStatement),
  /// A Telegram Passport element to be saved containing the user's driver license
  DriverLicense(InputPassportElementDriverLicense),
  /// A Telegram Passport element to be saved containing the user's email address
  EmailAddress(InputPassportElementEmailAddress),
  /// A Telegram Passport element to be saved containing the user's identity card
  IdentityCard(InputPassportElementIdentityCard),
  /// A Telegram Passport element to be saved containing the user's internal passport
  InternalPassport(InputPassportElementInternalPassport),
  /// A Telegram Passport element to be saved containing the user's passport
  Passport(InputPassportElementPassport),
  /// A Telegram Passport element to be saved containing the user's passport registration
  PassportRegistration(InputPassportElementPassportRegistration),
  /// A Telegram Passport element to be saved containing the user's personal details
  PersonalDetails(InputPassportElementPersonalDetails),
  /// A Telegram Passport element to be saved containing the user's phone number
  PhoneNumber(InputPassportElementPhoneNumber),
  /// A Telegram Passport element to be saved containing the user's rental agreement
  RentalAgreement(InputPassportElementRentalAgreement),
  /// A Telegram Passport element to be saved containing the user's temporary registration
  TemporaryRegistration(InputPassportElementTemporaryRegistration),
  /// A Telegram Passport element to be saved containing the user's utility bill
  UtilityBill(InputPassportElementUtilityBill),

}

impl Default for InputPassportElement {
  fn default() -> Self { InputPassportElement::_Default(()) }
}

impl<'de> Deserialize<'de> for InputPassportElement {
  fn deserialize<D>(deserializer: D) -> Result<InputPassportElement, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      InputPassportElement,
      (inputPassportElementAddress, Address);
      (inputPassportElementBankStatement, BankStatement);
      (inputPassportElementDriverLicense, DriverLicense);
      (inputPassportElementEmailAddress, EmailAddress);
      (inputPassportElementIdentityCard, IdentityCard);
      (inputPassportElementInternalPassport, InternalPassport);
      (inputPassportElementPassport, Passport);
      (inputPassportElementPassportRegistration, PassportRegistration);
      (inputPassportElementPersonalDetails, PersonalDetails);
      (inputPassportElementPhoneNumber, PhoneNumber);
      (inputPassportElementRentalAgreement, RentalAgreement);
      (inputPassportElementTemporaryRegistration, TemporaryRegistration);
      (inputPassportElementUtilityBill, UtilityBill);

    )(deserializer)
  }
}

impl RObject for InputPassportElement {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      InputPassportElement::Address(t) => t.td_name(),
      InputPassportElement::BankStatement(t) => t.td_name(),
      InputPassportElement::DriverLicense(t) => t.td_name(),
      InputPassportElement::EmailAddress(t) => t.td_name(),
      InputPassportElement::IdentityCard(t) => t.td_name(),
      InputPassportElement::InternalPassport(t) => t.td_name(),
      InputPassportElement::Passport(t) => t.td_name(),
      InputPassportElement::PassportRegistration(t) => t.td_name(),
      InputPassportElement::PersonalDetails(t) => t.td_name(),
      InputPassportElement::PhoneNumber(t) => t.td_name(),
      InputPassportElement::RentalAgreement(t) => t.td_name(),
      InputPassportElement::TemporaryRegistration(t) => t.td_name(),
      InputPassportElement::UtilityBill(t) => t.td_name(),

      _ => "-1",
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl InputPassportElement {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let InputPassportElement::_Default(_) = self { true } else { false } }

  pub fn is_address(&self) -> bool { if let InputPassportElement::Address(_) = self { true } else { false } }
  pub fn is_bank_statement(&self) -> bool { if let InputPassportElement::BankStatement(_) = self { true } else { false } }
  pub fn is_driver_license(&self) -> bool { if let InputPassportElement::DriverLicense(_) = self { true } else { false } }
  pub fn is_email_address(&self) -> bool { if let InputPassportElement::EmailAddress(_) = self { true } else { false } }
  pub fn is_identity_card(&self) -> bool { if let InputPassportElement::IdentityCard(_) = self { true } else { false } }
  pub fn is_internal_passport(&self) -> bool { if let InputPassportElement::InternalPassport(_) = self { true } else { false } }
  pub fn is_passport(&self) -> bool { if let InputPassportElement::Passport(_) = self { true } else { false } }
  pub fn is_passport_registration(&self) -> bool { if let InputPassportElement::PassportRegistration(_) = self { true } else { false } }
  pub fn is_personal_details(&self) -> bool { if let InputPassportElement::PersonalDetails(_) = self { true } else { false } }
  pub fn is_phone_number(&self) -> bool { if let InputPassportElement::PhoneNumber(_) = self { true } else { false } }
  pub fn is_rental_agreement(&self) -> bool { if let InputPassportElement::RentalAgreement(_) = self { true } else { false } }
  pub fn is_temporary_registration(&self) -> bool { if let InputPassportElement::TemporaryRegistration(_) = self { true } else { false } }
  pub fn is_utility_bill(&self) -> bool { if let InputPassportElement::UtilityBill(_) = self { true } else { false } }

  pub fn on_address<F: FnOnce(&InputPassportElementAddress)>(&self, fnc: F) -> &Self { if let InputPassportElement::Address(t) = self { fnc(t) }; self }
  pub fn on_bank_statement<F: FnOnce(&InputPassportElementBankStatement)>(&self, fnc: F) -> &Self { if let InputPassportElement::BankStatement(t) = self { fnc(t) }; self }
  pub fn on_driver_license<F: FnOnce(&InputPassportElementDriverLicense)>(&self, fnc: F) -> &Self { if let InputPassportElement::DriverLicense(t) = self { fnc(t) }; self }
  pub fn on_email_address<F: FnOnce(&InputPassportElementEmailAddress)>(&self, fnc: F) -> &Self { if let InputPassportElement::EmailAddress(t) = self { fnc(t) }; self }
  pub fn on_identity_card<F: FnOnce(&InputPassportElementIdentityCard)>(&self, fnc: F) -> &Self { if let InputPassportElement::IdentityCard(t) = self { fnc(t) }; self }
  pub fn on_internal_passport<F: FnOnce(&InputPassportElementInternalPassport)>(&self, fnc: F) -> &Self { if let InputPassportElement::InternalPassport(t) = self { fnc(t) }; self }
  pub fn on_passport<F: FnOnce(&InputPassportElementPassport)>(&self, fnc: F) -> &Self { if let InputPassportElement::Passport(t) = self { fnc(t) }; self }
  pub fn on_passport_registration<F: FnOnce(&InputPassportElementPassportRegistration)>(&self, fnc: F) -> &Self { if let InputPassportElement::PassportRegistration(t) = self { fnc(t) }; self }
  pub fn on_personal_details<F: FnOnce(&InputPassportElementPersonalDetails)>(&self, fnc: F) -> &Self { if let InputPassportElement::PersonalDetails(t) = self { fnc(t) }; self }
  pub fn on_phone_number<F: FnOnce(&InputPassportElementPhoneNumber)>(&self, fnc: F) -> &Self { if let InputPassportElement::PhoneNumber(t) = self { fnc(t) }; self }
  pub fn on_rental_agreement<F: FnOnce(&InputPassportElementRentalAgreement)>(&self, fnc: F) -> &Self { if let InputPassportElement::RentalAgreement(t) = self { fnc(t) }; self }
  pub fn on_temporary_registration<F: FnOnce(&InputPassportElementTemporaryRegistration)>(&self, fnc: F) -> &Self { if let InputPassportElement::TemporaryRegistration(t) = self { fnc(t) }; self }
  pub fn on_utility_bill<F: FnOnce(&InputPassportElementUtilityBill)>(&self, fnc: F) -> &Self { if let InputPassportElement::UtilityBill(t) = self { fnc(t) }; self }

  pub fn as_address(&self) -> Option<&InputPassportElementAddress> { if let InputPassportElement::Address(t) = self { return Some(t) } None }
  pub fn as_bank_statement(&self) -> Option<&InputPassportElementBankStatement> { if let InputPassportElement::BankStatement(t) = self { return Some(t) } None }
  pub fn as_driver_license(&self) -> Option<&InputPassportElementDriverLicense> { if let InputPassportElement::DriverLicense(t) = self { return Some(t) } None }
  pub fn as_email_address(&self) -> Option<&InputPassportElementEmailAddress> { if let InputPassportElement::EmailAddress(t) = self { return Some(t) } None }
  pub fn as_identity_card(&self) -> Option<&InputPassportElementIdentityCard> { if let InputPassportElement::IdentityCard(t) = self { return Some(t) } None }
  pub fn as_internal_passport(&self) -> Option<&InputPassportElementInternalPassport> { if let InputPassportElement::InternalPassport(t) = self { return Some(t) } None }
  pub fn as_passport(&self) -> Option<&InputPassportElementPassport> { if let InputPassportElement::Passport(t) = self { return Some(t) } None }
  pub fn as_passport_registration(&self) -> Option<&InputPassportElementPassportRegistration> { if let InputPassportElement::PassportRegistration(t) = self { return Some(t) } None }
  pub fn as_personal_details(&self) -> Option<&InputPassportElementPersonalDetails> { if let InputPassportElement::PersonalDetails(t) = self { return Some(t) } None }
  pub fn as_phone_number(&self) -> Option<&InputPassportElementPhoneNumber> { if let InputPassportElement::PhoneNumber(t) = self { return Some(t) } None }
  pub fn as_rental_agreement(&self) -> Option<&InputPassportElementRentalAgreement> { if let InputPassportElement::RentalAgreement(t) = self { return Some(t) } None }
  pub fn as_temporary_registration(&self) -> Option<&InputPassportElementTemporaryRegistration> { if let InputPassportElement::TemporaryRegistration(t) = self { return Some(t) } None }
  pub fn as_utility_bill(&self) -> Option<&InputPassportElementUtilityBill> { if let InputPassportElement::UtilityBill(t) = self { return Some(t) } None }



  pub fn address<T: AsRef<InputPassportElementAddress>>(t: T) -> Self { InputPassportElement::Address(t.as_ref().clone()) }

  pub fn bank_statement<T: AsRef<InputPassportElementBankStatement>>(t: T) -> Self { InputPassportElement::BankStatement(t.as_ref().clone()) }

  pub fn driver_license<T: AsRef<InputPassportElementDriverLicense>>(t: T) -> Self { InputPassportElement::DriverLicense(t.as_ref().clone()) }

  pub fn email_address<T: AsRef<InputPassportElementEmailAddress>>(t: T) -> Self { InputPassportElement::EmailAddress(t.as_ref().clone()) }

  pub fn identity_card<T: AsRef<InputPassportElementIdentityCard>>(t: T) -> Self { InputPassportElement::IdentityCard(t.as_ref().clone()) }

  pub fn internal_passport<T: AsRef<InputPassportElementInternalPassport>>(t: T) -> Self { InputPassportElement::InternalPassport(t.as_ref().clone()) }

  pub fn passport<T: AsRef<InputPassportElementPassport>>(t: T) -> Self { InputPassportElement::Passport(t.as_ref().clone()) }

  pub fn passport_registration<T: AsRef<InputPassportElementPassportRegistration>>(t: T) -> Self { InputPassportElement::PassportRegistration(t.as_ref().clone()) }

  pub fn personal_details<T: AsRef<InputPassportElementPersonalDetails>>(t: T) -> Self { InputPassportElement::PersonalDetails(t.as_ref().clone()) }

  pub fn phone_number<T: AsRef<InputPassportElementPhoneNumber>>(t: T) -> Self { InputPassportElement::PhoneNumber(t.as_ref().clone()) }

  pub fn rental_agreement<T: AsRef<InputPassportElementRentalAgreement>>(t: T) -> Self { InputPassportElement::RentalAgreement(t.as_ref().clone()) }

  pub fn temporary_registration<T: AsRef<InputPassportElementTemporaryRegistration>>(t: T) -> Self { InputPassportElement::TemporaryRegistration(t.as_ref().clone()) }

  pub fn utility_bill<T: AsRef<InputPassportElementUtilityBill>>(t: T) -> Self { InputPassportElement::UtilityBill(t.as_ref().clone()) }

}

impl AsRef<InputPassportElement> for InputPassportElement {
  fn as_ref(&self) -> &InputPassportElement { self }
}







/// A Telegram Passport element to be saved containing the user's address
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementAddress {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// The address to be saved
  address: Address,
  
}

impl RObject for InputPassportElementAddress {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementAddress" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputPassportElement for InputPassportElementAddress {}



impl InputPassportElementAddress {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputPassportElementAddressBuilder {
    let mut inner = InputPassportElementAddress::default();
    inner.td_name = "inputPassportElementAddress".to_string();
    RTDInputPassportElementAddressBuilder { inner }
  }

  pub fn address(&self) -> &Address { &self.address }

}

#[doc(hidden)]
pub struct RTDInputPassportElementAddressBuilder {
  inner: InputPassportElementAddress
}

impl RTDInputPassportElementAddressBuilder {
  pub fn build(&self) -> InputPassportElementAddress { self.inner.clone() }

   
  pub fn address<T: AsRef<Address>>(&mut self, address: T) -> &mut Self {
    self.inner.address = address.as_ref().clone();
    self
  }

}

impl AsRef<InputPassportElementAddress> for InputPassportElementAddress {
  fn as_ref(&self) -> &InputPassportElementAddress { self }
}

impl AsRef<InputPassportElementAddress> for RTDInputPassportElementAddressBuilder {
  fn as_ref(&self) -> &InputPassportElementAddress { &self.inner }
}







/// A Telegram Passport element to be saved containing the user's bank statement
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementBankStatement {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// The bank statement to be saved
  bank_statement: InputPersonalDocument,
  
}

impl RObject for InputPassportElementBankStatement {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementBankStatement" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputPassportElement for InputPassportElementBankStatement {}



impl InputPassportElementBankStatement {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputPassportElementBankStatementBuilder {
    let mut inner = InputPassportElementBankStatement::default();
    inner.td_name = "inputPassportElementBankStatement".to_string();
    RTDInputPassportElementBankStatementBuilder { inner }
  }

  pub fn bank_statement(&self) -> &InputPersonalDocument { &self.bank_statement }

}

#[doc(hidden)]
pub struct RTDInputPassportElementBankStatementBuilder {
  inner: InputPassportElementBankStatement
}

impl RTDInputPassportElementBankStatementBuilder {
  pub fn build(&self) -> InputPassportElementBankStatement { self.inner.clone() }

   
  pub fn bank_statement<T: AsRef<InputPersonalDocument>>(&mut self, bank_statement: T) -> &mut Self {
    self.inner.bank_statement = bank_statement.as_ref().clone();
    self
  }

}

impl AsRef<InputPassportElementBankStatement> for InputPassportElementBankStatement {
  fn as_ref(&self) -> &InputPassportElementBankStatement { self }
}

impl AsRef<InputPassportElementBankStatement> for RTDInputPassportElementBankStatementBuilder {
  fn as_ref(&self) -> &InputPassportElementBankStatement { &self.inner }
}







/// A Telegram Passport element to be saved containing the user's driver license
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementDriverLicense {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// The driver license to be saved
  driver_license: InputIdentityDocument,
  
}

impl RObject for InputPassportElementDriverLicense {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementDriverLicense" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputPassportElement for InputPassportElementDriverLicense {}



impl InputPassportElementDriverLicense {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputPassportElementDriverLicenseBuilder {
    let mut inner = InputPassportElementDriverLicense::default();
    inner.td_name = "inputPassportElementDriverLicense".to_string();
    RTDInputPassportElementDriverLicenseBuilder { inner }
  }

  pub fn driver_license(&self) -> &InputIdentityDocument { &self.driver_license }

}

#[doc(hidden)]
pub struct RTDInputPassportElementDriverLicenseBuilder {
  inner: InputPassportElementDriverLicense
}

impl RTDInputPassportElementDriverLicenseBuilder {
  pub fn build(&self) -> InputPassportElementDriverLicense { self.inner.clone() }

   
  pub fn driver_license<T: AsRef<InputIdentityDocument>>(&mut self, driver_license: T) -> &mut Self {
    self.inner.driver_license = driver_license.as_ref().clone();
    self
  }

}

impl AsRef<InputPassportElementDriverLicense> for InputPassportElementDriverLicense {
  fn as_ref(&self) -> &InputPassportElementDriverLicense { self }
}

impl AsRef<InputPassportElementDriverLicense> for RTDInputPassportElementDriverLicenseBuilder {
  fn as_ref(&self) -> &InputPassportElementDriverLicense { &self.inner }
}







/// A Telegram Passport element to be saved containing the user's email address
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementEmailAddress {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// The email address to be saved
  email_address: String,
  
}

impl RObject for InputPassportElementEmailAddress {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementEmailAddress" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputPassportElement for InputPassportElementEmailAddress {}



impl InputPassportElementEmailAddress {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputPassportElementEmailAddressBuilder {
    let mut inner = InputPassportElementEmailAddress::default();
    inner.td_name = "inputPassportElementEmailAddress".to_string();
    RTDInputPassportElementEmailAddressBuilder { inner }
  }

  pub fn email_address(&self) -> &String { &self.email_address }

}

#[doc(hidden)]
pub struct RTDInputPassportElementEmailAddressBuilder {
  inner: InputPassportElementEmailAddress
}

impl RTDInputPassportElementEmailAddressBuilder {
  pub fn build(&self) -> InputPassportElementEmailAddress { self.inner.clone() }

   
  pub fn email_address<T: AsRef<str>>(&mut self, email_address: T) -> &mut Self {
    self.inner.email_address = email_address.as_ref().to_string();
    self
  }

}

impl AsRef<InputPassportElementEmailAddress> for InputPassportElementEmailAddress {
  fn as_ref(&self) -> &InputPassportElementEmailAddress { self }
}

impl AsRef<InputPassportElementEmailAddress> for RTDInputPassportElementEmailAddressBuilder {
  fn as_ref(&self) -> &InputPassportElementEmailAddress { &self.inner }
}







/// A Telegram Passport element to be saved containing the user's identity card
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementIdentityCard {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// The identity card to be saved
  identity_card: InputIdentityDocument,
  
}

impl RObject for InputPassportElementIdentityCard {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementIdentityCard" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputPassportElement for InputPassportElementIdentityCard {}



impl InputPassportElementIdentityCard {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputPassportElementIdentityCardBuilder {
    let mut inner = InputPassportElementIdentityCard::default();
    inner.td_name = "inputPassportElementIdentityCard".to_string();
    RTDInputPassportElementIdentityCardBuilder { inner }
  }

  pub fn identity_card(&self) -> &InputIdentityDocument { &self.identity_card }

}

#[doc(hidden)]
pub struct RTDInputPassportElementIdentityCardBuilder {
  inner: InputPassportElementIdentityCard
}

impl RTDInputPassportElementIdentityCardBuilder {
  pub fn build(&self) -> InputPassportElementIdentityCard { self.inner.clone() }

   
  pub fn identity_card<T: AsRef<InputIdentityDocument>>(&mut self, identity_card: T) -> &mut Self {
    self.inner.identity_card = identity_card.as_ref().clone();
    self
  }

}

impl AsRef<InputPassportElementIdentityCard> for InputPassportElementIdentityCard {
  fn as_ref(&self) -> &InputPassportElementIdentityCard { self }
}

impl AsRef<InputPassportElementIdentityCard> for RTDInputPassportElementIdentityCardBuilder {
  fn as_ref(&self) -> &InputPassportElementIdentityCard { &self.inner }
}







/// A Telegram Passport element to be saved containing the user's internal passport
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementInternalPassport {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// The internal passport to be saved
  internal_passport: InputIdentityDocument,
  
}

impl RObject for InputPassportElementInternalPassport {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementInternalPassport" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputPassportElement for InputPassportElementInternalPassport {}



impl InputPassportElementInternalPassport {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputPassportElementInternalPassportBuilder {
    let mut inner = InputPassportElementInternalPassport::default();
    inner.td_name = "inputPassportElementInternalPassport".to_string();
    RTDInputPassportElementInternalPassportBuilder { inner }
  }

  pub fn internal_passport(&self) -> &InputIdentityDocument { &self.internal_passport }

}

#[doc(hidden)]
pub struct RTDInputPassportElementInternalPassportBuilder {
  inner: InputPassportElementInternalPassport
}

impl RTDInputPassportElementInternalPassportBuilder {
  pub fn build(&self) -> InputPassportElementInternalPassport { self.inner.clone() }

   
  pub fn internal_passport<T: AsRef<InputIdentityDocument>>(&mut self, internal_passport: T) -> &mut Self {
    self.inner.internal_passport = internal_passport.as_ref().clone();
    self
  }

}

impl AsRef<InputPassportElementInternalPassport> for InputPassportElementInternalPassport {
  fn as_ref(&self) -> &InputPassportElementInternalPassport { self }
}

impl AsRef<InputPassportElementInternalPassport> for RTDInputPassportElementInternalPassportBuilder {
  fn as_ref(&self) -> &InputPassportElementInternalPassport { &self.inner }
}







/// A Telegram Passport element to be saved containing the user's passport
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementPassport {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// The passport to be saved
  passport: InputIdentityDocument,
  
}

impl RObject for InputPassportElementPassport {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementPassport" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputPassportElement for InputPassportElementPassport {}



impl InputPassportElementPassport {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputPassportElementPassportBuilder {
    let mut inner = InputPassportElementPassport::default();
    inner.td_name = "inputPassportElementPassport".to_string();
    RTDInputPassportElementPassportBuilder { inner }
  }

  pub fn passport(&self) -> &InputIdentityDocument { &self.passport }

}

#[doc(hidden)]
pub struct RTDInputPassportElementPassportBuilder {
  inner: InputPassportElementPassport
}

impl RTDInputPassportElementPassportBuilder {
  pub fn build(&self) -> InputPassportElementPassport { self.inner.clone() }

   
  pub fn passport<T: AsRef<InputIdentityDocument>>(&mut self, passport: T) -> &mut Self {
    self.inner.passport = passport.as_ref().clone();
    self
  }

}

impl AsRef<InputPassportElementPassport> for InputPassportElementPassport {
  fn as_ref(&self) -> &InputPassportElementPassport { self }
}

impl AsRef<InputPassportElementPassport> for RTDInputPassportElementPassportBuilder {
  fn as_ref(&self) -> &InputPassportElementPassport { &self.inner }
}







/// A Telegram Passport element to be saved containing the user's passport registration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementPassportRegistration {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// The passport registration page to be saved
  passport_registration: InputPersonalDocument,
  
}

impl RObject for InputPassportElementPassportRegistration {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementPassportRegistration" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputPassportElement for InputPassportElementPassportRegistration {}



impl InputPassportElementPassportRegistration {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputPassportElementPassportRegistrationBuilder {
    let mut inner = InputPassportElementPassportRegistration::default();
    inner.td_name = "inputPassportElementPassportRegistration".to_string();
    RTDInputPassportElementPassportRegistrationBuilder { inner }
  }

  pub fn passport_registration(&self) -> &InputPersonalDocument { &self.passport_registration }

}

#[doc(hidden)]
pub struct RTDInputPassportElementPassportRegistrationBuilder {
  inner: InputPassportElementPassportRegistration
}

impl RTDInputPassportElementPassportRegistrationBuilder {
  pub fn build(&self) -> InputPassportElementPassportRegistration { self.inner.clone() }

   
  pub fn passport_registration<T: AsRef<InputPersonalDocument>>(&mut self, passport_registration: T) -> &mut Self {
    self.inner.passport_registration = passport_registration.as_ref().clone();
    self
  }

}

impl AsRef<InputPassportElementPassportRegistration> for InputPassportElementPassportRegistration {
  fn as_ref(&self) -> &InputPassportElementPassportRegistration { self }
}

impl AsRef<InputPassportElementPassportRegistration> for RTDInputPassportElementPassportRegistrationBuilder {
  fn as_ref(&self) -> &InputPassportElementPassportRegistration { &self.inner }
}







/// A Telegram Passport element to be saved containing the user's personal details
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementPersonalDetails {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Personal details of the user
  personal_details: PersonalDetails,
  
}

impl RObject for InputPassportElementPersonalDetails {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementPersonalDetails" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputPassportElement for InputPassportElementPersonalDetails {}



impl InputPassportElementPersonalDetails {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputPassportElementPersonalDetailsBuilder {
    let mut inner = InputPassportElementPersonalDetails::default();
    inner.td_name = "inputPassportElementPersonalDetails".to_string();
    RTDInputPassportElementPersonalDetailsBuilder { inner }
  }

  pub fn personal_details(&self) -> &PersonalDetails { &self.personal_details }

}

#[doc(hidden)]
pub struct RTDInputPassportElementPersonalDetailsBuilder {
  inner: InputPassportElementPersonalDetails
}

impl RTDInputPassportElementPersonalDetailsBuilder {
  pub fn build(&self) -> InputPassportElementPersonalDetails { self.inner.clone() }

   
  pub fn personal_details<T: AsRef<PersonalDetails>>(&mut self, personal_details: T) -> &mut Self {
    self.inner.personal_details = personal_details.as_ref().clone();
    self
  }

}

impl AsRef<InputPassportElementPersonalDetails> for InputPassportElementPersonalDetails {
  fn as_ref(&self) -> &InputPassportElementPersonalDetails { self }
}

impl AsRef<InputPassportElementPersonalDetails> for RTDInputPassportElementPersonalDetailsBuilder {
  fn as_ref(&self) -> &InputPassportElementPersonalDetails { &self.inner }
}







/// A Telegram Passport element to be saved containing the user's phone number
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementPhoneNumber {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// The phone number to be saved
  phone_number: String,
  
}

impl RObject for InputPassportElementPhoneNumber {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementPhoneNumber" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputPassportElement for InputPassportElementPhoneNumber {}



impl InputPassportElementPhoneNumber {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputPassportElementPhoneNumberBuilder {
    let mut inner = InputPassportElementPhoneNumber::default();
    inner.td_name = "inputPassportElementPhoneNumber".to_string();
    RTDInputPassportElementPhoneNumberBuilder { inner }
  }

  pub fn phone_number(&self) -> &String { &self.phone_number }

}

#[doc(hidden)]
pub struct RTDInputPassportElementPhoneNumberBuilder {
  inner: InputPassportElementPhoneNumber
}

impl RTDInputPassportElementPhoneNumberBuilder {
  pub fn build(&self) -> InputPassportElementPhoneNumber { self.inner.clone() }

   
  pub fn phone_number<T: AsRef<str>>(&mut self, phone_number: T) -> &mut Self {
    self.inner.phone_number = phone_number.as_ref().to_string();
    self
  }

}

impl AsRef<InputPassportElementPhoneNumber> for InputPassportElementPhoneNumber {
  fn as_ref(&self) -> &InputPassportElementPhoneNumber { self }
}

impl AsRef<InputPassportElementPhoneNumber> for RTDInputPassportElementPhoneNumberBuilder {
  fn as_ref(&self) -> &InputPassportElementPhoneNumber { &self.inner }
}







/// A Telegram Passport element to be saved containing the user's rental agreement
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementRentalAgreement {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// The rental agreement to be saved
  rental_agreement: InputPersonalDocument,
  
}

impl RObject for InputPassportElementRentalAgreement {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementRentalAgreement" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputPassportElement for InputPassportElementRentalAgreement {}



impl InputPassportElementRentalAgreement {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputPassportElementRentalAgreementBuilder {
    let mut inner = InputPassportElementRentalAgreement::default();
    inner.td_name = "inputPassportElementRentalAgreement".to_string();
    RTDInputPassportElementRentalAgreementBuilder { inner }
  }

  pub fn rental_agreement(&self) -> &InputPersonalDocument { &self.rental_agreement }

}

#[doc(hidden)]
pub struct RTDInputPassportElementRentalAgreementBuilder {
  inner: InputPassportElementRentalAgreement
}

impl RTDInputPassportElementRentalAgreementBuilder {
  pub fn build(&self) -> InputPassportElementRentalAgreement { self.inner.clone() }

   
  pub fn rental_agreement<T: AsRef<InputPersonalDocument>>(&mut self, rental_agreement: T) -> &mut Self {
    self.inner.rental_agreement = rental_agreement.as_ref().clone();
    self
  }

}

impl AsRef<InputPassportElementRentalAgreement> for InputPassportElementRentalAgreement {
  fn as_ref(&self) -> &InputPassportElementRentalAgreement { self }
}

impl AsRef<InputPassportElementRentalAgreement> for RTDInputPassportElementRentalAgreementBuilder {
  fn as_ref(&self) -> &InputPassportElementRentalAgreement { &self.inner }
}







/// A Telegram Passport element to be saved containing the user's temporary registration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementTemporaryRegistration {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// The temporary registration document to be saved
  temporary_registration: InputPersonalDocument,
  
}

impl RObject for InputPassportElementTemporaryRegistration {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementTemporaryRegistration" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputPassportElement for InputPassportElementTemporaryRegistration {}



impl InputPassportElementTemporaryRegistration {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputPassportElementTemporaryRegistrationBuilder {
    let mut inner = InputPassportElementTemporaryRegistration::default();
    inner.td_name = "inputPassportElementTemporaryRegistration".to_string();
    RTDInputPassportElementTemporaryRegistrationBuilder { inner }
  }

  pub fn temporary_registration(&self) -> &InputPersonalDocument { &self.temporary_registration }

}

#[doc(hidden)]
pub struct RTDInputPassportElementTemporaryRegistrationBuilder {
  inner: InputPassportElementTemporaryRegistration
}

impl RTDInputPassportElementTemporaryRegistrationBuilder {
  pub fn build(&self) -> InputPassportElementTemporaryRegistration { self.inner.clone() }

   
  pub fn temporary_registration<T: AsRef<InputPersonalDocument>>(&mut self, temporary_registration: T) -> &mut Self {
    self.inner.temporary_registration = temporary_registration.as_ref().clone();
    self
  }

}

impl AsRef<InputPassportElementTemporaryRegistration> for InputPassportElementTemporaryRegistration {
  fn as_ref(&self) -> &InputPassportElementTemporaryRegistration { self }
}

impl AsRef<InputPassportElementTemporaryRegistration> for RTDInputPassportElementTemporaryRegistrationBuilder {
  fn as_ref(&self) -> &InputPassportElementTemporaryRegistration { &self.inner }
}







/// A Telegram Passport element to be saved containing the user's utility bill
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementUtilityBill {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// The utility bill to be saved
  utility_bill: InputPersonalDocument,
  
}

impl RObject for InputPassportElementUtilityBill {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementUtilityBill" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputPassportElement for InputPassportElementUtilityBill {}



impl InputPassportElementUtilityBill {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputPassportElementUtilityBillBuilder {
    let mut inner = InputPassportElementUtilityBill::default();
    inner.td_name = "inputPassportElementUtilityBill".to_string();
    RTDInputPassportElementUtilityBillBuilder { inner }
  }

  pub fn utility_bill(&self) -> &InputPersonalDocument { &self.utility_bill }

}

#[doc(hidden)]
pub struct RTDInputPassportElementUtilityBillBuilder {
  inner: InputPassportElementUtilityBill
}

impl RTDInputPassportElementUtilityBillBuilder {
  pub fn build(&self) -> InputPassportElementUtilityBill { self.inner.clone() }

   
  pub fn utility_bill<T: AsRef<InputPersonalDocument>>(&mut self, utility_bill: T) -> &mut Self {
    self.inner.utility_bill = utility_bill.as_ref().clone();
    self
  }

}

impl AsRef<InputPassportElementUtilityBill> for InputPassportElementUtilityBill {
  fn as_ref(&self) -> &InputPassportElementUtilityBill { self }
}

impl AsRef<InputPassportElementUtilityBill> for RTDInputPassportElementUtilityBillBuilder {
  fn as_ref(&self) -> &InputPassportElementUtilityBill { &self.inner }
}



