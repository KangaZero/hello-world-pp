//! Hello-world snippet for `ProgrammingLanguageName::Xblite`.
//!
//! Imported from the Hello World Collection (`files/xblite.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/xblite.txt>
//! - <http://helloworldcollection.de/#xblite>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Xblite => "xblite",
    versions: [Xblite => "xblite"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Xblite,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Xblite,
            syntax: concat!(
                r"' ",
                output_for_hello_world!(),
                r#" in xblite, Windows GUI mode

	IMPORT "gdi32"
	IMPORT "user32"

DECLARE FUNCTION Entry ()

FUNCTION Entry ()
	MessageBoxA (0, &""#,
                output_for_hello_world!(),
                r#"", &""#,
                output_for_hello_world!(),
                r#" Window", $$MB_OK)
END FUNCTION
END PROGRAM"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"' ",
                    output_for_hello_world!(),
                    r#" in xblite, Windows GUI mode

	IMPORT "gdi32"
	IMPORT "user32"

DECLARE FUNCTION Entry ()

FUNCTION Entry ()
	MessageBoxA (0, &""#,
                    output_for_hello_world!(),
                    r#"", &""#,
                    output_for_hello_world!(),
                    r#" Window", $$MB_OK)
END FUNCTION
END PROGRAM"#
                ),
            )],
        }],
    };
