//! Hello-world snippet for `ProgrammingLanguageName::Gammaplex`.
//!
//! Imported from leachim6/hello-world (`g/Gammaplex.gammaplex` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/g/Gammaplex.gammaplex>
//! - <https://esolangs.org/wiki/Gammaplex> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Reflective;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Gammaplex => "Gammaplex",
    versions: [Gammaplex => "Gammaplex"],
    extensions: [Gammaplex => "gammaplex"],
}

const GAMMAPLEX: &str = concat!(r#"X""#, output_for_hello_world!(), r#""XXSXrRE"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gammaplex,
        extensions: &[FileExtension::Gammaplex],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Reflective],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gammaplex,
            syntax: GAMMAPLEX,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GAMMAPLEX,
                    [(
                        TokenKind::Plain,
                        concat!(r#"X""#, output_for_hello_world!(), r#""XXSXrRE"#),
                    )],
                )
            },
        }],
    };
