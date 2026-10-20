//! Hello-world snippet for `ProgrammingLanguageName::Qwerty`.
//!
//! Imported from leachim6/hello-world (`q/QWERTY.qwerty` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/q/QWERTY.qwerty>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Qwerty => "QWERTY",
    versions: [Qwerty => "QWERTY"],
    extensions: [Qwerty => "qwerty"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Qwerty,
        extensions: &[FileExtension::Qwerty],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Qwerty,
            syntax: concat!(
                r#"begin
	?hello = ""#,
                output_for_hello_world!(),
                r#""
	if 1 == 1 then
		output ?hello
	fi
end"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"begin
	?hello = ""#,
                    output_for_hello_world!(),
                    r#""
	if 1 == 1 then
		output ?hello
	fi
end"#
                ),
            )],
        }],
    };
