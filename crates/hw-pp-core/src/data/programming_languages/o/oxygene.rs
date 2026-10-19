//! Hello-world snippet for `ProgrammingLanguageName::Oxygene`.
//!
//! Imported from leachim6/hello-world (`o/Oxygene.pas` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/o/Oxygene.pas>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Compiled, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Oxygene => "Oxygene",
    versions: [Oxygene => "Oxygene"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Oxygene,
        extensions: &[],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Oxygene,
            syntax: concat!(
                r"implementation

class method ConsoleApp.Main;
begin
  Console.WriteLine('",
                output_for_hello_world!(),
                r"');
end;

end."
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"implementation"),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Keyword, r"class"),
                (
                    TokenKind::Plain,
                    r" method ConsoleApp.Main;
",
                ),
                (TokenKind::Keyword, r"begin"),
                (
                    TokenKind::Plain,
                    r"
  Console.WriteLine(",
                ),
                (
                    TokenKind::String,
                    concat!(r"'", output_for_hello_world!(), r"'"),
                ),
                (
                    TokenKind::Plain,
                    r");
",
                ),
                (TokenKind::Keyword, r"end"),
                (
                    TokenKind::Plain,
                    r";

",
                ),
                (TokenKind::Keyword, r"end"),
                (TokenKind::Plain, r"."),
            ],
        }],
    };
