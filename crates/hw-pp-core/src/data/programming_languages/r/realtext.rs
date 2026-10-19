//! Hello-world snippet for `ProgrammingLanguageName::Realtext`.
//!
//! Imported from the Hello World Collection (`files/RealText.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/RealText.txt>
//! - <http://helloworldcollection.de/#RealText>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Realtext => "RealText",
    versions: [Realtext => "RealText"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Realtext,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Realtext,
            syntax: concat!(
                r#"<window width="320" height="160">
<!-- "#,
                output_for_hello_world!(),
                r#" in RealText  -->
	<font size="12" name="Arial">
	<center>"#,
                output_for_hello_world!(),
                r"</center>
	</font>
</window>"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"<window width="320" height="160">
<!-- "#,
                    output_for_hello_world!(),
                    r#" in RealText  -->
	<font size="12" name="Arial">
	<center>"#,
                    output_for_hello_world!(),
                    r"</center>
	</font>
</window>"
                ),
            )],
        }],
    };
