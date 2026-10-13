use crate::Args;
use hw_pp_core::data::common::remove_punctuation_from_hello_world;

use std::borrow::Cow;

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
