//! Hello-world snippets for `ProgrammingLanguageName::Turing`.
//!
//! Imported from leachim6/hello-world (`t/Turing.oot`, `t/Turing.t` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/Turing.oot>
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/Turing.t>
//! - <https://www.wikidata.org/wiki/Q490481> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{ObjectOriented, Procedural};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Educational,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Algol, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Turing => "Turing",
    versions: [TuringOot => "Turing (.oot)", TuringT => "Turing (.t)"],
    extensions: [Oot => "oot"],
}

const TURING_OOT: &str = concat!(r#"put ""#, output_for_hello_world!(), r#"""#);
const TURING_T: &str = concat!(r#"put ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Turing,
        extensions: &[FileExtension::Oot],
        categories: &[Educational],
        families: &[Algol],
        paradigms: &[Procedural, ObjectOriented],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::TuringOot,
                syntax: TURING_OOT,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        TURING_OOT,
                        [
                            (TokenKind::Keyword, r"put"),
                            (TokenKind::Plain, r" "),
                            (
                                TokenKind::String,
                                concat!(r#"""#, output_for_hello_world!(), r#"""#),
                            ),
                        ],
                    )
                },
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::TuringT,
                syntax: TURING_T,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        TURING_T,
                        [
                            (TokenKind::Keyword, r"put"),
                            (TokenKind::Plain, r" "),
                            (
                                TokenKind::String,
                                concat!(r#"""#, output_for_hello_world!(), r#"""#),
                            ),
                        ],
                    )
                },
            },
        ],
    };
