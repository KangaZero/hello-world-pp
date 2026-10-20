//! Hello-world snippet for `ProgrammingLanguageName::Sather`.
//!
//! Imported from leachim6/hello-world (`s/Sather.sa` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/Sather.sa>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Sather => "Sather",
    versions: [Sather => "Sather"],
    extensions: [Sa => "sa"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sather,
        extensions: &[FileExtension::Sa],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sather,
            syntax: concat!(
                r#"class MAIN is
   main is 
      loop 
         #OUT + ""#,
                output_for_hello_world!(),
                r#"\n" 
      end 
   end
end"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"class"),
                (TokenKind::Plain, r" MAIN "),
                (TokenKind::Keyword, r"is"),
                (
                    TokenKind::Plain,
                    r"
   main ",
                ),
                (TokenKind::Keyword, r"is"),
                (
                    TokenKind::Plain,
                    r" 
      loop 
         #OUT + ",
                ),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"\n""#),
                ),
                (
                    TokenKind::Plain,
                    r" 
      ",
                ),
                (TokenKind::Keyword, r"end"),
                (
                    TokenKind::Plain,
                    r" 
   ",
                ),
                (TokenKind::Keyword, r"end"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"end"),
            ],
        }],
    };
