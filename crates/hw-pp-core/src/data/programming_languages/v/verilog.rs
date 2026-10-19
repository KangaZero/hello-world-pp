//! Hello-world snippet for `ProgrammingLanguageName::Verilog`.
//!
//! Imported from leachim6/hello-world (`v/Verilog.v` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Verilog => "Verilog",
    versions: [Verilog => "Verilog"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Verilog,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Verilog,
            syntax: concat!(
                r#"module main;
  initial
    begin
      $display(""#,
                output_for_hello_world!(),
                r#"");
      $finish;
    end
endmodule"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"module main;
  initial
    begin
      $display(""#,
                    output_for_hello_world!(),
                    r#"");
      $finish;
    end
endmodule"#
                ),
            )],
        }],
    };
