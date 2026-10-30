//! Hello-world snippet for `ProgrammingLanguageName::Broccoli`.
//!
//! Imported from leachim6/hello-world (`b/Broccoli.brocc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/Broccoli.brocc>
//! - <https://esolangs.org/wiki/Broccoli> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::StackBased;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Broccoli => "Broccoli",
    versions: [Broccoli => "Broccoli"],
    extensions: [Brocc => "brocc"],
}

const BROCCOLI: &str = concat!(r#"(out ""#, output_for_hello_world!(), r#"" crlf)"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Broccoli,
        extensions: &[FileExtension::Brocc],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Broccoli,
            syntax: BROCCOLI,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BROCCOLI,
                    [(
                        TokenKind::Plain,
                        concat!(r#"(out ""#, output_for_hello_world!(), r#"" crlf)"#),
                    )],
                )
            },
        }],
    };
