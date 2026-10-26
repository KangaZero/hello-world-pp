//! Hello-world snippet for `ProgrammingLanguageName::Actionscript3`.
//!
//! Imported from leachim6/hello-world (`a/ActionScript 3.as` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/ActionScript%203.as>
//! - <https://en.wikipedia.org/wiki/ActionScript> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Imperative, ObjectOriented, PrototypeBased,
};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Actionscript3 => "ActionScript 3",
    versions: [Actionscript3 => "ActionScript 3"],
    extensions: [],
}

const ACTIONSCRIPT3: &str = concat!(
    r#"package {

import flash.display.Sprite;
import flash.text.TextField;
import flash.text.TextFieldAutoSize;
import flash.text.TextFormat;

[SWF(width='800', height='600', backgroundColor='#cccccc', frameRate='30')]

	public class HelloFlash extends Sprite
	{
		public function HelloFlash()
		{
			var format:TextFormat = new TextFormat();
			format.font = "Arial";
			format.size = 20;
			format.color = 0x0000;

			var textField:TextField = new TextField();
			textField.defaultTextFormat = format;

			textField.border = false;
			textField.autoSize = TextFieldAutoSize.LEFT;
			textField.selectable = false;

			textField.text = ""#,
    output_for_hello_world!(),
    r#"";
			addChild(textField);


		}
	}
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Actionscript3,
        extensions: &[],
        categories: &[Scripting],
        families: &[],
        paradigms: &[Imperative, ObjectOriented, PrototypeBased, Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Actionscript3,
            syntax: ACTIONSCRIPT3,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ACTIONSCRIPT3,
                    [
                        (
                            TokenKind::Plain,
                            r"package {

",
                        ),
                        (TokenKind::Keyword, r"import"),
                        (
                            TokenKind::Plain,
                            r" flash.display.Sprite;
",
                        ),
                        (TokenKind::Keyword, r"import"),
                        (TokenKind::Plain, r" flash."),
                        (TokenKind::Function, r"text"),
                        (
                            TokenKind::Plain,
                            r".TextField;
",
                        ),
                        (TokenKind::Keyword, r"import"),
                        (TokenKind::Plain, r" flash."),
                        (TokenKind::Function, r"text"),
                        (
                            TokenKind::Plain,
                            r".TextFieldAutoSize;
",
                        ),
                        (TokenKind::Keyword, r"import"),
                        (TokenKind::Plain, r" flash."),
                        (TokenKind::Function, r"text"),
                        (
                            TokenKind::Plain,
                            r".TextFormat;

[SWF(",
                        ),
                        (TokenKind::Function, r"width"),
                        (TokenKind::Keyword, r"="),
                        (TokenKind::String, r"'800'"),
                        (TokenKind::Plain, r", "),
                        (TokenKind::Function, r"height"),
                        (TokenKind::Keyword, r"="),
                        (TokenKind::String, r"'600'"),
                        (TokenKind::Plain, r", "),
                        (TokenKind::Function, r"backgroundColor"),
                        (TokenKind::Keyword, r"="),
                        (TokenKind::String, r"'#cccccc'"),
                        (TokenKind::Plain, r", frameRate"),
                        (TokenKind::Keyword, r"="),
                        (TokenKind::String, r"'30'"),
                        (
                            TokenKind::Plain,
                            r")]

	",
                        ),
                        (TokenKind::Keyword, r"public"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"class"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Type, r"HelloFlash"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"extends"),
                        (
                            TokenKind::Plain,
                            r" Sprite
	{
		",
                        ),
                        (TokenKind::Keyword, r"public"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"function"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"HelloFlash"),
                        (TokenKind::Punctuation, r"()"),
                        (
                            TokenKind::Plain,
                            r"
		{
			",
                        ),
                        (TokenKind::Keyword, r"var"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"format"),
                        (TokenKind::Keyword, r":"),
                        (TokenKind::Plain, r"TextFormat "),
                        (TokenKind::Keyword, r"="),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"new"),
                        (
                            TokenKind::Plain,
                            r" TextFormat();
			",
                        ),
                        (TokenKind::Function, r"format"),
                        (TokenKind::Plain, r"."),
                        (TokenKind::Function, r"font"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"="),
                        (TokenKind::Plain, r" "),
                        (TokenKind::String, r#""Arial""#),
                        (
                            TokenKind::Plain,
                            r";
			",
                        ),
                        (TokenKind::Function, r"format"),
                        (TokenKind::Plain, r"."),
                        (TokenKind::Function, r"size"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"="),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"20"),
                        (
                            TokenKind::Plain,
                            r";
			",
                        ),
                        (TokenKind::Function, r"format"),
                        (TokenKind::Plain, r"."),
                        (TokenKind::Function, r"color"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"="),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"0x0000"),
                        (
                            TokenKind::Plain,
                            r";

			",
                        ),
                        (TokenKind::Keyword, r"var"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"textField"),
                        (TokenKind::Keyword, r":"),
                        (TokenKind::Plain, r"TextField "),
                        (TokenKind::Keyword, r"="),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"new"),
                        (
                            TokenKind::Plain,
                            r" TextField();
			",
                        ),
                        (TokenKind::Function, r"textField"),
                        (TokenKind::Plain, r".defaultTextFormat "),
                        (TokenKind::Keyword, r"="),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"format"),
                        (
                            TokenKind::Plain,
                            r";

			",
                        ),
                        (TokenKind::Function, r"textField"),
                        (TokenKind::Plain, r"."),
                        (TokenKind::Function, r"border"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"="),
                        (
                            TokenKind::Plain,
                            r" false;
			",
                        ),
                        (TokenKind::Function, r"textField"),
                        (TokenKind::Plain, r"."),
                        (TokenKind::Function, r"autoSize"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"="),
                        (
                            TokenKind::Plain,
                            r" TextFieldAutoSize.LEFT;
			",
                        ),
                        (TokenKind::Function, r"textField"),
                        (TokenKind::Plain, r"."),
                        (TokenKind::Function, r"selectable"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"="),
                        (
                            TokenKind::Plain,
                            r" false;

			",
                        ),
                        (TokenKind::Function, r"textField"),
                        (TokenKind::Plain, r"."),
                        (TokenKind::Function, r"text"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"="),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (
                            TokenKind::Plain,
                            r";
			addChild(",
                        ),
                        (TokenKind::Function, r"textField"),
                        (
                            TokenKind::Plain,
                            r");


		}
	}
}",
                        ),
                    ],
                )
            },
        }],
    };
