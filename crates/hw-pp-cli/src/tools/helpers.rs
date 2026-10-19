use crate::Args;
use hw_pp_core::data::common::{
    get_default_human_language_hello_world, remove_punctuation_from_hello_world,
    replace_default_hello_world_string_with,
};

use std::borrow::Cow;

/// Applies `args`' output options to `output`, borrowing it when nothing changes.
///
/// - `args`: parsed CLI options; only `no_punctuation` is read.
/// - `output`: the greeting to print.
///
/// With `-n`, `"¡Hola, Mundo!"` becomes `"Hola Mundo"`.
#[must_use]
pub fn hello_world_output_with_options<'output>(
    args: &Args,
    output: &'output str,
) -> Cow<'output, str> {
    if args.no_punctuation {
        Cow::Owned(remove_punctuation_from_hello_world(output))
    } else {
        Cow::Borrowed(output)
    }
}

/// Applies `args`' output options to a programming-language `snippet`.
///
/// With `-n`, only the hello-world message loses its punctuation, so the code
/// stays valid: `println!("Hello, World!");` becomes `println!("Hello World");`.
/// Snippets that encode the message (Brainfuck, Befunge) are returned unchanged.
#[must_use]
pub fn snippet_output_with_options(args: &Args, snippet: String) -> String {
    if !args.no_punctuation {
        return snippet;
    }
    let message = get_default_human_language_hello_world();
    replace_default_hello_world_string_with(&snippet, &remove_punctuation_from_hello_world(message))
        .unwrap_or(snippet)
}

/// Prints `stringified_list` space-separated when `compact`, otherwise one per line.
pub fn output_as_compact_list_or_not(stringified_list: &[&str], compact: bool) {
    let separator = if compact { " " } else { "\n" };
    println!("{}", stringified_list.join(separator));
}
