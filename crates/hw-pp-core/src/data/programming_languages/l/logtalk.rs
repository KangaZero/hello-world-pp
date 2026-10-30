//! Hello-world snippet for `ProgrammingLanguageName::Logtalk`.
//!
//! Imported from leachim6/hello-world (`l/Logtalk.lgt` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/l/Logtalk.lgt>
//! - <https://www.wikidata.org/wiki/Q3258085> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Logic, ObjectOriented, PrototypeBased,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Prolog, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Logtalk => "Logtalk",
    versions: [Logtalk => "Logtalk"],
    extensions: [Lgt => "lgt"],
}

const LOGTALK: &str = concat!(r"write('", output_for_hello_world!(), r"')");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Logtalk,
        extensions: &[FileExtension::Lgt],
        categories: &[Scripting],
        families: &[Prolog],
        paradigms: &[ObjectOriented, PrototypeBased, Logic],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Logtalk,
            syntax: LOGTALK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LOGTALK,
                    [
                        (TokenKind::Keyword, r"write"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r"'", output_for_hello_world!(), r"'"),
                        ),
                        (TokenKind::Punctuation, r")"),
                    ],
                )
            },
        }],
    };
