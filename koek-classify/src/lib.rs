// Design principles:
// classified data gains a separate type that wraps unclassified data
// the data class is a field on the wrapper, pointing to a constant
// we do not define any data classes in this module, only the wiring for them
// we process unclassified data by unwrapping it directly
// we process classified data by using it via one of the data class traits

// What would a "data classes package" provide for us?
// list of classes
// redaction configuration for each class

use std::fmt::{self, Display, Formatter};

use koek_redact::Redact;

/// A data class has a name that acts as a key to allow it to be distinguished from other data classes.
pub struct DataClass {
    /// A key that allows data of different classes to be easily distinguished.
    pub name: &'static str,

    /// What to do when displaying a classified value
    pub display_behavior: DisplayBehavior,
}

impl DataClass {
    pub fn classify<TValue>(&'static self, value: TValue) -> Classified<TValue> {
        Classified {
            class: self,
            value
        }
    }
}

pub enum DisplayBehavior {
    /// Values of this data class are not redacted and will be displayed in the clear.
    Clear,
    /// Values of this data class use default redaction behavior from koek-redact.
    DefaultRedact,
}

/// Data that has been classified.
pub struct Classified<TValue> {
    pub class: &'static DataClass,
    pub value: TValue,
}

impl<TValue: Display> Display for Classified<TValue> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.class.display_behavior {
            DisplayBehavior::DefaultRedact => f.write_str(self.value.redacted().as_str()),
            DisplayBehavior::Clear => self.value.fmt(f),
        }
    }
}
