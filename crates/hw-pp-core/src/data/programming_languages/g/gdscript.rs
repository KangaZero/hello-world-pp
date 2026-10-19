//! Hello-world snippet for `ProgrammingLanguageName::Gdscript`.
//!
//! Imported from leachim6/hello-world (`g/GDScript.gd` at `a152253`),
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
    language: Gdscript => "GDScript",
    versions: [Gdscript => "GDScript"],
    extensions: [Gd => "gd"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gdscript,
        extensions: &[FileExtension::Gd],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gdscript,
            syntax: concat!(
                r#"extends Node2D

func _ready():
	print(""#,
                output_for_hello_world!(),
                r#"")"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"extends Node2D

func _ready():
	print(""#,
                    output_for_hello_world!(),
                    r#"")"#
                ),
            )],
        }],
    };
