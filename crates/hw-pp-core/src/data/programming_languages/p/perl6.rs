//! Hello-world snippet for `ProgrammingLanguageName::Perl6`.
//!
//! Imported from leachim6/hello-world (`p/Perl6.p6` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Perl6.p6>
//! - <https://www.wikidata.org/wiki/Q2052676> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Imperative, ObjectOriented,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Perl6 => "Perl6",
    versions: [Perl6 => "Perl6"],
    extensions: [P6 => "p6"],
}

const PERL6: &str = concat!(
    r#"#!/usr/bin/env perl6

v6;

say ""#,
    output_for_hello_world!(),
    r#"";
"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Perl6,
        extensions: &[FileExtension::P6],
        categories: &[Scripting],
        families: &[],
        paradigms: &[Imperative, ObjectOriented, Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Perl6,
            syntax: PERL6,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PERL6,
                    [
                        (TokenKind::Comment, r"#!/usr/bin/env perl6"),
                        (
                            TokenKind::Plain,
                            r"

v6",
                        ),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"say"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                    ],
                )
            },
        }],
    };
