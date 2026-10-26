//! Hello-world snippet for `ProgrammingLanguageName::AssemblerTas`.
//!
//! Imported from the Hello World Collection (`files/Assembler&nbsp;(TAS).htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Assembler%26nbsp%3B%28TAS%29.htm>
//! - <http://helloworldcollection.de/#Assembler%C2%A0(TAS)>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: AssemblerTas => "Assembler (TAS)",
    versions: [AssemblerTas => "Assembler (TAS)"],
    extensions: [],
}

const ASSEMBLER_TAS: &str = concat!(
    r"-- ",
    output_for_hello_world!(),
    r" in TAS Assembler for the TR 440 --

HELLO.=SEGM,XBA VB616, SSR 6 16,
FB616=R&ENDE,
 
VB616=FB616/AG, 3/H, TEXT/AG, 3/H,
TEXT=''*020",
    output_for_hello_world!(),
    r"'',
ENDE,"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerTas,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerTas,
            syntax: ASSEMBLER_TAS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER_TAS,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"-- ",
                            output_for_hello_world!(),
                            r" in TAS Assembler for the TR 440 --

HELLO.=SEGM,XBA VB616, SSR 6 16,
FB616=R&ENDE,
 
VB616=FB616/AG, 3/H, TEXT/AG, 3/H,
TEXT=''*020",
                            output_for_hello_world!(),
                            r"'',
ENDE,"
                        ),
                    )],
                )
            },
        }],
    };
