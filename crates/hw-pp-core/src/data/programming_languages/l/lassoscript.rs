//! Hello-world snippet for `ProgrammingLanguageName::Lassoscript`.
//!
//! Imported from leachim6/hello-world (`l/LassoScript.lasso` at `a152253`),
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
    language: Lassoscript => "LassoScript",
    versions: [Lassoscript => "LassoScript"],
    extensions: [Lasso => "lasso"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lassoscript,
        extensions: &[FileExtension::Lasso],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lassoscript,
            syntax: concat!(
                r"<?LassoScript
	Variable: 'ScreenText' = '",
                output_for_hello_world!(),
                r"';
	$ScreenText;
?>"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"<?LassoScript
	Variable: 'ScreenText' = '",
                    output_for_hello_world!(),
                    r"';
	$ScreenText;
?>"
                ),
            )],
        }],
    };
