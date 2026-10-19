//! Hello-world snippet for `ProgrammingLanguageName::ConsolePostscript`.
//!
//! Imported from the Hello World Collection (`files/Console&nbsp;Postscript.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Console%26nbsp%3BPostscript.txt>
//! - <http://helloworldcollection.de/#Console%C2%A0Postscript>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: ConsolePostscript => "Console Postscript",
    versions: [ConsolePostscript => "Console Postscript"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ConsolePostscript,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ConsolePostscript,
            syntax: concat!(
                r"%% ",
                output_for_hello_world!(),
                r" in Console PostScript

serverdict begin 0 exitserver
/Courier findfont
48 scalefont setfont
22 22 moveto
(",
                output_for_hello_world!(),
                r") show
showpage

%% End"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"%% ",
                    output_for_hello_world!(),
                    r" in Console PostScript

serverdict begin 0 exitserver
/Courier findfont
48 scalefont setfont
22 22 moveto
(",
                    output_for_hello_world!(),
                    r") show
showpage

%% End"
                ),
            )],
        }],
    };
