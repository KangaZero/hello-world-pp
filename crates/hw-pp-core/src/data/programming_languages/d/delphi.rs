//! Hello-world snippet for `ProgrammingLanguageName::Delphi`.
//!
//! Imported from leachim6/hello-world (`d/Delphi.delphi` at `a152253`, `d/Delphi.pas` at `a152253`),
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
    language: Delphi => "Delphi",
    versions: [DelphiDelphi => "Delphi (.delphi)", DelphiPas => "Delphi (.pas)"],
    extensions: [Delphi => "delphi"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Delphi,
        extensions: &[FileExtension::Delphi],
        categories: &[],
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
                tokenized_syntax: &[(
                    TokenKind::Plain,
                    concat!(
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
                )],
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
