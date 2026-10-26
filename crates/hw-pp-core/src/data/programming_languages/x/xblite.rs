//! Hello-world snippet for `ProgrammingLanguageName::Xblite`.
//!
//! Imported from the Hello World Collection (`files/xblite.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/xblite.txt>
//! - <http://helloworldcollection.de/#xblite>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageFamily::Basic,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Xblite => "xblite",
    versions: [Xblite => "xblite"],
    extensions: [],
}

const XBLITE: &str = concat!(
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
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Xblite,
        extensions: &[],
        categories: &[],
        families: &[Basic],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Xblite,
            syntax: XBLITE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    XBLITE,
                    [
                        (
                            TokenKind::Comment,
                            concat!(
                                r"' ",
                                output_for_hello_world!(),
                                r" in xblite, Windows GUI mode"
                            ),
                        ),
                        (
                            TokenKind::Plain,
                            r"

	IMPORT ",
                        ),
                        (TokenKind::String, r#""gdi32""#),
                        (
                            TokenKind::Plain,
                            r"
	IMPORT ",
                        ),
                        (TokenKind::String, r#""user32""#),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"DECLARE"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"FUNCTION"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"Entry"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"()"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"FUNCTION"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"Entry"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"()"),
                        (
                            TokenKind::Plain,
                            r"
	",
                        ),
                        (TokenKind::Function, r"MessageBoxA"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Number, r"0"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" &"),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" &"),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#" Window""#),
                        ),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" $$MB_OK"),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"END"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"FUNCTION"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"END"),
                        (TokenKind::Plain, r" PROGRAM"),
                    ],
                )
            },
        }],
    };
