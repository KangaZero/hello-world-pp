//! Hello-world snippet for `ProgrammingLanguageName::Lassoscript`.
//!
//! Imported from leachim6/hello-world (`l/LassoScript.lasso` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/l/LassoScript.lasso>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Lassoscript => "LassoScript",
    versions: [Lassoscript => "LassoScript"],
    extensions: [Lasso => "lasso"],
}

const LASSOSCRIPT: &str = concat!(
    r"<?LassoScript
	Variable: 'ScreenText' = '",
    output_for_hello_world!(),
    r"';
	$ScreenText;
?>"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lassoscript,
        extensions: &[FileExtension::Lasso],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lassoscript,
            syntax: LASSOSCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LASSOSCRIPT,
                    [(
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
                )
            },
        }],
    };
