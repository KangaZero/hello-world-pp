//! Hello-world snippet for `ProgrammingLanguageName::ObjectPascal`.
//!
//! Imported from leachim6/hello-world (`o/Object Pascal.pp` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/o/Object%20Pascal.pp>
//! - <https://www.wikidata.org/wiki/Q633894> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Generic, Imperative, ObjectOriented, Procedural, Structured,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Pascal, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: ObjectPascal => "Object Pascal",
    versions: [ObjectPascal => "Object Pascal"],
    extensions: [Pp => "pp"],
}

const OBJECT_PASCAL: &str = concat!(
    r"program ObjectPascalExample;

type
   THelloWorld = class
      procedure Put;
   end;

procedure THelloWorld.Put;
begin
   Writeln('",
    output_for_hello_world!(),
    r"');
end;

var
   HelloWorld: THelloWorld;

begin
   HelloWorld := THelloWorld.Create;
   HelloWorld.Put;
   HelloWorld.Free;
end."
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ObjectPascal,
        extensions: &[FileExtension::Pp],
        categories: &[Compiled],
        families: &[Pascal],
        paradigms: &[Imperative, Procedural, Structured, ObjectOriented, Generic],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ObjectPascal,
            syntax: OBJECT_PASCAL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    OBJECT_PASCAL,
                    [
                        (TokenKind::Keyword, r"program"),
                        (TokenKind::Plain, r" ObjectPascalExample"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"type"),
                        (
                            TokenKind::Plain,
                            r"
   THelloWorld = class
      ",
                        ),
                        (TokenKind::Keyword, r"procedure"),
                        (TokenKind::Plain, r" Put"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"
   ",
                        ),
                        (TokenKind::Keyword, r"end"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"procedure"),
                        (TokenKind::Plain, r" THelloWorld"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"Put"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"begin"),
                        (
                            TokenKind::Plain,
                            r"
   ",
                        ),
                        (TokenKind::Function, r"Writeln"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r"'", output_for_hello_world!(), r"'"),
                        ),
                        (TokenKind::Punctuation, r");"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"end"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"var"),
                        (
                            TokenKind::Plain,
                            r"
   HelloWorld",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r" THelloWorld"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"begin"),
                        (
                            TokenKind::Plain,
                            r"
   HelloWorld ",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r"= THelloWorld"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"Create"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"
   HelloWorld",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"Put"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"
   HelloWorld",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"Free"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"end"),
                        (TokenKind::Punctuation, r"."),
                    ],
                )
            },
        }],
    };
