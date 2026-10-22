use crate::Args;
use hw_pp_core::data::{
    common::{
        get_default_human_language_hello_world, remove_punctuation_from_str,
        replace_default_hello_world_string_with,
    },
    notation::Notation,
};

use std::borrow::Cow;

/// Applies `args`' output options to `output`, borrowing it when nothing changes.
///
/// - `args`: parsed CLI options; only `no_punctuation` is read.
/// - `output`: the greeting to print.
///
/// With `-n`, `"¡Hola, Mundo!"` becomes `"Hola Mundo"`.
#[must_use]
pub fn hello_world_output_with_options(args: &Args, output: &str) -> String {
    let result = if args.no_punctuation {
        Cow::Owned(remove_punctuation_from_str(output))
    } else {
        Cow::Borrowed(output)
    };

    Notation::render(args.notation, &result)
}

/// Applies `args`' output options to a programming-language `snippet`.
///
/// With `-n`, only the hello-world message loses its punctuation, so the code
/// stays valid: `println!("Hello, World!");` becomes `println!("Hello World");`,
/// and a reversed message (Befunge-93's `"!dlroW ,olleH"`) becomes `"dlroW olleH"`.
/// Snippets that encode the message (e.g. Brainfuck) are returned unchanged.
#[must_use]
pub fn snippet_output_with_options(args: &Args, snippet: String) -> String {
    if !args.no_punctuation {
        return snippet;
    }
    let default_hello_world = get_default_human_language_hello_world();
    replace_default_hello_world_string_with(
        &snippet,
        &remove_punctuation_from_str(default_hello_world),
    )
}

/// Prints `stringified_list` space-separated when `compact`, otherwise one per line. adds count:
/// {count} at the end as a new line when `count`
pub fn output_list_command_with(stringified_list: &[&str], compact: bool, count: bool) {
    let separator = if compact { " " } else { "\n" };
    println!("{}", stringified_list.join(separator));
    if count {
        println!("Count: {}", stringified_list.len());
    }
}
