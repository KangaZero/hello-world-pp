//! Hello-world snippet for `ProgrammingLanguageName::Advpl`.
//!
//! Imported from leachim6/hello-world (`a/Advpl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Advpl>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageFamily::XBase,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Advpl => "Advpl",
    versions: [Advpl => "Advpl"],
    extensions: [],
}

const ADVPL: &str = concat!(
    r#"User Function Hello()
Local cMsg := ""#,
    output_for_hello_world!(),
    r#""
conout(cMsg)
MsgInfo(cMsg)
Return"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Advpl,
        extensions: &[],
        categories: &[],
        families: &[XBase],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Advpl,
            syntax: ADVPL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ADVPL,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"User Function Hello()
Local cMsg := ""#,
                            output_for_hello_world!(),
                            r#""
conout(cMsg)
MsgInfo(cMsg)
Return"#
                        ),
                    )],
                )
            },
        }],
    };
