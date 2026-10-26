//! Hello-world snippets for `ProgrammingLanguageName::Delphi`.
//!
//! Imported from leachim6/hello-world (`d/Delphi.delphi`, `d/Delphi.pas` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/d/Delphi.delphi>
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/d/Delphi.pas>
//! - <https://www.wikidata.org/wiki/Q4037189> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::ObjectOriented;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Pascal, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Delphi => "Delphi",
    versions: [DelphiDelphi => "Delphi (.delphi)", DelphiPas => "Delphi (.pas)"],
    extensions: [Delphi => "delphi"],
}

const DELPHI_DELPHI: &str = concat!(
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
);
const DELPHI_PAS: &str = concat!(
    r"program HelloWorld;
{$APPTYPE CONSOLE}

begin
	WriteLn('",
    output_for_hello_world!(),
    r"');
end."
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Delphi,
        extensions: &[FileExtension::Delphi],
        categories: &[Compiled],
        families: &[Pascal],
        paradigms: &[ObjectOriented],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::DelphiDelphi,
                syntax: DELPHI_DELPHI,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        DELPHI_DELPHI,
                        [
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
                    )
                },
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::DelphiPas,
                syntax: DELPHI_PAS,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        DELPHI_PAS,
                        [
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
                    )
                },
            },
        ],
    };
