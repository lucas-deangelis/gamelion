use std::io;
use std::io::Read;

const MARKER: &str = " % ";

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");

    match extract_between_markers_remove_last_line(&input, MARKER) {
        Some(result) => println!("{}", result),
        None => println!("Could not find two markers in the input or could not remove last line"),
    }
}

fn extract_between_markers_remove_last_line(text: &str, marker: &str) -> Option<String> {
    let markers_found: Vec<usize> = text
        .match_indices(marker)
        .map(|(index, _)| index)
        .collect();

    if markers_found.len() < 2 {
        return None;
    }

    let second_last = markers_found[markers_found.len() - 2];
    let last = markers_found[markers_found.len() - 1];

    let start = second_last + marker.len();

    // Find the start of the line containing the last marker
    let last_line_start = text[..last].rfind('\n').map_or(0, |pos| pos + 1);
    let end = last_line_start;

    Some(text[start..end].to_string())
}
