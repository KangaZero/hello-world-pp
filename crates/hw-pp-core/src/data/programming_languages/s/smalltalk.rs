//! Hello-world snippet for `ProgrammingLanguageName::Smalltalk`.
//!
//! Imported from leachim6/hello-world (`s/SmallTalk.sm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Imported from the Hello World Collection (`files/Smalltalk&nbsp;(simple)`, `files/Smalltalk&nbsp;(window)` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/SmallTalk.sm>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Smalltalk%26nbsp%3B%28simple%29>
//! - <http://helloworldcollection.de/#Smalltalk%C2%A0(simple)>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Smalltalk%26nbsp%3B%28window%29>
//! - <http://helloworldcollection.de/#Smalltalk%C2%A0(window)>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Smalltalk => "SmallTalk",
    versions: [Smalltalk => "SmallTalk", SmalltalkSimple => "Smalltalk (simple)", SmalltalkWindow => "Smalltalk (window)"],
    extensions: [Sm => "sm"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Smalltalk,
        extensions: &[FileExtension::Sm],
        categories: &[Scripting],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Smalltalk,
                syntax: concat!(r"Transcript show: '", output_for_hello_world!(), r"'."),
                tokenized_syntax: &[
                    (TokenKind::Keyword, r"Transcript"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"show"),
                    (TokenKind::Punctuation, r":"),
                    (TokenKind::Plain, r" "),
                    (
                        TokenKind::String,
                        concat!(r"'", output_for_hello_world!(), r"'"),
                    ),
                    (TokenKind::Punctuation, r"."),
                ],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::SmalltalkSimple,
                syntax: concat!(
                    r#"""#,
                    output_for_hello_world!(),
                    r#" in Smalltalk (simple version)"

Transcript show: '"#,
                    output_for_hello_world!(),
                    r"'."
                ),
                tokenized_syntax: &[
                    (
                        TokenKind::Comment,
                        concat!(
                            r#"""#,
                            output_for_hello_world!(),
                            r#" in Smalltalk (simple version)""#
                        ),
                    ),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Keyword, r"Transcript"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"show"),
                    (TokenKind::Punctuation, r":"),
                    (TokenKind::Plain, r" "),
                    (
                        TokenKind::String,
                        concat!(r"'", output_for_hello_world!(), r"'"),
                    ),
                    (TokenKind::Punctuation, r"."),
                ],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::SmalltalkWindow,
                syntax: concat!(
                    r#"""#,
                    output_for_hello_world!(),
                    r#" in Smalltalk (in an own window)"
"(to be entered in a special browser)"

VisualComponent subclass: #HelloWorldView
	instanceVariableNames: ''
	classVariableNames: ''
	poolDictionaries: ''
	category: 'test'

displayOn: aGraphicsContext

	'"#,
                    output_for_hello_world!(),
                    r"' asComposedText displayOn: aGraphicsContext.

open

	|window|
	window := ScheduledWindow new.
	window label: '",
                    output_for_hello_world!(),
                    r" Demo:'.
	window component: self new.
	window open."
                ),
                tokenized_syntax: &[
                    (
                        TokenKind::Comment,
                        concat!(
                            r#"""#,
                            output_for_hello_world!(),
                            r#" in Smalltalk (in an own window)""#
                        ),
                    ),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (
                        TokenKind::Comment,
                        r#""(to be entered in a special browser)""#,
                    ),
                    (
                        TokenKind::Plain,
                        r"

VisualComponent subclass",
                    ),
                    (TokenKind::Punctuation, r":"),
                    (
                        TokenKind::Plain,
                        r" #HelloWorldView
	instanceVariableNames",
                    ),
                    (TokenKind::Punctuation, r":"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::String, r"''"),
                    (
                        TokenKind::Plain,
                        r"
	classVariableNames",
                    ),
                    (TokenKind::Punctuation, r":"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::String, r"''"),
                    (
                        TokenKind::Plain,
                        r"
	poolDictionaries",
                    ),
                    (TokenKind::Punctuation, r":"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::String, r"''"),
                    (
                        TokenKind::Plain,
                        r"
	category",
                    ),
                    (TokenKind::Punctuation, r":"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::String, r"'test'"),
                    (
                        TokenKind::Plain,
                        r"

displayOn",
                    ),
                    (TokenKind::Punctuation, r":"),
                    (
                        TokenKind::Plain,
                        r" aGraphicsContext

	",
                    ),
                    (
                        TokenKind::String,
                        concat!(r"'", output_for_hello_world!(), r"'"),
                    ),
                    (TokenKind::Plain, r" asComposedText displayOn"),
                    (TokenKind::Punctuation, r":"),
                    (TokenKind::Plain, r" aGraphicsContext"),
                    (TokenKind::Punctuation, r"."),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Keyword, r"open"),
                    (
                        TokenKind::Plain,
                        r"

	|window|
	window ",
                    ),
                    (TokenKind::Punctuation, r":"),
                    (TokenKind::Plain, r"= ScheduledWindow "),
                    (TokenKind::Keyword, r"new"),
                    (TokenKind::Punctuation, r"."),
                    (
                        TokenKind::Plain,
                        r"
	window label",
                    ),
                    (TokenKind::Punctuation, r":"),
                    (TokenKind::Plain, r" "),
                    (
                        TokenKind::String,
                        concat!(r"'", output_for_hello_world!(), r" Demo:'"),
                    ),
                    (TokenKind::Punctuation, r"."),
                    (
                        TokenKind::Plain,
                        r"
	window component",
                    ),
                    (TokenKind::Punctuation, r":"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"self"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"new"),
                    (TokenKind::Punctuation, r"."),
                    (
                        TokenKind::Plain,
                        r"
	window ",
                    ),
                    (TokenKind::Keyword, r"open"),
                    (TokenKind::Punctuation, r"."),
                ],
            },
        ],
    };
