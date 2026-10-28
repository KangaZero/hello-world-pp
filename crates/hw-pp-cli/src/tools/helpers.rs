//! Applies the CLI's output options to greetings, snippets and lists.

use crate::Args;
use hw_pp_core::data::common::{
    remove_punctuation_from_str, replace_default_hello_world_string_with,
};
use hw_pp_core::data::human_languages::Greeting;

use std::borrow::Cow;

use anstream::eprintln;

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
/// Notations that emit quotes or backslashes (e.g. `--notation rust-literal`) are inserted
/// verbatim, so they can break or change the meaning of the string literal.
#[must_use]
pub fn snippet_output_with_default_hello_world_with_options(args: &Args, snippet: &str) -> String {
    let default_hello_world_with_options =
        hello_world_output_with_options(args, Greeting::default_text());
    replace_default_hello_world_string_with(snippet, &default_hello_world_with_options)
}

fn replace_all_except_the_match_before_until(s: &str, from: &str, to: &str, until: &str) -> String {
    match (s.rfind(from), s.find(until)) {
        (Some(_), Some(i)) => {
            format!("{}{}", s[..i - 1].replace(from, to), &s[i - 1..])
        }
        _ => s.replace(from, to),
    }
}

/// Prints `stringified_list` space-separated when `compact`, otherwise one per line.
///
/// If compact, Spaces inside a name become `-` (`"Catppuccin Latte"` → `Catppuccin-Latte`).
/// (Excludes  adding "-" in spaces for endonym)
/// With `count`, prints a final `Count: N` line.
pub fn output_list_command_with<T: AsRef<str>>(
    stringified_list: &[T],
    compact: bool,
    count: bool,
    endonym: bool,
) {
    let separator = if compact { " " } else { "\n" };
    println!(
        "{}",
        stringified_list
            .iter()
            .map(|item| match (compact, endonym) {
                (true, true) =>
                    replace_all_except_the_match_before_until(item.as_ref(), " ", "-", "(")
                        .replace(')', "),"),
                (true, false) => item.as_ref().replace(' ', "-"),
                _ => item.as_ref().to_string(),
            })
            .collect::<Vec<_>>()
            .join(separator)
    );
    if count {
        println!("Count: {}", stringified_list.len());
    }
}

/// Prints `message` to stderr after a `warning:` label in `args`' theme color.
///
/// `anstream` drops the color when stderr is not a terminal or `NO_COLOR` is set.
pub fn warn(args: &Args, message: &str) {
    let style = args.theme.colors().warning;
    eprintln!("{style}warning:{style:#} {message}");
}
