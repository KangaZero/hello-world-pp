//! Hello-world snippet for `ProgrammingLanguageName::Sml`.
//!
//! Imported from leachim6/hello-world (`s/SML.sml` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/SML.sml>
//! - <https://www.wikidata.org/wiki/Q597330> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Functional, Imperative, Procedural};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Sml => "SML",
    versions: [Sml => "SML"],
    extensions: [Sml => "sml"],
}

const SML: &str = concat!(r#"val () = print ""#, output_for_hello_world!(), r#"\n""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sml,
        extensions: &[FileExtension::Sml],
        categories: &[Compiled],
        families: &[],
        paradigms: &[Imperative, Procedural, Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sml,
            syntax: SML,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SML,
                    [
                        (TokenKind::Keyword, r"val"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"()"),
                        (TokenKind::Plain, r" = "),
                        (TokenKind::Keyword, r"print"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"\n""#),
                        ),
                    ],
                )
            },
        }],
    };
