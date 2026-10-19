//! Hello-world snippet for `ProgrammingLanguageName::Vhdl`.
//!
//! Imported from leachim6/hello-world (`v/VHDL.vhdl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/v/VHDL.vhdl>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::HardwareDescription,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Vhdl => "VHDL",
    versions: [Vhdl => "VHDL"],
    extensions: [Vhdl => "vhdl"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Vhdl,
        extensions: &[FileExtension::Vhdl],
        categories: &[HardwareDescription],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Vhdl,
            syntax: concat!(
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
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
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
                ),
            )],
        }],
    };
