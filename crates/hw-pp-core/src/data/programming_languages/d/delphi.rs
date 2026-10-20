//! Hello-world snippet for `ProgrammingLanguageName::Delphi`.
//!
//! Imported from leachim6/hello-world (`d/Delphi.delphi`, `d/Delphi.pas` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/d/Delphi.delphi>
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/d/Delphi.pas>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Delphi => "Delphi",
    versions: [DelphiDelphi => "Delphi (.delphi)", DelphiPas => "Delphi (.pas)"],
    extensions: [Delphi => "delphi"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Delphi,
        extensions: &[FileExtension::Delphi],
        categories: &[Compiled],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::DelphiDelphi,
                syntax: concat!(
                    r"// ",
                    output_for_hello_world!(),
                    r" in Delphi
Program Hello_World;

($APPTYPE CONSOLE)

Begin
  WriteLn('",
                    output_for_hello_world!(),
                    r"');
End."
                ),
                tokenized_syntax: &[
                    (
                        TokenKind::Comment,
                        concat!(r"// ", output_for_hello_world!(), r" in Delphi"),
                    ),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Keyword, r"Program"),
                    (TokenKind::Plain, r" Hello_World"),
                    (TokenKind::Punctuation, r";"),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"$APPTYPE CONSOLE"),
                    (TokenKind::Punctuation, r")"),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Keyword, r"Begin"),
                    (
                        TokenKind::Plain,
                        r"
  ",
                    ),
                    (TokenKind::Function, r"WriteLn"),
                    (TokenKind::Punctuation, r"("),
                    (
                        TokenKind::String,
                        concat!(r"'", output_for_hello_world!(), r"'"),
                    ),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Keyword, r"End"),
                    (TokenKind::Punctuation, r"."),
                ],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::DelphiPas,
                syntax: concat!(
                    r"program HelloWorld;
{$APPTYPE CONSOLE}

begin
	WriteLn('",
                    output_for_hello_world!(),
                    r"');
end."
                ),
                tokenized_syntax: &[
                    (TokenKind::Keyword, r"program"),
                    (
                        TokenKind::Plain,
                        r" HelloWorld;
",
                    ),
                    (TokenKind::Comment, r"{$APPTYPE CONSOLE}"),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Keyword, r"begin"),
                    (
                        TokenKind::Plain,
                        r"
	WriteLn(",
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
                    (TokenKind::Plain, r"."),
                ],
            },
        ],
    };
