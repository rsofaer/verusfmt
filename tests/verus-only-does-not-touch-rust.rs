#[test]
fn verus_only_extra_line() {
    let file = r"
// The newline below this comment is deleted with --verus-only.

use std::marker::PhantomData;

pub fn main(){ } // Formatting error here pops up without --verus-only
";
    let mut config = verusfmt::RunOptions::default();
    // Currently the only effect of the verus-only command line argument
    // is to set config.run_rustfmt to false, so use that.
    config.run_rustfmt = false;
    let formatted = verusfmt::run(file, config).unwrap();
    assert_eq!(formatted, file);

}