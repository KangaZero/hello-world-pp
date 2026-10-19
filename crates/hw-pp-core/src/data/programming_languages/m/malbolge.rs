//! Hello-world snippet for `ProgrammingLanguageName::Malbolge`.
//!
//! Imported from leachim6/hello-world (`m/Malbolge.mb` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Malbolge => "Malbolge",
    versions: [Malbolge => "Malbolge"],
    extensions: [Mb => "mb"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Malbolge,
        extensions: &[FileExtension::Mb],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Malbolge,
            syntax: r#"(=<`#9]~6ZY32Vx/4Rs+0No-&Jk)"Fh}|Bcy?,vNz]KZ%oG4UUS0/@-eMc(:'8"#,
            tokenized_syntax: &[(
                TokenKind::Plain,
                r#"(=<`#9]~6ZY32Vx/4Rs+0No-&Jk)"Fh}|Bcy?,vNz]KZ%oG4UUS0/@-eMc(:'8"#,
            )],
        }],
    };
