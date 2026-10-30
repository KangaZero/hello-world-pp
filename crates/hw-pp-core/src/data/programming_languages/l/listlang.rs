//! Hello-world snippet for `ProgrammingLanguageName::Listlang`.
//!
//! Imported from leachim6/hello-world (`l/ListLang` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/l/ListLang>
//! - <https://esolangs.org/wiki/ListLang> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Imperative, StackBased};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Listlang => "ListLang",
    versions: [Listlang => "ListLang"],
    extensions: [],
}

const LISTLANG: &str = r#"0dlroW391+*2+491+*4+olleH["],"#;

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Listlang,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Imperative, StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Listlang,
            syntax: LISTLANG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LISTLANG,
                    [(TokenKind::Plain, r#"0dlroW391+*2+491+*4+olleH["],"#)],
                )
            },
        }],
    };
