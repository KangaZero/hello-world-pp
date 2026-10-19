//! Hello-world snippet for `ProgrammingLanguageName::AmigaE`.
//!
//! Imported from leachim6/hello-world (`a/Amiga-E.amiga-e` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: AmigaE => "Amiga-E",
    versions: [AmigaE => "Amiga-E"],
    extensions: [AmigaE => "amiga-e"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AmigaE,
        extensions: &[FileExtension::AmigaE],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AmigaE,
            syntax: concat!(
                r"PROC main() IS WriteF('",
                output_for_hello_world!(),
                r"\n')"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"PROC main() IS WriteF('",
                    output_for_hello_world!(),
                    r"\n')"
                ),
            )],
        }],
    };
