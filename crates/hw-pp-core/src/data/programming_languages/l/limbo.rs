//! Hello-world snippet for `ProgrammingLanguageName::Limbo`.
//!
//! Imported from leachim6/hello-world (`l/Limbo.b` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/l/Limbo.b>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Compiled, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Limbo => "Limbo",
    versions: [Limbo => "Limbo"],
    extensions: [],
}

const LIMBO: &str = concat!(
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
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Limbo,
        extensions: &[],
        categories: &[Compiled],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Limbo,
            syntax: LIMBO,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LIMBO,
                    [(
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
                )
            },
        }],
    };
