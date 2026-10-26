//! Hello-world snippet for `ProgrammingLanguageName::Verbosefuck`.
//!
//! Imported from leachim6/hello-world (`v/VerboseFuck.vbfk` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/v/VerboseFuck.vbfk>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Verbosefuck => "VerboseFuck",
    versions: [Verbosefuck => "VerboseFuck"],
    extensions: [Vbfk => "vbfk"],
}

const VERBOSEFUCK: &str = r"~!comment!~VerboseFuck, a BrainFuck derivative focussing on verbosity. see more at <http://esolangs.org/wiki/VerboseFuck>~!uncomment!~
program.initialize();
math.equation(program.errors.handler.activated = boolean(false));
program.console.standardinput.openstream();
program.console.standardoutput.openstream();
define(defines.variable, variable(pointer));
implanttype(pointer, types.pointer(to:types.byte));
math.equation(pointer = void(0));
program.memory.allocate(pointer, void(math.infinity), program.memory.memorytype.bidirectional);
~!comment!~MANDATORY~!uncomment!~
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
~!comment!~MANDATORY~!uncomment!~
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
~!comment!~MANDATORY~!uncomment!~
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
define(defines.label, defines.label.createnew());
conditional(block.if, boolean.inequality(deref(pointer), byte(0))) { 
	math.equation(pointer = pointer + void(1));
	~!comment!~MANDATORY~!uncomment!~
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	~!comment!~MANDATORY~!uncomment!~
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	math.equation(pointer = pointer + void(1));
	~!comment!~MANDATORY~!uncomment!~
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	~!comment!~MANDATORY~!uncomment!~
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	~!comment!~MANDATORY~!uncomment!~
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	math.equation(pointer = pointer + void(1));
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	~!comment!~MANDATORY~!uncomment!~
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	math.equation(pointer = pointer + void(1));
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	~!comment!~MANDATORY~!uncomment!~
	math.equation(pointer = pointer - void(1));
	math.equation(pointer = pointer - void(1));
	math.equation(pointer = pointer - void(1));
	math.equation(pointer = pointer - void(1));
	~!comment!~MANDATORY~!uncomment!~
	math.equation(deref(pointer) = (deref(pointer) - byte(1)):binaryand:byte(255));
};
conditional(block.if, boolean.inequality(deref(pointer), byte(0))) { 
	program.flow.labeledjump(defines.label.last());
};
undefine(defines.label, defines.label.last());
math.equation(pointer = pointer + void(1));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
~!comment!~MANDATORY~!uncomment!~
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
program.console.standardoutput.stream.writeunbufferedchars(array.create(1, conversion.changedatatype(deref(pointer), types.character, conversion.method.binary)), 0, 1);
math.equation(pointer = pointer + void(1));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
~!comment!~MANDATORY~!uncomment!~
program.console.standardoutput.stream.writeunbufferedchars(array.create(1, conversion.changedatatype(deref(pointer), types.character, conversion.method.binary)), 0, 1);
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
~!comment!~MANDATORY~!uncomment!~
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
~!comment!~MANDATORY~!uncomment!~
program.console.standardoutput.stream.writeunbufferedchars(array.create(1, conversion.changedatatype(deref(pointer), types.character, conversion.method.binary)), 0, 1);
program.console.standardoutput.stream.writeunbufferedchars(array.create(1, conversion.changedatatype(deref(pointer), types.character, conversion.method.binary)), 0, 1);
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
~!comment!~MANDATORY~!uncomment!~
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
program.console.standardoutput.stream.writeunbufferedchars(array.create(1, conversion.changedatatype(deref(pointer), types.character, conversion.method.binary)), 0, 1);
math.equation(pointer = pointer + void(1));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
~!comment!~MANDATORY~!uncomment!~
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
program.console.standardoutput.stream.writeunbufferedchars(array.create(1, conversion.changedatatype(deref(pointer), types.character, conversion.method.binary)), 0, 1);
math.equation(pointer = pointer - void(1));
math.equation(pointer = pointer - void(1));
~!comment!~MANDATORY~!uncomment!~
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
~!comment!~MANDATORY~!uncomment!~
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
~!comment!~MANDATORY~!uncomment!~
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
~!comment!~MANDATORY~!uncomment!~
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
program.console.standardoutput.stream.writeunbufferedchars(array.create(1, conversion.changedatatype(deref(pointer), types.character, conversion.method.binary)), 0, 1);
~!comment!~MANDATORY~!uncomment!~
math.equation(pointer = pointer + void(1));
program.console.standardoutput.stream.writeunbufferedchars(array.create(1, conversion.changedatatype(deref(pointer), types.character, conversion.method.binary)), 0, 1);
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
~!comment!~MANDATORY~!uncomment!~
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
program.console.standardoutput.stream.writeunbufferedchars(array.create(1, conversion.changedatatype(deref(pointer), types.character, conversion.method.binary)), 0, 1);
math.equation(deref(pointer) = (deref(pointer) - byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) - byte(1)):binaryand:byte(255));
~!comment!~MANDATORY~!uncomment!~
math.equation(deref(pointer) = (deref(pointer) - byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) - byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) - byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) - byte(1)):binaryand:byte(255));
~!comment!~MANDATORY~!uncomment!~
program.console.standardoutput.stream.writeunbufferedchars(array.create(1, conversion.changedatatype(deref(pointer), types.character, conversion.method.binary)), 0, 1);
math.equation(deref(pointer) = (deref(pointer) - byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) - byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) - byte(1)):binaryand:byte(255));
~!comment!~MANDATORY~!uncomment!~
math.equation(deref(pointer) = (deref(pointer) - byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) - byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) - byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) - byte(1)):binaryand:byte(255));
~!comment!~MANDATORY~!uncomment!~
math.equation(deref(pointer) = (deref(pointer) - byte(1)):binaryand:byte(255));
program.console.standardoutput.stream.writeunbufferedchars(array.create(1, conversion.changedatatype(deref(pointer), types.character, conversion.method.binary)), 0, 1);
math.equation(pointer = pointer + void(1));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
~!comment!~MANDATORY~!uncomment!~
program.console.standardoutput.stream.writeunbufferedchars(array.create(1, conversion.changedatatype(deref(pointer), types.character, conversion.method.binary)), 0, 1);
math.equation(pointer = pointer + void(1));
program.console.standardoutput.stream.writeunbufferedchars(array.create(1, conversion.changedatatype(deref(pointer), types.character, conversion.method.binary)), 0, 1);
~!comment!~MANDATORY~!uncomment!~
program.memory.deallocate(pointer, void(math.infinity), program.memory.memorytype.bidirectional);
undefine(defines.variable, variable(pointer));
program.console.standardoutput.closestream();
program.console.standardinput.closestream();
program.terminate();
~!comment!~MANDATORY~!uncomment!~";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Verbosefuck,
        extensions: &[FileExtension::Vbfk],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Verbosefuck,
            syntax: VERBOSEFUCK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    VERBOSEFUCK,
                    [(
                        TokenKind::Plain,
                        r"~!comment!~VerboseFuck, a BrainFuck derivative focussing on verbosity. see more at <http://esolangs.org/wiki/VerboseFuck>~!uncomment!~
program.initialize();
math.equation(program.errors.handler.activated = boolean(false));
program.console.standardinput.openstream();
program.console.standardoutput.openstream();
define(defines.variable, variable(pointer));
implanttype(pointer, types.pointer(to:types.byte));
math.equation(pointer = void(0));
program.memory.allocate(pointer, void(math.infinity), program.memory.memorytype.bidirectional);
~!comment!~MANDATORY~!uncomment!~
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
~!comment!~MANDATORY~!uncomment!~
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
~!comment!~MANDATORY~!uncomment!~
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
define(defines.label, defines.label.createnew());
conditional(block.if, boolean.inequality(deref(pointer), byte(0))) { 
	math.equation(pointer = pointer + void(1));
	~!comment!~MANDATORY~!uncomment!~
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	~!comment!~MANDATORY~!uncomment!~
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	math.equation(pointer = pointer + void(1));
	~!comment!~MANDATORY~!uncomment!~
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	~!comment!~MANDATORY~!uncomment!~
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	~!comment!~MANDATORY~!uncomment!~
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	math.equation(pointer = pointer + void(1));
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	~!comment!~MANDATORY~!uncomment!~
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	math.equation(pointer = pointer + void(1));
	math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
	~!comment!~MANDATORY~!uncomment!~
	math.equation(pointer = pointer - void(1));
	math.equation(pointer = pointer - void(1));
	math.equation(pointer = pointer - void(1));
	math.equation(pointer = pointer - void(1));
	~!comment!~MANDATORY~!uncomment!~
	math.equation(deref(pointer) = (deref(pointer) - byte(1)):binaryand:byte(255));
};
conditional(block.if, boolean.inequality(deref(pointer), byte(0))) { 
	program.flow.labeledjump(defines.label.last());
};
undefine(defines.label, defines.label.last());
math.equation(pointer = pointer + void(1));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
~!comment!~MANDATORY~!uncomment!~
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
program.console.standardoutput.stream.writeunbufferedchars(array.create(1, conversion.changedatatype(deref(pointer), types.character, conversion.method.binary)), 0, 1);
math.equation(pointer = pointer + void(1));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
~!comment!~MANDATORY~!uncomment!~
program.console.standardoutput.stream.writeunbufferedchars(array.create(1, conversion.changedatatype(deref(pointer), types.character, conversion.method.binary)), 0, 1);
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
~!comment!~MANDATORY~!uncomment!~
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
~!comment!~MANDATORY~!uncomment!~
program.console.standardoutput.stream.writeunbufferedchars(array.create(1, conversion.changedatatype(deref(pointer), types.character, conversion.method.binary)), 0, 1);
program.console.standardoutput.stream.writeunbufferedchars(array.create(1, conversion.changedatatype(deref(pointer), types.character, conversion.method.binary)), 0, 1);
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
~!comment!~MANDATORY~!uncomment!~
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
program.console.standardoutput.stream.writeunbufferedchars(array.create(1, conversion.changedatatype(deref(pointer), types.character, conversion.method.binary)), 0, 1);
math.equation(pointer = pointer + void(1));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
~!comment!~MANDATORY~!uncomment!~
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
program.console.standardoutput.stream.writeunbufferedchars(array.create(1, conversion.changedatatype(deref(pointer), types.character, conversion.method.binary)), 0, 1);
math.equation(pointer = pointer - void(1));
math.equation(pointer = pointer - void(1));
~!comment!~MANDATORY~!uncomment!~
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
~!comment!~MANDATORY~!uncomment!~
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
~!comment!~MANDATORY~!uncomment!~
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
~!comment!~MANDATORY~!uncomment!~
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
program.console.standardoutput.stream.writeunbufferedchars(array.create(1, conversion.changedatatype(deref(pointer), types.character, conversion.method.binary)), 0, 1);
~!comment!~MANDATORY~!uncomment!~
math.equation(pointer = pointer + void(1));
program.console.standardoutput.stream.writeunbufferedchars(array.create(1, conversion.changedatatype(deref(pointer), types.character, conversion.method.binary)), 0, 1);
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
~!comment!~MANDATORY~!uncomment!~
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
program.console.standardoutput.stream.writeunbufferedchars(array.create(1, conversion.changedatatype(deref(pointer), types.character, conversion.method.binary)), 0, 1);
math.equation(deref(pointer) = (deref(pointer) - byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) - byte(1)):binaryand:byte(255));
~!comment!~MANDATORY~!uncomment!~
math.equation(deref(pointer) = (deref(pointer) - byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) - byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) - byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) - byte(1)):binaryand:byte(255));
~!comment!~MANDATORY~!uncomment!~
program.console.standardoutput.stream.writeunbufferedchars(array.create(1, conversion.changedatatype(deref(pointer), types.character, conversion.method.binary)), 0, 1);
math.equation(deref(pointer) = (deref(pointer) - byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) - byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) - byte(1)):binaryand:byte(255));
~!comment!~MANDATORY~!uncomment!~
math.equation(deref(pointer) = (deref(pointer) - byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) - byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) - byte(1)):binaryand:byte(255));
math.equation(deref(pointer) = (deref(pointer) - byte(1)):binaryand:byte(255));
~!comment!~MANDATORY~!uncomment!~
math.equation(deref(pointer) = (deref(pointer) - byte(1)):binaryand:byte(255));
program.console.standardoutput.stream.writeunbufferedchars(array.create(1, conversion.changedatatype(deref(pointer), types.character, conversion.method.binary)), 0, 1);
math.equation(pointer = pointer + void(1));
math.equation(deref(pointer) = (deref(pointer) + byte(1)):binaryand:byte(255));
~!comment!~MANDATORY~!uncomment!~
program.console.standardoutput.stream.writeunbufferedchars(array.create(1, conversion.changedatatype(deref(pointer), types.character, conversion.method.binary)), 0, 1);
math.equation(pointer = pointer + void(1));
program.console.standardoutput.stream.writeunbufferedchars(array.create(1, conversion.changedatatype(deref(pointer), types.character, conversion.method.binary)), 0, 1);
~!comment!~MANDATORY~!uncomment!~
program.memory.deallocate(pointer, void(math.infinity), program.memory.memorytype.bidirectional);
undefine(defines.variable, variable(pointer));
program.console.standardoutput.closestream();
program.console.standardinput.closestream();
program.terminate();
~!comment!~MANDATORY~!uncomment!~",
                    )],
                )
            },
        }],
    };
