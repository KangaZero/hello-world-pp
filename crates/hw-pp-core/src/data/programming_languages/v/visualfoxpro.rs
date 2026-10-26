//! Hello-world snippet for `ProgrammingLanguageName::Visualfoxpro`.
//!
//! Imported from leachim6/hello-world (`v/VisualFoxPro.prg` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/v/VisualFoxPro.prg>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Historical, ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::XBase, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Visualfoxpro => "VisualFoxPro",
    versions: [Visualfoxpro => "VisualFoxPro"],
    extensions: [],
}

const VISUALFOXPRO: &str = concat!(
    r#"MESSAGEBOX(""#,
    output_for_hello_world!(),
    r#"")

? ""#,
    output_for_hello_world!(),
    r#""

loForm = CREATEOBJECT("HiForm")
loForm.Show(1)

DEFINE CLASS HiForm AS Form
	AutoCenter	= .T.
	Caption		= ""#,
    output_for_hello_world!(),
    r#""
	
	ADD OBJECT lblHi as Label ;
		WITH Caption = ""#,
    output_for_hello_world!(),
    r#""
ENDDEFINE"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Visualfoxpro,
        extensions: &[],
        categories: &[Historical],
        families: &[XBase],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Visualfoxpro,
            syntax: VISUALFOXPRO,
            tokenized_syntax: &const {
                checked_token_lengths(
                    VISUALFOXPRO,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"MESSAGEBOX(""#,
                            output_for_hello_world!(),
                            r#"")

? ""#,
                            output_for_hello_world!(),
                            r#""

loForm = CREATEOBJECT("HiForm")
loForm.Show(1)

DEFINE CLASS HiForm AS Form
	AutoCenter	= .T.
	Caption		= ""#,
                            output_for_hello_world!(),
                            r#""
	
	ADD OBJECT lblHi as Label ;
		WITH Caption = ""#,
                            output_for_hello_world!(),
                            r#""
ENDDEFINE"#
                        ),
                    )],
                )
            },
        }],
    };
