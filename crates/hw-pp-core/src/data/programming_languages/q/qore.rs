//! Hello-world snippet for `ProgrammingLanguageName::Qore`.
//!
//! Imported from leachim6/hello-world (`q/Qore.q` at `a152253`),
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
    language: Qore => "Qore",
    versions: [Qore => "Qore"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Qore,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Qore,
            syntax: concat!(
                r#"#!/usr/bin/env qore
%exec-class HelloWorld
class HelloWorld
{
    constructor()
    {
	    background $.say(""#,
                output_for_hello_world!(),
                r#"");
    }
    private say($arg)
    {
	    printf("%s\n", $arg);
    }
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"#!/usr/bin/env qore
%exec-class HelloWorld
class HelloWorld
{
    constructor()
    {
	    background $.say(""#,
                    output_for_hello_world!(),
                    r#"");
    }
    private say($arg)
    {
	    printf("%s\n", $arg);
    }
}"#
                ),
            )],
        }],
    };
