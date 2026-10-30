//! Hello-world snippet for `ProgrammingLanguageName::Raku`.
//!
//! Imported from leachim6/hello-world (`r/Raku.raku` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/Raku.raku>
//! - <https://www.wikidata.org/wiki/Q2052676> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Imperative, ObjectOriented,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Perl, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Raku => "Raku",
    versions: [Raku => "Raku"],
    extensions: [Raku => "raku"],
}

const RAKU: &str = concat!(r#"say ""#, output_for_hello_world!(), r#"";"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Raku,
        extensions: &[FileExtension::Raku],
        categories: &[Scripting],
        families: &[Perl],
        paradigms: &[Imperative, ObjectOriented, Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Raku,
            syntax: RAKU,
            tokenized_syntax: &const {
                checked_token_lengths(
                    RAKU,
                    [
                        (TokenKind::Keyword, r"say"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r";"),
                    ],
                )
            },
        }],
    };
