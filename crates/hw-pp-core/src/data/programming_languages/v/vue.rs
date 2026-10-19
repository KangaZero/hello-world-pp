//! Hello-world snippet for `ProgrammingLanguageName::Vue`.
//!
//! Imported from leachim6/hello-world (`v/Vue.js` at `a152253`),
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
    language: Vue => "Vue",
    versions: [Vue => "Vue"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Vue,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Vue,
            syntax: concat!(
                r#"<div id="app">
  <p>{{ message }}</p>
</div>
<script>
new Vue({
  el: '#app',
  data: {
    message: '"#,
                output_for_hello_world!(),
                r"'
  }
})
</script>"
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"<"),
                (TokenKind::Plain, r"div id"),
                (TokenKind::Keyword, r"="),
                (TokenKind::String, r#""app""#),
                (TokenKind::Keyword, r">"),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::Keyword, r"<"),
                (TokenKind::Plain, r"p"),
                (TokenKind::Keyword, r">"),
                (TokenKind::Punctuation, r"{"),
                (TokenKind::Plain, r"{ message "),
                (TokenKind::Punctuation, r"}"),
                (TokenKind::Plain, r"}"),
                (TokenKind::Keyword, r"<"),
                (
                    TokenKind::String,
                    r"/p>
</",
                ),
                (TokenKind::Plain, r"div"),
                (TokenKind::Keyword, r">"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"<"),
                (TokenKind::Plain, r"script"),
                (TokenKind::Keyword, r">"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"new"),
                (TokenKind::Plain, r" Vue"),
                (TokenKind::Punctuation, r"({"),
                (
                    TokenKind::Plain,
                    r"
  el",
                ),
                (TokenKind::Punctuation, r":"),
                (TokenKind::Plain, r" "),
                (TokenKind::String, r"'#app'"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r"
  data",
                ),
                (TokenKind::Punctuation, r":"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"
    message",
                ),
                (TokenKind::Punctuation, r":"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r"'", output_for_hello_world!(), r"'"),
                ),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::Punctuation, r"}"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Punctuation, r"})"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"<"),
                (TokenKind::String, r"/script>"),
            ],
        }],
    };
