// testing on sd-id128
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
use sd_id128::ID128;

#[test]
fn debug_default() {
    assert_eq!(format!("{:?}", ID128::default()),
               "ID128 { ffi: sd_id128 { value: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] \
                } }");
}

#[test]
fn debug_alternative() {
    assert_eq!(format!("{:#?}", ID128::default()), "ID128 {\n    ffi: sd_id128 {\n        value: [\n            0,\n            0,\n            0,\n            0,\n            0,\n            0,\n            0,\n            0,\n            0,\n            0,\n            0,\n            0,\n            0,\n            0,\n            0,\n            0,\n        ],\n    },\n}");
}

#[test]
fn default() {
    let _ = ID128::default();
}

#[test]
fn clone() {
    let _ = ID128::default().clone();
}

#[test]
fn display() {
    let default = ID128::default();
    assert_eq!(format!("{}", default),
               "00000000-0000-0000-0000-000000000000");
}

#[test]
fn eq() {
    assert!(ID128::default().eq(&ID128::default()));
    let random = ID128::random_id().unwrap();
    assert_eq!(random, random);
}

#[test]
fn random_id() {
    let random1 = ID128::random_id().unwrap();
    let random2 = ID128::random_id().unwrap();
    assert_ne!(random1, random2);
}

#[test]
fn boot_id() {
    let _ = ID128::boot_id().unwrap();
}

#[test]
#[cfg(feature = "247")]
fn boot_id_hashed() {
    let random = ID128::random_id().unwrap();
    let boot = ID128::boot_id().unwrap();
    let boot_hashed = ID128::boot_id_app_specific(random).unwrap();
    assert_ne!(boot, boot_hashed);
}

#[test]
fn machine_id() {
    let _ = ID128::machine_id().unwrap();
}

#[test]
fn machine_id_hashed() {
    let random = ID128::random_id().unwrap();
    let machine = ID128::machine_id().unwrap();
    let machine_hashed = ID128::machine_id_app_specific(random).unwrap();
    assert_ne!(machine, machine_hashed);
}

// #[test]
// fn invocation_id() {
//     assert!(ID128::invocation_id().is_ok());
// }

#[test]
fn from_string_lower_libsystemd() {
    let parsed = ID128::from_str("0123456789abcdef0123456789abcdef").unwrap();
    assert_eq!(parsed.as_ref(), &[1, 35, 69, 103, 137, 171, 205, 239, 1,
                                  35, 69, 103, 137, 171, 205, 239]);
}

#[test]
fn from_string_upper_libsystemd() {
    let parsed = ID128::from_str("0123456789ABCDEF0123456789ABCDEF").unwrap();
    assert_eq!(parsed.as_ref(), &[1, 35, 69, 103, 137, 171, 205, 239, 1,
                                  35, 69, 103, 137, 171, 205, 239]);
}

#[test]
fn from_string_mixed_libsystemd() {
    let parsed = ID128::from_str("0123456789AbCdEf0123456789aBcDeF").unwrap();
    assert_eq!(parsed.as_ref(), &[1, 35, 69, 103, 137, 171, 205, 239, 1,
                                  35, 69, 103, 137, 171, 205, 239]);
}

#[allow(non_snake_case)]
#[test]
fn from_string_lower_RFC() {
    let parsed = ID128::from_str("01234567-89ab-cdef-0123-456789abcdef").unwrap();
    assert_eq!(parsed.as_ref(), &[1, 35, 69, 103, 137, 171, 205, 239, 1,
                                  35, 69, 103, 137, 171, 205, 239]);
}

#[allow(non_snake_case)]
#[test]
fn from_string_upper_RFC() {
    let parsed = ID128::from_str("01234567-89AB-CDEF-0123-456789ABCDEF").unwrap();
    assert_eq!(parsed.as_ref(), &[1, 35, 69, 103, 137, 171, 205, 239, 1,
                                  35, 69, 103, 137, 171, 205, 239]);
}

#[allow(non_snake_case)]
#[test]
fn from_string_mixed_RFC() {
    let parsed = ID128::from_str("01234567-89Ab-CdEf-0123-456789aBcDeF").unwrap();
    assert_eq!(parsed.as_ref(), &[1, 35, 69, 103, 137, 171, 205, 239, 1,
                                  35, 69, 103, 137, 171, 205, 239]);
}

#[test]
fn from_string_lower_simple() {
    let parsed = ID128::from_str("0123-4567-89ab-cdef-0123-4567-89ab-cdef").unwrap();
    assert_eq!(parsed.as_ref(), &[1, 35, 69, 103, 137, 171, 205, 239, 1,
                                  35, 69, 103, 137, 171, 205, 239]);
}

#[test]
fn from_string_upper_simple() {
    let parsed = ID128::from_str("0123-4567-89AB-CDEF-0123-4567-89AB-CDEF").unwrap();
    assert_eq!(parsed.as_ref(), &[1, 35, 69, 103, 137, 171, 205, 239, 1,
                                  35, 69, 103, 137, 171, 205, 239]);
}

