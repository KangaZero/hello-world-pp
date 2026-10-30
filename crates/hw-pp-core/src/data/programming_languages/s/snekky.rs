//! Hello-world snippet for `ProgrammingLanguageName::Snekky`.
//!
//! Imported from leachim6/hello-world (`s/Snekky.snek` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/Snekky.snek>
//! - <https://github.com/snekkylang/snekky> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Imperative;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Snekky => "Snekky",
    versions: [Snekky => "Snekky"],
    extensions: [Snek => "snek"],
}

const SNEKKY: &str = concat!(r#"Sys.println(""#, output_for_hello_world!(), r#"");"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Snekky,
        extensions: &[FileExtension::Snek],
        categories: &[],
        families: &[],
        paradigms: &[Imperative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Snekky,
            syntax: SNEKKY,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SNEKKY,
                    [(
                        TokenKind::Plain,
                        concat!(r#"Sys.println(""#, output_for_hello_world!(), r#"");"#),
                    )],
                )
            },
        }],
    };
