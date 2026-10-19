//! Hello-world snippet for `ProgrammingLanguageName::VaxMacro`.
//!
//! Imported from the Hello World Collection (`files/VAX&nbsp;Macro.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/VAX%26nbsp%3BMacro.txt>
//! - <http://helloworldcollection.de/#VAX%C2%A0Macro>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: VaxMacro => "VAX Macro",
    versions: [VaxMacro => "VAX Macro"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::VaxMacro,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::VaxMacro,
            syntax: concat!(
                output_for_hello_world!(),
                r" in VAX Macro.

        .title  helloworld
        .ident  /",
                output_for_hello_world!(),
                r"/
;
        .library        /sys$library:lib/
        $libdef
        $lib$routinesdef


        .psect  $data,wrt,noshr,noexe,long

hello:  .ascid  /",
                output_for_hello_world!(),
                r"/

        .psect  $code,nowrt,shr,exe,long

        .entry  helloworld,^m<r9,r10,r11>

        pushaq  hello                   ; output the
message
        calls   #1,g^lib$put_output     ;

        ret                             ; GTFOH
        .end    helloworld              ;"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    output_for_hello_world!(),
                    r" in VAX Macro.

        .title  helloworld
        .ident  /",
                    output_for_hello_world!(),
                    r"/
;
        .library        /sys$library:lib/
        $libdef
        $lib$routinesdef


        .psect  $data,wrt,noshr,noexe,long

hello:  .ascid  /",
                    output_for_hello_world!(),
                    r"/

        .psect  $code,nowrt,shr,exe,long

        .entry  helloworld,^m<r9,r10,r11>

        pushaq  hello                   ; output the
message
        calls   #1,g^lib$put_output     ;

        ret                             ; GTFOH
        .end    helloworld              ;"
                ),
            )],
        }],
    };
