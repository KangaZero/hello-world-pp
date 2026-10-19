//! Hello-world snippet for `ProgrammingLanguageName::Wurstscript`.
//!
//! Imported from leachim6/hello-world (`w/WurstScript.wurst` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/w/WurstScript.wurst>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Wurstscript => "WurstScript",
    versions: [Wurstscript => "WurstScript"],
    extensions: [Wurst => "wurst"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Wurstscript,
        extensions: &[FileExtension::Wurst],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Wurstscript,
            syntax: concat!(
                r#"package HelloWurst
import Printing

init
	print(""#,
                output_for_hello_world!(),
                r#"")"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"package HelloWurst
import Printing

init
	print(""#,
                    output_for_hello_world!(),
                    r#"")"#
                ),
            )],
        }],
    };
