//! Hello-world snippet for `ProgrammingLanguageName::Poketlang`.
//!
//! Imported from leachim6/hello-world (`p/Poketlang.pk` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Poketlang.pk>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Poketlang => "Poketlang",
    versions: [Poketlang => "Poketlang"],
    extensions: [Pk => "pk"],
}

const POKETLANG: &str = concat!(r#"print(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Poketlang,
        extensions: &[FileExtension::Pk],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Poketlang,
            syntax: POKETLANG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    POKETLANG,
                    [(
                        TokenKind::Plain,
                        concat!(r#"print(""#, output_for_hello_world!(), r#"")"#),
                    )],
                )
            },
        }],
    };
