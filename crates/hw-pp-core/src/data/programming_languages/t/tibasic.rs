//! Hello-world snippet for `ProgrammingLanguageName::Tibasic`.
//!
//! Imported from leachim6/hello-world (`t/TIBasic.ti` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/TIBasic.ti>
//! - <https://en.wikipedia.org/wiki/TI-BASIC_83> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Procedural;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Basic, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Tibasic => "TIBasic",
    versions: [Tibasic => "TIBasic"],
    extensions: [Ti => "ti"],
}

const TIBASIC: &str = concat!(
    r#"PROGRAM:HELLOW
:ClrHome
:Disp ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Tibasic,
        extensions: &[FileExtension::Ti],
        categories: &[Scripting],
        families: &[Basic],
        paradigms: &[Procedural],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Tibasic,
            syntax: TIBASIC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TIBASIC,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"PROGRAM:HELLOW
:ClrHome
:Disp ""#,
                            output_for_hello_world!(),
                            r#"""#
                        ),
                    )],
                )
            },
        }],
    };
