//! Hello-world snippet for `ProgrammingLanguageName::Aspectj`.
//!
//! Imported from leachim6/hello-world (`a/Aspectj.aj` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Aspectj.aj>
//! - <https://www.wikidata.org/wiki/Q735604> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::AspectOriented;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Java, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Aspectj => "Aspectj",
    versions: [Aspectj => "Aspectj"],
    extensions: [Aj => "aj"],
}

const ASPECTJ: &str = concat!(
    r#"System.out.println(""#,
    output_for_hello_world!(),
    r#"");"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Aspectj,
        extensions: &[FileExtension::Aj],
        categories: &[Compiled],
        families: &[Java],
        paradigms: &[AspectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Aspectj,
            syntax: ASPECTJ,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASPECTJ,
                    [
                        (TokenKind::Plain, r"System"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Keyword, r"out"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"println"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r");"),
                    ],
                )
            },
        }],
    };
