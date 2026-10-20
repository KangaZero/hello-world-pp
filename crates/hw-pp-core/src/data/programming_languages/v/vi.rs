//! Hello-world snippet for `ProgrammingLanguageName::Vi`.
//!
//! Imported from leachim6/hello-world (`v/Vi` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/v/Vi>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Vi => "Vi",
    versions: [Vi => "Vi"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Vi,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Vi,
            syntax: concat!(
                r#"The following tab indented lines will cause a true vi with modelines
activated to infinitely loop putting ""#,
                output_for_hello_world!(),
                r#"" in the buffer. Hit
 to abort the loop and see the output. None of the vi clones
support modelines this powerful, and modelines are disabled by default.
Set the environment variable EXINIT to "set ml" to activate modelines.

	vi: $  y a :
	vi: $-1y b :
	vi: @b :
	put a |@b
	"#,
                output_for_hello_world!(),
                r#"

Whitespace is largely insignificant, but these must be the last five
lines in the file to work properly. Unless it is in "vi: ... :" or
"ex: ... :" format, any preceeding text will be ignored."#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"The following tab indented lines will cause a true vi with modelines
activated to infinitely loop putting ""#,
                    output_for_hello_world!(),
                    r#"" in the buffer. Hit
 to abort the loop and see the output. None of the vi clones
support modelines this powerful, and modelines are disabled by default.
Set the environment variable EXINIT to "set ml" to activate modelines.

	vi: $  y a :
	vi: $-1y b :
	vi: @b :
	put a |@b
	"#,
                    output_for_hello_world!(),
                    r#"

Whitespace is largely insignificant, but these must be the last five
lines in the file to work properly. Unless it is in "vi: ... :" or
"ex: ... :" format, any preceeding text will be ignored."#
                ),
            )],
        }],
    };
