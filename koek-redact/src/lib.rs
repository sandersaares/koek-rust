// Redaction library with built-in redaction operations and flexibility to add more.
//
// By default, all types that implement Display are redactable. Redacted values are typically obscured
// entirely, although some types may be partially obscured where limited redaction is feasible (e.g. IP addresses).

#![feature(min_specialization)]

use std::{
    fmt::Display,
    net::{IpAddr, Ipv4Addr},
};

pub trait Redactable {
    fn redacted(&self) -> String;
}

// Anything that implements Display is extended to be redactable.
impl<T: Display> Redactable for T {
    default fn redacted(&self) -> String {
        String::from(DEFAULT_REDACTED_VALUE)
    }
}

// IP addresses have their own special default rules.
impl Redactable for IpAddr {
    fn redacted(&self) -> String {
        match self {
            IpAddr::V4(x) => x.redacted(),
            IpAddr::V6(x) => x.redacted(),
        }
    }
}

impl Redactable for Ipv4Addr {
    fn redacted(&self) -> String {
        // IPv4 addresses are redacted by removing the last octet.
        // 1.2.3.4 -> 1.2.3.xxx
        let octets = self.octets();
        format!("{}.{}.{}.xxx", octets[0], octets[1], octets[2])
    }
}

const DEFAULT_REDACTED_VALUE: &str = "***";

#[cfg(test)]
mod tests {
    use std::net::Ipv6Addr;

    use super::*;

    #[test]
    fn can_redact_string() {
        let value = String::from("qax qex qqx");
        let redacted = value.redacted();

        assert_ne!(value, redacted);
        assert_eq!(DEFAULT_REDACTED_VALUE, redacted);
    }

    #[test]
    fn can_redact_integer() {
        let value = 1234;
        let redacted = value.redacted();

        assert_ne!(value.to_string(), redacted);
        assert_eq!(DEFAULT_REDACTED_VALUE, redacted);
    }

    #[test]
    fn can_redact_float() {
        let value = 12.34;
        let redacted = value.redacted();

        assert_ne!(value.to_string(), redacted);
        assert_eq!(DEFAULT_REDACTED_VALUE, redacted);
    }

    #[test]
    fn can_redact_ipv4() {
        let value = Ipv4Addr::new(1, 2, 3, 4);
        let redacted = value.redacted();

        assert_ne!(value.to_string(), redacted);
        assert_eq!("1.2.3.xxx", redacted);
    }

    #[test]
    fn can_redact_ipv6() {
        let value = Ipv6Addr::new(1, 2, 3, 4, 5, 6, 7, 8);
        let redacted = value.redacted();

        assert_ne!(value.to_string(), redacted);
        assert_eq!(DEFAULT_REDACTED_VALUE, redacted);
    }

    #[test]
    fn can_redact_ip() {
        let value = IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4));
        let redacted = value.redacted();

        assert_ne!(value.to_string(), redacted);
        assert_eq!("1.2.3.xxx", redacted);
    }

    struct CustomSecretStructViaDisplay {
        tell_noone: String,
    }

    impl Display for CustomSecretStructViaDisplay {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(&self.tell_noone)
        }
    }

    #[test]
    fn can_redact_custom_struct_via_display() {
        let value = CustomSecretStructViaDisplay {
            tell_noone: String::from("the secret value"),
        };
        let redacted = value.redacted();

        assert_ne!(value.to_string(), redacted);
        assert_eq!(DEFAULT_REDACTED_VALUE, redacted);
    }

    struct CustomSecretStructViaCustomLogic {
        first_name: String,
        last_name: String,
    }

    impl Display for CustomSecretStructViaCustomLogic {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{} {}", &self.first_name, &self.last_name))
        }
    }

    impl Redactable for CustomSecretStructViaCustomLogic {
        fn redacted(&self) -> String {
            format!("{}. {}.", &self.first_name[0..1], &self.last_name[0..1])
        }
    }

    #[test]
    fn can_redact_custom_struct_via_custom_logic() {
        let value = CustomSecretStructViaCustomLogic {
            first_name: String::from("Firstname"),
            last_name: String::from("Lastname"),
        };
        let redacted = value.redacted();

        assert_ne!(value.to_string(), redacted);
        assert_eq!("F. L.", redacted);
    }
}
