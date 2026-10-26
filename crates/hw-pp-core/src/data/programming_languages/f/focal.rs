//! Hello-world snippet for `ProgrammingLanguageName::Focal`.
//!
//! Imported from leachim6/hello-world (`f/Focal.fc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/Focal.fc>
//! - <https://en.wikipedia.org/wiki/FOCAL_(programming_language)> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Imperative;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Historical,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Joss, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Focal => "Focal",
    versions: [Focal => "Focal"],
    extensions: [Fc => "fc"],
}

const FOCAL: &str = concat!(r#"0.1.0.1 TYPE ""#, output_for_hello_world!(), r#"" , !"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Focal,
        extensions: &[FileExtension::Fc],
        categories: &[Historical],
        families: &[Joss],
        paradigms: &[Imperative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Focal,
            syntax: FOCAL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FOCAL,
                    [(
                        TokenKind::Plain,
                        concat!(r#"0.1.0.1 TYPE ""#, output_for_hello_world!(), r#"" , !"#),
                    )],
                )
            },
        }],
    };
