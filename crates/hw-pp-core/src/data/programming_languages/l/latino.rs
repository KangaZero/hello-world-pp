//! Hello-world snippet for `ProgrammingLanguageName::Latino`.
//!
//! Imported from leachim6/hello-world (`l/Latino.lat` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/l/Latino.lat>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Latino => "Latino",
    versions: [Latino => "Latino"],
    extensions: [],
}

const LATINO: &str = concat!(r#"escribir(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Latino,
        extensions: &[],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Latino,
            syntax: LATINO,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LATINO,
                    [(
                        TokenKind::Plain,
                        concat!(r#"escribir(""#, output_for_hello_world!(), r#"")"#),
                    )],
                )
            },
        }],
    };
