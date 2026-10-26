//! Hello-world snippet for `ProgrammingLanguageName::Lotos`.
//!
//! Imported from the Hello World Collection (`files/LOTOS.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/LOTOS.txt>
//! - <http://helloworldcollection.de/#LOTOS>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Lotos => "LOTOS",
    versions: [Lotos => "LOTOS"],
    extensions: [],
}

const LOTOS: &str = concat!(
    r"(* ",
    output_for_hello_world!(),
    r#" in LOTOS (Language Of Temporal Ordering Specifications) *)
 
process HelloWorld [v]: exit :=
  v! ""#,
    output_for_hello_world!(),
    r#"";
  exit
endproc"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lotos,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lotos,
            syntax: LOTOS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LOTOS,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"(* ",
                            output_for_hello_world!(),
                            r#" in LOTOS (Language Of Temporal Ordering Specifications) *)
 
process HelloWorld [v]: exit :=
  v! ""#,
                            output_for_hello_world!(),
                            r#"";
  exit
endproc"#
                        ),
                    )],
                )
            },
        }],
    };
