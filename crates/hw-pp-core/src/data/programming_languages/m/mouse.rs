//! Hello-world snippet for `ProgrammingLanguageName::Mouse`.
//!
//! Imported from leachim6/hello-world (`m/Mouse.mse` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Mouse.mse>
//! - <https://www.wikidata.org/wiki/Q17148785> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::StackBased;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Mouse => "Mouse",
    versions: [Mouse => "Mouse"],
    extensions: [Mse => "mse"],
}

const MOUSE: &str = concat!(
    r#"""#,
    output_for_hello_world!(),
    r#""
'! !'
$"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mouse,
        extensions: &[FileExtension::Mse],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mouse,
            syntax: MOUSE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MOUSE,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"""#,
                            output_for_hello_world!(),
                            r#""
'! !'
$"#
                        ),
                    )],
                )
            },
        }],
    };
