//! Hello-world snippet for `ProgrammingLanguageName::Cherrypy`.
//!
//! Imported from leachim6/hello-world (`c/CherryPy.py` at `a152253`),
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
    language: Cherrypy => "CherryPy",
    versions: [Cherrypy => "CherryPy"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cherrypy,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cherrypy,
            syntax: concat!(
                r#"import cherrypy

class HelloWorld(object):
    @cherrypy.expose
    def index(self):
        return ""#,
                output_for_hello_world!(),
                r#""

cherrypy.quickstart(HelloWorld())"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"import"),
                (
                    TokenKind::Plain,
                    r" cherrypy

",
                ),
                (TokenKind::Keyword, r"class"),
                (TokenKind::Plain, r" HelloWorld"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"object"),
                (TokenKind::Punctuation, r"):"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Punctuation, r"@"),
                (TokenKind::Plain, r"cherrypy"),
                (TokenKind::Punctuation, r"."),
                (
                    TokenKind::Plain,
                    r"expose
    ",
                ),
                (TokenKind::Keyword, r"def"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"index"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"self"),
                (TokenKind::Punctuation, r"):"),
                (
                    TokenKind::Plain,
                    r"
        ",
                ),
                (TokenKind::Keyword, r"return"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (
                    TokenKind::Plain,
                    r"

cherrypy",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"quickstart"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Function, r"HelloWorld"),
                (TokenKind::Punctuation, r"())"),
            ],
        }],
    };
