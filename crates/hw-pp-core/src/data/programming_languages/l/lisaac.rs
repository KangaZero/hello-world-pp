//! Hello-world snippet for `ProgrammingLanguageName::Lisaac`.
//!
//! Imported from leachim6/hello-world (`l/Lisaac.li` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/l/Lisaac.li>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Compiled, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Lisaac => "Lisaac",
    versions: [Lisaac => "Lisaac"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lisaac,
        extensions: &[],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lisaac,
            syntax: concat!(
                r"// ",
                output_for_hello_world!(),
                r#" in Lisaac

Section Header
	+ name		:= HELLO_WORLD;

Section Inherit

	- parent_object:OBJECT := OBJECT;

Section Public

	- main <-
	(
		""#,
                output_for_hello_world!(),
                r#" !\n".print;
	);"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"// ",
                    output_for_hello_world!(),
                    r#" in Lisaac

Section Header
	+ name		:= HELLO_WORLD;

Section Inherit

	- parent_object:OBJECT := OBJECT;

Section Public

	- main <-
	(
		""#,
                    output_for_hello_world!(),
                    r#" !\n".print;
	);"#
                ),
            )],
        }],
    };
