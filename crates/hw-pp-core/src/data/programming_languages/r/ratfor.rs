//! Hello-world snippet for `ProgrammingLanguageName::Ratfor`.
//!
//! Imported from leachim6/hello-world (`r/RatFor.ratfor` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Ratfor => "RatFor",
    versions: [Ratfor => "RatFor"],
    extensions: [Ratfor => "ratfor"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ratfor,
        extensions: &[FileExtension::Ratfor],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ratfor,
            syntax: concat!(
                r"print *, '",
                output_for_hello_world!(),
                r"'
end"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"print *, '",
                    output_for_hello_world!(),
                    r"'
end"
                ),
            )],
        }],
    };
