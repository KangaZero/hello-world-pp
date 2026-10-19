//! Hello-world snippet for `ProgrammingLanguageName::AssemblerVp`.
//!
//! Imported from the Hello World Collection (`files/Assembler&nbsp;(VP).txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Assembler%26nbsp%3B%28VP%29.txt>
//! - <http://helloworldcollection.de/#Assembler%C2%A0(VP)>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: AssemblerVp => "Assembler (VP)",
    versions: [AssemblerVp => "Assembler (VP)"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerVp,
        extensions: &[],
        categories: &[Assembly],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerVp,
            syntax: concat!(
                r"; ",
                output_for_hello_world!(),
                r#" in VP Assembler for intent (Amiga Anywhere)

.include 'tao'

tool 'home/hello',VP,TF_MAIN,8192,0
	ent (-:-)
		qcall lib/print,(hello_world.p : i~)
		ret ()
	entend

	data

hello_world:
	dc.b ""#,
                output_for_hello_world!(),
                r#"",ASCII_LF,0

toolend"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"; ",
                    output_for_hello_world!(),
                    r#" in VP Assembler for intent (Amiga Anywhere)

.include 'tao'

tool 'home/hello',VP,TF_MAIN,8192,0
	ent (-:-)
		qcall lib/print,(hello_world.p : i~)
		ret ()
	entend

	data

hello_world:
	dc.b ""#,
                    output_for_hello_world!(),
                    r#"",ASCII_LF,0

toolend"#
                ),
            )],
        }],
    };
