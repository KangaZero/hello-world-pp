//! Hello-world snippet for `ProgrammingLanguageName::Powerbuilder`.
//!
//! Imported from leachim6/hello-world (`p/PowerBuilder.psr` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/PowerBuilder.psr>
//! - <https://en.wikipedia.org/wiki/PowerBuilder> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::ObjectOriented;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Powerbuilder => "PowerBuilder",
    versions: [Powerbuilder => "PowerBuilder"],
    extensions: [Psr => "psr"],
}

const POWERBUILDER: &str = concat!(r#"MessageBox(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Powerbuilder,
        extensions: &[FileExtension::Psr],
        categories: &[],
        families: &[],
        paradigms: &[ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Powerbuilder,
            syntax: POWERBUILDER,
            tokenized_syntax: &const {
                checked_token_lengths(
                    POWERBUILDER,
                    [(
                        TokenKind::Plain,
                        concat!(r#"MessageBox(""#, output_for_hello_world!(), r#"")"#),
                    )],
                )
            },
        }],
    };
