//! Hello-world snippet for `ProgrammingLanguageName::AssemblerNasmWin64`.
//!
//! Imported from leachim6/hello-world (`a/Assembler NASM Win64.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%20NASM%20Win64.asm>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: AssemblerNasmWin64 => "Assembler NASM Win64",
    versions: [AssemblerNasmWin64 => "Assembler NASM Win64"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerNasmWin64,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerNasmWin64,
            syntax: concat!(
                r#"BITS 64
SECTION .text
global main
main:
xor rcx, rcx            
mov rax, [gs:rcx + 0x60]
mov rax, [rax + 0x18]   
mov rsi, [rax + 0x20]  
lodsq                   
xchg rax, rsi         
lodsq                   
mov rbx, [rax + 0x20]    
xor r8, r8                
mov r8d, [rbx + 0x3c]    
mov rdx, r8               
add rdx, rbx             
mov r8d, [rdx + 0x88]     
add r8, rbx             
xor rsi, rsi               
mov esi, [r8 + 0x20]       
add rsi, rbx               
xor rcx, rcx              
mov r9, 0x41636f7250746547 


Get_Function:

inc rcx                    
xor rax, rax               
mov eax, [rsi + rcx * 4]   
add rax, rbx              
cmp QWORD [rax], r9        
jnz Get_Function
xor rsi, rsi             
mov esi, [r8 + 0x24]       
add rsi, rbx               
mov cx, [rsi + rcx * 2] 
xor rsi, rsi               
mov esi, [r8 + 0x1c]       
add rsi, rbx           
xor rdx, rdx               
mov edx, [rsi + rcx * 4]   
add rdx, rbx             
mov rdi, rdx              


mov rcx, 0x41797261         
push rcx                    
mov rcx, 0x7262694c64616f4c  
push rcx                     
mov rdx, rsp                 
mov rcx, rbx                
sub rsp, 0x30               
call rdi                   
add rsp, 0x30                
add rsp, 0x10                
mov rsi, rax                 

mov rcx, 0x41656C6F73 
push rcx
mov rcx, 0x6E6F436574697257
push rcx
mov rdx, rsp
mov rcx, rbx
sub rsp, 0x30
call rdi
mov rsi, rax


mov rcx, 0x656C646E
push rcx
mov rcx, 0x6148647453746547   
push rcx                      
mov rdx, rsp                 
mov rcx, rbx              
sub rsp, 0x30                
call rdi                
add rsp, 0x28                
add rsp, 0x18               
mov r15, rax 

mov rcx, offset
sub rsp, 0x30
call r15
mov r15, rax
add   RSP, 32                                 

sub   RSP, 32 + 8 + 8  
mov rcx, r15
mov rdx, message
mov r8, len
mov r9,  written
mov qword[rsp+2*8], 0
sub rsp, 0x30
call rsi

xor rcx, rcx           
mov rcx, 0x737365         
push rcx                
mov rcx, 0x636f725074697845  
push rcx 
mov rdx, rsp                
mov rcx, rbx                 
sub rsp, 0x30            
call rdi                    
add rsp, 0x30                
add rsp, 0x10         
mov r15, rax                 


mov rcx, 0     
call r15      
section .data

	offset equ -11
	message db ""#,
                output_for_hello_world!(),
                r#"",0
	len equ $-message

section .bss
	written resq 1"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"BITS"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"64"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"SECTION"),
                (TokenKind::Plain, r" "),
                (TokenKind::Macro, r".text"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"global"),
                (
                    TokenKind::Plain,
                    r" main
main",
                ),
                (TokenKind::Punctuation, r":"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"xor"),
                (TokenKind::Plain, r" rcx"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" rcx            
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" rax"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"["),
                (TokenKind::Plain, r"gs"),
                (TokenKind::Punctuation, r":"),
                (TokenKind::Plain, r"rcx + "),
                (TokenKind::Number, r"0x60"),
                (TokenKind::Punctuation, r"]"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" rax"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"["),
                (TokenKind::Plain, r"rax + "),
                (TokenKind::Number, r"0x18"),
                (TokenKind::Punctuation, r"]"),
                (
                    TokenKind::Plain,
                    r"   
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" rsi"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"["),
                (TokenKind::Plain, r"rax + "),
                (TokenKind::Number, r"0x20"),
                (TokenKind::Punctuation, r"]"),
                (
                    TokenKind::Plain,
                    r"  
",
                ),
                (TokenKind::Keyword, r"lodsq"),
                (
                    TokenKind::Plain,
                    r"                   
",
                ),
                (TokenKind::Keyword, r"xchg"),
                (TokenKind::Plain, r" rax"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" rsi         
",
                ),
                (TokenKind::Keyword, r"lodsq"),
                (
                    TokenKind::Plain,
                    r"                   
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" rbx"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"["),
                (TokenKind::Plain, r"rax + "),
                (TokenKind::Number, r"0x20"),
                (TokenKind::Punctuation, r"]"),
                (
                    TokenKind::Plain,
                    r"    
",
                ),
                (TokenKind::Keyword, r"xor"),
                (TokenKind::Plain, r" r8"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" r8                
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" r8d"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"["),
                (TokenKind::Plain, r"rbx + "),
                (TokenKind::Number, r"0x3c"),
                (TokenKind::Punctuation, r"]"),
                (
                    TokenKind::Plain,
                    r"    
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" rdx"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" r8               
",
                ),
                (TokenKind::Keyword, r"add"),
                (TokenKind::Plain, r" rdx"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" rbx             
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" r8d"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"["),
                (TokenKind::Plain, r"rdx + "),
                (TokenKind::Number, r"0x88"),
                (TokenKind::Punctuation, r"]"),
                (
                    TokenKind::Plain,
                    r"     
",
                ),
                (TokenKind::Keyword, r"add"),
                (TokenKind::Plain, r" r8"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" rbx             
",
                ),
                (TokenKind::Keyword, r"xor"),
                (TokenKind::Plain, r" rsi"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" rsi               
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" esi"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"["),
                (TokenKind::Plain, r"r8 + "),
                (TokenKind::Number, r"0x20"),
                (TokenKind::Punctuation, r"]"),
                (
                    TokenKind::Plain,
                    r"       
",
                ),
                (TokenKind::Keyword, r"add"),
                (TokenKind::Plain, r" rsi"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" rbx               
",
                ),
                (TokenKind::Keyword, r"xor"),
                (TokenKind::Plain, r" rcx"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" rcx              
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" r9"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0x41636f7250746547"),
                (
                    TokenKind::Plain,
                    r" 


Get_Function",
                ),
                (TokenKind::Punctuation, r":"),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Keyword, r"inc"),
                (
                    TokenKind::Plain,
                    r" rcx                    
",
                ),
                (TokenKind::Keyword, r"xor"),
                (TokenKind::Plain, r" rax"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" rax               
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" eax"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"["),
                (TokenKind::Plain, r"rsi + rcx * "),
                (TokenKind::Number, r"4"),
                (TokenKind::Punctuation, r"]"),
                (
                    TokenKind::Plain,
                    r"   
",
                ),
                (TokenKind::Keyword, r"add"),
                (TokenKind::Plain, r" rax"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" rbx              
",
                ),
                (TokenKind::Keyword, r"cmp"),
                (TokenKind::Plain, r" QWORD "),
                (TokenKind::Punctuation, r"["),
                (TokenKind::Plain, r"rax"),
                (TokenKind::Punctuation, r"],"),
                (
                    TokenKind::Plain,
                    r" r9        
",
                ),
                (TokenKind::Keyword, r"jnz"),
                (
                    TokenKind::Plain,
                    r" Get_Function
",
                ),
                (TokenKind::Keyword, r"xor"),
                (TokenKind::Plain, r" rsi"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" rsi             
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" esi"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"["),
                (TokenKind::Plain, r"r8 + "),
                (TokenKind::Number, r"0x24"),
                (TokenKind::Punctuation, r"]"),
                (
                    TokenKind::Plain,
                    r"       
",
                ),
                (TokenKind::Keyword, r"add"),
                (TokenKind::Plain, r" rsi"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" rbx               
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" cx"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"["),
                (TokenKind::Plain, r"rsi + rcx * "),
                (TokenKind::Number, r"2"),
                (TokenKind::Punctuation, r"]"),
                (
                    TokenKind::Plain,
                    r" 
",
                ),
                (TokenKind::Keyword, r"xor"),
                (TokenKind::Plain, r" rsi"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" rsi               
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" esi"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"["),
                (TokenKind::Plain, r"r8 + "),
                (TokenKind::Number, r"0x1c"),
                (TokenKind::Punctuation, r"]"),
                (
                    TokenKind::Plain,
                    r"       
",
                ),
                (TokenKind::Keyword, r"add"),
                (TokenKind::Plain, r" rsi"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" rbx           
",
                ),
                (TokenKind::Keyword, r"xor"),
                (TokenKind::Plain, r" rdx"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" rdx               
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" edx"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"["),
                (TokenKind::Plain, r"rsi + rcx * "),
                (TokenKind::Number, r"4"),
                (TokenKind::Punctuation, r"]"),
                (
                    TokenKind::Plain,
                    r"   
",
                ),
                (TokenKind::Keyword, r"add"),
                (TokenKind::Plain, r" rdx"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" rbx             
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" rdi"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" rdx              


",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" rcx"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0x41797261"),
                (
                    TokenKind::Plain,
                    r"         
",
                ),
                (TokenKind::Keyword, r"push"),
                (
                    TokenKind::Plain,
                    r" rcx                    
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" rcx"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0x7262694c64616f4c"),
                (
                    TokenKind::Plain,
                    r"  
",
                ),
                (TokenKind::Keyword, r"push"),
                (
                    TokenKind::Plain,
                    r" rcx                     
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" rdx"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" rsp                 
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" rcx"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" rbx                
",
                ),
                (TokenKind::Keyword, r"sub"),
                (TokenKind::Plain, r" rsp"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0x30"),
                (
                    TokenKind::Plain,
                    r"               
",
                ),
                (TokenKind::Keyword, r"call"),
                (
                    TokenKind::Plain,
                    r" rdi                   
",
                ),
                (TokenKind::Keyword, r"add"),
                (TokenKind::Plain, r" rsp"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0x30"),
                (
                    TokenKind::Plain,
                    r"                
",
                ),
                (TokenKind::Keyword, r"add"),
                (TokenKind::Plain, r" rsp"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0x10"),
                (
                    TokenKind::Plain,
                    r"                
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" rsi"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" rax                 

",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" rcx"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0x41656C6F73"),
                (
                    TokenKind::Plain,
                    r" 
",
                ),
                (TokenKind::Keyword, r"push"),
                (
                    TokenKind::Plain,
                    r" rcx
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" rcx"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0x6E6F436574697257"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"push"),
                (
                    TokenKind::Plain,
                    r" rcx
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" rdx"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" rsp
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" rcx"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" rbx
",
                ),
                (TokenKind::Keyword, r"sub"),
                (TokenKind::Plain, r" rsp"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0x30"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"call"),
                (
                    TokenKind::Plain,
                    r" rdi
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" rsi"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" rax


",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" rcx"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0x656C646E"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"push"),
                (
                    TokenKind::Plain,
                    r" rcx
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" rcx"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0x6148647453746547"),
                (
                    TokenKind::Plain,
                    r"   
",
                ),
                (TokenKind::Keyword, r"push"),
                (
                    TokenKind::Plain,
                    r" rcx                      
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" rdx"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" rsp                 
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" rcx"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" rbx              
",
                ),
                (TokenKind::Keyword, r"sub"),
                (TokenKind::Plain, r" rsp"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0x30"),
                (
                    TokenKind::Plain,
                    r"                
",
                ),
                (TokenKind::Keyword, r"call"),
                (
                    TokenKind::Plain,
                    r" rdi                
",
                ),
                (TokenKind::Keyword, r"add"),
                (TokenKind::Plain, r" rsp"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0x28"),
                (
                    TokenKind::Plain,
                    r"                
",
                ),
                (TokenKind::Keyword, r"add"),
                (TokenKind::Plain, r" rsp"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0x18"),
                (
                    TokenKind::Plain,
                    r"               
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" r15"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" rax 

",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" rcx"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" offset
",
                ),
                (TokenKind::Keyword, r"sub"),
                (TokenKind::Plain, r" rsp"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0x30"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"call"),
                (
                    TokenKind::Plain,
                    r" r15
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" r15"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" rax
",
                ),
                (TokenKind::Keyword, r"add"),
                (TokenKind::Plain, r"   RSP"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"32"),
                (
                    TokenKind::Plain,
                    r"                                 

",
                ),
                (TokenKind::Keyword, r"sub"),
                (TokenKind::Plain, r"   RSP"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"32"),
                (TokenKind::Plain, r" + "),
                (TokenKind::Number, r"8"),
                (TokenKind::Plain, r" + "),
                (TokenKind::Number, r"8"),
                (
                    TokenKind::Plain,
                    r"  
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" rcx"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" r15
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" rdx"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" message
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" r8"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" len
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" r9"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r"  written
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" qword"),
                (TokenKind::Punctuation, r"["),
                (TokenKind::Plain, r"rsp+"),
                (TokenKind::Number, r"2"),
                (TokenKind::Plain, r"*"),
                (TokenKind::Number, r"8"),
                (TokenKind::Punctuation, r"],"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"sub"),
                (TokenKind::Plain, r" rsp"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0x30"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"call"),
                (
                    TokenKind::Plain,
                    r" rsi

",
                ),
                (TokenKind::Keyword, r"xor"),
                (TokenKind::Plain, r" rcx"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" rcx           
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" rcx"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0x737365"),
                (
                    TokenKind::Plain,
                    r"         
",
                ),
                (TokenKind::Keyword, r"push"),
                (
                    TokenKind::Plain,
                    r" rcx                
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" rcx"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0x636f725074697845"),
                (
                    TokenKind::Plain,
                    r"  
",
                ),
                (TokenKind::Keyword, r"push"),
                (
                    TokenKind::Plain,
                    r" rcx 
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" rdx"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" rsp                
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" rcx"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" rbx                 
",
                ),
                (TokenKind::Keyword, r"sub"),
                (TokenKind::Plain, r" rsp"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0x30"),
                (
                    TokenKind::Plain,
                    r"            
",
                ),
                (TokenKind::Keyword, r"call"),
                (
                    TokenKind::Plain,
                    r" rdi                    
",
                ),
                (TokenKind::Keyword, r"add"),
                (TokenKind::Plain, r" rsp"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0x30"),
                (
                    TokenKind::Plain,
                    r"                
",
                ),
                (TokenKind::Keyword, r"add"),
                (TokenKind::Plain, r" rsp"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0x10"),
                (
                    TokenKind::Plain,
                    r"         
",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" r15"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::Plain,
                    r" rax                 


",
                ),
                (TokenKind::Keyword, r"mov"),
                (TokenKind::Plain, r" rcx"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0"),
                (
                    TokenKind::Plain,
                    r"     
",
                ),
                (TokenKind::Keyword, r"call"),
                (
                    TokenKind::Plain,
                    r" r15      
",
                ),
                (TokenKind::Keyword, r"section"),
                (TokenKind::Plain, r" "),
                (TokenKind::Macro, r".data"),
                (
                    TokenKind::Plain,
                    r"

	offset ",
                ),
                (TokenKind::Keyword, r"equ"),
                (TokenKind::Plain, r" -"),
                (TokenKind::Number, r"11"),
                (
                    TokenKind::Plain,
                    r"
	message ",
                ),
                (TokenKind::Keyword, r"db"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r","),
                (TokenKind::Number, r"0"),
                (
                    TokenKind::Plain,
                    r"
	len ",
                ),
                (TokenKind::Keyword, r"equ"),
                (
                    TokenKind::Plain,
                    r" $-message

",
                ),
                (TokenKind::Keyword, r"section"),
                (TokenKind::Plain, r" "),
                (TokenKind::Macro, r".bss"),
                (
                    TokenKind::Plain,
                    r"
	",
                ),
                (TokenKind::Keyword, r"written"),
                (TokenKind::Plain, r" resq "),
                (TokenKind::Number, r"1"),
            ],
        }],
    };
