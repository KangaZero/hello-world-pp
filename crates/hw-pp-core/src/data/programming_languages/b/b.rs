//! Hello-world snippet for `ProgrammingLanguageName::B`.
//!
//! Imported from leachim6/hello-world (`b/B.b` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: B => "B",
    versions: [B => "B"],
    extensions: [B => "b"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::B,
        extensions: &[FileExtension::B],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::B,
            syntax: r"main() {
    putchar ('Hell'); putchar ('o Wo'); putchar ('rld'); putchar ('*n');
}",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"main() {
    putchar ('Hell'); putchar ('o Wo'); putchar ('rld'); putchar ('*n');
}",
            )],
        }],
    };
