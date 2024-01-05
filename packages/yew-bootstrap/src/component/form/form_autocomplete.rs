use yew::prelude::*;

/// HTML attribute: autocomplete
/// https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/autocomplete
/// 
#[derive(Clone, PartialEq)]
pub enum AutocompleteNameType {
    /// The field expects the value to be a person's full name.
    FullName,
    /// The prefix or title, such as "Mrs.", "Mr.", "Miss", "Ms.", "Dr.", or "Mlle.".
    HonorificPrefix,
    /// The given (or "first") name.
    GivenName,
    /// The middle name.
    AdditionalName,
    /// The family (or "last") name.
    FamilyName,
    /// The suffix, such as "Jr.", "B.Sc.", "PhD.", "MBASW", or "IV".
    HonorificSuffix,
    /// A nickname or handle.
    Nickname,
}

/// The street address to send the product. 
/// This can be combined with other tokens, 
/// such as "shipping street-address" and "shipping address-level2" 
/// or "billing street-address" and "billing address-level2".
#[derive(Clone, PartialEq)]
pub enum AutocompleteAddressType {
    StreetAddress,
    AddressLevel2,
}


#[derive(Clone, PartialEq)]
pub enum AutocompletePasswordType {
    /// A new password. When creating a new account or changing passwords, 
    /// this should be used for an "Enter your new password" or "Confirm new password" field, 
    /// as opposed to a general "Enter your current password" field that might be present.
    NewPassword,
    /// The user's current password.
    CurrentPassword,
}

#[derive(Clone, PartialEq)]
pub enum AutocompleteTelType {
    /// A full telephone number, including the country code.
    Tel,
    /// The country code, such as "1" for the United States, Canada, and 
    /// other areas in North America and parts of the Caribbean.
    TelCountryCode,
    /// The entire phone number without the country code component, 
    /// including a country-internal prefix. For the phone number "1-855-555-6502", 
    /// this field's value would be "855-555-6502".
    TelNational,
    /// The area code, with any country-internal prefix applied if appropriate.
    TelAreaCode,
    /// The phone number without the country or area code. This can be split further into two parts, 
    /// for phone numbers which have an exchange number and then a number within the exchange. 
    /// For the phone number "555-6502", use "tel-local-prefix" for "555" and "tel-local-suffix" for "6502".
    TelLocal,
    TelLocalPrefix,
    TelLocalSuffix,
    /// A telephone extension code within the phone number, 
    /// such as a room or suite number in a hotel or an office extension in a company.
    TelExtension,
}

/// # Type of form autocomplete, to be used with [crate::component::form::FormControl].
#[derive(Clone, PartialEq)]

pub enum FormAutocompleteType {
    /// The browser is not permitted to automatically enter or select a value for this field. 
    /// It is possible that the document or application provides its own autocomplete feature, 
    /// or that security concerns require that the field's value not be automatically entered.
    Off,
    /// The browser is allowed to automatically complete the input. 
    /// No guidance is provided as to the type of data expected in the field, so the browser may use its own judgement.
    On,

    Name { value: AutocompleteNameType },
    /// An email address.
    Email,
    /// A username or account name.
    Username,

    Password { value: AutocompletePasswordType },
    ///A one-time password (OTP) for verifying user identity that is used as an additional factor in a sign-in flow. 
    /// Most commonly this is a code received via some out-of-channel mechanism, such as SMS, email, or authenticator application.
    OTP,
    /// A job title, or the title a person has within an organization, such as 
    /// "Senior Technical Writer", "President", or "Assistant Troop Leader".
    OrganizationTitle,
    /// A company or organization name, such as "Acme Widget Company" or "Girl Scouts of America".
    Organization,
    /// A street address. This can be multiple lines of text, and should fully identify the location of the address 
    /// within its second administrative level (typically a city or town), but should not include the city name, 
    /// ZIP or postal code, or country name.
    StreetAddress,

