use crate::types::{CharacterCodes, CommentKind, SyntaxKind};
use js_sys::Function;
use num_traits::FromPrimitive;
use std::char;
use std::iter::{Enumerate, Peekable, Skip};
use std::str::Chars;
use wasm_bindgen::prelude::*;

const ABSTRACT: &'static str = "abstract";
const ANY: &'static str = "any";
const AS: &'static str = "as";
const BIGINT: &'static str = "bigint";
const BOOLEAN: &'static str = "boolean";
const BREAK: &'static str = "break";
const CASE: &'static str = "case";
const CATCH: &'static str = "catch";
const CLASS: &'static str = "class";
const CONTINUE: &'static str = "continue";
const CONST: &'static str = "const";
const CONSTRUCTOR: &'static str = "constructor";
const DEBUGGER: &'static str = "debugger";
const DECLARE: &'static str = "declare";
const DEFAULT: &'static str = "default";
const DELETE: &'static str = "delete";
const DO: &'static str = "do";
const ELSE: &'static str = "else";
const ENUM: &'static str = "enum";
const EXPORT: &'static str = "export";
const EXTENDS: &'static str = "extends";
const FALSE: &'static str = "false";
const FINALLY: &'static str = "finally";
const FOR: &'static str = "for";
const FROM: &'static str = "from";
const FUNCTION: &'static str = "function";
const GET: &'static str = "get";
const IF: &'static str = "if";
const IMPLEMENTS: &'static str = "implements";
const IMPORT: &'static str = "import";
const IN: &'static str = "in";
const INFER: &'static str = "infer";
const INSTANCEOF: &'static str = "instanceof";
const INTERFACE: &'static str = "interface";
const IS: &'static str = "is";
const KEYOF: &'static str = "keyof";
const LET: &'static str = "let";
const MODULE: &'static str = "module";
const NAMESPACE: &'static str = "namespace";
const NEVER: &'static str = "never";
const NEW: &'static str = "new";
const NULL: &'static str = "null";
const NUMBER: &'static str = "number";
const OBJECT: &'static str = "object";
const PACKAGE: &'static str = "package";
const PRIVATE: &'static str = "private";
const PROTECTED: &'static str = "protected";
const PUBLIC: &'static str = "public";
const READONLY: &'static str = "readonly";
const REQUIRE: &'static str = "require";
const GLOBAL: &'static str = "global";
const RETURN: &'static str = "return";
const SET: &'static str = "set";
const STATIC: &'static str = "static";
const STRING: &'static str = "string";
const SUPER: &'static str = "super";
const SWITCH: &'static str = "switch";
const SYMBOL: &'static str = "symbol";
const THIS: &'static str = "this";
const THROW: &'static str = "throw";
const TRUE: &'static str = "true";
const TRY: &'static str = "try";
const TYPE: &'static str = "type";
const TYPEOF: &'static str = "typeof";
const UNDEFINED: &'static str = "undefined";
const UNIQUE: &'static str = "unique";
const UNKNOWN: &'static str = "unknown";
const VAR: &'static str = "var";
const VOID: &'static str = "void";
const WHILE: &'static str = "while";
const WITH: &'static str = "with";
const YIELD: &'static str = "yield";
const ASYNC: &'static str = "async";
const AWAIT: &'static str = "await";
const OF: &'static str = "of";
const OPEN_BRACE_TOKEN: &'static str = "{";
const CLOSE_BRACE_TOKEN: &'static str = "}";
const OPEN_PAREN_TOKEN: &'static str = "(";
const CLOSE_PAREN_TOKEN: &'static str = ")";
const OPEN_BRACKET_TOKEN: &'static str = "[";
const CLOSE_BRACKET_TOKEN: &'static str = "]";
const DOT_TOKEN: &'static str = ".";
const DOT_DOT_DOT_TOKEN: &'static str = "...";
const SEMICOLON_TOKEN: &'static str = ";";
const COMMA_TOKEN: &'static str = ",";
const LESS_THAN_TOKEN: &'static str = "<";
const GREATER_THAN_TOKEN: &'static str = ">";
const LESS_THAN_EQUALS_TOKEN: &'static str = "<=";
const GREATER_THAN_EQUALS_TOKEN: &'static str = ">=";
const EQUALS_EQUALS_TOKEN: &'static str = "==";
const EXCLAMATION_EQUALS_TOKEN: &'static str = "!=";
const EQUALS_EQUALS_EQUALS_TOKEN: &'static str = "===";
const EXCLAMATION_EQUALS_EQUALS_TOKEN: &'static str = "!==";
const EQUALS_GREATER_THAN_TOKEN: &'static str = "=>";
const PLUS_TOKEN: &'static str = "+";
const MINUS_TOKEN: &'static str = "-";
const ASTERISK_ASTERISK_TOKEN: &'static str = "**";
const ASTERISK_TOKEN: &'static str = "*";
const SLASH_TOKEN: &'static str = "/";
const PERCENT_TOKEN: &'static str = "%";
const PLUS_PLUS_TOKEN: &'static str = "++";
const MINUS_MINUS_TOKEN: &'static str = "--";
const LESS_THAN_LESS_THAN_TOKEN: &'static str = "<<";
const LESS_THAN_SLASH_TOKEN: &'static str = "</";
const GREATER_THAN_GREATER_THAN_TOKEN: &'static str = ">>";
const GREATER_THAN_GREATER_THAN_GREATER_THAN_TOKEN: &'static str = ">>>";
const AMPERSAND_TOKEN: &'static str = "&";
const BAR_TOKEN: &'static str = "|";
const CARET_TOKEN: &'static str = "^";
const EXCLAMATION_TOKEN: &'static str = "!";
const TILDE_TOKEN: &'static str = "~";
const AMPERSAND_AMPERSAND_TOKEN: &'static str = "&&";
const BAR_BAR_TOKEN: &'static str = "||";
const QUESTION_TOKEN: &'static str = "?";
const COLON_TOKEN: &'static str = ":";
const EQUALS_TOKEN: &'static str = "=";
const PLUS_EQUALS_TOKEN: &'static str = "+=";
const MINUS_EQUALS_TOKEN: &'static str = "-=";
const ASTERISK_EQUALS_TOKEN: &'static str = "*=";
const ASTERISK_ASTERISK_EQUALS_TOKEN: &'static str = "**=";
const SLASH_EQUALS_TOKEN: &'static str = "/=";
const PERCENT_EQUALS_TOKEN: &'static str = "%=";
const LESS_THAN_LESS_THAN_EQUALS_TOKEN: &'static str = "<<=";
const GREATER_THAN_GREATER_THAN_EQUALS_TOKEN: &'static str = ">>=";
const GREATER_THAN_GREATER_THAN_GREATER_THAN_EQUALS_TOKEN: &'static str = ">>>=";
const AMPERSAND_EQUALS_TOKEN: &'static str = "&=";
const BAR_EQUALS_TOKEN: &'static str = "|=";
const CARET_EQUALS_TOKEN: &'static str = "^=";
const AT_TOKEN: &'static str = "@";

