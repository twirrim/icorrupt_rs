use rand::seq::IteratorRandom;
use std::env;
use std::sync::LazyLock;

// Use `once_cell` to create a lazily-initialized static vector of diacritics.
// This ensures the list is generated only once, the first time it's needed.
static DIACRITICS: LazyLock<Vec<char>> = LazyLock::new(|| {
    // The "Combining Diacritical Marks" Unicode block (U+0300 to U+036F).
    // `filter_map` is used because not all u32 values are valid chars,
    // although all values in this specific range are valid.
    (0x0300..=0x036F).filter_map(std::char::from_u32).collect()
});

/// Adds a specified number of random diacritics to a single character.
///
/// # Arguments
/// * `c` - The character to corrupt.
/// * `level` - The number of diacritics to add.
///
/// # Returns
/// A `String` containing the original character followed by the diacritics.
fn corrupt_letter(c: char, level: usize) -> String {
    // Avoid adding diacritics to whitespace for better readability.
    if c.is_whitespace() {
        return c.to_string();
    }

    let mut rng = rand::rng();

    // `choose_multiple` samples `level` items from the slice without replacement.
    // It returns an iterator of `&char`, which we can `collect` into a String.
    let diacritics_to_add: String = DIACRITICS
        .iter()
        .choose_multiple(&mut rng, level)
        .into_iter()
        .collect();

    // Combine the original character with the chosen diacritics.
    format!("{}{}", c, diacritics_to_add)
}

/// Applies a corruption effect to an entire string.
///
/// # Arguments
/// * `text` - The string slice to corrupt.
/// * `level` - The intensity of the corruption (diacritics per char).
///
/// # Returns
/// A new, corrupted `String`.
fn corrupt_text(text: &str, level: usize) -> String {
    text.chars().map(|c| corrupt_letter(c, level)).collect()
}

/// Main entry point for the script. Handles argument parsing and execution.
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        // Print usage information to stderr and exit.
        eprintln!("Usage: {} <text to corrupt> [corruption_level]", args[0]);
        eprintln!("Example: {} \"Hello World\" 10", args[0]);
        std::process::exit(1);
    }

    let mut level = 8; // Default corruption level
    let text_args: &[String];

    // Check if the last argument is a number to use as the corruption level.
    if let Some(last_arg) = args.last() {
        if let Ok(parsed_level) = last_arg.parse::<usize>() {
            level = parsed_level;
            // The text is all arguments except the program name and the level.
            text_args = &args[1..args.len() - 1];
        } else {
            // The last argument is not a number, so it's part of the text.
            text_args = &args[1..];
        }
    } else {
        // Should be unreachable due to the length check above, but good for safety.
        text_args = &args[1..];
    }

    if text_args.is_empty() {
        eprintln!("Error: No text provided.");
        eprintln!("If you provided a corruption level, you must also provide text.");
        std::process::exit(1);
    }

    // Join the text arguments with spaces to form the complete input string.
    let text_to_corrupt = text_args.join(" ");

    let corrupted_output = corrupt_text(&text_to_corrupt, level);
    println!("{}", corrupted_output);
}
