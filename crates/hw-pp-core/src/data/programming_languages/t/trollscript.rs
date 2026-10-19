//! Hello-world snippet for `ProgrammingLanguageName::Trollscript`.
//!
//! Imported from leachim6/hello-world (`t/TrollScript.troll` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Trollscript => "TrollScript",
    versions: [Trollscript => "TrollScript"],
    extensions: [Troll => "troll"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Trollscript,
        extensions: &[FileExtension::Troll],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Trollscript,
            syntax: r"Trooloolooloolooloolooloolooloolollooooolooloolooloolooloolooooolooloolooloolooloolooloolooloooooloolooloooooloooloolooloololllllooooloololoooooololooolooloolooloolooloololoolooolooloololooooooloololooooloololooloolooloolooloolooloolooloolooloolooloololooooolooolooloololooollollollollollolllooollollollollollollollollloooooololooooolooll.
",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"Trooloolooloolooloolooloolooloolollooooolooloolooloolooloolooooolooloolooloolooloolooloolooloooooloolooloooooloooloolooloololllllooooloololoooooololooolooloolooloolooloololoolooolooloololooooooloololooooloololooloolooloolooloolooloolooloolooloolooloololooooolooolooloololooollollollollollolllooollollollollollollollollloooooololooooolooll.
",
            )],
        }],
    };
