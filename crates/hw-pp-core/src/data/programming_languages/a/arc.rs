//! Hello-world snippet for `ProgrammingLanguageName::Arc`.
//!
//! Imported from leachim6/hello-world (`a/ARC.arc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/ARC.arc>
//! - <https://www.wikidata.org/wiki/Q2438046> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Metaprogramming, Procedural,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Lisp, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Arc => "ARC",
    versions: [Arc => "ARC"],
    extensions: [Arc => "arc"],
}

const ARC: &str = concat!(r#"(prn ""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Arc,
        extensions: &[FileExtension::Arc],
        categories: &[Scripting],
        families: &[Lisp],
        paradigms: &[Procedural, Functional, Metaprogramming],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Arc,
            syntax: ARC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ARC,
                    [
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Keyword, r"prn"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r")"),
                    ],
                )
            },
        }],
    };