#[wasm_bindgen(js_name = "tokenToString")]
pub fn token_to_string(t: u32) -> Option<String> {
    FromPrimitive::from_u32(t)
        .map(|t: SyntaxKind| match t {
            SyntaxKind::AbstractKeyword => Some(ABSTRACT),
            SyntaxKind::AnyKeyword => Some(ANY),
            SyntaxKind::AsKeyword => Some(AS),
            SyntaxKind::BigIntKeyword => Some(BIGINT),
            SyntaxKind::BooleanKeyword => Some(BOOLEAN),
            SyntaxKind::BreakKeyword => Some(BREAK),
            SyntaxKind::CaseKeyword => Some(CASE),
            SyntaxKind::CatchKeyword => Some(CATCH),
            SyntaxKind::ClassKeyword => Some(CLASS),
            SyntaxKind::ContinueKeyword => Some(CONTINUE),
            SyntaxKind::ConstKeyword => Some(CONST),
            SyntaxKind::ConstructorKeyword => Some(CONSTRUCTOR),
            SyntaxKind::DebuggerKeyword => Some(DEBUGGER),
            SyntaxKind::DeclareKeyword => Some(DECLARE),
            SyntaxKind::DefaultKeyword => Some(DEFAULT),
            SyntaxKind::DeleteKeyword => Some(DELETE),
            SyntaxKind::DoKeyword => Some(DO),
            SyntaxKind::ElseKeyword => Some(ELSE),
            SyntaxKind::EnumKeyword => Some(ENUM),
            SyntaxKind::ExportKeyword => Some(EXPORT),
            SyntaxKind::ExtendsKeyword => Some(EXTENDS),
            SyntaxKind::FalseKeyword => Some(FALSE),
            SyntaxKind::FinallyKeyword => Some(FINALLY),
            SyntaxKind::ForKeyword => Some(FOR),
            SyntaxKind::FromKeyword => Some(FROM),
            SyntaxKind::FunctionKeyword => Some(FUNCTION),
            SyntaxKind::GetKeyword => Some(GET),
            SyntaxKind::IfKeyword => Some(IF),
            SyntaxKind::ImplementsKeyword => Some(IMPLEMENTS),
            SyntaxKind::ImportKeyword => Some(IMPORT),
            SyntaxKind::InKeyword => Some(IN),
            SyntaxKind::InferKeyword => Some(INFER),
            SyntaxKind::InstanceOfKeyword => Some(INSTANCEOF),
            SyntaxKind::InterfaceKeyword => Some(INTERFACE),
            SyntaxKind::IsKeyword => Some(IS),
            SyntaxKind::KeyOfKeyword => Some(KEYOF),
            SyntaxKind::LetKeyword => Some(LET),
            SyntaxKind::ModuleKeyword => Some(MODULE),
            SyntaxKind::NamespaceKeyword => Some(NAMESPACE),
            SyntaxKind::NeverKeyword => Some(NEVER),
            SyntaxKind::NewKeyword => Some(NEW),
            SyntaxKind::NullKeyword => Some(NULL),
            SyntaxKind::NumberKeyword => Some(NUMBER),
            SyntaxKind::ObjectKeyword => Some(OBJECT),
            SyntaxKind::PackageKeyword => Some(PACKAGE),
            SyntaxKind::PrivateKeyword => Some(PRIVATE),
            SyntaxKind::ProtectedKeyword => Some(PROTECTED),
            SyntaxKind::PublicKeyword => Some(PUBLIC),
            SyntaxKind::ReadonlyKeyword => Some(READONLY),
            SyntaxKind::RequireKeyword => Some(REQUIRE),
            SyntaxKind::GlobalKeyword => Some(GLOBAL),
            SyntaxKind::ReturnKeyword => Some(RETURN),
            SyntaxKind::SetKeyword => Some(SET),
            SyntaxKind::StaticKeyword => Some(STATIC),
            SyntaxKind::StringKeyword => Some(STRING),
            SyntaxKind::SuperKeyword => Some(SUPER),
            SyntaxKind::SwitchKeyword => Some(SWITCH),
            SyntaxKind::SymbolKeyword => Some(SYMBOL),
            SyntaxKind::ThisKeyword => Some(THIS),
            SyntaxKind::ThrowKeyword => Some(THROW),
            SyntaxKind::TrueKeyword => Some(TRUE),
            SyntaxKind::TryKeyword => Some(TRY),
            SyntaxKind::TypeKeyword => Some(TYPE),
            SyntaxKind::TypeOfKeyword => Some(TYPEOF),
            SyntaxKind::UndefinedKeyword => Some(UNDEFINED),
            SyntaxKind::UniqueKeyword => Some(UNIQUE),
            SyntaxKind::UnknownKeyword => Some(UNKNOWN),
            SyntaxKind::VarKeyword => Some(VAR),
            SyntaxKind::VoidKeyword => Some(VOID),
            SyntaxKind::WhileKeyword => Some(WHILE),
            SyntaxKind::WithKeyword => Some(WITH),
            SyntaxKind::YieldKeyword => Some(YIELD),
            SyntaxKind::AsyncKeyword => Some(ASYNC),
            SyntaxKind::AwaitKeyword => Some(AWAIT),
            SyntaxKind::OfKeyword => Some(OF),
            SyntaxKind::OpenBraceToken => Some(OPEN_BRACE_TOKEN),
            SyntaxKind::CloseBraceToken => Some(CLOSE_BRACE_TOKEN),
            SyntaxKind::OpenParenToken => Some(OPEN_PAREN_TOKEN),
            SyntaxKind::CloseParenToken => Some(CLOSE_PAREN_TOKEN),
            SyntaxKind::OpenBracketToken => Some(OPEN_BRACKET_TOKEN),
            SyntaxKind::CloseBracketToken => Some(CLOSE_BRACKET_TOKEN),
            SyntaxKind::DotToken => Some(DOT_TOKEN),
            SyntaxKind::DotDotDotToken => Some(DOT_DOT_DOT_TOKEN),
            SyntaxKind::SemicolonToken => Some(SEMICOLON_TOKEN),
            SyntaxKind::CommaToken => Some(COMMA_TOKEN),
            SyntaxKind::LessThanToken => Some(LESS_THAN_TOKEN),
            SyntaxKind::GreaterThanToken => Some(GREATER_THAN_TOKEN),
            SyntaxKind::LessThanEqualsToken => Some(LESS_THAN_EQUALS_TOKEN),
            SyntaxKind::GreaterThanEqualsToken => Some(GREATER_THAN_EQUALS_TOKEN),
            SyntaxKind::EqualsEqualsToken => Some(EQUALS_EQUALS_TOKEN),
            SyntaxKind::ExclamationEqualsToken => Some(EXCLAMATION_EQUALS_TOKEN),
            SyntaxKind::EqualsEqualsEqualsToken => Some(EQUALS_EQUALS_EQUALS_TOKEN),
            SyntaxKind::ExclamationEqualsEqualsToken => Some(EXCLAMATION_EQUALS_EQUALS_TOKEN),
            SyntaxKind::EqualsGreaterThanToken => Some(EQUALS_GREATER_THAN_TOKEN),
            SyntaxKind::PlusToken => Some(PLUS_TOKEN),
            SyntaxKind::MinusToken => Some(MINUS_TOKEN),
            SyntaxKind::AsteriskAsteriskToken => Some(ASTERISK_ASTERISK_TOKEN),
            SyntaxKind::AsteriskToken => Some(ASTERISK_TOKEN),
            SyntaxKind::SlashToken => Some(SLASH_TOKEN),
            SyntaxKind::PercentToken => Some(PERCENT_TOKEN),
            SyntaxKind::PlusPlusToken => Some(PLUS_PLUS_TOKEN),
            SyntaxKind::MinusMinusToken => Some(MINUS_MINUS_TOKEN),
            SyntaxKind::LessThanLessThanToken => Some(LESS_THAN_LESS_THAN_TOKEN),
            SyntaxKind::LessThanSlashToken => Some(LESS_THAN_SLASH_TOKEN),
            SyntaxKind::GreaterThanGreaterThanToken => Some(GREATER_THAN_GREATER_THAN_TOKEN),
            SyntaxKind::GreaterThanGreaterThanGreaterThanToken => {
                Some(GREATER_THAN_GREATER_THAN_GREATER_THAN_TOKEN)
            }
            SyntaxKind::AmpersandToken => Some(AMPERSAND_TOKEN),
            SyntaxKind::BarToken => Some(BAR_TOKEN),
            SyntaxKind::CaretToken => Some(CARET_TOKEN),
            SyntaxKind::ExclamationToken => Some(EXCLAMATION_TOKEN),
            SyntaxKind::TildeToken => Some(TILDE_TOKEN),
            SyntaxKind::AmpersandAmpersandToken => Some(AMPERSAND_AMPERSAND_TOKEN),
            SyntaxKind::BarBarToken => Some(BAR_BAR_TOKEN),
            SyntaxKind::QuestionToken => Some(QUESTION_TOKEN),
            SyntaxKind::ColonToken => Some(COLON_TOKEN),
            SyntaxKind::EqualsToken => Some(EQUALS_TOKEN),
            SyntaxKind::PlusEqualsToken => Some(PLUS_EQUALS_TOKEN),
            SyntaxKind::MinusEqualsToken => Some(MINUS_EQUALS_TOKEN),
            SyntaxKind::AsteriskEqualsToken => Some(ASTERISK_EQUALS_TOKEN),
            SyntaxKind::AsteriskAsteriskEqualsToken => Some(ASTERISK_ASTERISK_EQUALS_TOKEN),
            SyntaxKind::SlashEqualsToken => Some(SLASH_EQUALS_TOKEN),
            SyntaxKind::PercentEqualsToken => Some(PERCENT_EQUALS_TOKEN),
            SyntaxKind::LessThanLessThanEqualsToken => Some(LESS_THAN_LESS_THAN_EQUALS_TOKEN),
            SyntaxKind::GreaterThanGreaterThanEqualsToken => {
                Some(GREATER_THAN_GREATER_THAN_EQUALS_TOKEN)
            }
            SyntaxKind::GreaterThanGreaterThanGreaterThanEqualsToken => {
                Some(GREATER_THAN_GREATER_THAN_GREATER_THAN_EQUALS_TOKEN)
            }
            SyntaxKind::AmpersandEqualsToken => Some(AMPERSAND_EQUALS_TOKEN),
            SyntaxKind::BarEqualsToken => Some(BAR_EQUALS_TOKEN),
            SyntaxKind::CaretEqualsToken => Some(CARET_EQUALS_TOKEN),
            SyntaxKind::AtToken => Some(AT_TOKEN),
            _ => None,
        })
        .unwrap_or_default()
        .map(String::from)
}

