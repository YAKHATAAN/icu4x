// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;
    use icu_codepointtrie::codepointtrie::{CodePointTrie, ValueWidth};

    #[diplomat::opaque]
    /// An ICU4X CodePointTrie.
    /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_codepointtrie/codepointtrie/struct.CodePointTrie.html) for more information.
    pub struct ICU4XCodePointTrie<'data, W: ValueWidth>(pub CodePointTrie<'data, W>);
}