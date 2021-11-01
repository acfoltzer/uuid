// Copyright 2013-2014 The Rust Project Developers.
// Copyright 2018 The Uuid Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! [`Uuid`] parsing constructs and utilities.
//!
//! [`Uuid`]: ../struct.Uuid.html

use crate::{
    error::*,
    std::{convert::TryFrom, str},
    Uuid,
};

#[path = "../shared/parser.rs"]
mod imp;

impl str::FromStr for Uuid {
    type Err = Error;

    fn from_str(uuid_str: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(uuid_str)
    }
}

impl TryFrom<&'_ str> for Uuid {
    type Error = Error;

    fn try_from(uuid_str: &'_ str) -> Result<Self, Self::Error> {
        Uuid::parse_str(uuid_str)
    }
}

impl Uuid {
    /// Parses a `Uuid` from a string of hexadecimal digits with optional
    /// hyphens.
    ///
    /// Any of the formats generated by this module (simple, hyphenated, urn)
    /// are supported by this parsing function.
    ///
    /// # Examples
    ///
    /// Parse a hyphenated UUID:
    ///
    /// ```
    /// # use uuid::{Uuid, Version, Variant};
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let uuid = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000")?;
    ///
    /// assert_eq!(Some(Version::Random), uuid.get_version());
    /// assert_eq!(Variant::RFC4122, uuid.get_variant());
    /// # Ok(())
    /// # }
    /// ```
    pub fn parse_str(input: &str) -> Result<Uuid, Error> {
        Ok(Uuid::from_bytes(imp::parse_str(input)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{fmt, std::string::ToString, tests::new};

    #[test]
    fn test_parse_uuid_v4_valid() {
        let from_hyphenated =
            Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8").unwrap();
        let from_simple =
            Uuid::parse_str("67e5504410b1426f9247bb680e5fe0c8").unwrap();
        let from_urn =
            Uuid::parse_str("urn:uuid:67e55044-10b1-426f-9247-bb680e5fe0c8")
                .unwrap();
        let from_guid =
            Uuid::parse_str("{67e55044-10b1-426f-9247-bb680e5fe0c8}").unwrap();

        assert_eq!(from_hyphenated, from_simple);
        assert_eq!(from_hyphenated, from_urn);
        assert_eq!(from_hyphenated, from_guid);

        assert!(Uuid::parse_str("00000000000000000000000000000000").is_ok());
        assert!(Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8").is_ok());
        assert!(Uuid::parse_str("F9168C5E-CEB2-4faa-B6BF-329BF39FA1E4").is_ok());
        assert!(Uuid::parse_str("67e5504410b1426f9247bb680e5fe0c8").is_ok());
        assert!(Uuid::parse_str("01020304-1112-2122-3132-414243444546").is_ok());
        assert!(Uuid::parse_str(
            "urn:uuid:67e55044-10b1-426f-9247-bb680e5fe0c8"
        )
        .is_ok());
        assert!(
            Uuid::parse_str("{6d93bade-bd9f-4e13-8914-9474e1e3567b}").is_ok()
        );

        // Nil
        let nil = Uuid::nil();
        assert_eq!(
            Uuid::parse_str("00000000000000000000000000000000").unwrap(),
            nil
        );
        assert_eq!(
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap(),
            nil
        );
    }

    #[test]
    fn test_parse_uuid_v4_invalid() {
        const EXPECTED_UUID_LENGTHS: ExpectedLength = ExpectedLength::Any(&[
            fmt::Hyphenated::LENGTH,
            fmt::Simple::LENGTH,
        ]);

        const EXPECTED_GROUP_COUNTS: ExpectedLength =
            ExpectedLength::Any(&[1, 5]);

        const EXPECTED_CHARS: &'static str = "0123456789abcdefABCDEF-";

        // Invalid
        assert_eq!(
            Uuid::parse_str(""),
            Err(Error(ErrorKind::InvalidLength {
                expected: EXPECTED_UUID_LENGTHS,
                found: 0,
            }))
        );

        assert_eq!(
            Uuid::parse_str("!"),
            Err(Error(ErrorKind::InvalidLength {
                expected: EXPECTED_UUID_LENGTHS,
                found: 1
            }))
        );

        assert_eq!(
            Uuid::parse_str("F9168C5E-CEB2-4faa-B6BF-329BF39FA1E45"),
            Err(Error(ErrorKind::InvalidLength {
                expected: EXPECTED_UUID_LENGTHS,
                found: 37,
            }))
        );

        assert_eq!(
            Uuid::parse_str("F9168C5E-CEB2-4faa-BBF-329BF39FA1E4"),
            Err(Error(ErrorKind::InvalidLength {
                expected: EXPECTED_UUID_LENGTHS,
                found: 35
            }))
        );

        assert_eq!(
            Uuid::parse_str("F9168C5E-CEB2-4faa-BGBF-329BF39FA1E4"),
            Err(Error(ErrorKind::InvalidCharacter {
                expected: EXPECTED_CHARS,
                found: 'G',
                index: 20,
                urn: UrnPrefix::Optional,
            }))
        );

        assert_eq!(
            Uuid::parse_str("F9168C5E-CEB2F4faaFB6BFF329BF39FA1E4"),
            Err(Error(ErrorKind::InvalidGroupCount {
                expected: EXPECTED_GROUP_COUNTS,
                found: 2
            }))
        );

        assert_eq!(
            Uuid::parse_str("F9168C5E-CEB2-4faaFB6BFF329BF39FA1E4"),
            Err(Error(ErrorKind::InvalidGroupCount {
                expected: EXPECTED_GROUP_COUNTS,
                found: 3,
            }))
        );

        assert_eq!(
            Uuid::parse_str("F9168C5E-CEB2-4faa-B6BFF329BF39FA1E4"),
            Err(Error(ErrorKind::InvalidGroupCount {
                expected: EXPECTED_GROUP_COUNTS,
                found: 4,
            }))
        );

        assert_eq!(
            Uuid::parse_str("F9168C5E-CEB2-4faa"),
            Err(Error(ErrorKind::InvalidLength {
                expected: EXPECTED_UUID_LENGTHS,
                found: 18,
            }))
        );

        assert_eq!(
            Uuid::parse_str("F9168C5E-CEB2-4faaXB6BFF329BF39FA1E4"),
            Err(Error(ErrorKind::InvalidCharacter {
                expected: EXPECTED_CHARS,
                found: 'X',
                index: 18,
                urn: UrnPrefix::Optional,
            }))
        );

        assert_eq!(
            Uuid::parse_str("{F9168C5E-CEB2-4faa9B6BFF329BF39FA1E41"),
            Err(Error(ErrorKind::InvalidLength {
                expected: ExpectedLength::Any(&[36, 32]),
                found: 38
            }))
        );

        assert_eq!(
            Uuid::parse_str("F9168C5E-CEB-24fa-eB6BFF32-BF39FA1E4"),
            Err(Error(ErrorKind::InvalidGroupLength {
                expected: ExpectedLength::Exact(4),
                found: 3,
                group: 1,
            }))
        );
        // (group, found, expecting)
        //
        assert_eq!(
            Uuid::parse_str("01020304-1112-2122-3132-41424344"),
            Err(Error(ErrorKind::InvalidGroupLength {
                expected: ExpectedLength::Exact(12),
                found: 8,
                group: 4,
            }))
        );

        assert_eq!(
            Uuid::parse_str("67e5504410b1426f9247bb680e5fe0c"),
            Err(Error(ErrorKind::InvalidLength {
                expected: EXPECTED_UUID_LENGTHS,
                found: 31,
            }))
        );

        assert_eq!(
            Uuid::parse_str("67e5504410b1426f9247bb680e5fe0c88"),
            Err(Error(ErrorKind::InvalidLength {
                expected: EXPECTED_UUID_LENGTHS,
                found: 33,
            }))
        );

        assert_eq!(
            Uuid::parse_str("67e5504410b1426f9247bb680e5fe0cg8"),
            Err(Error(ErrorKind::InvalidLength {
                expected: EXPECTED_UUID_LENGTHS,
                found: 33,
            }))
        );

        assert_eq!(
            Uuid::parse_str("67e5504410b1426%9247bb680e5fe0c8"),
            Err(Error(ErrorKind::InvalidCharacter {
                expected: EXPECTED_CHARS,
                found: '%',
                index: 15,
                urn: UrnPrefix::Optional,
            }))
        );

        assert_eq!(
            Uuid::parse_str("231231212212423424324323477343246663"),
            Err(Error(ErrorKind::InvalidLength {
                expected: EXPECTED_UUID_LENGTHS,
                found: 36,
            }))
        );

        assert_eq!(
            Uuid::parse_str("{00000000000000000000000000000000}"),
            Err(Error(ErrorKind::InvalidLength {
                expected: EXPECTED_UUID_LENGTHS,
                found: 34,
            }))
        );

        // Test error reporting
        assert_eq!(
            Uuid::parse_str("67e5504410b1426f9247bb680e5fe0c"),
            Err(Error(ErrorKind::InvalidLength {
                expected: EXPECTED_UUID_LENGTHS,
                found: 31,
            }))
        );
        assert_eq!(
            Uuid::parse_str("67e550X410b1426f9247bb680e5fe0cd"),
            Err(Error(ErrorKind::InvalidCharacter {
                expected: EXPECTED_CHARS,
                found: 'X',
                index: 6,
                urn: UrnPrefix::Optional,
            }))
        );
        assert_eq!(
            Uuid::parse_str("67e550-4105b1426f9247bb680e5fe0c"),
            Err(Error(ErrorKind::InvalidGroupLength {
                expected: ExpectedLength::Exact(8),
                found: 6,
                group: 0,
            }))
        );
        assert_eq!(
            Uuid::parse_str("F9168C5E-CEB2-4faa-B6BF1-02BF39FA1E4"),
            Err(Error(ErrorKind::InvalidGroupLength {
                expected: ExpectedLength::Exact(4),
                found: 5,
                group: 3,
            }))
        );
    }

    #[test]
    fn test_roundtrip_default() {
        let uuid_orig = new();
        let orig_str = uuid_orig.to_string();
        let uuid_out = Uuid::parse_str(&orig_str).unwrap();
        assert_eq!(uuid_orig, uuid_out);
    }

    #[test]
    fn test_roundtrip_hyphenated() {
        let uuid_orig = new();
        let orig_str = uuid_orig.to_hyphenated().to_string();
        let uuid_out = Uuid::parse_str(&orig_str).unwrap();
        assert_eq!(uuid_orig, uuid_out);
    }

    #[test]
    fn test_roundtrip_simple() {
        let uuid_orig = new();
        let orig_str = uuid_orig.to_simple().to_string();
        let uuid_out = Uuid::parse_str(&orig_str).unwrap();
        assert_eq!(uuid_orig, uuid_out);
    }

    #[test]
    fn test_roundtrip_urn() {
        let uuid_orig = new();
        let orig_str = uuid_orig.to_urn().to_string();
        let uuid_out = Uuid::parse_str(&orig_str).unwrap();
        assert_eq!(uuid_orig, uuid_out);
    }

    #[test]
    fn test_roundtrip_braced() {
        let uuid_orig = new();
        let orig_str = uuid_orig.to_braced().to_string();
        let uuid_out = Uuid::parse_str(&orig_str).unwrap();
        assert_eq!(uuid_orig, uuid_out);
    }
}
