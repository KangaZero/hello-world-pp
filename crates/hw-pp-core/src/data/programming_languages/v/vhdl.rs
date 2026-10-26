//! Hello-world snippet for `ProgrammingLanguageName::Vhdl`.
//!
//! Imported from leachim6/hello-world (`v/VHDL.vhdl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/v/VHDL.vhdl>
//! - <https://en.wikipedia.org/wiki/VHDL> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Concurrent, Dataflow, Reactive};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::HardwareDescription,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Ada, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Vhdl => "VHDL",
    versions: [Vhdl => "VHDL"],
    extensions: [Vhdl => "vhdl"],
}

const VHDL: &str = concat!(
    r#"use std.textio.all;

entity hello_world is
end hello_world;

architecture behaviour of hello_world is
begin
	process
    begin
       write (output, String'(""#,
    output_for_hello_world!(),
    r#""));
       wait;
    end process;
end behaviour;"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Vhdl,
        extensions: &[FileExtension::Vhdl],
        categories: &[HardwareDescription],
        families: &[Ada],
        paradigms: &[Concurrent, Reactive, Dataflow],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Vhdl,
            syntax: VHDL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    VHDL,
                    [
                        (TokenKind::Keyword, r"use"),
                        (TokenKind::Plain, r" std"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"textio"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Keyword, r"all"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"entity"),
                        (TokenKind::Plain, r" hello_world "),
                        (TokenKind::Keyword, r"is"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"end"),
                        (TokenKind::Plain, r" hello_world"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"architecture"),
                        (TokenKind::Plain, r" behaviour "),
                        (TokenKind::Keyword, r"of"),
                        (TokenKind::Plain, r" hello_world "),
                        (TokenKind::Keyword, r"is"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"begin"),
                        (
                            TokenKind::Plain,
                            r"
	",
                        ),
                        (TokenKind::Keyword, r"process"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Keyword, r"begin"),
                        (
                            TokenKind::Plain,
                            r"
       ",
                        ),
                        (TokenKind::Function, r"write"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Keyword, r"output"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"String"),
                        (TokenKind::Plain, r"'"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r"));"),
                        (
                            TokenKind::Plain,
                            r"
       ",
                        ),
                        (TokenKind::Keyword, r"wait"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Keyword, r"end"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"process"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"end"),
                        (TokenKind::Plain, r" behaviour"),
                        (TokenKind::Punctuation, r";"),
                    ],
                )
            },
        }],
    };
