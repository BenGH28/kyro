use std::{char, path::Path, process::Command};

use crate::Command;

pub fn get_mono_font_name() -> String {
    let output = Command::new("fc-match")
        .arg("monospace")
        .output()
        .expect("failed to execute fc-match");
    let stdout: Vec<String> = output
        .stdout
        .iter()
        .map(|u| String::from(*u as char))
        .collect();

    let joined_output = stdout.join(" ");

    let removed_spaces = joined_output.replace(' ', "");
    let split_on_colon: Vec<&str> = removed_spaces.split(':').collect();

    split_on_colon[0].to_string()
}

pub fn find_ttf_path(font_name: String) -> Path {
    let output = Command::new("find")
        .arg(&font_name)
        .output()
        .expect("failed to execute `find`");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_mono_font_name_test() {
        let font_name = get_mono_font_name();
        let expected = "NotoSansMono-Regular.ttf".to_string();
        assert_eq!(expected, font_name);
    }
}
