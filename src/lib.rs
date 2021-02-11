// sd-id128: wrapper for sd-id128 of libsystemd
// Copyright (C) 2020 Christian Klaue [mail@ck76.de]
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! [![buy me a coffee](https://img.shields.io/badge/buy%20me%20a%20coffee-or%20I%20sing-53a0d0?style=flat&logo=Buy-Me-A-Coffee)](https://www.buymeacoffee.com/ente)  [![donate@paypal](https://img.shields.io/badge/paypal-donation-53a0d0?style=flat&logo=paypal)](https://www.paypal.com/donate?hosted_button_id=CRGNTJBS4AD4G)
//!
//! [sd-id128](https://github.com/ente76/sd-id128) is a rust wrapper for sd-id128 in the systemd API of [libsystemd](https://www.freedesktop.org/software/systemd/man/sd-id128.html). sd-id128 is part of the [systemd.rs](https://github.com/ente76/systemd.rs) project.
use sd_sys::id128 as ffi;
use std::{convert::TryFrom,
          error,
          ffi::{CString, IntoStringError, NulError},
          fmt};

/// Wrapper for sd-id128 as offered in libsystemd.
///
/// ID128 fully implements translations to FFI calls to libsystemd and native
/// Rust functionality to handle UUIDs.
///
/// FFI Constructors -> Result<ID128, Error>
/// - bood_id: get boot id
/// - boot_id_app_specific: get hashed boot id
/// - machine_id: get machine id
/// - machine_id_app_specific: get hashed machine id
/// - invocation_id: get service invocation id
/// - random_id: get a random id
/// - from_string_sd: parse string into id using libsystemd
///
/// Native Constructors -> Result<ID128, Error>
/// - from_string: parse string into id using native Rust
/// - from_string_lax: parse string into id using native Rust with lax rules
///
/// FFI Methods -> Result<T, Error>
/// - to_string_sd: format an id as String using libsystemd
/// - into_cstring_sd: format an id as CString using libsystemd
///
/// Native Method -> T
/// - to_string: format an id as String in default format using native Rust
/// - to_string_formatted: format an id as String using native Rust
///
/// Implemented Traits
/// - Display: provides `to_string(&ID128) -> String` and `format!(..., &ID128)`
/// - From<ID128> -> String: provides `into(ID128) -> String`
/// - TryFrom<String> -> ID128: provides `try_into(String) -> ID128`
/// - From<<ffi::sd_id128>> -> ID128: provides `into(ffi::sd_id128) -> ID128`
/// - From<ID128> -> ffi::sd_id128: provides `into(ID128) -> ffi::sd_id128`
/// - AsRef<[u8; 16]>: provides `as_ref(&ID128) -> &[u8; 16]`
/// - Clone: provides `clone(&ID128) -> ID128`
/// - From<ID128> -> [u8; 16]: provides `into(ID128) -> [u8; 16]`
/// - From<[u8; 16]> -> ID128: provides `into([u8; 16]) -> ID128`
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ID128 {
    ffi: ffi::sd_id128
}

/// Errors raised in sd-id128
///
/// Variants:
/// - NullError: This error is raised during translation of a native String into
///   C compatible CString. The error is caused by contained 0x00 bytes in
///   String which are not compatible with C & CString.
/// - SDError: This error is raised after libsystemd returned a negative return
///   code, i.e. an error code.
/// - StringError: This error is raised during translation of C compatible
///   CString back into native String. The error is caused by non-UTF8 symbols.
#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    NullError(NulError),
    SDError(i32),
    IntoStringError(IntoStringError),
    ParseStringError(&'static str, usize)
}

/// String formats available during transformation from an ID into text
///
/// Variants:
/// - Simple: 0000-0000-0000-0000-0000-0000-0000-0000
/// - LibSystemD: 00000000000000000000000000000000, this format is applied on
///   all formatting performed by calling FFI functionality
/// - RFC: 00000000-0000-0000-0000-000000000000, this format is applied by
///   default to all native formatting
#[derive(Debug, Eq, PartialEq)]
pub enum Format {
    Simple,
    LibSystemD,
    RFC
}

/// Format of hexadecimal letters during transformation from an ID into text
///
/// Variants:
/// - Upper
/// - Lower: lower case is applied as default to all formatting
#[derive(Debug, Eq, PartialEq)]
pub enum Case {
    Upper,
    Lower
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::NullError(ref error) => error.fmt(formatter),
            Error::SDError(_) => todo!(),
            Error::IntoStringError(ref error) => error.fmt(formatter),
            Error::ParseStringError(ref message, ref pos) => {
                write!(formatter, "{}{}", message, pos)
            }
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::NullError(ref error) => Some(error),
            Error::SDError(_) => None,
            Error::IntoStringError(ref error) => Some(error),
            Error::ParseStringError(_, _) => None
        }
    }
}

