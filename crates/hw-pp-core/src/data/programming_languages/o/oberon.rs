//! Hello-world snippet for `ProgrammingLanguageName::Oberon`.
//!
//! Imported from leachim6/hello-world (`o/Oberon.o` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/o/Oberon.o>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Oberon => "Oberon",
    versions: [Oberon => "Oberon"],
    extensions: [O => "o"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Oberon,
        extensions: &[FileExtension::O],
        categories: &[Compiled, Historical],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Oberon,
            syntax: concat!(
                r"MODULE HelloWorld;

IMPORT Out;

BEGIN
   Out.Open;
   Out.String('",
                output_for_hello_world!(),
                r"');
END HelloWorld."
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"MODULE HelloWorld;

IMPORT Out;

BEGIN
   Out.Open;
   Out.String('",
                    output_for_hello_world!(),
                    r"');
END HelloWorld."
                ),
            )],
        }],
    };
