//! Hello-world snippet for `ProgrammingLanguageName::Lotusscript`.
//!
//! Imported from leachim6/hello-world (`l/LotusScript.lss` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/l/LotusScript.lss>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Basic, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Lotusscript => "LotusScript",
    versions: [Lotusscript => "LotusScript"],
    extensions: [Lss => "lss"],
}

const LOTUSSCRIPT: &str = concat!(
    r#"Sub Initialize
	Print ""#,
    output_for_hello_world!(),
    r#""
End Sub"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lotusscript,
        extensions: &[FileExtension::Lss],
        categories: &[Scripting],
        families: &[Basic],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lotusscript,
            syntax: LOTUSSCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LOTUSSCRIPT,
                    [
                        (TokenKind::Keyword, r"Sub"),
                        (
                            TokenKind::Plain,
                            r" Initialize
	",
                        ),
                        (TokenKind::Keyword, r"Print"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"End"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"Sub"),
                    ],
                )
            },
        }],
    };
