// auto-generated: "lalrpop 0.20.0"
// sha3: 735077b6b4093f01fb7b214915ef9ad00d67cc4f3b387413f7b18c21180aab65
use crate::tok::{self, Tok};
use crate::ast;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as ___lalrpop_util;
#[allow(unused_imports)]
use self::___lalrpop_util::state_machine as ___state_machine;
extern crate core;
extern crate alloc;

#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::all)]
mod ___parse___Program {

    use crate::tok::{self, Tok};
    use crate::ast;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as ___lalrpop_util;
    #[allow(unused_imports)]
    use self::___lalrpop_util::state_machine as ___state_machine;
    extern crate core;
    extern crate alloc;
    use super::___ToTriple;
    #[allow(dead_code)]
    pub(crate) enum ___Symbol<'input>
     {
        Variant0(Tok<'input>),
        Variant1(&'input str),
        Variant2(u32),
        Variant3(core::option::Option<Tok<'input>>),
        Variant4((Tok<'input>, ast::ReturnType)),
        Variant5(core::option::Option<(Tok<'input>, ast::ReturnType)>),
        Variant6((Tok<'input>, ast::Statement)),
        Variant7(core::option::Option<(Tok<'input>, ast::Statement)>),
        Variant8(usize),
        Variant9(ast::ArgumentType),
        Variant10(core::option::Option<ast::ArgumentType>),
        Variant11(ast::ComparisonOp),
        Variant12(ast::Definition),
        Variant13(alloc::vec::Vec<ast::Definition>),
        Variant14(Box<ast::Expr>),
        Variant15(ast::Expr),
        Variant16(ast::Statement),
        Variant17(ast::Ident),
        Variant18(ast::InlineCode),
        Variant19(ast::Integer),
        Variant20(ast::IntegerExpr),
        Variant21(ast::Program),
        Variant22(ast::ReturnType),
        Variant23(alloc::vec::Vec<ast::Statement>),
        Variant24(ast::Variable),
    }
    const ___ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 4, 0, 33, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 4, 0, 33, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0,
        // State 8
        0, 0, 0, 41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 43, 0, 0, 0,
        // State 9
        0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 0, 0, 0, 0, 0, 0, 0, 0, 46, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 58, 0, 0, 0, 0, 16, 0, 17, 0, 18, 0, 19, 12, 59,
        // State 12
        0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 58, 0, 0, 0, 0, 16, 0, 17, 0, 18, 0, 19, 12, 66,
        // State 15
        23, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 74, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0,
        // State 17
        23, 0, 24, 0, 0, -53, 0, 0, 0, 0, 0, 0, 36, 0, 0, 74, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        23, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 74, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 0, 0, 0, 0, 0, 0, 0, 0, 46, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0,
        // State 21
        0, 81, 0, 0, 0, 0, 82, 83, 0, 84, 85, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        23, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 74, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        23, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 74, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 58, 0, 0, 0, 0, 16, 0, 17, 0, 18, 0, 19, 12, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -22, 0, -22, 0, -22, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25, 0, -25, 0, -25, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -21, 0, -21, 0, -21, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -26, 0, -26, 0, -26, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, -38, -38, 0, -38, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, 0,
        // State 36
        0, 0, 0, 0, 0, 44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, -44, 0, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -44, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -37, 0, -37, 0, -37, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, -50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -50, 0,
        // State 41
        0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, 0, -20, 0, -20, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -55, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -56, 0,
        // State 46
        0, 0, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -64, 0, 0, 0, -64, 0, 0, 0, 0, -64, 0, -64, 0, -64, 0, -64, -64, -64,
        // State 49
        0, 0, 0, 0, 0, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, -59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -62, 0, 0, 0, -62, 0, 0, 0, 0, -62, 0, -62, 0, -62, 0, -62, -62, -62,
        // State 52
        0, 0, 0, 0, 0, -58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -67, 0, 0, 0, -67, 0, 0, 0, 0, -67, 0, -67, 0, -67, 0, -67, -67, -67,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -61, 0, 0, 0, -61, 0, 0, 0, 0, -61, 0, -61, 0, -61, 0, -61, -61, -61,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -63, 0, 0, 0, -63, 0, 0, 0, 0, -63, 0, -63, 0, -63, 0, -63, -63, -63,
        // State 57
        0, 0, 67, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -69, 0, 0, 0, -69, 0, -69, -69, -69, -69, -69, -69, 0, -69, 0, -69, -69, -69,
        // State 59
        0, 0, 0, 0, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -49, 0,
        // State 60
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -35, 0, -35, 0, -35, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -36, 0, -36, 0, -36, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        0, 0, 0, 77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -60, 0, 0, 0, -60, 0, 0, 0, 0, -60, 0, -60, 0, -60, 0, -60, -60, -60,
        // State 64
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -68, 0, 0, 0, -68, 0, 0, 0, 0, -68, 0, -68, 0, -68, 0, -68, -68, -68,
        // State 65
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -70, 0, 0, 0, -70, 0, -70, -70, -70, -70, -70, -70, 0, -70, 0, -70, -70, -70,
        // State 66
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        0, 0, 0, -27, 0, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -27, 0,
        // State 68
        0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 69
        0, 0, 0, -42, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0,
        // State 70
        0, 0, 0, -29, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0,
        // State 71
        0, 0, 0, -30, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0,
        // State 72
        0, 0, 0, -28, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -28, 0,
        // State 73
        0, -71, 0, 0, 0, 0, -71, -71, 0, -71, -71, -71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 74
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -47, 0, 0, 0, -47, 0, 0, 0, 0, -47, 0, -47, 0, -47, 0, -47, -47, -47,
        // State 75
        0, 0, 0, 0, 0, -54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 76
        0, 0, 0, 0, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 77
        0, 0, 0, 91, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 78
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, -40, 0, 0, 28, 0, -40, 0, -40, 0, -40, 0, -40, -40, -40,
        // State 79
        0, 0, 0, 92, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 80
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 81
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 82
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 83
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -14, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 84
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 85
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 86
        0, 0, 0, -48, 0, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -48, 0,
        // State 87
        0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 88
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -73, 0, 0, 0, -73, 0, 0, 0, 0, -73, 0, -73, 0, -73, 0, -73, -73, -73,
        // State 89
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -34, 0, -34, 0, -34, 0, 0, 0, 0, 0, 0, 0,
        // State 90
        0, 0, 0, -41, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0,
        // State 91
        0, 0, 0, -32, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, 0,
        // State 92
        0, 0, 0, -46, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, 0,
        // State 93
        0, 0, 0, -45, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, 0,
        // State 94
        0, 0, 0, -72, 0, -72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -72, 0,
        // State 95
        0, 0, 0, -31, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0,
        // State 96
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39, 0, 0, 0, -39, 0, 0, 0, 0, -39, 0, -39, 0, -39, 0, -39, -39, -39,
    ];
    fn ___action(state: i8, integer: usize) -> i8 {
        ___ACTION[(state as usize) * 30 + integer]
    }
    const ___EOF_ACTION: &[i8] = &[
        // State 0
        -51,
        // State 1
        -52,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        -22,
        // State 29
        -25,
        // State 30
        -21,
        // State 31
        -74,
        // State 32
        0,
        // State 33
        -26,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        -37,
        // State 39
        0,
        // State 40
        0,
        // State 41
        0,
        // State 42
        0,
        // State 43
        -20,
        // State 44
        0,
        // State 45
        0,
        // State 46
        0,
        // State 47
        0,
        // State 48
        0,
        // State 49
        0,
        // State 50
        0,
        // State 51
        0,
        // State 52
        0,
        // State 53
        0,
        // State 54
        0,
        // State 55
        0,
        // State 56
        0,
        // State 57
        0,
        // State 58
        -69,
        // State 59
        0,
        // State 60
        -35,
        // State 61
        -36,
        // State 62
        0,
        // State 63
        0,
        // State 64
        0,
        // State 65
        -70,
        // State 66
        0,
        // State 67
        0,
        // State 68
        0,
        // State 69
        0,
        // State 70
        0,
        // State 71
        0,
        // State 72
        0,
        // State 73
        0,
        // State 74
        0,
        // State 75
        0,
        // State 76
        0,
        // State 77
        0,
        // State 78
        0,
        // State 79
        0,
        // State 80
        0,
        // State 81
        0,
        // State 82
        0,
        // State 83
        0,
        // State 84
        0,
        // State 85
        0,
        // State 86
        0,
        // State 87
        0,
        // State 88
        0,
        // State 89
        -34,
        // State 90
        0,
        // State 91
        0,
        // State 92
        0,
        // State 93
        0,
        // State 94
        0,
        // State 95
        0,
        // State 96
        0,
    ];
    fn ___goto(state: i8, nt: usize) -> i8 {
        match nt {
            6 => 39,
            8 => 26,
            9 => 28,
            10 => match state {
                1 => 33,
                _ => 29,
            },
            12 => 1,
            13 => match state {
                18 => 24,
                17 => 75,
                22 => 86,
                23 => 87,
                _ => 20,
            },
            14 => 67,
            15 => 46,
            16 => 30,
            17 => match state {
                3 => 4,
                5 => 9,
                2 => 34,
                15 | 17..=18 | 22..=23 => 68,
                26 => 92,
                _ => 47,
            },
            18 => 48,
            19 => match state {
                15 | 17..=18 | 22..=23 => 69,
                _ => 49,
            },
            20 => 70,
            21 => 50,
            22 => match state {
                26 => 93,
                _ => 36,
            },
            23 => 94,
            24 => 51,
            25 => 71,
            26 => match state {
                9 => 12,
                _ => 7,
            },
            27 => 31,
            28 => 52,
            29 => match state {
                19 => 25,
                _ => 13,
            },
            30 => 53,
            31 => match state {
                14 => 64,
                27 => 96,
                _ => 54,
            },
            33 => 14,
            34 => match state {
                7 => 38,
                12 => 60,
                13 => 61,
                16 => 74,
                20 => 78,
                24 => 88,
                25 => 89,
                _ => 55,
            },
            35 => 21,
            36 => 72,
            37 => 56,
            _ => 0,
        }
    }
    const ___TERMINAL: &[&str] = &[
        r###""!""###,
        r###""!=""###,
        r###""(""###,
        r###"")""###,
        r###""->""###,
        r###"";""###,
        r###""<""###,
        r###""<=""###,
        r###""=""###,
        r###""==""###,
        r###"">""###,
        r###"">=""###,
        r###""Id""###,
        r###""Integer""###,
        r###""StringLiteral""###,
        r###""Var""###,
        r###""__code__""###,
        r###""arg""###,
        r###""const""###,
        r###""else""###,
        r###""fn""###,
        r###""if""###,
        r###""inline""###,
        r###""loop""###,
        r###""repeat""###,
        r###""return""###,
        r###""state""###,
        r###""while""###,
        r###""{""###,
        r###""}""###,
    ];
    fn ___expected_tokens(___state: i8) -> alloc::vec::Vec<alloc::string::String> {
        ___TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = ___action(___state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    fn ___expected_tokens_from_states<
        'input,
    >(
        ___states: &[i8],
        _: core::marker::PhantomData<(&'input ())>,
    ) -> alloc::vec::Vec<alloc::string::String>
    {
        ___TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            if ___accepts(None, ___states, Some(index), core::marker::PhantomData::<(&())>) {
                Some(alloc::string::ToString::to_string(terminal))
            } else {
                None
            }
        }).collect()
    }
    pub(crate) struct ___StateMachine<'input>
    where 
    {
        text: &'input str,
        ___phantom: core::marker::PhantomData<(&'input ())>,
    }
    impl<'input> ___state_machine::ParserDefinition for ___StateMachine<'input>
    where 
    {
        type Location = usize;
        type Error = tok::Error;
        type Token = Tok<'input>;
        type TokenIndex = usize;
        type Symbol = ___Symbol<'input>;
        type Success = ast::Program;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            ___token_to_integer(token, core::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            ___action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            ___action(state, 30 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            ___EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            ___goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            ___token_to_symbol(token_index, token, core::marker::PhantomData::<(&())>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            ___expected_tokens(state)
        }

        fn expected_tokens_from_states(&self, states: &[i8]) -> alloc::vec::Vec<alloc::string::String> {
            ___expected_tokens_from_states(states, core::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: ___state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<___state_machine::SymbolTriple<Self>>,
        ) -> Option<___state_machine::ParseResult<Self>> {
            ___reduce(
                self.text,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> ___state_machine::SimulatedReduce<Self> {
            ___simulate_reduce(action, core::marker::PhantomData::<(&())>)
        }
    }
    fn ___token_to_integer<
        'input,
    >(
        ___token: &Tok<'input>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<usize>
    {
        match *___token {
            Tok::Bang if true => Some(0),
            Tok::NotEqual if true => Some(1),
            Tok::LeftParen if true => Some(2),
            Tok::RightParen if true => Some(3),
            Tok::RightArrow if true => Some(4),
            Tok::Semi if true => Some(5),
            Tok::LessThan if true => Some(6),
            Tok::LessOrEqual if true => Some(7),
            Tok::Assign if true => Some(8),
            Tok::Equal if true => Some(9),
            Tok::GreaterThan if true => Some(10),
            Tok::GreaterOrEqual if true => Some(11),
            Tok::Id(_) if true => Some(12),
            Tok::Integer(_) if true => Some(13),
            Tok::StringLiteral(_) if true => Some(14),
            Tok::Var(_) if true => Some(15),
            Tok::InlineCode if true => Some(16),
            Tok::Arg if true => Some(17),
            Tok::Const if true => Some(18),
            Tok::Else if true => Some(19),
            Tok::Fn if true => Some(20),
            Tok::If if true => Some(21),
            Tok::Inline if true => Some(22),
            Tok::Loop if true => Some(23),
            Tok::Repeat if true => Some(24),
            Tok::Return if true => Some(25),
            Tok::State if true => Some(26),
            Tok::While if true => Some(27),
            Tok::LeftBrace if true => Some(28),
            Tok::RightBrace if true => Some(29),
            _ => None,
        }
    }
    fn ___token_to_symbol<
        'input,
    >(
        ___token_index: usize,
        ___token: Tok<'input>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> ___Symbol<'input>
    {
        match ___token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 => ___Symbol::Variant0(___token),
            12 | 14 | 15 => match ___token {
                Tok::Id(___tok0) | Tok::StringLiteral(___tok0) | Tok::Var(___tok0) if true => ___Symbol::Variant1(___tok0),
                _ => unreachable!(),
            },
            13 => match ___token {
                Tok::Integer(___tok0) if true => ___Symbol::Variant2(___tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn ___simulate_reduce<
        'input,
    >(
        ___reduce_index: i8,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> ___state_machine::SimulatedReduce<___StateMachine<'input>>
    {
        match ___reduce_index {
            0 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 0,
                }
            }
            2 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 2,
                }
            }
            4 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 2,
                }
            }
            5 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 3,
                }
            }
            6 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 4,
                }
            }
            7 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 4,
                }
            }
            8 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 5,
                }
            }
            9 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            10 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            11 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 7,
                }
            }
            12 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 7,
                }
            }
            13 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            14 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            15 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            16 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            17 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            18 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            19 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 9,
                }
            }
            20 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            21 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            22 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 11,
                }
            }
            23 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 11,
                }
            }
            24 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 12,
                }
            }
            25 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 12,
                }
            }
            26 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            27 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            28 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            29 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            30 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 13,
                }
            }
            31 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 14,
                }
            }
            32 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 15,
                }
            }
            33 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 16,
                }
            }
            34 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 16,
                }
            }
            35 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 16,
                }
            }
            36 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 16,
                }
            }
            37 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 17,
                }
            }
            38 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 18,
                }
            }
            39 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 18,
                }
            }
            40 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 19,
                }
            }
            41 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            42 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            43 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            44 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            45 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            46 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 24,
                }
            }
            47 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 25,
                }
            }
            48 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 26,
                }
            }
            49 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 26,
                }
            }
            50 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 27,
                }
            }
            51 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 27,
                }
            }
            52 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 28,
                }
            }
            53 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 28,
                }
            }
            54 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 29,
                }
            }
            55 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 29,
                }
            }
            56 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 30,
                }
            }
            57 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 30,
                }
            }
            58 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 30,
                }
            }
            59 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 31,
                }
            }
            60 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 31,
                }
            }
            61 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 31,
                }
            }
            62 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 31,
                }
            }
            63 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 31,
                }
            }
            64 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 32,
                }
            }
            65 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 32,
                }
            }
            66 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 33,
                }
            }
            67 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 33,
                }
            }
            68 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 34,
                }
            }
            69 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 34,
                }
            }
            70 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 35,
                }
            }
            71 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 36,
                }
            }
            72 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 37,
                }
            }
            73 => ___state_machine::SimulatedReduce::Accept,
            _ => panic!("invalid reduction index {}", ___reduce_index)
        }
    }
    pub struct ProgramParser {
        _priv: (),
    }

    impl ProgramParser {
        pub fn new() -> ProgramParser {
            ProgramParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            ___TOKEN: ___ToTriple<'input, >,
            ___TOKENS: IntoIterator<Item=___TOKEN>,
        >(
            &self,
            text: &'input str,
            ___tokens0: ___TOKENS,
        ) -> Result<ast::Program, ___lalrpop_util::ParseError<usize, Tok<'input>, tok::Error>>
        {
            let ___tokens = ___tokens0.into_iter();
            let mut ___tokens = ___tokens.map(|t| ___ToTriple::to_triple(t));
            ___state_machine::Parser::drive(
                ___StateMachine {
                    text,
                    ___phantom: core::marker::PhantomData::<(&())>,
                },
                ___tokens,
            )
        }
    }
    fn ___accepts<
        'input,
    >(
        ___error_state: Option<i8>,
        ___states: &[i8],
        ___opt_integer: Option<usize>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> bool
    {
        let mut ___states = ___states.to_vec();
        ___states.extend(___error_state);
        loop {
            let mut ___states_len = ___states.len();
            let ___top = ___states[___states_len - 1];
            let ___action = match ___opt_integer {
                None => ___EOF_ACTION[___top as usize],
                Some(___integer) => ___action(___top, ___integer),
            };
            if ___action == 0 { return false; }
            if ___action > 0 { return true; }
            let (___to_pop, ___nt) = match ___simulate_reduce(-(___action + 1), core::marker::PhantomData::<(&())>) {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop, nonterminal_produced
                } => (states_to_pop, nonterminal_produced),
                ___state_machine::SimulatedReduce::Accept => return true,
            };
            ___states_len -= ___to_pop;
            ___states.truncate(___states_len);
            let ___top = ___states[___states_len - 1];
            let ___next_state = ___goto(___top, ___nt);
            ___states.push(___next_state);
        }
    }
    pub(crate) fn ___reduce<
        'input,
    >(
        text: &'input str,
        ___action: i8,
        ___lookahead_start: Option<&usize>,
        ___states: &mut alloc::vec::Vec<i8>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<Result<ast::Program,___lalrpop_util::ParseError<usize, Tok<'input>, tok::Error>>>
    {
        let (___pop_states, ___nonterminal) = match ___action {
            0 => {
                ___reduce0(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            1 => {
                ___reduce1(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            2 => {
                ___reduce2(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            3 => {
                ___reduce3(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            4 => {
                ___reduce4(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            5 => {
                ___reduce5(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            6 => {
                ___reduce6(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            7 => {
                ___reduce7(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            8 => {
                ___reduce8(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            9 => {
                ___reduce9(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            10 => {
                ___reduce10(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            11 => {
                ___reduce11(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            12 => {
                ___reduce12(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            13 => {
                ___reduce13(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            14 => {
                ___reduce14(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            15 => {
                ___reduce15(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            16 => {
                ___reduce16(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            17 => {
                ___reduce17(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            18 => {
                ___reduce18(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            19 => {
                ___reduce19(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            20 => {
                ___reduce20(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            21 => {
                ___reduce21(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            22 => {
                ___reduce22(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            23 => {
                ___reduce23(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            24 => {
                ___reduce24(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            25 => {
                ___reduce25(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            26 => {
                ___reduce26(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            27 => {
                ___reduce27(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            28 => {
                ___reduce28(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            29 => {
                ___reduce29(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            30 => {
                ___reduce30(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            31 => {
                ___reduce31(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            32 => {
                ___reduce32(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            33 => {
                ___reduce33(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            34 => {
                ___reduce34(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            35 => {
                ___reduce35(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            36 => {
                ___reduce36(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            37 => {
                ___reduce37(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            38 => {
                ___reduce38(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            39 => {
                ___reduce39(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            40 => {
                ___reduce40(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            41 => {
                ___reduce41(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            42 => {
                ___reduce42(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            43 => {
                ___reduce43(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            44 => {
                ___reduce44(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            45 => {
                ___reduce45(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            46 => {
                ___reduce46(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            47 => {
                ___reduce47(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            48 => {
                ___reduce48(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            49 => {
                ___reduce49(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            50 => {
                ___reduce50(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            51 => {
                ___reduce51(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            52 => {
                ___reduce52(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            53 => {
                ___reduce53(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            54 => {
                ___reduce54(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            55 => {
                ___reduce55(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            56 => {
                ___reduce56(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            57 => {
                ___reduce57(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            58 => {
                ___reduce58(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            59 => {
                ___reduce59(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            60 => {
                ___reduce60(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            61 => {
                ___reduce61(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            62 => {
                ___reduce62(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            63 => {
                ___reduce63(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            64 => {
                ___reduce64(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            65 => {
                ___reduce65(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            66 => {
                ___reduce66(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            67 => {
                ___reduce67(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            68 => {
                ___reduce68(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            69 => {
                ___reduce69(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            70 => {
                ___reduce70(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            71 => {
                ___reduce71(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            72 => {
                ___reduce72(text, ___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            73 => {
                // ___Program = Program => ActionFn(0);
                let ___sym0 = ___pop_Variant21(___symbols);
                let ___start = ___sym0.0;
                let ___end = ___sym0.2;
                let ___nt = super::___action0::<>(text, ___sym0);
                return Some(Ok(___nt));
            }
            _ => panic!("invalid action code {}", ___action)
        };
        let ___states_len = ___states.len();
        ___states.truncate(___states_len - ___pop_states);
        let ___state = *___states.last().unwrap();
        let ___next_state = ___goto(___state, ___nonterminal);
        ___states.push(___next_state);
        None
    }
    #[inline(never)]
    fn ___symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn ___pop_Variant4<
      'input,
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>
    ) -> (usize, (Tok<'input>, ast::ReturnType), usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant4(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant6<
      'input,
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>
    ) -> (usize, (Tok<'input>, ast::Statement), usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant6(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant14<
      'input,
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>
    ) -> (usize, Box<ast::Expr>, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant14(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant0<
      'input,
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant0(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant13<
      'input,
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<ast::Definition>, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant13(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant23<
      'input,
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<ast::Statement>, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant23(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant9<
      'input,
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>
    ) -> (usize, ast::ArgumentType, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant9(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant11<
      'input,
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>
    ) -> (usize, ast::ComparisonOp, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant11(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant12<
      'input,
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>
    ) -> (usize, ast::Definition, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant12(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant15<
      'input,
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>
    ) -> (usize, ast::Expr, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant15(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant17<
      'input,
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>
    ) -> (usize, ast::Ident, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant17(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant18<
      'input,
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>
    ) -> (usize, ast::InlineCode, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant18(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant19<
      'input,
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>
    ) -> (usize, ast::Integer, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant19(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant20<
      'input,
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>
    ) -> (usize, ast::IntegerExpr, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant20(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant21<
      'input,
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>
    ) -> (usize, ast::Program, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant21(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant22<
      'input,
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>
    ) -> (usize, ast::ReturnType, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant22(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant16<
      'input,
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>
    ) -> (usize, ast::Statement, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant16(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant24<
      'input,
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>
    ) -> (usize, ast::Variable, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant24(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant5<
      'input,
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<(Tok<'input>, ast::ReturnType)>, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant5(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant7<
      'input,
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<(Tok<'input>, ast::Statement)>, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant7(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant3<
      'input,
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<Tok<'input>>, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant3(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant10<
      'input,
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<ast::ArgumentType>, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant10(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant2<
      'input,
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>
    ) -> (usize, u32, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant2(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant8<
      'input,
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant8(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    fn ___pop_Variant1<
      'input,
    >(
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match ___symbols.pop() {
            Some((___l, ___Symbol::Variant1(___v), ___r)) => (___l, ___v, ___r),
            _ => ___symbol_type_mismatch()
        }
    }
    pub(crate) fn ___reduce0<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // "inline"? = "inline" => ActionFn(58);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action58::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant3(___nt), ___end));
        (1, 0)
    }
    pub(crate) fn ___reduce1<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // "inline"? =  => ActionFn(59);
        let ___start = ___lookahead_start.cloned().or_else(|| ___symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let ___end = ___start.clone();
        let ___nt = super::___action59::<>(text, &___start, &___end);
        ___symbols.push((___start, ___Symbol::Variant3(___nt), ___end));
        (0, 0)
    }
    pub(crate) fn ___reduce2<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ("->" ReturnType) = "->", ReturnType => ActionFn(57);
        assert!(___symbols.len() >= 2);
        let ___sym1 = ___pop_Variant22(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym1.2;
        let ___nt = super::___action57::<>(text, ___sym0, ___sym1);
        ___symbols.push((___start, ___Symbol::Variant4(___nt), ___end));
        (2, 1)
    }
    pub(crate) fn ___reduce3<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ("->" ReturnType)? = "->", ReturnType => ActionFn(69);
        assert!(___symbols.len() >= 2);
        let ___sym1 = ___pop_Variant22(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym1.2;
        let ___nt = super::___action69::<>(text, ___sym0, ___sym1);
        ___symbols.push((___start, ___Symbol::Variant5(___nt), ___end));
        (2, 2)
    }
    pub(crate) fn ___reduce4<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ("->" ReturnType)? =  => ActionFn(56);
        let ___start = ___lookahead_start.cloned().or_else(|| ___symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let ___end = ___start.clone();
        let ___nt = super::___action56::<>(text, &___start, &___end);
        ___symbols.push((___start, ___Symbol::Variant5(___nt), ___end));
        (0, 2)
    }
    pub(crate) fn ___reduce5<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ("else" Statement) = "else", Statement => ActionFn(50);
        assert!(___symbols.len() >= 2);
        let ___sym1 = ___pop_Variant16(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym1.2;
        let ___nt = super::___action50::<>(text, ___sym0, ___sym1);
        ___symbols.push((___start, ___Symbol::Variant6(___nt), ___end));
        (2, 3)
    }
    pub(crate) fn ___reduce6<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ("else" Statement)? = "else", Statement => ActionFn(74);
        assert!(___symbols.len() >= 2);
        let ___sym1 = ___pop_Variant16(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym1.2;
        let ___nt = super::___action74::<>(text, ___sym0, ___sym1);
        ___symbols.push((___start, ___Symbol::Variant7(___nt), ___end));
        (2, 4)
    }
    pub(crate) fn ___reduce7<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ("else" Statement)? =  => ActionFn(49);
        let ___start = ___lookahead_start.cloned().or_else(|| ___symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let ___end = ___start.clone();
        let ___nt = super::___action49::<>(text, &___start, &___end);
        ___symbols.push((___start, ___Symbol::Variant7(___nt), ___end));
        (0, 4)
    }
    pub(crate) fn ___reduce8<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(60);
        let ___start = ___lookahead_start.cloned().or_else(|| ___symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let ___end = ___start.clone();
        let ___nt = super::___action60::<>(text, &___start, &___end);
        ___symbols.push((___start, ___Symbol::Variant8(___nt), ___end));
        (0, 5)
    }
    pub(crate) fn ___reduce9<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ArgumentType = "arg" => ActionFn(10);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action10::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant9(___nt), ___end));
        (1, 6)
    }
    pub(crate) fn ___reduce10<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ArgumentType = "state" => ActionFn(11);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action11::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant9(___nt), ___end));
        (1, 6)
    }
    pub(crate) fn ___reduce11<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ArgumentType? = ArgumentType => ActionFn(53);
        let ___sym0 = ___pop_Variant9(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action53::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant10(___nt), ___end));
        (1, 7)
    }
    pub(crate) fn ___reduce12<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ArgumentType? =  => ActionFn(54);
        let ___start = ___lookahead_start.cloned().or_else(|| ___symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let ___end = ___start.clone();
        let ___nt = super::___action54::<>(text, &___start, &___end);
        ___symbols.push((___start, ___Symbol::Variant10(___nt), ___end));
        (0, 7)
    }
    pub(crate) fn ___reduce13<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ComparisonOp = "==" => ActionFn(41);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action41::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant11(___nt), ___end));
        (1, 8)
    }
    pub(crate) fn ___reduce14<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ComparisonOp = "!=" => ActionFn(42);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action42::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant11(___nt), ___end));
        (1, 8)
    }
    pub(crate) fn ___reduce15<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ComparisonOp = ">" => ActionFn(43);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action43::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant11(___nt), ___end));
        (1, 8)
    }
    pub(crate) fn ___reduce16<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ComparisonOp = "<" => ActionFn(44);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action44::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant11(___nt), ___end));
        (1, 8)
    }
    pub(crate) fn ___reduce17<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ComparisonOp = ">=" => ActionFn(45);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action45::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant11(___nt), ___end));
        (1, 8)
    }
    pub(crate) fn ___reduce18<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ComparisonOp = "<=" => ActionFn(46);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action46::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant11(___nt), ___end));
        (1, 8)
    }
    pub(crate) fn ___reduce19<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConstDefinition = "const", Ident, "=", Integer, ";" => ActionFn(77);
        assert!(___symbols.len() >= 5);
        let ___sym4 = ___pop_Variant0(___symbols);
        let ___sym3 = ___pop_Variant19(___symbols);
        let ___sym2 = ___pop_Variant0(___symbols);
        let ___sym1 = ___pop_Variant17(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym4.2;
        let ___nt = super::___action77::<>(text, ___sym0, ___sym1, ___sym2, ___sym3, ___sym4);
        ___symbols.push((___start, ___Symbol::Variant12(___nt), ___end));
        (5, 9)
    }
    pub(crate) fn ___reduce20<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Definition = FunctionDefinition => ActionFn(2);
        let ___sym0 = ___pop_Variant12(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action2::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant12(___nt), ___end));
        (1, 10)
    }
    pub(crate) fn ___reduce21<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Definition = ConstDefinition => ActionFn(3);
        let ___sym0 = ___pop_Variant12(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action3::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant12(___nt), ___end));
        (1, 10)
    }
    pub(crate) fn ___reduce22<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Definition* =  => ActionFn(61);
        let ___start = ___lookahead_start.cloned().or_else(|| ___symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let ___end = ___start.clone();
        let ___nt = super::___action61::<>(text, &___start, &___end);
        ___symbols.push((___start, ___Symbol::Variant13(___nt), ___end));
        (0, 11)
    }
    pub(crate) fn ___reduce23<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Definition* = Definition+ => ActionFn(62);
        let ___sym0 = ___pop_Variant13(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action62::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant13(___nt), ___end));
        (1, 11)
    }
    pub(crate) fn ___reduce24<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Definition+ = Definition => ActionFn(63);
        let ___sym0 = ___pop_Variant12(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action63::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant13(___nt), ___end));
        (1, 12)
    }
    pub(crate) fn ___reduce25<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Definition+ = Definition+, Definition => ActionFn(64);
        assert!(___symbols.len() >= 2);
        let ___sym1 = ___pop_Variant12(___symbols);
        let ___sym0 = ___pop_Variant13(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym1.2;
        let ___nt = super::___action64::<>(text, ___sym0, ___sym1);
        ___symbols.push((___start, ___Symbol::Variant13(___nt), ___end));
        (2, 12)
    }
    pub(crate) fn ___reduce26<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr = FunctionCallResultExpr => ActionFn(29);
        let ___sym0 = ___pop_Variant15(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action29::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant14(___nt), ___end));
        (1, 13)
    }
    pub(crate) fn ___reduce27<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr = VariableComparisonExpr => ActionFn(30);
        let ___sym0 = ___pop_Variant15(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action30::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant14(___nt), ___end));
        (1, 13)
    }
    pub(crate) fn ___reduce28<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr = InlineCodeExpr => ActionFn(31);
        let ___sym0 = ___pop_Variant15(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action31::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant14(___nt), ___end));
        (1, 13)
    }
    pub(crate) fn ___reduce29<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr = NegativeExpr => ActionFn(32);
        let ___sym0 = ___pop_Variant15(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action32::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant14(___nt), ___end));
        (1, 13)
    }
    pub(crate) fn ___reduce30<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr = "(", Expr, ")" => ActionFn(33);
        assert!(___symbols.len() >= 3);
        let ___sym2 = ___pop_Variant0(___symbols);
        let ___sym1 = ___pop_Variant14(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym2.2;
        let ___nt = super::___action33::<>(text, ___sym0, ___sym1, ___sym2);
        ___symbols.push((___start, ___Symbol::Variant14(___nt), ___end));
        (3, 13)
    }
    pub(crate) fn ___reduce31<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // FunctionCallResultExpr = Ident, "(", ")" => ActionFn(78);
        assert!(___symbols.len() >= 3);
        let ___sym2 = ___pop_Variant0(___symbols);
        let ___sym1 = ___pop_Variant0(___symbols);
        let ___sym0 = ___pop_Variant17(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym2.2;
        let ___nt = super::___action78::<>(text, ___sym0, ___sym1, ___sym2);
        ___symbols.push((___start, ___Symbol::Variant15(___nt), ___end));
        (3, 14)
    }
    pub(crate) fn ___reduce32<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // FunctionCallStatement = Ident, "(", ")" => ActionFn(79);
        assert!(___symbols.len() >= 3);
        let ___sym2 = ___pop_Variant0(___symbols);
        let ___sym1 = ___pop_Variant0(___symbols);
        let ___sym0 = ___pop_Variant17(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym2.2;
        let ___nt = super::___action79::<>(text, ___sym0, ___sym1, ___sym2);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (3, 15)
    }
    pub(crate) fn ___reduce33<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // FunctionDefinition = "inline", "fn", Ident, Params, "->", ReturnType, StatementBlock => ActionFn(80);
        assert!(___symbols.len() >= 7);
        let ___sym6 = ___pop_Variant16(___symbols);
        let ___sym5 = ___pop_Variant22(___symbols);
        let ___sym4 = ___pop_Variant0(___symbols);
        let ___sym3 = ___pop_Variant9(___symbols);
        let ___sym2 = ___pop_Variant17(___symbols);
        let ___sym1 = ___pop_Variant0(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym6.2;
        let ___nt = super::___action80::<>(text, ___sym0, ___sym1, ___sym2, ___sym3, ___sym4, ___sym5, ___sym6);
        ___symbols.push((___start, ___Symbol::Variant12(___nt), ___end));
        (7, 16)
    }
    pub(crate) fn ___reduce34<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // FunctionDefinition = "inline", "fn", Ident, Params, StatementBlock => ActionFn(81);
        assert!(___symbols.len() >= 5);
        let ___sym4 = ___pop_Variant16(___symbols);
        let ___sym3 = ___pop_Variant9(___symbols);
        let ___sym2 = ___pop_Variant17(___symbols);
        let ___sym1 = ___pop_Variant0(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym4.2;
        let ___nt = super::___action81::<>(text, ___sym0, ___sym1, ___sym2, ___sym3, ___sym4);
        ___symbols.push((___start, ___Symbol::Variant12(___nt), ___end));
        (5, 16)
    }
    pub(crate) fn ___reduce35<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // FunctionDefinition = "fn", Ident, Params, "->", ReturnType, StatementBlock => ActionFn(82);
        assert!(___symbols.len() >= 6);
        let ___sym5 = ___pop_Variant16(___symbols);
        let ___sym4 = ___pop_Variant22(___symbols);
        let ___sym3 = ___pop_Variant0(___symbols);
        let ___sym2 = ___pop_Variant9(___symbols);
        let ___sym1 = ___pop_Variant17(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym5.2;
        let ___nt = super::___action82::<>(text, ___sym0, ___sym1, ___sym2, ___sym3, ___sym4, ___sym5);
        ___symbols.push((___start, ___Symbol::Variant12(___nt), ___end));
        (6, 16)
    }
    pub(crate) fn ___reduce36<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // FunctionDefinition = "fn", Ident, Params, StatementBlock => ActionFn(83);
        assert!(___symbols.len() >= 4);
        let ___sym3 = ___pop_Variant16(___symbols);
        let ___sym2 = ___pop_Variant9(___symbols);
        let ___sym1 = ___pop_Variant17(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym3.2;
        let ___nt = super::___action83::<>(text, ___sym0, ___sym1, ___sym2, ___sym3);
        ___symbols.push((___start, ___Symbol::Variant12(___nt), ___end));
        (4, 16)
    }
    pub(crate) fn ___reduce37<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Ident = "Id" => ActionFn(4);
        let ___sym0 = ___pop_Variant1(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action4::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant17(___nt), ___end));
        (1, 17)
    }
    pub(crate) fn ___reduce38<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // IfStatement = "if", Expr, StatementBlock, "else", Statement => ActionFn(84);
        assert!(___symbols.len() >= 5);
        let ___sym4 = ___pop_Variant16(___symbols);
        let ___sym3 = ___pop_Variant0(___symbols);
        let ___sym2 = ___pop_Variant16(___symbols);
        let ___sym1 = ___pop_Variant14(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym4.2;
        let ___nt = super::___action84::<>(text, ___sym0, ___sym1, ___sym2, ___sym3, ___sym4);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (5, 18)
    }
    pub(crate) fn ___reduce39<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // IfStatement = "if", Expr, StatementBlock => ActionFn(85);
        assert!(___symbols.len() >= 3);
        let ___sym2 = ___pop_Variant16(___symbols);
        let ___sym1 = ___pop_Variant14(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym2.2;
        let ___nt = super::___action85::<>(text, ___sym0, ___sym1, ___sym2);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (3, 18)
    }
    pub(crate) fn ___reduce40<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // InlineCode = "__code__", "(", "StringLiteral", ")" => ActionFn(22);
        assert!(___symbols.len() >= 4);
        let ___sym3 = ___pop_Variant0(___symbols);
        let ___sym2 = ___pop_Variant1(___symbols);
        let ___sym1 = ___pop_Variant0(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym3.2;
        let ___nt = super::___action22::<>(text, ___sym0, ___sym1, ___sym2, ___sym3);
        ___symbols.push((___start, ___Symbol::Variant18(___nt), ___end));
        (4, 19)
    }
    pub(crate) fn ___reduce41<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // InlineCodeExpr = InlineCode => ActionFn(86);
        let ___sym0 = ___pop_Variant18(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action86::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant15(___nt), ___end));
        (1, 20)
    }
    pub(crate) fn ___reduce42<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // InlineCodeStatement = InlineCode => ActionFn(87);
        let ___sym0 = ___pop_Variant18(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action87::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (1, 21)
    }
    pub(crate) fn ___reduce43<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Integer = "Integer" => ActionFn(47);
        let ___sym0 = ___pop_Variant2(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action47::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant19(___nt), ___end));
        (1, 22)
    }
    pub(crate) fn ___reduce44<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // IntegerExpr = Integer => ActionFn(38);
        let ___sym0 = ___pop_Variant19(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action38::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant20(___nt), ___end));
        (1, 23)
    }
    pub(crate) fn ___reduce45<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // IntegerExpr = Ident => ActionFn(39);
        let ___sym0 = ___pop_Variant17(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action39::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant20(___nt), ___end));
        (1, 23)
    }
    pub(crate) fn ___reduce46<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LoopStatement = "loop", StatementBlock => ActionFn(88);
        assert!(___symbols.len() >= 2);
        let ___sym1 = ___pop_Variant16(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym1.2;
        let ___nt = super::___action88::<>(text, ___sym0, ___sym1);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (2, 24)
    }
    pub(crate) fn ___reduce47<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // NegativeExpr = "!", Expr => ActionFn(89);
        assert!(___symbols.len() >= 2);
        let ___sym1 = ___pop_Variant14(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym1.2;
        let ___nt = super::___action89::<>(text, ___sym0, ___sym1);
        ___symbols.push((___start, ___Symbol::Variant15(___nt), ___end));
        (2, 25)
    }
    pub(crate) fn ___reduce48<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Params = "(", ArgumentType, ")" => ActionFn(95);
        assert!(___symbols.len() >= 3);
        let ___sym2 = ___pop_Variant0(___symbols);
        let ___sym1 = ___pop_Variant9(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym2.2;
        let ___nt = super::___action95::<>(text, ___sym0, ___sym1, ___sym2);
        ___symbols.push((___start, ___Symbol::Variant9(___nt), ___end));
        (3, 26)
    }
    pub(crate) fn ___reduce49<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Params = "(", ")" => ActionFn(96);
        assert!(___symbols.len() >= 2);
        let ___sym1 = ___pop_Variant0(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym1.2;
        let ___nt = super::___action96::<>(text, ___sym0, ___sym1);
        ___symbols.push((___start, ___Symbol::Variant9(___nt), ___end));
        (2, 26)
    }
    pub(crate) fn ___reduce50<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program =  => ActionFn(97);
        let ___start = ___lookahead_start.cloned().or_else(|| ___symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let ___end = ___start.clone();
        let ___nt = super::___action97::<>(text, &___start, &___end);
        ___symbols.push((___start, ___Symbol::Variant21(___nt), ___end));
        (0, 27)
    }
    pub(crate) fn ___reduce51<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = Definition+ => ActionFn(98);
        let ___sym0 = ___pop_Variant13(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action98::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant21(___nt), ___end));
        (1, 27)
    }
    pub(crate) fn ___reduce52<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ReturnStatement = "return" => ActionFn(90);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action90::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (1, 28)
    }
    pub(crate) fn ___reduce53<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ReturnStatement = "return", Expr => ActionFn(91);
        assert!(___symbols.len() >= 2);
        let ___sym1 = ___pop_Variant14(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym1.2;
        let ___nt = super::___action91::<>(text, ___sym0, ___sym1);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (2, 28)
    }
    pub(crate) fn ___reduce54<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ReturnType = "arg" => ActionFn(8);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action8::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant22(___nt), ___end));
        (1, 29)
    }
    pub(crate) fn ___reduce55<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ReturnType = "state" => ActionFn(9);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action9::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant22(___nt), ___end));
        (1, 29)
    }
    pub(crate) fn ___reduce56<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // SingleStatement = FunctionCallStatement => ActionFn(18);
        let ___sym0 = ___pop_Variant16(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action18::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (1, 30)
    }
    pub(crate) fn ___reduce57<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // SingleStatement = ReturnStatement => ActionFn(19);
        let ___sym0 = ___pop_Variant16(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action19::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (1, 30)
    }
    pub(crate) fn ___reduce58<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // SingleStatement = InlineCodeStatement => ActionFn(20);
        let ___sym0 = ___pop_Variant16(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action20::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (1, 30)
    }
    pub(crate) fn ___reduce59<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement = SingleStatement, ";" => ActionFn(13);
        assert!(___symbols.len() >= 2);
        let ___sym1 = ___pop_Variant0(___symbols);
        let ___sym0 = ___pop_Variant16(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym1.2;
        let ___nt = super::___action13::<>(text, ___sym0, ___sym1);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (2, 31)
    }
    pub(crate) fn ___reduce60<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement = StatementBlock => ActionFn(14);
        let ___sym0 = ___pop_Variant16(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action14::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (1, 31)
    }
    pub(crate) fn ___reduce61<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement = LoopStatement => ActionFn(15);
        let ___sym0 = ___pop_Variant16(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action15::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (1, 31)
    }
    pub(crate) fn ___reduce62<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement = WhileStatement => ActionFn(16);
        let ___sym0 = ___pop_Variant16(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action16::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (1, 31)
    }
    pub(crate) fn ___reduce63<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement = IfStatement => ActionFn(17);
        let ___sym0 = ___pop_Variant16(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action17::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (1, 31)
    }
    pub(crate) fn ___reduce64<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement* =  => ActionFn(51);
        let ___start = ___lookahead_start.cloned().or_else(|| ___symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let ___end = ___start.clone();
        let ___nt = super::___action51::<>(text, &___start, &___end);
        ___symbols.push((___start, ___Symbol::Variant23(___nt), ___end));
        (0, 32)
    }
    pub(crate) fn ___reduce65<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement* = Statement+ => ActionFn(52);
        let ___sym0 = ___pop_Variant23(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action52::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant23(___nt), ___end));
        (1, 32)
    }
    pub(crate) fn ___reduce66<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement+ = Statement => ActionFn(65);
        let ___sym0 = ___pop_Variant16(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action65::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant23(___nt), ___end));
        (1, 33)
    }
    pub(crate) fn ___reduce67<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement+ = Statement+, Statement => ActionFn(66);
        assert!(___symbols.len() >= 2);
        let ___sym1 = ___pop_Variant16(___symbols);
        let ___sym0 = ___pop_Variant23(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym1.2;
        let ___nt = super::___action66::<>(text, ___sym0, ___sym1);
        ___symbols.push((___start, ___Symbol::Variant23(___nt), ___end));
        (2, 33)
    }
    pub(crate) fn ___reduce68<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // StatementBlock = "{", "}" => ActionFn(99);
        assert!(___symbols.len() >= 2);
        let ___sym1 = ___pop_Variant0(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym1.2;
        let ___nt = super::___action99::<>(text, ___sym0, ___sym1);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (2, 34)
    }
    pub(crate) fn ___reduce69<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // StatementBlock = "{", Statement+, "}" => ActionFn(100);
        assert!(___symbols.len() >= 3);
        let ___sym2 = ___pop_Variant0(___symbols);
        let ___sym1 = ___pop_Variant23(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym2.2;
        let ___nt = super::___action100::<>(text, ___sym0, ___sym1, ___sym2);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (3, 34)
    }
    pub(crate) fn ___reduce70<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Variable = "Var" => ActionFn(40);
        let ___sym0 = ___pop_Variant1(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action40::<>(text, ___sym0);
        ___symbols.push((___start, ___Symbol::Variant24(___nt), ___end));
        (1, 35)
    }
    pub(crate) fn ___reduce71<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // VariableComparisonExpr = Variable, ComparisonOp, IntegerExpr => ActionFn(93);
        assert!(___symbols.len() >= 3);
        let ___sym2 = ___pop_Variant20(___symbols);
        let ___sym1 = ___pop_Variant11(___symbols);
        let ___sym0 = ___pop_Variant24(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym2.2;
        let ___nt = super::___action93::<>(text, ___sym0, ___sym1, ___sym2);
        ___symbols.push((___start, ___Symbol::Variant15(___nt), ___end));
        (3, 36)
    }
    pub(crate) fn ___reduce72<
        'input,
    >(
        text: &'input str,
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // WhileStatement = "while", Expr, StatementBlock => ActionFn(94);
        assert!(___symbols.len() >= 3);
        let ___sym2 = ___pop_Variant16(___symbols);
        let ___sym1 = ___pop_Variant14(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym2.2;
        let ___nt = super::___action94::<>(text, ___sym0, ___sym1, ___sym2);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (3, 37)
    }
}
pub use self::___parse___Program::ProgramParser;

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action0<
    'input,
>(
    text: &'input str,
    (_, ___0, _): (usize, ast::Program, usize),
) -> ast::Program
{
    ___0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action1<
    'input,
>(
    text: &'input str,
    (_, definitions, _): (usize, alloc::vec::Vec<ast::Definition>, usize),
) -> ast::Program
{
    ast::Program { definitions }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action2<
    'input,
>(
    text: &'input str,
    (_, ___0, _): (usize, ast::Definition, usize),
) -> ast::Definition
{
    ___0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action3<
    'input,
>(
    text: &'input str,
    (_, ___0, _): (usize, ast::Definition, usize),
) -> ast::Definition
{
    ___0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action4<
    'input,
>(
    text: &'input str,
    (_, i, _): (usize, &'input str, usize),
) -> ast::Ident
{
    i.into()
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action5<
    'input,
>(
    text: &'input str,
    (_, location, _): (usize, usize, usize),
    (_, is_inline, _): (usize, core::option::Option<Tok<'input>>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, name, _): (usize, ast::Ident, usize),
    (_, arg, _): (usize, ast::ArgumentType, usize),
    (_, returns, _): (usize, core::option::Option<(Tok<'input>, ast::ReturnType)>, usize),
    (_, block, _): (usize, ast::Statement, usize),
) -> ast::Definition
{
    {
        let returns = match returns {
            None => ast::ReturnType::Nothing,
            Some((_, rt)) => rt,
        };
        let node = if is_inline.is_none() {
            ast::DefinitionKind::Function { name, arg, returns, body: vec![block] }
        } else {
            // TODO: error if there are any args or returns
            ast::DefinitionKind::InlineFunction { name, body: vec![block] }
        };
        ast::Definition::new(location, node)
    }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action6<
    'input,
>(
    text: &'input str,
    (_, location, _): (usize, usize, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, name, _): (usize, ast::Ident, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, value, _): (usize, ast::Integer, usize),
    (_, _, _): (usize, Tok<'input>, usize),
) -> ast::Definition
{
    {
        let node = ast::DefinitionKind::Const { name, value };
        ast::Definition::new(location, node)
    }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action7<
    'input,
>(
    text: &'input str,
    (_, _, _): (usize, Tok<'input>, usize),
    (_, arg, _): (usize, core::option::Option<ast::ArgumentType>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
) -> ast::ArgumentType
{
    {
        arg.unwrap_or(ast::ArgumentType::Nothing)
    }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action8<
    'input,
>(
    text: &'input str,
    (_, ___0, _): (usize, Tok<'input>, usize),
) -> ast::ReturnType
{
    ast::ReturnType::Argument
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action9<
    'input,
>(
    text: &'input str,
    (_, ___0, _): (usize, Tok<'input>, usize),
) -> ast::ReturnType
{
    ast::ReturnType::State
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action10<
    'input,
>(
    text: &'input str,
    (_, ___0, _): (usize, Tok<'input>, usize),
) -> ast::ArgumentType
{
    ast::ArgumentType::Argument
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action11<
    'input,
>(
    text: &'input str,
    (_, ___0, _): (usize, Tok<'input>, usize),
) -> ast::ArgumentType
{
    ast::ArgumentType::State
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action12<
    'input,
>(
    text: &'input str,
    (_, location, _): (usize, usize, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, body, _): (usize, alloc::vec::Vec<ast::Statement>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
) -> ast::Statement
{
    {
        ast::Statement::new(
            location,
            ast::StatementKind::StatementBlock { body }
        )
    }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action13<
    'input,
>(
    text: &'input str,
    (_, s, _): (usize, ast::Statement, usize),
    (_, _, _): (usize, Tok<'input>, usize),
) -> ast::Statement
{
    s
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action14<
    'input,
>(
    text: &'input str,
    (_, ___0, _): (usize, ast::Statement, usize),
) -> ast::Statement
{
    ___0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action15<
    'input,
>(
    text: &'input str,
    (_, ___0, _): (usize, ast::Statement, usize),
) -> ast::Statement
{
    ___0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action16<
    'input,
>(
    text: &'input str,
    (_, ___0, _): (usize, ast::Statement, usize),
) -> ast::Statement
{
    ___0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action17<
    'input,
>(
    text: &'input str,
    (_, ___0, _): (usize, ast::Statement, usize),
) -> ast::Statement
{
    ___0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action18<
    'input,
>(
    text: &'input str,
    (_, ___0, _): (usize, ast::Statement, usize),
) -> ast::Statement
{
    ___0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action19<
    'input,
>(
    text: &'input str,
    (_, ___0, _): (usize, ast::Statement, usize),
) -> ast::Statement
{
    ___0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action20<
    'input,
>(
    text: &'input str,
    (_, ___0, _): (usize, ast::Statement, usize),
) -> ast::Statement
{
    ___0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action21<
    'input,
>(
    text: &'input str,
    (_, location, _): (usize, usize, usize),
    (_, body, _): (usize, ast::InlineCode, usize),
) -> ast::Statement
{
    {
        let node = ast::StatementKind::InlineCode { body };
        ast::Statement::new(location, node)
    }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action22<
    'input,
>(
    text: &'input str,
    (_, _, _): (usize, Tok<'input>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, code, _): (usize, &'input str, usize),
    (_, _, _): (usize, Tok<'input>, usize),
) -> ast::InlineCode
{
    ast::InlineCode { code: code.into() }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action23<
    'input,
>(
    text: &'input str,
    (_, location, _): (usize, usize, usize),
    (_, _, _): (usize, Tok<'input>, usize),
) -> ast::Statement
{
    {
        ast::Statement::new(location, ast::StatementKind::Return { expr: None })
    }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action24<
    'input,
>(
    text: &'input str,
    (_, location, _): (usize, usize, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, expr, _): (usize, Box<ast::Expr>, usize),
) -> ast::Statement
{
    {
        ast::Statement::new(location, ast::StatementKind::Return { expr: Some(expr) })
    }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action25<
    'input,
>(
    text: &'input str,
    (_, location, _): (usize, usize, usize),
    (_, name, _): (usize, ast::Ident, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
) -> ast::Statement
{
    {
        let node = ast::StatementKind::FunctionCall { name };
        ast::Statement { location, node }
    }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action26<
    'input,
>(
    text: &'input str,
    (_, location, _): (usize, usize, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, block, _): (usize, ast::Statement, usize),
) -> ast::Statement
{
    {
        let node = ast::StatementKind::Loop { body: vec![block] };
        ast::Statement::new(location, node)
    }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action27<
    'input,
>(
    text: &'input str,
    (_, location, _): (usize, usize, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, condition, _): (usize, Box<ast::Expr>, usize),
    (_, block, _): (usize, ast::Statement, usize),
) -> ast::Statement
{
    {
        let node = ast::StatementKind::While { condition, body: vec![block] };
        ast::Statement::new(location, node)
    }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action28<
    'input,
>(
    text: &'input str,
    (_, location, _): (usize, usize, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, condition, _): (usize, Box<ast::Expr>, usize),
    (_, block, _): (usize, ast::Statement, usize),
    (_, orelse, _): (usize, core::option::Option<(Tok<'input>, ast::Statement)>, usize),
) -> ast::Statement
{
    {
        let orelse = match orelse {
            Some((_, stmt)) => vec![stmt],
            None => vec![],
        };
        let node = ast::StatementKind::If { condition, body: vec![block], orelse };
        ast::Statement::new(location, node)
    }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action29<
    'input,
>(
    text: &'input str,
    (_, ___0, _): (usize, ast::Expr, usize),
) -> Box<ast::Expr>
{
    Box::new(___0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action30<
    'input,
>(
    text: &'input str,
    (_, ___0, _): (usize, ast::Expr, usize),
) -> Box<ast::Expr>
{
    Box::new(___0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action31<
    'input,
>(
    text: &'input str,
    (_, ___0, _): (usize, ast::Expr, usize),
) -> Box<ast::Expr>
{
    Box::new(___0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action32<
    'input,
>(
    text: &'input str,
    (_, ___0, _): (usize, ast::Expr, usize),
) -> Box<ast::Expr>
{
    Box::new(___0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action33<
    'input,
>(
    text: &'input str,
    (_, _, _): (usize, Tok<'input>, usize),
    (_, c, _): (usize, Box<ast::Expr>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
) -> Box<ast::Expr>
{
    c
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action34<
    'input,
>(
    text: &'input str,
    (_, location, _): (usize, usize, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, expr, _): (usize, Box<ast::Expr>, usize),
) -> ast::Expr
{
    {
        let node = ast::ExprKind::Negative { expr };
        ast::Expr::new(location, node)
    }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action35<
    'input,
>(
    text: &'input str,
    (_, location, _): (usize, usize, usize),
    (_, body, _): (usize, ast::InlineCode, usize),
) -> ast::Expr
{
    {
         let node = ast::ExprKind::InlineCode { body };
         ast::Expr::new(location, node)
     }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action36<
    'input,
>(
    text: &'input str,
    (_, location, _): (usize, usize, usize),
    (_, name, _): (usize, ast::Ident, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
) -> ast::Expr
{
    {
        // TODO: check that function returns an argument or a state.
        let node = ast::ExprKind::FunctionCallResult { name };
        ast::Expr::new(location, node)
    }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action37<
    'input,
>(
    text: &'input str,
    (_, location, _): (usize, usize, usize),
    (_, var, _): (usize, ast::Variable, usize),
    (_, op, _): (usize, ast::ComparisonOp, usize),
    (_, value, _): (usize, ast::IntegerExpr, usize),
) -> ast::Expr
{
    {
        let node = ast::ExprKind::VariableComparison { var, value, op };
        ast::Expr::new(location, node)
    }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action38<
    'input,
>(
    text: &'input str,
    (_, value, _): (usize, ast::Integer, usize),
) -> ast::IntegerExpr
{
    ast::IntegerExpr::Literal { value }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action39<
    'input,
>(
    text: &'input str,
    (_, name, _): (usize, ast::Ident, usize),
) -> ast::IntegerExpr
{
    ast::IntegerExpr::Const { name }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action40<
    'input,
>(
    text: &'input str,
    (_, v, _): (usize, &'input str, usize),
) -> ast::Variable
{
    ast::Variable { name: v.into() }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action41<
    'input,
>(
    text: &'input str,
    (_, ___0, _): (usize, Tok<'input>, usize),
) -> ast::ComparisonOp
{
    ast::ComparisonOp::Eq
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action42<
    'input,
>(
    text: &'input str,
    (_, ___0, _): (usize, Tok<'input>, usize),
) -> ast::ComparisonOp
{
    ast::ComparisonOp::NotEq
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action43<
    'input,
>(
    text: &'input str,
    (_, ___0, _): (usize, Tok<'input>, usize),
) -> ast::ComparisonOp
{
    ast::ComparisonOp::GreaterThan
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action44<
    'input,
>(
    text: &'input str,
    (_, ___0, _): (usize, Tok<'input>, usize),
) -> ast::ComparisonOp
{
    ast::ComparisonOp::LessThan
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action45<
    'input,
>(
    text: &'input str,
    (_, ___0, _): (usize, Tok<'input>, usize),
) -> ast::ComparisonOp
{
    ast::ComparisonOp::GreaterOrEqual
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action46<
    'input,
>(
    text: &'input str,
    (_, ___0, _): (usize, Tok<'input>, usize),
) -> ast::ComparisonOp
{
    ast::ComparisonOp::LessOrEqual
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action47<
    'input,
>(
    text: &'input str,
    (_, i, _): (usize, u32, usize),
) -> ast::Integer
{
    i
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action48<
    'input,
>(
    text: &'input str,
    (_, ___0, _): (usize, (Tok<'input>, ast::Statement), usize),
) -> core::option::Option<(Tok<'input>, ast::Statement)>
{
    Some(___0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action49<
    'input,
>(
    text: &'input str,
    ___lookbehind: &usize,
    ___lookahead: &usize,
) -> core::option::Option<(Tok<'input>, ast::Statement)>
{
    None
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action50<
    'input,
>(
    text: &'input str,
    (_, ___0, _): (usize, Tok<'input>, usize),
    (_, ___1, _): (usize, ast::Statement, usize),
) -> (Tok<'input>, ast::Statement)
{
    (___0, ___1)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action51<
    'input,
>(
    text: &'input str,
    ___lookbehind: &usize,
    ___lookahead: &usize,
) -> alloc::vec::Vec<ast::Statement>
{
    alloc::vec![]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action52<
    'input,
>(
    text: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<ast::Statement>, usize),
) -> alloc::vec::Vec<ast::Statement>
{
    v
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action53<
    'input,
>(
    text: &'input str,
    (_, ___0, _): (usize, ast::ArgumentType, usize),
) -> core::option::Option<ast::ArgumentType>
{
    Some(___0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action54<
    'input,
>(
    text: &'input str,
    ___lookbehind: &usize,
    ___lookahead: &usize,
) -> core::option::Option<ast::ArgumentType>
{
    None
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action55<
    'input,
>(
    text: &'input str,
    (_, ___0, _): (usize, (Tok<'input>, ast::ReturnType), usize),
) -> core::option::Option<(Tok<'input>, ast::ReturnType)>
{
    Some(___0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action56<
    'input,
>(
    text: &'input str,
    ___lookbehind: &usize,
    ___lookahead: &usize,
) -> core::option::Option<(Tok<'input>, ast::ReturnType)>
{
    None
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action57<
    'input,
>(
    text: &'input str,
    (_, ___0, _): (usize, Tok<'input>, usize),
    (_, ___1, _): (usize, ast::ReturnType, usize),
) -> (Tok<'input>, ast::ReturnType)
{
    (___0, ___1)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action58<
    'input,
>(
    text: &'input str,
    (_, ___0, _): (usize, Tok<'input>, usize),
) -> core::option::Option<Tok<'input>>
{
    Some(___0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action59<
    'input,
>(
    text: &'input str,
    ___lookbehind: &usize,
    ___lookahead: &usize,
) -> core::option::Option<Tok<'input>>
{
    None
}

#[allow(unused_variables)]
fn ___action60<
    'input,
>(
    text: &'input str,
    ___lookbehind: &usize,
    ___lookahead: &usize,
) -> usize
{
    *___lookahead
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action61<
    'input,
>(
    text: &'input str,
    ___lookbehind: &usize,
    ___lookahead: &usize,
) -> alloc::vec::Vec<ast::Definition>
{
    alloc::vec![]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action62<
    'input,
>(
    text: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<ast::Definition>, usize),
) -> alloc::vec::Vec<ast::Definition>
{
    v
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action63<
    'input,
>(
    text: &'input str,
    (_, ___0, _): (usize, ast::Definition, usize),
) -> alloc::vec::Vec<ast::Definition>
{
    alloc::vec![___0]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action64<
    'input,
>(
    text: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<ast::Definition>, usize),
    (_, e, _): (usize, ast::Definition, usize),
) -> alloc::vec::Vec<ast::Definition>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action65<
    'input,
>(
    text: &'input str,
    (_, ___0, _): (usize, ast::Statement, usize),
) -> alloc::vec::Vec<ast::Statement>
{
    alloc::vec![___0]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action66<
    'input,
>(
    text: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<ast::Statement>, usize),
    (_, e, _): (usize, ast::Statement, usize),
) -> alloc::vec::Vec<ast::Statement>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action67<
    'input,
>(
    text: &'input str,
    ___0: (usize, usize, usize),
    ___1: (usize, Tok<'input>, usize),
    ___2: (usize, Tok<'input>, usize),
    ___3: (usize, ast::Ident, usize),
    ___4: (usize, ast::ArgumentType, usize),
    ___5: (usize, core::option::Option<(Tok<'input>, ast::ReturnType)>, usize),
    ___6: (usize, ast::Statement, usize),
) -> ast::Definition
{
    let ___start0 = ___1.0;
    let ___end0 = ___1.2;
    let ___temp0 = ___action58(
        text,
        ___1,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action5(
        text,
        ___0,
        ___temp0,
        ___2,
        ___3,
        ___4,
        ___5,
        ___6,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action68<
    'input,
>(
    text: &'input str,
    ___0: (usize, usize, usize),
    ___1: (usize, Tok<'input>, usize),
    ___2: (usize, ast::Ident, usize),
    ___3: (usize, ast::ArgumentType, usize),
    ___4: (usize, core::option::Option<(Tok<'input>, ast::ReturnType)>, usize),
    ___5: (usize, ast::Statement, usize),
) -> ast::Definition
{
    let ___start0 = ___0.2;
    let ___end0 = ___1.0;
    let ___temp0 = ___action59(
        text,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action5(
        text,
        ___0,
        ___temp0,
        ___1,
        ___2,
        ___3,
        ___4,
        ___5,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action69<
    'input,
>(
    text: &'input str,
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, ast::ReturnType, usize),
) -> core::option::Option<(Tok<'input>, ast::ReturnType)>
{
    let ___start0 = ___0.0;
    let ___end0 = ___1.2;
    let ___temp0 = ___action57(
        text,
        ___0,
        ___1,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action55(
        text,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action70<
    'input,
>(
    text: &'input str,
    ___0: (usize, usize, usize),
    ___1: (usize, Tok<'input>, usize),
    ___2: (usize, Tok<'input>, usize),
    ___3: (usize, ast::Ident, usize),
    ___4: (usize, ast::ArgumentType, usize),
    ___5: (usize, Tok<'input>, usize),
    ___6: (usize, ast::ReturnType, usize),
    ___7: (usize, ast::Statement, usize),
) -> ast::Definition
{
    let ___start0 = ___5.0;
    let ___end0 = ___6.2;
    let ___temp0 = ___action69(
        text,
        ___5,
        ___6,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action67(
        text,
        ___0,
        ___1,
        ___2,
        ___3,
        ___4,
        ___temp0,
        ___7,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action71<
    'input,
>(
    text: &'input str,
    ___0: (usize, usize, usize),
    ___1: (usize, Tok<'input>, usize),
    ___2: (usize, Tok<'input>, usize),
    ___3: (usize, ast::Ident, usize),
    ___4: (usize, ast::ArgumentType, usize),
    ___5: (usize, ast::Statement, usize),
) -> ast::Definition
{
    let ___start0 = ___4.2;
    let ___end0 = ___5.0;
    let ___temp0 = ___action56(
        text,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action67(
        text,
        ___0,
        ___1,
        ___2,
        ___3,
        ___4,
        ___temp0,
        ___5,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action72<
    'input,
>(
    text: &'input str,
    ___0: (usize, usize, usize),
    ___1: (usize, Tok<'input>, usize),
    ___2: (usize, ast::Ident, usize),
    ___3: (usize, ast::ArgumentType, usize),
    ___4: (usize, Tok<'input>, usize),
    ___5: (usize, ast::ReturnType, usize),
    ___6: (usize, ast::Statement, usize),
) -> ast::Definition
{
    let ___start0 = ___4.0;
    let ___end0 = ___5.2;
    let ___temp0 = ___action69(
        text,
        ___4,
        ___5,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action68(
        text,
        ___0,
        ___1,
        ___2,
        ___3,
        ___temp0,
        ___6,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action73<
    'input,
>(
    text: &'input str,
    ___0: (usize, usize, usize),
    ___1: (usize, Tok<'input>, usize),
    ___2: (usize, ast::Ident, usize),
    ___3: (usize, ast::ArgumentType, usize),
    ___4: (usize, ast::Statement, usize),
) -> ast::Definition
{
    let ___start0 = ___3.2;
    let ___end0 = ___4.0;
    let ___temp0 = ___action56(
        text,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action68(
        text,
        ___0,
        ___1,
        ___2,
        ___3,
        ___temp0,
        ___4,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action74<
    'input,
>(
    text: &'input str,
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, ast::Statement, usize),
) -> core::option::Option<(Tok<'input>, ast::Statement)>
{
    let ___start0 = ___0.0;
    let ___end0 = ___1.2;
    let ___temp0 = ___action50(
        text,
        ___0,
        ___1,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action48(
        text,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action75<
    'input,
>(
    text: &'input str,
    ___0: (usize, usize, usize),
    ___1: (usize, Tok<'input>, usize),
    ___2: (usize, Box<ast::Expr>, usize),
    ___3: (usize, ast::Statement, usize),
    ___4: (usize, Tok<'input>, usize),
    ___5: (usize, ast::Statement, usize),
) -> ast::Statement
{
    let ___start0 = ___4.0;
    let ___end0 = ___5.2;
    let ___temp0 = ___action74(
        text,
        ___4,
        ___5,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action28(
        text,
        ___0,
        ___1,
        ___2,
        ___3,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action76<
    'input,
>(
    text: &'input str,
    ___0: (usize, usize, usize),
    ___1: (usize, Tok<'input>, usize),
    ___2: (usize, Box<ast::Expr>, usize),
    ___3: (usize, ast::Statement, usize),
) -> ast::Statement
{
    let ___start0 = ___3.2;
    let ___end0 = ___3.2;
    let ___temp0 = ___action49(
        text,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action28(
        text,
        ___0,
        ___1,
        ___2,
        ___3,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action77<
    'input,
>(
    text: &'input str,
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, ast::Ident, usize),
    ___2: (usize, Tok<'input>, usize),
    ___3: (usize, ast::Integer, usize),
    ___4: (usize, Tok<'input>, usize),
) -> ast::Definition
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(
        text,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action6(
        text,
        ___temp0,
        ___0,
        ___1,
        ___2,
        ___3,
        ___4,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action78<
    'input,
>(
    text: &'input str,
    ___0: (usize, ast::Ident, usize),
    ___1: (usize, Tok<'input>, usize),
    ___2: (usize, Tok<'input>, usize),
) -> ast::Expr
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(
        text,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action36(
        text,
        ___temp0,
        ___0,
        ___1,
        ___2,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action79<
    'input,
>(
    text: &'input str,
    ___0: (usize, ast::Ident, usize),
    ___1: (usize, Tok<'input>, usize),
    ___2: (usize, Tok<'input>, usize),
) -> ast::Statement
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(
        text,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action25(
        text,
        ___temp0,
        ___0,
        ___1,
        ___2,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action80<
    'input,
>(
    text: &'input str,
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, Tok<'input>, usize),
    ___2: (usize, ast::Ident, usize),
    ___3: (usize, ast::ArgumentType, usize),
    ___4: (usize, Tok<'input>, usize),
    ___5: (usize, ast::ReturnType, usize),
    ___6: (usize, ast::Statement, usize),
) -> ast::Definition
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(
        text,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action70(
        text,
        ___temp0,
        ___0,
        ___1,
        ___2,
        ___3,
        ___4,
        ___5,
        ___6,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action81<
    'input,
>(
    text: &'input str,
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, Tok<'input>, usize),
    ___2: (usize, ast::Ident, usize),
    ___3: (usize, ast::ArgumentType, usize),
    ___4: (usize, ast::Statement, usize),
) -> ast::Definition
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(
        text,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action71(
        text,
        ___temp0,
        ___0,
        ___1,
        ___2,
        ___3,
        ___4,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action82<
    'input,
>(
    text: &'input str,
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, ast::Ident, usize),
    ___2: (usize, ast::ArgumentType, usize),
    ___3: (usize, Tok<'input>, usize),
    ___4: (usize, ast::ReturnType, usize),
    ___5: (usize, ast::Statement, usize),
) -> ast::Definition
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(
        text,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action72(
        text,
        ___temp0,
        ___0,
        ___1,
        ___2,
        ___3,
        ___4,
        ___5,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action83<
    'input,
>(
    text: &'input str,
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, ast::Ident, usize),
    ___2: (usize, ast::ArgumentType, usize),
    ___3: (usize, ast::Statement, usize),
) -> ast::Definition
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(
        text,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action73(
        text,
        ___temp0,
        ___0,
        ___1,
        ___2,
        ___3,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action84<
    'input,
>(
    text: &'input str,
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, Box<ast::Expr>, usize),
    ___2: (usize, ast::Statement, usize),
    ___3: (usize, Tok<'input>, usize),
    ___4: (usize, ast::Statement, usize),
) -> ast::Statement
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(
        text,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action75(
        text,
        ___temp0,
        ___0,
        ___1,
        ___2,
        ___3,
        ___4,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action85<
    'input,
>(
    text: &'input str,
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, Box<ast::Expr>, usize),
    ___2: (usize, ast::Statement, usize),
) -> ast::Statement
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(
        text,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action76(
        text,
        ___temp0,
        ___0,
        ___1,
        ___2,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action86<
    'input,
>(
    text: &'input str,
    ___0: (usize, ast::InlineCode, usize),
) -> ast::Expr
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(
        text,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action35(
        text,
        ___temp0,
        ___0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action87<
    'input,
>(
    text: &'input str,
    ___0: (usize, ast::InlineCode, usize),
) -> ast::Statement
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(
        text,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action21(
        text,
        ___temp0,
        ___0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action88<
    'input,
>(
    text: &'input str,
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, ast::Statement, usize),
) -> ast::Statement
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(
        text,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action26(
        text,
        ___temp0,
        ___0,
        ___1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action89<
    'input,
>(
    text: &'input str,
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, Box<ast::Expr>, usize),
) -> ast::Expr
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(
        text,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action34(
        text,
        ___temp0,
        ___0,
        ___1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action90<
    'input,
>(
    text: &'input str,
    ___0: (usize, Tok<'input>, usize),
) -> ast::Statement
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(
        text,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action23(
        text,
        ___temp0,
        ___0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action91<
    'input,
>(
    text: &'input str,
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, Box<ast::Expr>, usize),
) -> ast::Statement
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(
        text,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action24(
        text,
        ___temp0,
        ___0,
        ___1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action92<
    'input,
>(
    text: &'input str,
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, alloc::vec::Vec<ast::Statement>, usize),
    ___2: (usize, Tok<'input>, usize),
) -> ast::Statement
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(
        text,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action12(
        text,
        ___temp0,
        ___0,
        ___1,
        ___2,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action93<
    'input,
>(
    text: &'input str,
    ___0: (usize, ast::Variable, usize),
    ___1: (usize, ast::ComparisonOp, usize),
    ___2: (usize, ast::IntegerExpr, usize),
) -> ast::Expr
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(
        text,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action37(
        text,
        ___temp0,
        ___0,
        ___1,
        ___2,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action94<
    'input,
>(
    text: &'input str,
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, Box<ast::Expr>, usize),
    ___2: (usize, ast::Statement, usize),
) -> ast::Statement
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(
        text,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action27(
        text,
        ___temp0,
        ___0,
        ___1,
        ___2,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action95<
    'input,
>(
    text: &'input str,
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, ast::ArgumentType, usize),
    ___2: (usize, Tok<'input>, usize),
) -> ast::ArgumentType
{
    let ___start0 = ___1.0;
    let ___end0 = ___1.2;
    let ___temp0 = ___action53(
        text,
        ___1,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action7(
        text,
        ___0,
        ___temp0,
        ___2,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action96<
    'input,
>(
    text: &'input str,
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, Tok<'input>, usize),
) -> ast::ArgumentType
{
    let ___start0 = ___0.2;
    let ___end0 = ___1.0;
    let ___temp0 = ___action54(
        text,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action7(
        text,
        ___0,
        ___temp0,
        ___1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action97<
    'input,
>(
    text: &'input str,
    ___lookbehind: &usize,
    ___lookahead: &usize,
) -> ast::Program
{
    let ___start0 = *___lookbehind;
    let ___end0 = *___lookahead;
    let ___temp0 = ___action61(
        text,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action1(
        text,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action98<
    'input,
>(
    text: &'input str,
    ___0: (usize, alloc::vec::Vec<ast::Definition>, usize),
) -> ast::Program
{
    let ___start0 = ___0.0;
    let ___end0 = ___0.2;
    let ___temp0 = ___action62(
        text,
        ___0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action1(
        text,
        ___temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action99<
    'input,
>(
    text: &'input str,
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, Tok<'input>, usize),
) -> ast::Statement
{
    let ___start0 = ___0.2;
    let ___end0 = ___1.0;
    let ___temp0 = ___action51(
        text,
        &___start0,
        &___end0,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action92(
        text,
        ___0,
        ___temp0,
        ___1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn ___action100<
    'input,
>(
    text: &'input str,
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, alloc::vec::Vec<ast::Statement>, usize),
    ___2: (usize, Tok<'input>, usize),
) -> ast::Statement
{
    let ___start0 = ___1.0;
    let ___end0 = ___1.2;
    let ___temp0 = ___action52(
        text,
        ___1,
    );
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action92(
        text,
        ___0,
        ___temp0,
        ___2,
    )
}
#[allow(clippy::type_complexity)]

pub trait ___ToTriple<'input, >
{
    fn to_triple(value: Self) -> Result<(usize,Tok<'input>,usize), ___lalrpop_util::ParseError<usize, Tok<'input>, tok::Error>>;
}

impl<'input, > ___ToTriple<'input, > for (usize, Tok<'input>, usize)
{
    fn to_triple(value: Self) -> Result<(usize,Tok<'input>,usize), ___lalrpop_util::ParseError<usize, Tok<'input>, tok::Error>> {
        Ok(value)
    }
}
impl<'input, > ___ToTriple<'input, > for Result<(usize, Tok<'input>, usize), tok::Error>
{
    fn to_triple(value: Self) -> Result<(usize,Tok<'input>,usize), ___lalrpop_util::ParseError<usize, Tok<'input>, tok::Error>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(___lalrpop_util::ParseError::User { error }),
        }
    }
}
