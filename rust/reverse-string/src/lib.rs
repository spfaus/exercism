// [dependencies]
// unicode-segmentation = "1.7.1"

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect()
}
