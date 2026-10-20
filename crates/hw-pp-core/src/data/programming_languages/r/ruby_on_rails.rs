//! Hello-world snippet for `ProgrammingLanguageName::RubyOnRails`.
//!
//! Imported from leachim6/hello-world (`r/Ruby on Rails.rb` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/Ruby%20on%20Rails.rb>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Framework, ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Ruby, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: RubyOnRails => "Ruby on Rails",
    versions: [RubyOnRails => "Ruby on Rails"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::RubyOnRails,
        extensions: &[],
        categories: &[Framework],
        families: &[Ruby],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::RubyOnRails,
            syntax: concat!(
                r#"class HelloWorld < app
  print ""#,
                output_for_hello_world!(),
                r#""
  end
end"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"class"),
                (TokenKind::Plain, r" HelloWorld "),
                (TokenKind::Punctuation, r"<"),
                (
                    TokenKind::Plain,
                    r" app
  ",
                ),
                (TokenKind::Function, r"print"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
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
            ],
        }],
    };
