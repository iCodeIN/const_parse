#![no_std]

//! Provides `const fn` for parsing.

/// Parses the input into a `u128` value.
///
/// * If the input string is empty, the output is 0.
/// * If the input string is exclusively decimal ASCII digits that spell a
///   `u128` value then the output will be that value.
/// * Otherwise the output is an unspecified value. In this case, differing
///   input strings are *likely* to produce different output values. Think of it
///   as a very bad hash function. Use it in your roguelikes to turn the
///   player's name into a PRNG seed, or something like that.
pub const fn parse_u128(input_string: &str) -> u128 {
  let bytes = input_string.as_bytes();
  let len = bytes.len();
  let mut index = 0_usize;
  let mut total = 0_u128;
  while index < len {
    total = total.wrapping_mul(10);
    let u = bytes[index].wrapping_sub(b'0') as u128;
    total = total.wrapping_add(u);
    //
    index += 1;
  }
  total
}

#[cfg(test)]
mod tests {
  extern crate std;

  use super::*;

  #[test]
  fn test_parse_u128() {
    assert_eq!(parse_u128("0"), 0);
    assert_eq!(parse_u128("12"), 12);
    assert_eq!(parse_u128("1234567890"), 1234567890);
    assert_eq!(parse_u128(&std::format!("{}", u128::MAX)), u128::MAX);

    // here we're just checking that large input strings won't wrap into a panic
    let big = "zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz";
    let mut s = std::string::String::new();
    (0..20).for_each(|_| s += big);
    parse_u128(&s);
  }
}
