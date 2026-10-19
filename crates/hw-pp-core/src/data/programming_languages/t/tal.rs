//! Hello-world snippet for `ProgrammingLanguageName::Tal`.
//!
//! Imported from the Hello World Collection (`files/TAL.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/TAL.txt>
//! - <http://helloworldcollection.de/#TAL>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Tal => "TAL",
    versions: [Tal => "TAL"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Tal,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Tal,
            syntax: concat!(
                r"!     ",
                output_for_hello_world!(),
                r#" in Tandem TAL (Transaction Application Language)
      proc Hello^World main;
            begin
            int    .term[0:12] := [ 12 * [ "  " ] ],
 .out [0:19];
            string .sout := @out << 1, .sp;
 
            call myterm ( term[1] );
            call open ( term[1], term );
if <> then call abend;
 
sout := ""#,
                output_for_hello_world!(),
                r#"" -> @sp;
            call write ( term, out, @sp-@sout );
if <> then call abend;
end;"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"!     ",
                    output_for_hello_world!(),
                    r#" in Tandem TAL (Transaction Application Language)
      proc Hello^World main;
            begin
            int    .term[0:12] := [ 12 * [ "  " ] ],
 .out [0:19];
            string .sout := @out << 1, .sp;
 
            call myterm ( term[1] );
            call open ( term[1], term );
if <> then call abend;
 
sout := ""#,
                    output_for_hello_world!(),
                    r#"" -> @sp;
            call write ( term, out, @sp-@sout );
if <> then call abend;
end;"#
                ),
            )],
        }],
    };
