//! Hello-world snippet for `ProgrammingLanguageName::Unicomal`.
//!
//! Imported from leachim6/hello-world (`u/Unicomal.lst` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/u/Unicomal.lst>
//! - <https://en.wikipedia.org/wiki/COMAL> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Structured;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Historical,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Unicomal => "Unicomal",
    versions: [Unicomal => "Unicomal"],
    extensions: [Lst => "lst"],
}

const UNICOMAL: &str = concat!(r#"0100 PRINT ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Unicomal,
        extensions: &[FileExtension::Lst],
        categories: &[Historical],
        families: &[],
        paradigms: &[Structured],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Unicomal,
            syntax: UNICOMAL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    UNICOMAL,
                    [(
                        TokenKind::Plain,
                        concat!(r#"0100 PRINT ""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };
