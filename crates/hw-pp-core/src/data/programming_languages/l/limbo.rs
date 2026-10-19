//! Hello-world snippet for `ProgrammingLanguageName::Limbo`.
//!
//! Imported from leachim6/hello-world (`l/Limbo.b` at `a152253`),
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
    language: Limbo => "Limbo",
    versions: [Limbo => "Limbo"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Limbo,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Limbo,
            syntax: concat!(
                r#"implement Hello;

include "sys.m";
	sys: Sys;
include "draw.m";

Hello: module
{
	init:	fn(ctxt: ref Draw->Context, argv: list of string);
};

init(ctxt: ref Draw->Context, argv: list of string)
{
	sys = load Sys Sys->PATH;
	sys->print(""#,
                output_for_hello_world!(),
                r#"\n");
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"implement Hello;

include "sys.m";
	sys: Sys;
include "draw.m";

Hello: module
{
	init:	fn(ctxt: ref Draw->Context, argv: list of string);
};

init(ctxt: ref Draw->Context, argv: list of string)
{
	sys = load Sys Sys->PATH;
	sys->print(""#,
                    output_for_hello_world!(),
                    r#"\n");
}"#
                ),
            )],
        }],
    };
