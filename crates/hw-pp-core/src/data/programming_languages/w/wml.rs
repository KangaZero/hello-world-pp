//! Hello-world snippet for `ProgrammingLanguageName::Wml`.
//!
//! Imported from leachim6/hello-world (`w/WML.wml` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/w/WML.wml>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Markup,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Wml => "WML",
    versions: [Wml => "WML"],
    extensions: [Wml => "wml"],
}

const WML: &str = concat!(
    r#"#define HELLOWORLD
  [message]
    speaker="narrator"
    message=_""#,
    output_for_hello_world!(),
    r#""
  [/message]
#enddef"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Wml,
        extensions: &[FileExtension::Wml],
        categories: &[Markup],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Wml,
            syntax: WML,
            tokenized_syntax: &const {
                checked_token_lengths(
                    WML,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"#define HELLOWORLD
  [message]
    speaker="narrator"
    message=_""#,
                            output_for_hello_world!(),
                            r#""
  [/message]
#enddef"#
                        ),
                    )],
                )
            },
        }],
    };