/* Does not include line breaks. For that, see isWhiteSpaceLike(). */
#[wasm_bindgen(js_name = "isWhiteSpaceSingleLine")]
pub fn is_white_space_single_line(ch: u32) -> bool {
    // Note: NextLine is in the Zs space, and should be considered to be a whitespace.
    // It is explicitly not a line-break as it isn't in the exact set specified by EcmaScript.

    FromPrimitive::from_u32(ch)
        .map(|charcode: CharacterCodes| {
            charcode == CharacterCodes::Space
                || charcode == CharacterCodes::Tab
                || charcode == CharacterCodes::VerticalTab
                || charcode == CharacterCodes::FormFeed
                || charcode == CharacterCodes::NonBreakingSpace
                || charcode == CharacterCodes::NextLine
                || charcode == CharacterCodes::Ogham
                || charcode >= CharacterCodes::EnQuad && charcode <= CharacterCodes::ZeroWidthSpace
                || charcode == CharacterCodes::NarrowNoBreakSpace
                || charcode == CharacterCodes::MathematicalSpace
                || charcode == CharacterCodes::IdeographicSpace
                || charcode == CharacterCodes::IdeographicSpace
                || charcode == CharacterCodes::ByteOrderMark
        })
        .unwrap_or_default() // the default of bool is false
}

