use icu_provider_fs::FsDataProvider;
use icu_uniset::props;
use icu_uniset::enum_props;



/// Instructions:
///   1. cd $ICU4X/experimental/provider_ppucd && \
///          wget https://raw.githubusercontent.com/unicode-org/icu/master/icu4c/source/data/unidata/ppucd.txt
///   2. rm -r $ICU4X/icu4x-data
///   3. run datagen tool to export/dump data from providers to local FS
///         (https://unicode-org.github.io/icu4x-docs/doc/icu_provider_fs/index.html)
///     Note: Apologies for how long it takes to parse PPUCD (a few mins). We switched to
///           a better approach (more accurate, efficient, tied to upstream)
///           instead of fixing the inefficient iteration in the parser.
///   4. edit the path in the test below to $ICU4X/icu4x-data

#[test]
fn name() {
    let data_provider = FsDataProvider::try_new("/path/to/icu4x/icu4x-data").expect("Provider");
    // General_Category enumerated property data didn't parse by provider_ppucd
    // PPUCD parser, so cannot get the data for it.
    // let letter_set = props::get_general_category_val_set(&data_provider, enum_props::GeneralCategory::Letter)
    //         .expect("Didn't get set");
    let wspace_set = props::get_white_space_property(&data_provider)
            .expect("Didn't get set");
    println!("UnicodeSet for Whitespace = {:?}", wspace_set);
}