//! Hello-world snippet for `ProgrammingLanguageName::ObjectPascal`.
//!
//! Imported from leachim6/hello-world (`o/Object Pascal.pp` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/o/Object%20Pascal.pp>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: ObjectPascal => "Object Pascal",
    versions: [ObjectPascal => "Object Pascal"],
    extensions: [Pp => "pp"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ObjectPascal,
        extensions: &[FileExtension::Pp],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ObjectPascal,
            syntax: concat!(
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
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
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
                ),
            )],
        }],
    };