#[wasm_bindgen(js_name = "isLineBreak")]
pub fn is_line_break(ch: u32) -> bool {
    // ES5 7.3:
    // The ECMAScript line terminator characters are listed in Table 3.
    //     Table 3: Line Terminator Characters
    //     Code Unit Value     Name                    Formal Name
    //     \u000A              Line Feed               <LF>
    //     \u000D              Carriage Return         <CR>
    //     \u2028              Line separator          <LS>
    //     \u2029              Paragraph separator     <PS>
    // Only the characters in Table 3 are treated as line terminators. Other new line or line
    // breaking characters are treated as white space but not as line terminators.

    FromPrimitive::from_u32(ch)
        .map(|charcode: CharacterCodes| {
            charcode == CharacterCodes::LineFeed
                || charcode == CharacterCodes::CarriageReturn
                || charcode == CharacterCodes::LineSeparator
                || charcode == CharacterCodes::ParagraphSeparator
        })
        .unwrap_or_default() // the default of bool is false
}

#[wasm_bindgen(js_name = "isWhiteSpaceLike")]
pub fn is_white_space_like(ch: u32) -> bool {
    is_white_space_single_line(ch) || is_line_break(ch)
}

#[wasm_bindgen(js_name = "couldStartTrivia")]
pub fn could_start_trivia(text: &str, pos: usize) -> bool {
    text.chars()
        .nth(pos)
        .map(|ch| {
            FromPrimitive::from_u32(ch as u32)
                .map(|charcode: CharacterCodes| match charcode {
                    CharacterCodes::CarriageReturn |
                    CharacterCodes::LineFeed |
                    CharacterCodes::Tab |
                    CharacterCodes::VerticalTab |
                    CharacterCodes::FormFeed |
                    CharacterCodes::Space |
                    CharacterCodes::Slash |
                        // starts of normal trivia
                    CharacterCodes::LessThan |
                    CharacterCodes::Bar |
                    CharacterCodes::Equals |
                    CharacterCodes::GreaterThan =>
                        // Starts of conflict marker trivia
                        true,
                    CharacterCodes::Hash =>
                        // Only if its the beginning can we have #! trivia
                        pos == 0,
                    _ => ch as u32 > CharacterCodes::MaxAsciiCharacter as u32,
                })
                .unwrap_or_default()
        })
        .unwrap_or_default()
}

