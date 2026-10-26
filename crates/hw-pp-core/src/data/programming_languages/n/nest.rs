//! Hello-world snippets for `ProgrammingLanguageName::Nest`.
//!
//! Imported from leachim6/hello-world (PR #1805 `n/Nest.js`, PR #1805 `n/Nest.ts`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1805>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Framework, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Nest => "Nest",
    versions: [NestJs => "Nest (.js)", NestTs => "Nest (.ts)"],
    extensions: [],
}

const NEST_JS: &str = concat!(
    r"const { Controller, Get } = require('@nestjs/common');

@Controller()
class AppController {
  @Get()
  getHello() {
    return '",
    output_for_hello_world!(),
    r"';
  }
}

module.exports = { AppController };"
);
const NEST_TS: &str = concat!(
    r"import { Controller, Get } from '@nestjs/common';

@Controller()
export class AppController {
  @Get()
  getHello(): string {
    return '",
    output_for_hello_world!(),
    r"';
  }
}"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Nest,
        extensions: &[],
        categories: &[Framework],
        families: &[],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::NestJs,
                syntax: NEST_JS,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        NEST_JS,
                        [
                            (TokenKind::Type, r"const"),
                            (TokenKind::Plain, r" "),
                            (TokenKind::Punctuation, r"{"),
                            (TokenKind::Plain, r" Controller"),
                            (TokenKind::Punctuation, r","),
                            (TokenKind::Plain, r" Get "),
                            (TokenKind::Punctuation, r"}"),
                            (TokenKind::Plain, r" "),
                            (TokenKind::Keyword, r"="),
                            (TokenKind::Plain, r" "),
                            (TokenKind::Function, r"require"),
                            (TokenKind::Punctuation, r"("),
                            (TokenKind::String, r"'@nestjs/common'"),
                            (TokenKind::Punctuation, r");"),
                            (
                                TokenKind::Plain,
                                r"

@",
                            ),
                            (TokenKind::Function, r"Controller"),
                            (TokenKind::Punctuation, r"()"),
                            (
                                TokenKind::Plain,
                                r"
",
                            ),
                            (TokenKind::Keyword, r"class"),
                            (TokenKind::Plain, r" AppController "),
                            (TokenKind::Punctuation, r"{"),
                            (
                                TokenKind::Plain,
                                r"
  @",
                            ),
                            (TokenKind::Function, r"Get"),
                            (TokenKind::Punctuation, r"()"),
                            (
                                TokenKind::Plain,
                                r"
  ",
                            ),
                            (TokenKind::Function, r"getHello"),
                            (TokenKind::Punctuation, r"()"),
                            (TokenKind::Plain, r" "),
                            (TokenKind::Punctuation, r"{"),
                            (
                                TokenKind::Plain,
                                r"
    ",
                            ),
                            (TokenKind::Keyword, r"return"),
                            (TokenKind::Plain, r" "),
                            (
                                TokenKind::String,
                                concat!(r"'", output_for_hello_world!(), r"'"),
                            ),
                            (TokenKind::Punctuation, r";"),
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
                            (TokenKind::Punctuation, r"}"),
                            (
                                TokenKind::Plain,
                                r"

",
                            ),
                            (TokenKind::Type, r"module"),
                            (TokenKind::Punctuation, r"."),
                            (TokenKind::Type, r"exports"),
                            (TokenKind::Plain, r" "),
                            (TokenKind::Keyword, r"="),
                            (TokenKind::Plain, r" "),
                            (TokenKind::Punctuation, r"{"),
                            (TokenKind::Plain, r" AppController "),
                            (TokenKind::Punctuation, r"};"),
                        ],
                    )
                },
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::NestTs,
                syntax: NEST_TS,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        NEST_TS,
                        [(
                            TokenKind::Plain,
                            concat!(
                                r"import { Controller, Get } from '@nestjs/common';

@Controller()
export class AppController {
  @Get()
  getHello(): string {
    return '",
                                output_for_hello_world!(),
                                r"';
  }
}"
                            ),
                        )],
                    )
                },
            },
        ],
    };