#[test]
fn from_string_mixed_simple() {
    let parsed = ID128::from_str("0123-4567-89Ab-CdEf-0123-4567-89aB-cDeF").unwrap();
    assert_eq!(parsed.as_ref(), &[1, 35, 69, 103, 137, 171, 205, 239, 1,
                                  35, 69, 103, 137, 171, 205, 239]);
}

#[test]
fn from_string_too_long_fails() {
    assert!(ID128::from_str("0123-4567-89Ab-CdEf-0123-4567-89aB-cDeF-1234").is_err());
}

#[test]
fn from_string_too_short_fails() {
    assert!(ID128::from_str("0123-4567-89Ab-CdEf-0123-4567-89aB").is_err());
}

#[test]
fn from_string_invalid_character() {
    assert!(ID128::from_str("0123-4567-89AB-CDEX-0123-4567-89AB-CDEF").is_err());
}

#[test]
fn from_string_invalid_dash_simple() {
    assert!(ID128::from_str("0123-4567-89A-BCDEF-0123-4567-89AB-CDEF").is_err());
}

#[test]
fn from_string_missing_dash_simple() {
    assert!(ID128::from_str("01234567-89AB-CDEF-0123-4567-89AB-CDEF").is_err());
}

#[test]
#[allow(non_snake_case)]
fn from_string_invalid_dash_RFC() {
    assert!(ID128::from_str("01234567-89AB-CDEF-0123-456789ABCDE-F").is_err());
}

#[test]
#[allow(non_snake_case)]
fn from_string_missing_dash_RFC() {
    assert!(ID128::from_str("01234567-89AB-CDEF-0123456789ABCDE-F").is_err());
}

#[test]
fn from_string_invalid_dash_libsystemd() {
    assert!(ID128::from_str("0123456789ABCDEF0123456789ABCDE-F").is_err());
}

#[test]
fn from_string_based_on_random_succeeds() {
    let random = ID128::random_id().unwrap();
    let parsed = ID128::from_str(random.to_string().as_str()).unwrap();
    assert_eq!(random, parsed);
}

#[test]
fn from_string_lax_too_long_fails() {
    assert!(ID128::from_str_lax("0123-4567-89Ab-CdEf-0123-4567-89aB-cDeF-1234").is_err());
}

#[test]
fn from_string_lax_too_short_fails() {
    assert!(ID128::from_str_lax("0123-4567-89Ab-CdEf-0123-4567-89aB").is_err());
}

#[test]
fn from_string_lax_invalid_character() {
    assert!(ID128::from_str_lax("0123-4567-89AB-CDEX-0123-4567-89AB-CDEF").is_err());
}

#[test]
fn from_string_lax_invalid_dash_simple() {
    assert!(ID128::from_str_lax("0123-4567-89A-BCDEF-0123-4567-89AB-CDEF").is_ok());
}

#[test]
fn from_string_lax_missing_dash_simple() {
    assert!(ID128::from_str_lax("01234567-89AB-CDEF-0123-4567-89AB-CDEF").is_ok());
}

#[test]
fn from_string_lax_full_mess() {
    assert!(ID128::from_str_lax("    01-23-456789AB-C----DEF01234567-89ABCDEF----     ").is_ok());
}

#[test]
#[allow(non_snake_case)]
fn from_string_lax_invalid_dash_RFC() {
    assert!(ID128::from_str_lax("01234567-89AB-CDEF-0123-456789ABCDE-F").is_ok());
}

#[test]
#[allow(non_snake_case)]
fn from_string_lax_missing_dash_RFC() {
    assert!(ID128::from_str_lax("01234567-89AB-CDEF-0123456789ABCDE-F").is_ok());
}

#[test]
fn from_string_lax_invalid_dash_libsystemd() {
    assert!(ID128::from_str_lax("0123456789ABCDEF0123456789ABCDE-F").is_ok());
}
#[test]
fn from_string_sd_upper_libsystemd_succeeds() {
    assert!(ID128::from_str_sd("1234567890ABCDEF1234567890ABCDEF").is_ok());
}

#[test]
fn from_string_sd_lower_libsystemd_succeeds() {
    assert!(ID128::from_str_sd("1234567890abcdef1234567890abcdef").is_ok());
}

#[test]
fn from_string_sd_too_short_fails() {
    assert!(ID128::from_str_sd("1234567890abcd567890abcdef").is_err());
}

#[test]
fn from_string_sd_too_long_fails() {
    assert!(ID128::from_str_sd("1234567890ABCDEF1234567890ABCDEF1").is_err());
}

#[test]
fn ffi_from_string_upper_eq_lower_case() {
    let upper = ID128::from_str_sd("1234567890ABCDEF1234567890ABCDEF").unwrap();
    let lower = ID128::from_str_sd("1234567890abcdef1234567890abcdef").unwrap();
    assert_eq!(upper.as_ref(), lower.as_ref());
}