/** Optionally, get the shebang */
#[wasm_bindgen(js_name = "getShebang")]
pub fn get_shebang(text: &str) -> Option<String> {
    if text.starts_with("#!") {
        let shebang = if let Some(length) = text.find(|c: char| c == '\n' || c == '\r') {
            &text[..length]
        } else {
            text
        };
        Some(String::from(shebang))
    } else {
        None
    }
}

struct PeekablePosChar<'a>(Peekable<Skip<Enumerate<Chars<'a>>>>);

impl<'a> Iterator for PeekablePosChar<'a> {
    type Item = (usize, char);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a> PeekablePosChar<'a> {
    fn from(text: &'a str, skip: usize) -> Self {
        PeekablePosChar(text.chars().enumerate().skip(skip).peekable())
    }

    fn next_char(&mut self) -> Option<char> {
        self.0.next().map(|(_, ch)| ch)
    }

    fn next_character_code(&mut self) -> Option<CharacterCodes> {
        self.next_char()
            .map(|ch| FromPrimitive::from_u32(ch as u32))
            .unwrap_or_default()
    }

    fn peek(&mut self) -> Option<(usize, char)> {
        match self.0.peek() {
            Some((pos, ch)) => Some((*pos, *ch)),
            None => None,
        }
    }

    fn peek_next_character_code(&mut self) -> Option<CharacterCodes> {
        self.0
            .peek()
            .map(|(_, ch)| FromPrimitive::from_u32(*ch as u32))
            .unwrap_or_default()
    }
}

