//! Hello-world snippet for `ProgrammingLanguageName::Listlang`.
//!
//! Imported from leachim6/hello-world (`l/ListLang` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Listlang => "ListLang",
    versions: [Listlang => "ListLang"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Listlang,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Listlang,
            syntax: r#"0dlroW391+*2+491+*4+olleH["],"#,
            tokenized_syntax: &[(TokenKind::Plain, r#"0dlroW391+*2+491+*4+olleH["],"#)],
        }],
    };
