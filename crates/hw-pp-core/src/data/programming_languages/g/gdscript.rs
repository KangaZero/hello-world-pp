//! Hello-world snippet for `ProgrammingLanguageName::Gdscript`.
//!
//! Imported from leachim6/hello-world (`g/GDScript.gd` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/g/GDScript.gd>
//! - <https://www.wikidata.org/wiki/Q28942599> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::ObjectOriented;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Gdscript => "GDScript",
    versions: [Gdscript => "GDScript"],
    extensions: [Gd => "gd"],
}

const GDSCRIPT: &str = concat!(
    r#"extends Node2D

func _ready():
	print(""#,
    output_for_hello_world!(),
    r#"")"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gdscript,
        extensions: &[FileExtension::Gd],
        categories: &[Scripting],
        families: &[],
        paradigms: &[ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gdscript,
            syntax: GDSCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GDSCRIPT,
                    [
                        (TokenKind::Keyword, r"extends"),
                        (
                            TokenKind::Plain,
                            r" Node2D

",
                        ),
                        (TokenKind::Keyword, r"func"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"_ready"),
                        (TokenKind::Punctuation, r"():"),
                        (
                            TokenKind::Plain,
                            r"
	",
                        ),
                        (TokenKind::Function, r"print"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r")"),
                    ],
                )
            },
        }],
    };
