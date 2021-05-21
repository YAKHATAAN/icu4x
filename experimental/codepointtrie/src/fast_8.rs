// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::codepointtrie::{
    CodePointTrie, CodePointTrieData, CodePointTrieType, CodePointTrieValueWidth,
};

// trait+impl fn polymorphism testing code

struct _MyStruct<T> {
    x: T,
}

fn _trait_impl_polymorphism_scratch_code() {
    let _xyz = _MyStruct { x: 42 };

    let _xyz = _MyStruct::<CodePointTrieValueWidth> {
        x: CodePointTrieValueWidth::Bits8,
    };
}

trait CodePointTrieReader<'t> {
    fn get_trie_type(trie_type_int: u8) -> CodePointTrieType;
    fn get_value_width(value_width_int: u8) -> CodePointTrieValueWidth;
    fn internal_small_index(
        trie: &CodePointTrie<CodePointTrieType, CodePointTrieValueWidth>,
        c: u32,
    ) -> u32;
    fn small_index(trie: &CodePointTrie<CodePointTrieType, CodePointTrieValueWidth>, c: u32)
        -> u32;
    fn fast_index(trie: &CodePointTrie<CodePointTrieType, CodePointTrieValueWidth>, c: u32) -> u32;
}

// can't do this because:
//
// expected type, found variant `CodePointTrieType::Fast`
// not a type
// help: try using the variant's enum: `crate::CodePointTrieType`rustc(E0573)
//
// impl<'trie> CodePointTrieReader<'trie> for CodePointTrie<'trie, CodePointTrieType::Fast, CodePointTrieValueWidth::Bits8> {
//
// }

// empty trait+structs polymorphism testing code

// Using empty traits and structs instead of enums, following this strategy:
// https://stackoverflow.com/questions/59426358/how-to-make-a-struct-containing-an-enum-generic-over-the-enum-variant
// in order to allow polymorphism on methods for the `CodePointTrie` struct.
// This is because you cannot have `impl MyTrait for MyStruct<enum1::varianta, enum2::variantb> {...}`.

// alternative to having an enum `ValueWidth` with variants `Bits16`, `Bits32`, `Bits8`.

trait ValueWidth {}

struct Bits16;
struct Bits32;
struct Bits8;
struct BitsAny;

impl ValueWidth for Bits16 {}
impl ValueWidth for Bits32 {}
impl ValueWidth for Bits8 {}
impl ValueWidth for BitsAny {}

// alternative to having an enum `TrieType` with variants `Fast`, `Small`.

trait TrieType {}

struct FastType;
struct SmallType;
struct AnyType;

impl TrieType for FastType {}
impl TrieType for SmallType {}
impl TrieType for AnyType {}

fn trie_internal_small_index(
    trie: &CodePointTrie<CodePointTrieType, CodePointTrieValueWidth>,
    c: u32,
) -> u32 {
    crate::fast::trie_internal_small_index(trie, c)
}

/// Internal trie getter for a code point at or above the fast limit. Returns the data index.
fn trie_small_index(
    trie: &CodePointTrie<CodePointTrieType, CodePointTrieValueWidth>,
    c: u32,
) -> u32 {
    crate::fast::trie_small_index(trie, c)
}

/// Internal trie getter for a code point below the fast limit. Returns the data index.
fn trie_fast_index(
    trie: &CodePointTrie<CodePointTrieType, CodePointTrieValueWidth>,
    c: u32,
) -> u32 {
    crate::fast::trie_fast_index(trie, c)
}

/// Internal trie getter to get trie data array index position for code point
/// value `c` that is beyond ASCII range. Also checks that c is in
/// U+0000..10FFFF.
fn trie_cp_index(trie: &CodePointTrie<CodePointTrieType, CodePointTrieValueWidth>, c: u32) -> u32 {
    crate::fast::trie_cp_index(trie, c)
}

/// Helper function that gets the data array value at the provided index
fn trie_get_value(
    data: &CodePointTrieData,
    value_width: &CodePointTrieValueWidth,
    data_index: u32,
) -> u32 {
    crate::fast::trie_get_value(data, value_width, data_index)
}

fn trie_get(trie: &CodePointTrie<CodePointTrieType, CodePointTrieValueWidth>, c: u32) -> u32 {
    crate::fast::trie_get(trie, c)
}

