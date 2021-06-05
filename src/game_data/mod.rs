pub mod aoe2dat;
pub mod key_value;
pub mod tech_tree;

fn short_help_text_offset(idx: i32) -> i32 {
    idx - 99000
}

fn help_text_offset(idx: i32) -> i32 {
    idx - 79000
}