    ShippingAddress { value: AutocompleteAddressType },
    BillingAddress { value: AutocompleteAddressType },
    /// Each individual line of the street address. These should only be present if the "street-address" is not present.
    AddressLine1,
    AddressLine2,
    AddressLine3,
    /// The finest-grained administrative level, in addresses which have four levels.
    AddressLevel4,
    /// The third administrative level, in addresses with at least three administrative levels.
    AddressLevel3,
    /// The second administrative level, in addresses with at least two of them. In countries with two administrative levels, 
    /// this would typically be the city, town, village, or other locality in which the address is located.
    AddressLevel2,
    /// The first administrative level in the address. This is typically the province in which the address is located. 
    /// In the United States, this would be the state. In Switzerland, the canton. In the United Kingdom, the post town.
    AddressLevel1,
    /// A country or territory code.
    Country,
    /// A country or territory name.
    CountryName,
    /// A postal code (in the United States, this is the ZIP code).
    PostalCode,
    /// The full name as printed on or associated with a payment instrument such as a credit card. 
    /// Using a full name field is preferred, typically, over breaking the name into pieces.
    CcName,
    /// A given (first) name as given on a payment instrument like a credit card.
    CcGivenName,
    /// A middle name as given on a payment instrument or credit card.
    CcAdditionalName,
    /// A family name, as given on a credit card.
    CcFamilyName,
    /// A credit card number or other number identifying a payment method, such as an account number.
    CcNumber,
    /// A payment method expiration date, typically in the form "MM/YY" or "MM/YYYY".
    CcExp,
    /// The month in which the payment method expires.
    CcExpMonth,
    /// The year in which the payment method expires.
    CcExpYear,
    /// The security code for the payment instrument; on credit cards, 
    /// this is the 3-digit verification number on the back of the card.
    CcCsc,
    /// The type of payment instrument (such as "Visa" or "Master Card").
    CcType,
    /// The currency in which the transaction is to take place.
    TransactionCurrency,
    /// The amount, given in the currency specified by "transaction-currency", of the transaction, for a payment form.
    TransactionAmount,
    /// A preferred language, given as a valid BCP 47 language tag
    Language,
    /// A birth date, as a full date.
    Bday,
    /// The day of the month of a birth date.
    BdayDay,
    /// The month of the year of a birth date.
    BdayMonth,
    /// The year of a birth date.
    BdayYear,
    /// A gender identity (such as "Female", "Fa'afafine", "Hijra", "Male", "Nonbinary"), as freeform text without newlines.
    Sex,
    Tel { value: AutocompleteTelType },
    /// A URL for an instant messaging protocol endpoint, such as "xmpp:username@example.net".
    Impp,
    /// A URL, such as a home page or company website address as appropriate given the context of the other fields in the form.
    Url,
    /// The URL of an image representing the person, company, or contact information given in the other fields in the form.
    Photo,
    /// Passkeys generated by the Web Authentication API, as requested by a conditional navigator.credentials.get() call (i.e., one that includes mediation: 'conditional'). 
    Webauthn,
}

impl FormAutocompleteType {
    /// Convert enum to HTML type string
    pub fn to_str(&self) -> AttrValue {
        let value = match &self {
            Self::Off => "off",
            Self::On => "on",
            Self::Name { value } => {
                match value {
                    AutocompleteNameType::FullName => "name",
                    AutocompleteNameType::HonorificPrefix => "honorific-prefix",
                    AutocompleteNameType::GivenName => "given-name",
                    AutocompleteNameType::AdditionalName => "additional-name",
                    AutocompleteNameType::FamilyName => "family-name",
                    AutocompleteNameType::HonorificSuffix => "honorific-suffix",
                    AutocompleteNameType::Nickname => "nickname"
                }
            }
            Self::Email => "email",
            Self::Username => "username",
            Self::Password { value } => {
                match value {
                    AutocompletePasswordType::NewPassword => "new-password",
                    AutocompletePasswordType::CurrentPassword => "current-password"
                }
            },
            Self::OTP => "one-time-code",
            Self::OrganizationTitle => "organization-title",
            Self::Organization => "organization",

            Self::StreetAddress => "street-address",
            Self::ShippingAddress { value } => {
                match  value {
                    AutocompleteAddressType::StreetAddress => "shipping street-address",
                    AutocompleteAddressType::AddressLevel2 => "shipping address-level2"
                }
            }
            Self::BillingAddress { value } => {
                match  value {
                    AutocompleteAddressType::StreetAddress => "billing street-address",
                    AutocompleteAddressType::AddressLevel2 => "billing address-level2"
                }
            }
            Self::AddressLine1 => "address-line1",
            Self::AddressLine2 => "address-line2",
            Self::AddressLine3 => "address-line3",
            Self::AddressLevel4 => "address-level4",
            Self::AddressLevel3 => "address-level3",
            Self::AddressLevel2 => "address-level2",
            Self::AddressLevel1 => "address-level1",
            Self::Country => "country",
            Self::CountryName => "country-name",
            Self::PostalCode => "postal-code",
            Self::CcName => "cc-name",
            Self::CcGivenName => "cc-given-name",
            Self::CcAdditionalName => "cc-additional-name",
            Self::CcFamilyName => "cc-family-name",
            Self::CcNumber => "cc-number",
            Self::CcExp => "cc-exp",
            Self::CcExpMonth => "cc-exp-month",
            Self::CcExpYear => "cc-exp-year",
            Self::CcCsc => "cc-csc",
            Self::CcType => "cc-type",
            Self::TransactionCurrency => "transaction-currency",
            Self::TransactionAmount => "transaction-amount",
            Self::Language => "language",
            Self::Bday => "bday",
            Self::BdayDay => "bday-day",
            Self::BdayMonth => "bday-month",
            Self::BdayYear => "bday-year",
            Self::Sex => "sex",
            Self::Tel { value } => {
                match value {
                    AutocompleteTelType::Tel => "tel",
                    AutocompleteTelType::TelCountryCode => "tel-country-code",
                    AutocompleteTelType::TelNational => "tel-national",
                    AutocompleteTelType::TelAreaCode => "tel-area-code",
                    AutocompleteTelType::TelLocal => "tel-local",
                    AutocompleteTelType::TelLocalPrefix => "tel-local-prefix",
                    AutocompleteTelType::TelLocalSuffix => "tel-local-suffix",
                    AutocompleteTelType::TelExtension => "tel-extension",
                }
            }
            Self::Impp => "impp",
            Self::Url => "url",
            Self::Photo => "photo",
            Self::Webauthn => "webauthn",

        };

        AttrValue::from(value)
    }
}

