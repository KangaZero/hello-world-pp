//! Hello-world snippet for `ProgrammingLanguageName::Eiffel`.
//!
//! Imported from leachim6/hello-world (`e/Eiffel.eiff` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/e/Eiffel.eiff>
//! - <https://en.wikipedia.org/wiki/Eiffel_(programming_language)> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Generic, ObjectOriented};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Eiffel => "Eiffel",
    versions: [Eiffel => "Eiffel"],
    extensions: [Eiff => "eiff"],
}

const EIFFEL: &str = concat!(
    r#"indexing ""#,
    output_for_hello_world!(),
    r#" in Eiffel , from http://roesler-ac.de/wolfram/hello.htm#Eiffel"

class HELLO

creation
	run

feature

	run is
		local
			io : BASIC_IO;
		do
			!!io;
			io.put_string(""#,
    output_for_hello_world!(),
    r#"");
			io.put_newline
		end; -- run
end; -- class HELLO"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Eiffel,
        extensions: &[FileExtension::Eiff],
        categories: &[Compiled],
        families: &[],
        paradigms: &[ObjectOriented, Generic],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Eiffel,
            syntax: EIFFEL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    EIFFEL,
                    [
                        (TokenKind::Keyword, r"indexing"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(
                                r#"""#,
                                output_for_hello_world!(),
                                r#" in Eiffel , from http://roesler-ac.de/wolfram/hello.htm#Eiffel""#
                            ),
                        ),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"class"),
                        (
                            TokenKind::Plain,
                            r" HELLO

",
                        ),
                        (TokenKind::Keyword, r"creation"),
                        (
                            TokenKind::Plain,
                            r"
	run

",
                        ),
                        (TokenKind::Keyword, r"feature"),
                        (
                            TokenKind::Plain,
                            r"

	run ",
                        ),
                        (TokenKind::Keyword, r"is"),
                        (
                            TokenKind::Plain,
                            r"
		",
                        ),
                        (TokenKind::Keyword, r"local"),
                        (
                            TokenKind::Plain,
                            r"
			io ",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r" BASIC_IO"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"
		",
                        ),
                        (TokenKind::Keyword, r"do"),
                        (
                            TokenKind::Plain,
                            r"
			!!io",
                        ),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"
			io",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"put_string"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r");"),
                        (
                            TokenKind::Plain,
                            r"
			io",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (
                            TokenKind::Plain,
                            r"put_newline
		",
                        ),
                        (TokenKind::Keyword, r"end"),
                        (TokenKind::Punctuation, r";"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Comment, r"-- run"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"end"),
                        (TokenKind::Punctuation, r";"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Comment, r"-- class HELLO"),
                    ],
                )
            },
        }],
    };