fn check_trie(
    trie: &CodePointTrie<CodePointTrieType, CodePointTrieValueWidth>,
    check_ranges: &[u32],
) {
    assert_eq!(
        0,
        check_ranges.len() % 2,
        "check_ranges must have an even number of 32-bit values in (limit,value) pairs"
    );

    let mut i: u32 = 0;
    let check_range_tuples = check_ranges.chunks(2);
    // Iterate over each check range
    for range_tuple in check_range_tuples {
        let range_end = range_tuple[0];
        let range_value = range_tuple[1];
        // Check all values in this range, one-by-one
        while i < range_end {
            assert_eq!(
                range_value,
                trie_get(trie, i),
                "expected trie_get({}) == {}",
                i,
                range_value
            );
            i = i + 1;
        }
    }
}

#[cfg(test)]
mod fast_8_test {
    use super::*;

    const INDEX: [u16; 1024] = [
        0, 0x40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0x80, 0xc0, 0xc0, 0xc0, 0xc0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    const DATA_8: [u8; 260] = [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 1, 1, 1, 0xad,
    ];

    const CHECK_RANGES: [u32; 10] = [0, 1, 0x740, 1, 0x780, 2, 0x880, 3, 0x110000, 1];

    // Exported trie data from free-blocks.8.toml. This file represents a
    // fast-type trie with 8-bit width data.
    fn get_testing_fast_type_8_bit_trie<'trie>(
    ) -> CodePointTrie<'trie, CodePointTrieType, CodePointTrieValueWidth> {
        let index_length: u32 = 1024;
        let data_length: u32 = 260;
        // Question: in ICU4C, `highStart` is a `UChar32` type. Does it make sense
        // to represent it as a u32 since UnicodeSet deals with `u32` instead of
        // the Rust `char` type?
        let high_start: u32 = 0xa00;
        let shifted12_high_start: u16 = 0x1;
        let trie_type: u8 = 0;
        let value_width: u8 = 2;
        let index3_null_offset: u16 = 0x7fff;
        let data_null_offset: u32 = 0x0;
        let null_value: u32 = 0x1;

        let trie: CodePointTrie<CodePointTrieType, CodePointTrieValueWidth> = CodePointTrie {
            index_length,
            data_length,
            high_start,
            shifted12_high_start,
            trie_type: crate::codepointtrie::get_code_point_trie_type(trie_type),
            value_width: crate::codepointtrie::get_code_point_trie_value_width(value_width),
            index3_null_offset,
            data_null_offset,
            null_value,
            index: &INDEX,
            data: &CodePointTrieData {
                data_8_bit: Some(&DATA_8),
                data_16_bit: None,
                data_32_bit: None,
            },
        };

        trie
    }

    #[test]
    pub fn cp_index_test() {
        let trie = get_testing_fast_type_8_bit_trie();

        assert_eq!(0, trie_cp_index(&trie, 0), "trie_cp_index(&trie, 0)");
        assert_eq!(1, trie_cp_index(&trie, 1), "trie_cp_index(&trie, 1)");
        assert_eq!(2, trie_cp_index(&trie, 2), "trie_cp_index(&trie, 2)");

        // fastIndex == index[64 >> FAST_SHIFT] + (64 & FAST_DATA_MASK)
        //   == index[63 >> 6] + (63 & 63)
        //   == index[0] + 63
        //   == 0 + 63
        //   == 63
        assert_eq!(63, trie_cp_index(&trie, 63), "trie_cp_index(&trie, 63)");

        // fastIndex == index[64 >> FAST_SHIFT] + (64 & FAST_DATA_MASK)
        //   == index[64 >> 6] + (64 & 63)
        //   == index[1] + 0
        //   == 64 + 0
        //   == 64
        assert_eq!(64, trie_cp_index(&trie, 64), "trie_cp_index(&trie, 64)");

        // fastIndex == index[127 >> FAST_SHIFT] + (127 & FAST_DATA_MASK)
        //   == index[127 >> 6] + (127 & 63)
        //   == index[1] + 63
        //   == 64 + 63
        //   == 127
        assert_eq!(127, trie_cp_index(&trie, 127), "trie_cp_index(&trie, 127)");

        // fastIndex == index[999 >> FAST_SHIFT] + (999 & FAST_DATA_MASK)
        //   == index[999 >> 6] + (999 & 63)
        //   == index[15] + 39
        //   == 39
        assert_eq!(trie_cp_index(&trie, 999), 39);
    }

    #[test]
    pub fn get_test() {
        let trie = get_testing_fast_type_8_bit_trie();

        assert_eq!(trie_get(&trie, 0), 1);
        assert_eq!(trie_get(&trie, 1), 1);
        assert_eq!(trie_get(&trie, 2), 1);
        assert_eq!(trie_get(&trie, 28), 1);
        assert_eq!(trie_get(&trie, 29), 1);
    }

    #[test]
    pub fn check_ranges_test() {
        let trie = get_testing_fast_type_8_bit_trie();

        check_trie(&trie, &CHECK_RANGES);
    }
}