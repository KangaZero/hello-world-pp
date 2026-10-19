use crate::Args;
use hw_pp_core::data::common::remove_punctuation_from_hello_world;

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

/// Prints `stringified_list` space-separated when `compact`, otherwise one per line.
pub fn output_as_compact_list_or_not(stringified_list: &[&str], compact: bool) {
    let separator = if compact { " " } else { "\n" };
    println!("{}", stringified_list.join(separator));
}
