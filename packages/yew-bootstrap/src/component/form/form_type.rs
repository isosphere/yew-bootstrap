use yew::prelude::*;


/// # Type of form control, to be used with [crate::component::form::FormControl].
#[derive(Clone, PartialEq)]

pub enum FormControlType {
    /// Simple text control
    Text,
    /// Textarea with optional rows and columns
    TextArea { cols: Option<u32>, rows: Option<u32> },
    /// Email with optional pattern (regexp, for example ".+@example\.com")
    Email { pattern: Option<AttrValue> },
    /// Password whose value is obscured
    Password,
    /// Url with optional pattern (regexp, for example "https://.+")
    Url { pattern: Option<AttrValue>},
    /// Number with optional min and max
    Number { min: Option<u32>, max: Option<u32> },
    /// Range selection from min to max, with optional step
    Range { min: i32, max: i32, step: Option<u32> },

    /// Select to select one of more options. It typically contains [crate::component::form::SelectOption]
    /// or [crate::component::form::SelectOptgroup] children
    Select,
    /// Checkbox
    Checkbox,
    /// Radio button
    Radio,

    /// Date picker, format YYYY-mm-dd
    Date,
    /// Date picker with optional min and max boundaries (Format "YYYY-mm-dd")
    DateMinMax { min: Option<AttrValue>, max: Option<AttrValue> },
    /// Date and time (Local), format YYYY-mm-ddTHH:MM
    Datetime,
    /// Date and time (Local) with min and max boundaries (Format "YYYY-mm-ddTHH:MM")
    DatetimeMinMax { min: Option<AttrValue>, max: Option<AttrValue> },
    /// Time, format HH:MM
    Time,
    /// Time with min and max values (format "HH:MM")
    TimeMinMax { min: Option<AttrValue>, max: Option<AttrValue> },

    /// Color picker
    Color,
    /// File input. Accept is a vector of formats, like "image/png", ".docx"
    File { accept: Vec<AttrValue> },
    /// Hidden input, used to transmit data
    Hidden,

    /// Submit button
    Submit,
}

impl FormControlType {
    /// Convert enum to HTML type string
    pub fn to_str(&self) -> AttrValue {
        let value = match &self {
            Self::Checkbox => "checkbox",
            Self::Color => "color",
            Self::Date | Self::DateMinMax { .. } => "date",
            Self::Datetime | Self::DatetimeMinMax { .. } => "datetime-local",
            Self::Email { .. } => "email",
            Self::File { .. } => "file",
            Self::Hidden => "hidden",
            Self::Number { .. } => "number",
            Self::Password { .. } => "password",
            Self::Radio => "radio",
            Self::Range { .. } => "range",
            Self::Select => "select",
            Self::Submit => "submit",
            Self::Text => "text",
            Self::TextArea { .. } => "",
            Self::Time | Self::TimeMinMax { .. } => "time",
            Self::Url { .. } => "url",
        };

        AttrValue::from(value)
    }
}
