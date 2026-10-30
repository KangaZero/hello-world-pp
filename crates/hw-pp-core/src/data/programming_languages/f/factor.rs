//! Hello-world snippet for `ProgrammingLanguageName::Factor`.
//!
//! Imported from leachim6/hello-world (`f/Factor.factor` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/Factor.factor>
//! - <https://www.wikidata.org/wiki/Q1391724> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Concatenative, Functional, StackBased,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Factor => "Factor",
    versions: [Factor => "Factor"],
    extensions: [Factor => "factor"],
}

const FACTOR: &str = concat!(
    r#"USING: io ;
""#,
    output_for_hello_world!(),
    r#"" print
"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Factor,
        extensions: &[FileExtension::Factor],
        categories: &[Scripting],
        families: &[],
        paradigms: &[Functional, StackBased, Concatenative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Factor,
            syntax: FACTOR,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FACTOR,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"USING: io ;
""#,
                            output_for_hello_world!(),
                            r#"" print
"#
                        ),
                    )],
                )
            },
        }],
    };
