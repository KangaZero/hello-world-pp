//! Hello-world snippet for `ProgrammingLanguageName::Ansible`.
//!
//! Imported from leachim6/hello-world (`a/Ansible.yml` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Ansible.yml>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Configuration,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Ansible => "Ansible",
    versions: [Ansible => "Ansible"],
    extensions: [Yml => "yml"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ansible,
        extensions: &[FileExtension::Yml],
        categories: &[Configuration],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ansible,
            syntax: concat!(
                r"---
- name: Print 
  hosts: 127.0.0.1
  connection: local

  tasks:
  - name: Print ",
                output_for_hello_world!(),
                r"
    debug:
      msg: ",
                output_for_hello_world!()
            ),
            tokenized_syntax: &[
                (
                    TokenKind::Plain,
                    r"---
",
                ),
                (TokenKind::Punctuation, r"-"),
                (TokenKind::Plain, r" "),
                (TokenKind::String, r"name"),
                (TokenKind::Punctuation, r":"),
                (TokenKind::Plain, r" "),
                (TokenKind::String, r"Print"),
                (
                    TokenKind::Plain,
                    r" 
  ",
                ),
                (TokenKind::String, r"hosts"),
                (TokenKind::Punctuation, r":"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"127.0.0.1"),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::String, r"connection"),
                (TokenKind::Punctuation, r":"),
                (TokenKind::Plain, r" "),
                (TokenKind::String, r"local"),
                (
                    TokenKind::Plain,
                    r"

  ",
                ),
                (TokenKind::String, r"tasks"),
                (TokenKind::Punctuation, r":"),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::Punctuation, r"-"),
                (TokenKind::Plain, r" "),
                (TokenKind::String, r"name"),
                (TokenKind::Punctuation, r":"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r"Print ", output_for_hello_world!()),
                ),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::String, r"debug"),
                (TokenKind::Punctuation, r":"),
                (
                    TokenKind::Plain,
                    r"
      ",
                ),
                (TokenKind::String, r"msg"),
                (TokenKind::Punctuation, r":"),
                (TokenKind::Plain, r" "),
                (TokenKind::String, output_for_hello_world!()),
            ],
        }],
    };
