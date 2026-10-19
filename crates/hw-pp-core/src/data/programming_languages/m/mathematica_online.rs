//! Hello-world snippet for `ProgrammingLanguageName::MathematicaOnline`.
//!
//! Imported from leachim6/hello-world (`m/Mathematica Online.nb` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Mathematica%20Online.nb>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scientific, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: MathematicaOnline => "Mathematica Online",
    versions: [MathematicaOnline => "Mathematica Online"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::MathematicaOnline,
        extensions: &[],
        categories: &[Scientific],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::MathematicaOnline,
            syntax: concat!(
                r#"Notebook[{Cell[
BoxData[RowBox[{"Print","[","\""#,
                output_for_hello_world!(),
                r#"\"","]"}]],"Input",CellID->1]},
StyleDefinitions->"Default.nb",FrontEndVersion->"10.0 for Wolfram Cloud Platform (June 2 2014)"]"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"Notebook[{Cell[
BoxData[RowBox[{"Print","[","\""#,
                    output_for_hello_world!(),
                    r#"\"","]"}]],"Input",CellID->1]},
StyleDefinitions->"Default.nb",FrontEndVersion->"10.0 for Wolfram Cloud Platform (June 2 2014)"]"#
                ),
            )],
        }],
    };