impl From<ID128> for String {
    fn from(id128: ID128) -> String {
        id128.to_string()
    }
}

impl TryFrom<&str> for ID128 {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Error> {
        ID128::from_str(value)
    }
}

impl fmt::Display for ID128 {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter,
               "{}",
               self.to_string_formatted(Format::RFC, Case::Lower))
    }
}

impl From<ID128> for ffi::sd_id128 {
    fn from(id128: ID128) -> ffi::sd_id128 {
        id128.ffi
    }
}

impl From<ffi::sd_id128> for ID128 {
    fn from(sd_id128: ffi::sd_id128) -> ID128 {
        ID128 { ffi: sd_id128 }
    }
}

impl AsRef<[u8; 16]> for ID128 {
    fn as_ref(&self) -> &[u8; 16] {
        &self.ffi.value
    }
}

impl From<ID128> for [u8; 16] {
    fn from(id128: ID128) -> [u8; 16] {
        id128.ffi.value
    }
}

impl From<[u8; 16]> for ID128 {
    fn from(value: [u8; 16]) -> ID128 {
        ID128 { ffi: ffi::sd_id128 { value } }
    }
}

impl ID128 {
    /// Generates a new randomized 128-bit ID
    /// ([`sd_id128_randomize`](https://www.freedesktop.org/software/systemd/man/sd_id128_randomize.html#)).
    ///
    /// Every invocation returns a new randomly generated ID. The libsystemd API
    /// uses the /dev/urandom kernel random number generator. Note that
    /// `sd_id128_randomize()` always returns a UUID v4-compatible ID.
    ///
    /// # Return Values
    /// - Ok(ID128): initialized ID128 struct
    /// - Err(Error::SDError(i32)): sd-id128 returned an error code
    pub fn random_id() -> Result<Self, Error> {
        let mut id128 = ffi::sd_id128::default();
        let result = unsafe { ffi::sd_id128_randomize(&mut id128) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(id128.into())
    }

    /// Returns the boot ID of the executing kernel
    /// ([`sd_id128_get_boot`](https://www.freedesktop.org/software/systemd/man/sd_id128_get_machine.html#)).
    ///
    /// libsystemd API reads and parses the /proc/sys/kernel/random/boot_id file
    /// exposed by the kernel. It is randomly generated early at boot and is
    /// unique for every running kernel instance. This function also internally
    /// caches the returned ID to make this call a cheap operation.
    ///
    /// # Return Values
    /// - Ok(ID128): initialized ID128 struct
    /// - Err(Error::SDError(i32)): sd-id128 returned an error code
    pub fn boot_id() -> Result<Self, Error> {
        let mut id128 = ffi::sd_id128::default();
        let result = unsafe { ffi::sd_id128_get_boot(&mut id128) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(id128.into())
    }

    /// Returns an app specific boot id
    /// ([`sd_id128_get_boot_app_specific`](https://www.freedesktop.org/software/systemd/man/sd_id128_get_machine.html#)).
    ///
    /// It is recommended to use this function instead of `boot_id` when passing
    /// an ID to untrusted environments, in order to make sure that the original
    /// boot ID may not be determined externally. This way, the ID used by the
    /// application remains stable on a given machine, but cannot be easily
    /// correlated with IDs used in other applications on the same machine.
    ///
    /// # Return Values
    /// - Ok(ID128): initialized ID128 struct
    /// - Err(Error::SDError(i32)): sd-id128 returned an error code
    #[cfg(feature = "240")]
    pub fn boot_id_app_specific(app: ID128) -> Result<Self, Error> {
        let mut boot = ffi::sd_id128::default();
        let result = unsafe { ffi::sd_id128_get_boot_app_specific(app.ffi, &mut boot) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(boot.into())
    }

    /// Returns the machine ID of the executing host
    /// ([`sd_id128_get_machine`](https://www.freedesktop.org/software/systemd/man/sd_id128_get_machine.html#))
    ///
    /// This reads and parses the machine-id file. This libsystemd API caches
    /// the machine ID internally to make retrieving the machine ID a cheap
    /// operation. This ID may be used wherever a unique identifier for the
    /// local system is needed.
    ///
    /// # Return Values
    /// - Ok(ID128): initialized ID128 struct
    /// - Err(Error::SDError(i32)): sd-id128 returned an error code
    pub fn machine_id() -> Result<Self, Error> {
        let mut id128 = ffi::sd_id128::default();
        let result = unsafe { ffi::sd_id128_get_machine(&mut id128) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(id128.into())
    }

    /// Returns an app specific machine id
    /// ([`sd_id128_get_machine_app_specific`](https://www.freedesktop.org/software/systemd/man/sd_id128_get_machine.html#)).
    ///
    /// It is recommended to use this function instead of `machine_id` when
    /// passing an ID to untrusted environments, in order to make sure that the
    /// original machine ID may not be determined externally. This way, the ID
    /// used by the application remains stable on a given machine, but cannot be
    /// easily correlated with IDs used in other applications on the same
    /// machine.
    ///
    /// # Return Values
    /// - Ok(ID128): initialized ID128 struct
    /// - Err(Error::SDError(i32)): sd-id128 returned an error code
    #[cfg(any(feature = "233", feature = "240"))]
    pub fn machine_id_app_specific(app: ID128) -> Result<Self, Error> {
        let mut machine = ffi::sd_id128::default();
        let result = unsafe { ffi::sd_id128_get_machine_app_specific(app.ffi, &mut machine) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(machine.into())
    }

    /// Returns the invocation ID of the service
    /// ([`sd_id128_get_invocation`](https://www.freedesktop.org/software/systemd/man/sd_id128_get_machine.html#)).
    ///
    /// In its current implementation, this reads and parses the $INVOCATION_ID
    /// environment variable that the service manager sets when activating a
    /// service.
    ///
    /// # Return Values
    /// - Ok(ID128): initialized ID128 struct
    /// - Err(Error::SDError(i32)): sd-id128 returned an error code
    pub fn invocation_id() -> Result<Self, Error> {
        let mut id128 = ffi::sd_id128::default();
        let result = unsafe { ffi::sd_id128_get_invocation(&mut id128) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(id128.into())
    }

    /// Parses a string into an ID applying strict rules using native Rust
    /// functionality (i.e. without any FFI call).
    ///
    /// Takes a character string and tries to parse it into a valid ID. This
    /// method parses the string using native Rust functionality and strict
    /// formatting rules are applied to the source. There are two alternatives
    /// to this method:
    /// - from_str_lax: allows some formatting errors on the source string
    /// - from_str_sd: performs a FFI call to libsystemd in order to parse the
    ///   source string
    ///
    /// This method is strict with regards to the format of the source string:
    /// - only dashes an hexadecimal numbers are allowed
    /// - letter casing can be either upper or lower case
    /// - dashes must conform precisely to any of the formats
    ///
    /// # Return Values
    /// - Ok(ID128): success
    /// - Err(Error::ParseStringError): the source string did not strictly
    ///   comply with the expected format
    pub fn from_str(string: &str) -> Result<Self, Error> {
        let mut id = ID128::default();
        let mut idseg = 0;
        let mut value = 0;
        let mut pair = false;
        let format = match (string.len(), string.matches('-').count()) {
            (39, 7) => Format::Simple,
            (32, 0) => Format::LibSystemD,
            (36, 4) => Format::RFC,
            _ => return Err(Error::ParseStringError("Invalid string length: ", string.len()))
        };
        for (charpos, char) in string.char_indices() {
            value += match char {
                '0'..='9' => char as u32 - '0' as u32,
                'a'..='f' => char as u32 - 'a' as u32 + 10,
                'A'..='F' => char as u32 - 'A' as u32 + 10,
                '-' => match format {
                    Format::LibSystemD => {
                        return Err(Error::ParseStringError("String contains an unexpected \
                                                            dash at position: ",
                                                           charpos))
                    },
                    Format::RFC => {
                        if charpos == 8 || charpos == 13 || charpos == 18 || charpos == 23 {
                            continue;
                        } else {
                            return Err(Error::ParseStringError("String contains an unexpected \
                                                                dash at position: ",
                                                               charpos));
                        }
                    },
                    Format::Simple => {
                        if charpos == 4
                           || charpos == 9
                           || charpos == 14
                           || charpos == 19
                           || charpos == 24
                           || charpos == 29
                           || charpos == 34
                        {
                            continue;
                        } else {
                            return Err(Error::ParseStringError("String contains an unexpected \
                                                                dash at position: ",
                                                               charpos));
                        }
                    }
                },
                _ => {
                    return Err(Error::ParseStringError("String contains an invalid \
                                                        character at position: ",
                                                       charpos))
                },
            };
            if pair {
                id.ffi.value[idseg] = value as u8;
                idseg += 1;
                value = 0;
            } else {
                value *= 16;
            }
            pair = !pair;
        }
        Ok(id)
    }

    /// Parses a string into an ID using native Rust functionality.
    ///
    /// Takes a character string and tries to parse it into a valid ID. This
    /// method parses the string using native Rust functionality and lax
    /// formatting rules are applied to the source. There are two alternatives
    /// to this method:
    /// - from_str: strict formatting rules are applied on the source string
    /// - from_str_sd: performs a FFI call to libsystemd in order to parse the
    ///   source string
    ///
    /// This method reuses the strict parsing of `from_string` after
    /// pre-processing the source string as follows:
    /// - trim
    /// - remove all dashes: transform the string from any valid or invalid
    ///   format into a libsystemd conforming format
    ///
    /// # Return Values
    /// - Ok(ID128): success
    /// - Err(Error::ParseStringError): the source string did not comply with
    ///   the expected format
    pub fn from_str_lax(string: &str) -> Result<Self, Error> {
        let string = string.trim().replace("-", "");
        ID128::from_str(string.as_str())
    }

    /// Parses a string into an ID using libsystemd
    /// ([`sd_id128_from_string`](https://www.freedesktop.org/software/systemd/man/sd_id128_to_string.html#)).
    ///
    /// Takes a character string with 32 hexadecimal digits (either lowercase or
    /// uppercase) and parses them back into a 128-bit ID.
    ///
    /// # Return Values
    /// - Ok(ID128): initialized ID128 struct
    /// - Err(Error::NulError): the source string did contain a 0-byte
    /// - Err(Error::SDError): sd-id128 returned an error code
    pub fn from_str_sd(string: &str) -> Result<Self, Error> {
        let string = CString::new(string).map_err(Error::NullError)?;
        let mut id128 = ffi::sd_id128::default();
        let result = unsafe { ffi::sd_id128_from_string(string.as_ptr(), &mut id128) };
        if result < 0 {
            return Err(Error::SDError(result));
        }
        Ok(id128.into())
    }

    /// Formats an ID as CString using libsystemd
    /// ([`sd_id128_to_string`](https://www.freedesktop.org/software/systemd/man/sd_id128_to_string.html#)).
    ///
    /// This function performs a FFI call to libsystemd to transform an ID into
    /// a string. There are some alternative methods available:
    /// - ID128 implements fmt::Display using native Rust functionality, thus
    ///   the following becomes available:
    ///   - to_string method and format!() will use a default format
    ///   - to_string_formatted offers formatting an ID in multiple variants
    /// - to_string_sd: is a convinience wrapper around into_string_sd
    ///
    /// This function is supposed to always be successful. Since there are some
    /// rather theoretical errors possible, the return value is a Result<>.
    ///
    /// # Return Values
    /// - Ok(String): a 128-bit ID as a character string. The ID will be
    ///   formatted as 32 lowercase hexadecimal digits and be terminated by a
    ///   NUL byte.
    /// - Err(Error::NullError): If this error is reported, it indicates an
    ///   error in this library.
    /// - Err(Error::SDError): If this error is reported, it indicates an error
    ///   in libsystemd and/or in this library. The error code is always 0 and
    ///   thus won't reveal any further information.
    pub fn into_cstring_sd(self) -> Result<CString, Error> {
        let c_string = CString::new("0123456789ABCDEF0123456789ABCDEF").map_err(Error::NullError)?;
        let raw = c_string.into_raw();
        let result = unsafe { ffi::sd_id128_to_string(self.ffi, raw) };
        let c_string = unsafe { CString::from_raw(raw) };
        if result.is_null() {
            return Err(Error::SDError(0));
        }
        Ok(c_string)
    }

    /// Formats an ID as String using libsystemd
    /// ([`sd_id128_to_string`](https://www.freedesktop.org/software/systemd/man/sd_id128_to_string.html#)).
    ///
    /// This function performs a FFI call to libsystemd to transform an ID into
    /// a string. There are some alternative methods available:
    /// - ID128 implements fmt::Display using native Rust functionality, thus
    ///   the following becomes available:
    ///   - to_string method and format!() will use a default format
    ///   - to_string_formatted offers formatting an ID in multiple variants
    /// - into_string_sd: to_string_sd is a convinience wrapper around
    ///   into_string_sd
    ///
    /// This function is supposed to always be successful. Since there are some
    /// rather theoretical errors possible, the return value is a Result<>.
    ///
    /// # Return Values
    /// - Ok(String): a 128-bit ID as a character string. The ID will be
    ///   formatted as 32 lowercase hexadecimal digits and be terminated by a
    ///   NUL byte.
    /// - Err(Error::NullError): If this error is reported, it indicates an
    ///   error in this library.
    /// - Err(Error::SDError): If this error is reported, it indicates an error
    ///   in libsystemd and/or in this library. The error code is always 0 and
    ///   thus won't reveal any further information.
    pub fn to_string_sd(&self) -> Result<String, Error> {
        let clone = self.clone();
        let c_string = clone.into_cstring_sd()?;
        c_string.into_string().map_err(Error::IntoStringError)
    }

    /// Formats an ID as String using Rust native functionality.
    ///
    /// This function transforms an ID into a String using native Rust
    /// functionality. As an alternative the methods `into_cstring_sd` and
    /// `to_string_sd` perform a FFI call to transform an ID into string.
    /// This Rust native function offers the possibility to apply the same
    /// format tho: choose format "LibSystemD" and lower case.
    ///
    /// The formatting default is in Rust native function to_string is RFC:
    /// 01234567-89ab-cdef-0123-456789abcdef. This is the official defined
    /// standard in RFC 4122.
    ///
    /// # Return Values
    /// - String: text representation of the id
    pub fn to_string_formatted(&self, format: Format, case: Case) -> String {
        self.ffi
            .value
            .iter()
            .enumerate()
            .map(move |(pos, digit)| {
                let dash = match format {
                    Format::Simple => {
                        if (pos + 1) % 2 == 0 && pos < 15 {
                            "-"
                        } else {
                            ""
                        }
                    },
                    Format::RFC => {
                        if pos == 3 || pos == 5 || pos == 7 || pos == 9 {
                            "-"
                        } else {
                            ""
                        }
                    },
                    Format::LibSystemD => ""
                };
                match case {
                    Case::Lower => format!("{:02x}{}", digit, dash),
                    Case::Upper => format!("{:02X}{}", digit, dash)
                }
            })
            .collect::<String>()
    }

    /// Transform an ID128 into a FFI binding sd_id128.
    ///
    /// The FFI binding struct sd_id128 is only required for direct FFI calls.
    pub fn into_ffi(self) -> ffi::sd_id128 {
        self.ffi
    }

    /// Returns a reference to the inner FFI binding sd_id128.
    ///
    /// The FFI binding struct sd_id128 is only required for direct FFI calls.
    pub fn as_ffi(&self) -> &ffi::sd_id128 {
        &self.ffi
    }

    /// Constructs an ID128 from a FFI binding sd_id128.
    ///
    /// The FFI binding struct sd_id128 retrieved from a direct FFI call may be
    /// used to construct a full ID128.
    pub fn from_ffi(ffi: ffi::sd_id128) -> ID128 {
        ID128 { ffi }
    }

    /// Returns a slice of the raw ID.
    pub fn as_raw_value(&self) -> &[u8; 16] {
        &self.ffi.value
    }

    /// Returns a mutable slice of the raw ID.
    pub fn as_mut_raw_value(&mut self) -> &mut [u8; 16] {
        &mut self.ffi.value
    }

    /// Transforms the ID128 into a raw value slice.
    pub fn into_raw_value(self) -> [u8; 16] {
        self.ffi.value
    }

    /// Constructs an ID128 from a raw value slice.
    pub fn from_raw_value(value: [u8; 16]) -> ID128 {
        ID128 { ffi: ffi::sd_id128 { value } }
    }
}
