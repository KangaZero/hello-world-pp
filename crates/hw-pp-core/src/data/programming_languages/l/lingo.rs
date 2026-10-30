//! Hello-world snippet for `ProgrammingLanguageName::Lingo`.
//!
//! Imported from leachim6/hello-world (`l/Lingo.lg4` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/l/Lingo.lg4>
//! - <https://www.wikidata.org/wiki/Q664867> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::ObjectOriented;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Lingo => "Lingo",
    versions: [Lingo => "Lingo"],
    extensions: [Lg4 => "lg4"],
}

const LINGO: &str = concat!(
    r#"on startmovie
  alert ""#,
    output_for_hello_world!(),
    r#""
end"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lingo,
        extensions: &[FileExtension::Lg4],
        categories: &[Scripting],
        families: &[],
        paradigms: &[ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lingo,
            syntax: LINGO,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LINGO,
                    [
                        (TokenKind::Keyword, r"on"),
                        (
                            TokenKind::Plain,
                            r" startmovie
  alert ",
                        ),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"end"),
                    ],
                )
            },
        }],
    };
