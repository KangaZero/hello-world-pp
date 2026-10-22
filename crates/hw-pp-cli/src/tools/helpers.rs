//! Applies the CLI's output options to greetings, snippets and lists.

use crate::Args;
use hw_pp_core::data::common::{
    get_default_human_language_hello_world, remove_punctuation_from_str,
    replace_default_hello_world_string_with,
};

use std::borrow::Cow;

/// Applies `args`' output options to `output`: `--no-punctuation`, then `--notation`.
///
/// - `args`: parsed CLI options; `no_punctuation` and `notation` are read.
/// - `output`: the greeting to print.
///
/// With `--no-punctuation`, `"¡Hola, Mundo!"` → `"Hola Mundo"`.
/// With `--notation base64`, `"Hello, World!"` → `"SGVsbG8sIFdvcmxkIQ=="`.
#[must_use]
pub fn hello_world_output_with_options(args: &Args, output: &str) -> String {
    let result = if args.no_punctuation {
        Cow::Owned(remove_punctuation_from_str(output))
    } else {
        Cow::Borrowed(output)
    };

    args.notation.render(&result)
}

/// Applies `args`' output options to a programming-language `snippet`.
///
/// The default greeting is run through [`hello_world_output_with_options`]
/// (`--no-punctuation`, then `--notation`) and replaces the hello-world message in
/// the code; the rest of the code is untouched. With `--no-punctuation`,
/// `println!("Hello, World!");` → `println!("Hello World");`; with
/// `--notation base64` → `println!("SGVsbG8sIFdvcmxkIQ==");`. A reversed message
/// (Befunge-93's `"!dlroW ,olleH"`) gets the processed greeting reversed
/// (`--no-punctuation` → `"dlroW olleH"`).
/// Snippets that encode the message (e.g. Brainfuck) are returned unchanged.
///
/// Notations that emit quotes or backslashes (e.g. `"Rust-Literal"`) are inserted
/// verbatim, so they can break or change the meaning of the string literal.
#[must_use]
pub fn snippet_output_with_default_hello_world_with_options(args: &Args, snippet: &str) -> String {
    let default_hello_world_with_options =
        hello_world_output_with_options(args, get_default_human_language_hello_world());
    replace_default_hello_world_string_with(snippet, &default_hello_world_with_options)
}

/// Prints `stringified_list` space-separated when `compact`, otherwise one per line.
///
/// With `count`, prints a final `Count: N` line.
pub fn output_list_command_with(stringified_list: &[&str], compact: bool, count: bool) {
    let separator = if compact { " " } else { "\n" };
    println!(
        "{}",
        stringified_list
            .iter()
            .map(|item| item.replace(' ', "-"))
            .collect::<Vec<_>>()
            .join(separator)
    );
    if count {
        println!("Count: {}", stringified_list.len());
    }
}