type CommentRangesIteratorCallback<T> = fn(usize, usize, CommentKind, bool, Option<T>) -> bool;

#[derive(Clone, Copy)]
struct PendingCommentRange {
    pending_position: usize,
    pending_end: usize,
    pending_kind: CommentKind,
    pending_has_trailing_new_line: bool,
}

struct CommentRangesIterator<'a, T, U: Copy>
{
    reduce: bool,
    pos_char: PeekablePosChar<'a>,
    trailing: bool,
    collecting: bool,
    callback: CommentRangesIteratorCallback<T>,
    state: Option<T>,
    pending_comment_range: Option<PendingCommentRange>,
    accumulator: Option<U>,
}

fn handle_line_feed<T, U: Copy>(
    CommentRangesIterator {
        trailing,
        collecting,
        pending_comment_range,
        accumulator,
        ..
    }: &mut CommentRangesIterator<T, U>,
) -> Option<Option<U>>
{
    if *trailing {
        None
    } else {
        *collecting = true;
        if let Some(pending_comment_range) = pending_comment_range.as_mut() {
            pending_comment_range.pending_has_trailing_new_line = true;
        }
        Some(*accumulator)
    }
}

impl<'a, T, U: Copy> Iterator for CommentRangesIterator<'a, T, U>
{
    type Item = Option<U>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((pos, ch)) = self.pos_char.next() {
            let charcode: Option<CharacterCodes> = FromPrimitive::from_u32(ch as u32);
            match charcode {
                Some(CharacterCodes::CarriageReturn) => {
                    if self.pos_char.peek_next_character_code() == Some(CharacterCodes::LineFeed) {
                        // Advance the iterator by one
                        self.pos_char.next();
                    }

                    handle_line_feed(self)
                }
                Some(CharacterCodes::LineFeed) => handle_line_feed(self),
                Some(CharacterCodes::Tab)
                | Some(CharacterCodes::VerticalTab)
                | Some(CharacterCodes::FormFeed)
                | Some(CharacterCodes::Space) => Some(self.accumulator),
                Some(CharacterCodes::Slash) => {
                    let next_char = self.pos_char.peek_next_character_code();
                    let mut has_trailing_new_line = false;
                    if next_char == Some(CharacterCodes::Slash)
                        || next_char == Some(CharacterCodes::Asterisk)
                    {
                        let kind = if next_char == Some(CharacterCodes::Slash) {
                            CommentKind::SingleLineCommentTrivia
                        } else {
                            CommentKind::MultiLineCommentTrivia
                        };
                        let start_pos = pos;
                        // Advance the iterator by one
                        self.pos_char.next();
                        if next_char == Some(CharacterCodes::Slash) {
                            while let Some(ch) = self.pos_char.next_char() {
                                if is_line_break(ch as u32) {
                                    has_trailing_new_line = true;
                                    break;
                                }
                            }
                        } else {
                            while let Some(ch) = self.pos_char.next_character_code() {
                                let next_char = self.pos_char.peek_next_character_code();
                                if ch == CharacterCodes::Asterisk
                                    && next_char == Some(CharacterCodes::Slash)
                                {
                                    self.pos_char.next();
                                    break;
                                }
                            }
                        }

                        if self.collecting {
                            if let Some(PendingCommentRange {
                                pending_position,
                                pending_end,
                                pending_kind,
                                pending_has_trailing_new_line,
                            }) = self.pending_comment_range
                            {
                                self.accumulator = self.callback(
                                    pending_position,
                                    pending_end,
                                    pending_end,
                                    pending_has_trailing_new_line,
                                    self.state,
                                    self.accumulator,
                                );
                                if !self.reduce && self.accumulator.is_some() {
                                    // TODO: need to return the accumulator and end the iterator!
                                    return None;
                                }
                            }
                        }

                        Some(self.accumulator)
                    } else {
                        None
                    }
                }
                _ => None,
            }
        } else {
            None
        }
    }
}

