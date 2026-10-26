//! Hello-world snippet for `ProgrammingLanguageName::Ratfor`.
//!
//! Imported from leachim6/hello-world (`r/RatFor.ratfor` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/RatFor.ratfor>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Historical,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Fortran, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Ratfor => "RatFor",
    versions: [Ratfor => "RatFor"],
    extensions: [Ratfor => "ratfor"],
}

const RATFOR: &str = concat!(
    r"print *, '",
    output_for_hello_world!(),
    r"'
end"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ratfor,
        extensions: &[FileExtension::Ratfor],
        categories: &[Historical],
        families: &[Fortran],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ratfor,
            syntax: RATFOR,
            tokenized_syntax: &const {
                checked_token_lengths(
                    RATFOR,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"print *, '",
                            output_for_hello_world!(),
                            r"'
end"
                        ),
                    )],
                )
            },
        }],
    };
