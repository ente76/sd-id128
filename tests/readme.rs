#[test]
fn basic() {
    use sd_id128::{Case, Format, ID128};
    let id128 = ID128::boot_id().unwrap();
    println!("The boot id in RFC format is: {}", id128);
    println!("The boot id in upper case libsystemd format is: {}",
             id128.to_string_formatted(Format::LibSystemD, Case::Lower));
}