impl<'a, T, U: Copy> CommentRangesIterator<'a, T, U>
{
    fn new(
        reduce: bool,
        text: &'a str,
        position: usize,
        trailing: bool,
        callback: CommentRangesIteratorCallback<T>,
        state: Option<T>,
        initial: Option<U>,
    ) -> Self {
        let (position, collecting) = if position == 0 {
            (
                if let Some(shebang) = get_shebang(text) {
                    shebang.chars().count()
                } else {
                    position
                },
                true,
            )
        } else {
            (position, trailing)
        };

        CommentRangesIterator {
            reduce,
            pos_char: PeekablePosChar::from(text, position),
            trailing,
            collecting,
            callback,
            state,
            accumulator: initial,
            pending_comment_range: None,
        }
    }
}

pub fn for_each_leading_comment_range<T, U: Copy>(
    text: &str,
    position: usize,
    callback: CommentRangesIteratorCallback<T>,
    state: Option<T>,
) -> Option<U>
{
    let iter = CommentRangesIterator::new(
        /* reduce */ false, text, position, /* trailing */ false, callback, state,
        /* initial */ None,
    );
    None
}

/*
    export function forEachLeadingCommentRange<U>(text: string, pos: number, cb: (pos: number, end: number, kind: CommentKind, hasTrailingNewLine: boolean) => U): U | undefined;
    export function forEachLeadingCommentRange<T, U>(text: string, pos: number, cb: (pos: number, end: number, kind: CommentKind, hasTrailingNewLine: boolean, state: T) => U, state: T): U | undefined;
    export function forEachLeadingCommentRange<T, U>(text: string, pos: number, cb: (pos: number, end: number, kind: CommentKind, hasTrailingNewLine: boolean, state: T) => U, state?: T): U | undefined {
        return iterateCommentRanges(/*reduce*/ false, text, pos, /*trailing*/ false, cb, state);
    }
*/
