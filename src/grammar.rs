// auto-generated: "lalrpop 0.20.0"
// sha3: 68c5b172b9dbe270062ba8780cf803ec324a73bd6bac89ef55f46974f618da65
use crate::ast;
use crate::tok::{self, Tok};
#[allow(unused_extern_crates)]
extern crate lalrpop_util as ___lalrpop_util;
#[allow(unused_imports)]
use self::___lalrpop_util::state_machine as ___state_machine;
extern crate alloc;
extern crate core;

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
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 4, 0, 37, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 4, 0, 37, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0,
        // State 8
        0, 0, 0, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 0, 0,
        // State 9
        0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 49, 0, 0, 0, 0, 0, 0, 0, 0, 50, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 0, 0, 0, 62, 0, 0, 0, 0, 16, 0, 17, 0, 18, 0, 19, 12, 63,
        // State 12
        0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 0, 0, 0, 62, 0, 0, 0, 0, 16, 0, 17, 0, 18, 0, 19, 12, 70,
        // State 15
        23, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 0, 0, 77, 62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0,
        // State 17
        0, 0, 24, 0, 0, -55, 0, 0, 0, 0, 0, 0, 40, 0, 0, 77, 62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        26, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 0, 0, 77, 62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 49, 0, 0, 0, 0, 0, 0, 0, 0, 50, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0,
        // State 21
        0, 84, 0, 0, 0, 0, 85, 86, 0, 87, 88, 89, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 0, 0, 77, 62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 0, 0, 77, 62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0,
        // State 25
        0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 0, 0, 77, 62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 0, 0, 0, 62, 0, 0, 0, 0, 16, 0, 17, 0, 18, 0, 19, 12, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 0, 0, 0, 62, 0, 0, 0, 0, 16, 0, 17, 0, 18, 0, 19, 12, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24, 0, -24, 0, -24, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -27, 0, -27, 0, -27, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -23, 0, -23, 0, -23, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -28, 0, -28, 0, -28, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, -39, -39, 0, -39, 0, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39, 0,
        // State 40
        0, 0, 0, 0, 0, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, -47, 0, -47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -47, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, 0, -38, 0, -38, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, -52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -52, 0,
        // State 45
        0, 0, 0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -22, 0, -22, 0, -22, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -57, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -58, 0,
        // State 50
        0, 0, 0, 0, 0, -59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 67, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -66, 0, 0, 0, -66, 0, 0, 0, 0, -66, 0, -66, 0, -66, 0, -66, -66, -66,
        // State 53
        0, 0, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, -61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -64, 0, 0, 0, -64, 0, 0, 0, 0, -64, 0, -64, 0, -64, 0, -64, -64, -64,
        // State 56
        0, 0, 0, 0, 0, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -69, 0, 0, 0, -69, 0, 0, 0, 0, -69, 0, -69, 0, -69, 0, -69, -69, -69,
        // State 59
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -63, 0, 0, 0, -63, 0, 0, 0, 0, -63, 0, -63, 0, -63, 0, -63, -63, -63,
        // State 60
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -65, 0, 0, 0, -65, 0, 0, 0, 0, -65, 0, -65, 0, -65, 0, -65, -65, -65,
        // State 61
        0, 0, 71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -71, 0, 0, 0, -71, 0, -71, -71, -71, -71, -71, -71, 0, -71, 0, -71, -71, -71,
        // State 63
        0, 0, 0, 0, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -51, 0,
        // State 64
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -36, 0, -36, 0, -36, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -37, 0, -37, 0, -37, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -62, 0, 0, 0, -62, 0, 0, 0, 0, -62, 0, -62, 0, -62, 0, -62, -62, -62,
        // State 68
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -70, 0, 0, 0, -70, 0, 0, 0, 0, -70, 0, -70, 0, -70, 0, -70, -70, -70,
        // State 69
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -72, 0, 0, 0, -72, 0, -72, -72, -72, -72, -72, -72, 0, -72, 0, -72, -72, -72,
        // State 70
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 71
        0, 0, 0, -29, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0,
        // State 72
        0, 0, 83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 73
        0, 0, 0, -45, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, 0,
        // State 74
        0, 0, 0, -31, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0,
        // State 75
        0, 0, 0, -30, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0,
        // State 76
        0, -73, 0, 0, 0, 0, -73, -73, 0, -73, -73, -73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 77
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -50, 0, 0, 0, -50, 0, 0, 0, 0, -50, 0, -50, 0, -50, 0, -50, -50, -50,
        // State 78
        0, 0, 0, 0, 0, -56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 79
        0, 0, 0, 0, 0, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 80
        0, 0, 0, 93, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 81
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -43, 0, 0, 0, -43, 0, 0, 31, 0, -43, 0, -43, 0, -43, 0, -43, -43, -43,
        // State 82
        0, 0, 0, 94, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 83
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 84
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 85
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -21, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 86
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 87
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 88
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 89
        0, 0, 0, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 90
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -76, 0, 0, 0, -76, 0, 0, 0, 0, -76, 0, -76, 0, -76, 0, -76, -76, -76,
        // State 91
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -35, 0, -35, 0, -35, 0, 0, 0, 0, 0, 0, 0,
        // State 92
        0, 0, 0, -44, 0, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -44, 0,
        // State 93
        0, 0, 0, -33, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, 0,
        // State 94
        0, 0, 0, -49, 0, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -49, 0,
        // State 95
        0, 0, 0, -48, 0, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -48, 0,
        // State 96
        0, 0, 0, -74, 0, -74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -74, 0,
        // State 97
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, -41, 0, 0, 32, 0, -41, 0, -41, 0, -41, 0, -41, -41, -41,
        // State 98
        0, 0, 0, -32, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, 0,
        // State 99
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -75, 0, 0, 0, -75, 0, 0, 0, 0, -75, 0, -75, 0, -75, 0, -75, -75, -75,
        // State 100
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, -42, 0, 0, 0, 0, -42, 0, -42, 0, -42, 0, -42, -42, -42,
        // State 101
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, -40, 0, 0, 0, 0, -40, 0, -40, 0, -40, 0, -40, -40, -40,
    ];
    fn ___action(state: i8, integer: usize) -> i8 {
        ___ACTION[(state as usize) * 30 + integer]
    }
    const ___EOF_ACTION: &[i8] = &[
        // State 0
        -53,
        // State 1
        -54,
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
        0,
        // State 29
        0,
        // State 30
        0,
        // State 31
        0,
        // State 32
        -24,
        // State 33
        -27,
        // State 34
        -23,
        // State 35
        -77,
        // State 36
        0,
        // State 37
        -28,
        // State 38
        0,
        // State 39
        0,
        // State 40
        0,
        // State 41
        0,
        // State 42
        -38,
        // State 43
        0,
        // State 44
        0,
        // State 45
        0,
        // State 46
        0,
        // State 47
        -22,
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
        0,
        // State 59
        0,
        // State 60
        0,
        // State 61
        0,
        // State 62
        -71,
        // State 63
        0,
        // State 64
        -36,
        // State 65
        -37,
        // State 66
        0,
        // State 67
        0,
        // State 68
        0,
        // State 69
        -72,
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
        0,
        // State 90
        0,
        // State 91
        -35,
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
        // State 97
        0,
        // State 98
        0,
        // State 99
        0,
        // State 100
        0,
        // State 101
        0,
    ];
    fn ___goto(state: i8, nt: usize) -> i8 {
        match nt {
            7 => 43,
            9 => 27,
            10 => 32,
            11 => match state {
                1 => 37,
                _ => 33,
            },
            13 => 1,
            14 => match state {
                18 => 24,
                22 => 28,
                25 => 29,
                17 => 78,
                23 => 89,
                _ => 20,
            },
            15 => 71,
            16 => 50,
            17 => 34,
            18 => match state {
                3 => 4,
                5 => 9,
                2 => 38,
                11 | 14 | 30..=31 => 51,
                27 => 94,
                _ => 72,
            },
            19 => 52,
            20 => match state {
                11 | 14 | 30..=31 => 53,
                _ => 73,
            },
            21 => 74,
            22 => 54,
            23 => match state {
                27 => 95,
                _ => 40,
            },
            24 => 96,
            25 => 55,
            26 => match state {
                9 => 12,
                _ => 7,
            },
            27 => 35,
            28 => 56,
            29 => match state {
                19 => 26,
                _ => 13,
            },
            30 => 57,
            31 => match state {
                14 => 68,
                30 => 100,
                31 => 101,
                _ => 58,
            },
            33 => 14,
            34 => match state {
                7 => 42,
                12 => 64,
                13 => 65,
                16 => 77,
                20 => 81,
                24 => 90,
                26 => 91,
                28 => 97,
                29 => 99,
                _ => 59,
            },
            35 => 21,
            36 => 75,
            37 => 60,
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
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 1,
                }
            }
            4 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
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
                    states_to_pop: 0,
                    nonterminal_produced: 3,
                }
            }
            7 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 4,
                }
            }
            8 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 5,
                }
            }
            9 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 5,
                }
            }
            10 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
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
                    states_to_pop: 1,
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
                    states_to_pop: 0,
                    nonterminal_produced: 8,
                }
            }
            15 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            16 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            17 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            18 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            19 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            20 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            21 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 10,
                }
            }
            22 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
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
                    states_to_pop: 0,
                    nonterminal_produced: 12,
                }
            }
            25 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
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
                    states_to_pop: 2,
                    nonterminal_produced: 13,
                }
            }
            28 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            29 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            30 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
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
                    states_to_pop: 3,
                    nonterminal_produced: 16,
                }
            }
            34 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 17,
                }
            }
            35 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 17,
                }
            }
            36 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 17,
                }
            }
            37 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 17,
                }
            }
            38 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            39 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 19,
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
                    states_to_pop: 5,
                    nonterminal_produced: 19,
                }
            }
            42 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 19,
                }
            }
            43 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 20,
                }
            }
            44 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            45 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            46 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            47 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 24,
                }
            }
            48 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 24,
                }
            }
            49 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 25,
                }
            }
            50 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 26,
                }
            }
            51 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 26,
                }
            }
            52 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 27,
                }
            }
            53 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 27,
                }
            }
            54 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 28,
                }
            }
            55 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 28,
                }
            }
            56 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 29,
                }
            }
            57 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 29,
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
                    states_to_pop: 1,
                    nonterminal_produced: 30,
                }
            }
            60 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 30,
                }
            }
            61 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
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
                    states_to_pop: 1,
                    nonterminal_produced: 31,
                }
            }
            65 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 31,
                }
            }
            66 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 32,
                }
            }
            67 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 32,
                }
            }
            68 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 33,
                }
            }
            69 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 33,
                }
            }
            70 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 34,
                }
            }
            71 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 34,
                }
            }
            72 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 35,
                }
            }
            73 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 36,
                }
            }
            74 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 37,
                }
            }
            75 => {
                ___state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 37,
                }
            }
            76 => ___state_machine::SimulatedReduce::Accept,
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
            ___tokens0: ___TOKENS,
        ) -> Result<ast::Program, ___lalrpop_util::ParseError<usize, Tok<'input>, tok::Error>>
        {
            let ___tokens = ___tokens0.into_iter();
            let mut ___tokens = ___tokens.map(|t| ___ToTriple::to_triple(t));
            ___state_machine::Parser::drive(
                ___StateMachine {
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
        ___action: i8,
        ___lookahead_start: Option<&usize>,
        ___states: &mut alloc::vec::Vec<i8>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<Result<ast::Program,___lalrpop_util::ParseError<usize, Tok<'input>, tok::Error>>>
    {
        let (___pop_states, ___nonterminal) = match ___action {
            0 => {
                ___reduce0(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            1 => {
                ___reduce1(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            2 => {
                ___reduce2(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            3 => {
                ___reduce3(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            4 => {
                ___reduce4(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            5 => {
                ___reduce5(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            6 => {
                ___reduce6(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            7 => {
                ___reduce7(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            8 => {
                ___reduce8(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            9 => {
                ___reduce9(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            10 => {
                ___reduce10(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            11 => {
                ___reduce11(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            12 => {
                ___reduce12(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            13 => {
                ___reduce13(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            14 => {
                ___reduce14(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            15 => {
                ___reduce15(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            16 => {
                ___reduce16(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            17 => {
                ___reduce17(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            18 => {
                ___reduce18(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            19 => {
                ___reduce19(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            20 => {
                ___reduce20(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            21 => {
                ___reduce21(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            22 => {
                ___reduce22(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            23 => {
                ___reduce23(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            24 => {
                ___reduce24(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            25 => {
                ___reduce25(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            26 => {
                ___reduce26(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            27 => {
                ___reduce27(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            28 => {
                ___reduce28(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            29 => {
                ___reduce29(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            30 => {
                ___reduce30(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            31 => {
                ___reduce31(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            32 => {
                ___reduce32(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            33 => {
                ___reduce33(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            34 => {
                ___reduce34(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            35 => {
                ___reduce35(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            36 => {
                ___reduce36(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            37 => {
                ___reduce37(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            38 => {
                ___reduce38(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            39 => {
                ___reduce39(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            40 => {
                ___reduce40(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            41 => {
                ___reduce41(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            42 => {
                ___reduce42(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            43 => {
                ___reduce43(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            44 => {
                ___reduce44(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            45 => {
                ___reduce45(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            46 => {
                ___reduce46(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            47 => {
                ___reduce47(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            48 => {
                ___reduce48(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            49 => {
                ___reduce49(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            50 => {
                ___reduce50(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            51 => {
                ___reduce51(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            52 => {
                ___reduce52(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            53 => {
                ___reduce53(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            54 => {
                ___reduce54(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            55 => {
                ___reduce55(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            56 => {
                ___reduce56(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            57 => {
                ___reduce57(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            58 => {
                ___reduce58(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            59 => {
                ___reduce59(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            60 => {
                ___reduce60(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            61 => {
                ___reduce61(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            62 => {
                ___reduce62(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            63 => {
                ___reduce63(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            64 => {
                ___reduce64(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            65 => {
                ___reduce65(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            66 => {
                ___reduce66(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            67 => {
                ___reduce67(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            68 => {
                ___reduce68(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            69 => {
                ___reduce69(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            70 => {
                ___reduce70(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            71 => {
                ___reduce71(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            72 => {
                ___reduce72(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            73 => {
                ___reduce73(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            74 => {
                ___reduce74(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            75 => {
                ___reduce75(___lookahead_start, ___symbols, core::marker::PhantomData::<(&())>)
            }
            76 => {
                // ___Program = Program => ActionFn(0);
                let ___sym0 = ___pop_Variant21(___symbols);
                let ___start = ___sym0.0;
                let ___end = ___sym0.2;
                let ___nt = super::___action0::<>(___sym0);
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
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // "!"? = "!" => ActionFn(49);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action49::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant3(___nt), ___end));
        (1, 0)
    }
    pub(crate) fn ___reduce1<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // "!"? =  => ActionFn(50);
        let ___start = ___lookahead_start.cloned().or_else(|| ___symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let ___end = ___start.clone();
        let ___nt = super::___action50::<>(&___start, &___end);
        ___symbols.push((___start, ___Symbol::Variant3(___nt), ___end));
        (0, 0)
    }
    pub(crate) fn ___reduce2<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // "inline"? = "inline" => ActionFn(58);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action58::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant3(___nt), ___end));
        (1, 1)
    }
    pub(crate) fn ___reduce3<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // "inline"? =  => ActionFn(59);
        let ___start = ___lookahead_start.cloned().or_else(|| ___symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let ___end = ___start.clone();
        let ___nt = super::___action59::<>(&___start, &___end);
        ___symbols.push((___start, ___Symbol::Variant3(___nt), ___end));
        (0, 1)
    }
    pub(crate) fn ___reduce4<
        'input,
    >(
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
        let ___nt = super::___action57::<>(___sym0, ___sym1);
        ___symbols.push((___start, ___Symbol::Variant4(___nt), ___end));
        (2, 2)
    }
    pub(crate) fn ___reduce5<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ("->" ReturnType)? = "->", ReturnType => ActionFn(73);
        assert!(___symbols.len() >= 2);
        let ___sym1 = ___pop_Variant22(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym1.2;
        let ___nt = super::___action73::<>(___sym0, ___sym1);
        ___symbols.push((___start, ___Symbol::Variant5(___nt), ___end));
        (2, 3)
    }
    pub(crate) fn ___reduce6<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ("->" ReturnType)? =  => ActionFn(56);
        let ___start = ___lookahead_start.cloned().or_else(|| ___symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let ___end = ___start.clone();
        let ___nt = super::___action56::<>(&___start, &___end);
        ___symbols.push((___start, ___Symbol::Variant5(___nt), ___end));
        (0, 3)
    }
    pub(crate) fn ___reduce7<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ("else" Statement) = "else", Statement => ActionFn(48);
        assert!(___symbols.len() >= 2);
        let ___sym1 = ___pop_Variant16(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym1.2;
        let ___nt = super::___action48::<>(___sym0, ___sym1);
        ___symbols.push((___start, ___Symbol::Variant6(___nt), ___end));
        (2, 4)
    }
    pub(crate) fn ___reduce8<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ("else" Statement)? = "else", Statement => ActionFn(78);
        assert!(___symbols.len() >= 2);
        let ___sym1 = ___pop_Variant16(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym1.2;
        let ___nt = super::___action78::<>(___sym0, ___sym1);
        ___symbols.push((___start, ___Symbol::Variant7(___nt), ___end));
        (2, 5)
    }
    pub(crate) fn ___reduce9<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ("else" Statement)? =  => ActionFn(47);
        let ___start = ___lookahead_start.cloned().or_else(|| ___symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let ___end = ___start.clone();
        let ___nt = super::___action47::<>(&___start, &___end);
        ___symbols.push((___start, ___Symbol::Variant7(___nt), ___end));
        (0, 5)
    }
    pub(crate) fn ___reduce10<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(60);
        let ___start = ___lookahead_start.cloned().or_else(|| ___symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let ___end = ___start.clone();
        let ___nt = super::___action60::<>(&___start, &___end);
        ___symbols.push((___start, ___Symbol::Variant8(___nt), ___end));
        (0, 6)
    }
    pub(crate) fn ___reduce11<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ArgumentType = "arg" => ActionFn(10);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action10::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant9(___nt), ___end));
        (1, 7)
    }
    pub(crate) fn ___reduce12<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ArgumentType = "state" => ActionFn(11);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action11::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant9(___nt), ___end));
        (1, 7)
    }
    pub(crate) fn ___reduce13<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ArgumentType? = ArgumentType => ActionFn(53);
        let ___sym0 = ___pop_Variant9(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action53::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant10(___nt), ___end));
        (1, 8)
    }
    pub(crate) fn ___reduce14<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ArgumentType? =  => ActionFn(54);
        let ___start = ___lookahead_start.cloned().or_else(|| ___symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let ___end = ___start.clone();
        let ___nt = super::___action54::<>(&___start, &___end);
        ___symbols.push((___start, ___Symbol::Variant10(___nt), ___end));
        (0, 8)
    }
    pub(crate) fn ___reduce15<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ComparisonOp = "==" => ActionFn(39);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action39::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant11(___nt), ___end));
        (1, 9)
    }
    pub(crate) fn ___reduce16<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ComparisonOp = "!=" => ActionFn(40);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action40::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant11(___nt), ___end));
        (1, 9)
    }
    pub(crate) fn ___reduce17<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ComparisonOp = ">" => ActionFn(41);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action41::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant11(___nt), ___end));
        (1, 9)
    }
    pub(crate) fn ___reduce18<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ComparisonOp = "<" => ActionFn(42);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action42::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant11(___nt), ___end));
        (1, 9)
    }
    pub(crate) fn ___reduce19<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ComparisonOp = ">=" => ActionFn(43);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action43::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant11(___nt), ___end));
        (1, 9)
    }
    pub(crate) fn ___reduce20<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ComparisonOp = "<=" => ActionFn(44);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action44::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant11(___nt), ___end));
        (1, 9)
    }
    pub(crate) fn ___reduce21<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConstDefinition = "const", Ident, "=", Integer, ";" => ActionFn(83);
        assert!(___symbols.len() >= 5);
        let ___sym4 = ___pop_Variant0(___symbols);
        let ___sym3 = ___pop_Variant19(___symbols);
        let ___sym2 = ___pop_Variant0(___symbols);
        let ___sym1 = ___pop_Variant17(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym4.2;
        let ___nt = super::___action83::<>(___sym0, ___sym1, ___sym2, ___sym3, ___sym4);
        ___symbols.push((___start, ___Symbol::Variant12(___nt), ___end));
        (5, 10)
    }
    pub(crate) fn ___reduce22<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Definition = FunctionDefinition => ActionFn(2);
        let ___sym0 = ___pop_Variant12(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action2::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant12(___nt), ___end));
        (1, 11)
    }
    pub(crate) fn ___reduce23<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Definition = ConstDefinition => ActionFn(3);
        let ___sym0 = ___pop_Variant12(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action3::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant12(___nt), ___end));
        (1, 11)
    }
    pub(crate) fn ___reduce24<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Definition* =  => ActionFn(61);
        let ___start = ___lookahead_start.cloned().or_else(|| ___symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let ___end = ___start.clone();
        let ___nt = super::___action61::<>(&___start, &___end);
        ___symbols.push((___start, ___Symbol::Variant13(___nt), ___end));
        (0, 12)
    }
    pub(crate) fn ___reduce25<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Definition* = Definition+ => ActionFn(62);
        let ___sym0 = ___pop_Variant13(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action62::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant13(___nt), ___end));
        (1, 12)
    }
    pub(crate) fn ___reduce26<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Definition+ = Definition => ActionFn(63);
        let ___sym0 = ___pop_Variant12(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action63::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant13(___nt), ___end));
        (1, 13)
    }
    pub(crate) fn ___reduce27<
        'input,
    >(
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
        let ___nt = super::___action64::<>(___sym0, ___sym1);
        ___symbols.push((___start, ___Symbol::Variant13(___nt), ___end));
        (2, 13)
    }
    pub(crate) fn ___reduce28<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr = FunctionCallResultExpr => ActionFn(29);
        let ___sym0 = ___pop_Variant15(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action29::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant14(___nt), ___end));
        (1, 14)
    }
    pub(crate) fn ___reduce29<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr = VariableComparisonExpr => ActionFn(30);
        let ___sym0 = ___pop_Variant15(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action30::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant14(___nt), ___end));
        (1, 14)
    }
    pub(crate) fn ___reduce30<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr = InlineCodeExpr => ActionFn(31);
        let ___sym0 = ___pop_Variant15(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action31::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant14(___nt), ___end));
        (1, 14)
    }
    pub(crate) fn ___reduce31<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr = "(", Expr, ")" => ActionFn(32);
        assert!(___symbols.len() >= 3);
        let ___sym2 = ___pop_Variant0(___symbols);
        let ___sym1 = ___pop_Variant14(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym2.2;
        let ___nt = super::___action32::<>(___sym0, ___sym1, ___sym2);
        ___symbols.push((___start, ___Symbol::Variant14(___nt), ___end));
        (3, 14)
    }
    pub(crate) fn ___reduce32<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // FunctionCallResultExpr = Ident, "(", ")" => ActionFn(84);
        assert!(___symbols.len() >= 3);
        let ___sym2 = ___pop_Variant0(___symbols);
        let ___sym1 = ___pop_Variant0(___symbols);
        let ___sym0 = ___pop_Variant17(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym2.2;
        let ___nt = super::___action84::<>(___sym0, ___sym1, ___sym2);
        ___symbols.push((___start, ___Symbol::Variant15(___nt), ___end));
        (3, 15)
    }
    pub(crate) fn ___reduce33<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // FunctionCallStatement = Ident, "(", ")" => ActionFn(85);
        assert!(___symbols.len() >= 3);
        let ___sym2 = ___pop_Variant0(___symbols);
        let ___sym1 = ___pop_Variant0(___symbols);
        let ___sym0 = ___pop_Variant17(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym2.2;
        let ___nt = super::___action85::<>(___sym0, ___sym1, ___sym2);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (3, 16)
    }
    pub(crate) fn ___reduce34<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // FunctionDefinition = "inline", "fn", Ident, Params, "->", ReturnType, StatementBlock => ActionFn(86);
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
        let ___nt = super::___action86::<>(___sym0, ___sym1, ___sym2, ___sym3, ___sym4, ___sym5, ___sym6);
        ___symbols.push((___start, ___Symbol::Variant12(___nt), ___end));
        (7, 17)
    }
    pub(crate) fn ___reduce35<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // FunctionDefinition = "inline", "fn", Ident, Params, StatementBlock => ActionFn(87);
        assert!(___symbols.len() >= 5);
        let ___sym4 = ___pop_Variant16(___symbols);
        let ___sym3 = ___pop_Variant9(___symbols);
        let ___sym2 = ___pop_Variant17(___symbols);
        let ___sym1 = ___pop_Variant0(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym4.2;
        let ___nt = super::___action87::<>(___sym0, ___sym1, ___sym2, ___sym3, ___sym4);
        ___symbols.push((___start, ___Symbol::Variant12(___nt), ___end));
        (5, 17)
    }
    pub(crate) fn ___reduce36<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // FunctionDefinition = "fn", Ident, Params, "->", ReturnType, StatementBlock => ActionFn(88);
        assert!(___symbols.len() >= 6);
        let ___sym5 = ___pop_Variant16(___symbols);
        let ___sym4 = ___pop_Variant22(___symbols);
        let ___sym3 = ___pop_Variant0(___symbols);
        let ___sym2 = ___pop_Variant9(___symbols);
        let ___sym1 = ___pop_Variant17(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym5.2;
        let ___nt = super::___action88::<>(___sym0, ___sym1, ___sym2, ___sym3, ___sym4, ___sym5);
        ___symbols.push((___start, ___Symbol::Variant12(___nt), ___end));
        (6, 17)
    }
    pub(crate) fn ___reduce37<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // FunctionDefinition = "fn", Ident, Params, StatementBlock => ActionFn(89);
        assert!(___symbols.len() >= 4);
        let ___sym3 = ___pop_Variant16(___symbols);
        let ___sym2 = ___pop_Variant9(___symbols);
        let ___sym1 = ___pop_Variant17(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym3.2;
        let ___nt = super::___action89::<>(___sym0, ___sym1, ___sym2, ___sym3);
        ___symbols.push((___start, ___Symbol::Variant12(___nt), ___end));
        (4, 17)
    }
    pub(crate) fn ___reduce38<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Ident = "Id" => ActionFn(4);
        let ___sym0 = ___pop_Variant1(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action4::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant17(___nt), ___end));
        (1, 18)
    }
    pub(crate) fn ___reduce39<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // IfStatement = "if", "!", Expr, StatementBlock, "else", Statement => ActionFn(90);
        assert!(___symbols.len() >= 6);
        let ___sym5 = ___pop_Variant16(___symbols);
        let ___sym4 = ___pop_Variant0(___symbols);
        let ___sym3 = ___pop_Variant16(___symbols);
        let ___sym2 = ___pop_Variant14(___symbols);
        let ___sym1 = ___pop_Variant0(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym5.2;
        let ___nt = super::___action90::<>(___sym0, ___sym1, ___sym2, ___sym3, ___sym4, ___sym5);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (6, 19)
    }
    pub(crate) fn ___reduce40<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // IfStatement = "if", "!", Expr, StatementBlock => ActionFn(91);
        assert!(___symbols.len() >= 4);
        let ___sym3 = ___pop_Variant16(___symbols);
        let ___sym2 = ___pop_Variant14(___symbols);
        let ___sym1 = ___pop_Variant0(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym3.2;
        let ___nt = super::___action91::<>(___sym0, ___sym1, ___sym2, ___sym3);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (4, 19)
    }
    pub(crate) fn ___reduce41<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // IfStatement = "if", Expr, StatementBlock, "else", Statement => ActionFn(92);
        assert!(___symbols.len() >= 5);
        let ___sym4 = ___pop_Variant16(___symbols);
        let ___sym3 = ___pop_Variant0(___symbols);
        let ___sym2 = ___pop_Variant16(___symbols);
        let ___sym1 = ___pop_Variant14(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym4.2;
        let ___nt = super::___action92::<>(___sym0, ___sym1, ___sym2, ___sym3, ___sym4);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (5, 19)
    }
    pub(crate) fn ___reduce42<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // IfStatement = "if", Expr, StatementBlock => ActionFn(93);
        assert!(___symbols.len() >= 3);
        let ___sym2 = ___pop_Variant16(___symbols);
        let ___sym1 = ___pop_Variant14(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym2.2;
        let ___nt = super::___action93::<>(___sym0, ___sym1, ___sym2);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (3, 19)
    }
    pub(crate) fn ___reduce43<
        'input,
    >(
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
        let ___nt = super::___action22::<>(___sym0, ___sym1, ___sym2, ___sym3);
        ___symbols.push((___start, ___Symbol::Variant18(___nt), ___end));
        (4, 20)
    }
    pub(crate) fn ___reduce44<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // InlineCodeExpr = InlineCode => ActionFn(94);
        let ___sym0 = ___pop_Variant18(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action94::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant15(___nt), ___end));
        (1, 21)
    }
    pub(crate) fn ___reduce45<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // InlineCodeStatement = InlineCode => ActionFn(95);
        let ___sym0 = ___pop_Variant18(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action95::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (1, 22)
    }
    pub(crate) fn ___reduce46<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Integer = "Integer" => ActionFn(45);
        let ___sym0 = ___pop_Variant2(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action45::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant19(___nt), ___end));
        (1, 23)
    }
    pub(crate) fn ___reduce47<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // IntegerExpr = Integer => ActionFn(36);
        let ___sym0 = ___pop_Variant19(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action36::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant20(___nt), ___end));
        (1, 24)
    }
    pub(crate) fn ___reduce48<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // IntegerExpr = Ident => ActionFn(37);
        let ___sym0 = ___pop_Variant17(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action37::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant20(___nt), ___end));
        (1, 24)
    }
    pub(crate) fn ___reduce49<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LoopStatement = "loop", StatementBlock => ActionFn(96);
        assert!(___symbols.len() >= 2);
        let ___sym1 = ___pop_Variant16(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym1.2;
        let ___nt = super::___action96::<>(___sym0, ___sym1);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (2, 25)
    }
    pub(crate) fn ___reduce50<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Params = "(", ArgumentType, ")" => ActionFn(103);
        assert!(___symbols.len() >= 3);
        let ___sym2 = ___pop_Variant0(___symbols);
        let ___sym1 = ___pop_Variant9(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym2.2;
        let ___nt = super::___action103::<>(___sym0, ___sym1, ___sym2);
        ___symbols.push((___start, ___Symbol::Variant9(___nt), ___end));
        (3, 26)
    }
    pub(crate) fn ___reduce51<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Params = "(", ")" => ActionFn(104);
        assert!(___symbols.len() >= 2);
        let ___sym1 = ___pop_Variant0(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym1.2;
        let ___nt = super::___action104::<>(___sym0, ___sym1);
        ___symbols.push((___start, ___Symbol::Variant9(___nt), ___end));
        (2, 26)
    }
    pub(crate) fn ___reduce52<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program =  => ActionFn(105);
        let ___start = ___lookahead_start.cloned().or_else(|| ___symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let ___end = ___start.clone();
        let ___nt = super::___action105::<>(&___start, &___end);
        ___symbols.push((___start, ___Symbol::Variant21(___nt), ___end));
        (0, 27)
    }
    pub(crate) fn ___reduce53<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = Definition+ => ActionFn(106);
        let ___sym0 = ___pop_Variant13(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action106::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant21(___nt), ___end));
        (1, 27)
    }
    pub(crate) fn ___reduce54<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ReturnStatement = "return" => ActionFn(97);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action97::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (1, 28)
    }
    pub(crate) fn ___reduce55<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ReturnStatement = "return", Expr => ActionFn(98);
        assert!(___symbols.len() >= 2);
        let ___sym1 = ___pop_Variant14(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym1.2;
        let ___nt = super::___action98::<>(___sym0, ___sym1);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (2, 28)
    }
    pub(crate) fn ___reduce56<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ReturnType = "arg" => ActionFn(8);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action8::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant22(___nt), ___end));
        (1, 29)
    }
    pub(crate) fn ___reduce57<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ReturnType = "state" => ActionFn(9);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action9::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant22(___nt), ___end));
        (1, 29)
    }
    pub(crate) fn ___reduce58<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // SingleStatement = FunctionCallStatement => ActionFn(18);
        let ___sym0 = ___pop_Variant16(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action18::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (1, 30)
    }
    pub(crate) fn ___reduce59<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // SingleStatement = ReturnStatement => ActionFn(19);
        let ___sym0 = ___pop_Variant16(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action19::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (1, 30)
    }
    pub(crate) fn ___reduce60<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // SingleStatement = InlineCodeStatement => ActionFn(20);
        let ___sym0 = ___pop_Variant16(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action20::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (1, 30)
    }
    pub(crate) fn ___reduce61<
        'input,
    >(
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
        let ___nt = super::___action13::<>(___sym0, ___sym1);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (2, 31)
    }
    pub(crate) fn ___reduce62<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement = StatementBlock => ActionFn(14);
        let ___sym0 = ___pop_Variant16(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action14::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (1, 31)
    }
    pub(crate) fn ___reduce63<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement = LoopStatement => ActionFn(15);
        let ___sym0 = ___pop_Variant16(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action15::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (1, 31)
    }
    pub(crate) fn ___reduce64<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement = WhileStatement => ActionFn(16);
        let ___sym0 = ___pop_Variant16(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action16::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (1, 31)
    }
    pub(crate) fn ___reduce65<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement = IfStatement => ActionFn(17);
        let ___sym0 = ___pop_Variant16(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action17::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (1, 31)
    }
    pub(crate) fn ___reduce66<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement* =  => ActionFn(51);
        let ___start = ___lookahead_start.cloned().or_else(|| ___symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let ___end = ___start.clone();
        let ___nt = super::___action51::<>(&___start, &___end);
        ___symbols.push((___start, ___Symbol::Variant23(___nt), ___end));
        (0, 32)
    }
    pub(crate) fn ___reduce67<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement* = Statement+ => ActionFn(52);
        let ___sym0 = ___pop_Variant23(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action52::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant23(___nt), ___end));
        (1, 32)
    }
    pub(crate) fn ___reduce68<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement+ = Statement => ActionFn(65);
        let ___sym0 = ___pop_Variant16(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action65::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant23(___nt), ___end));
        (1, 33)
    }
    pub(crate) fn ___reduce69<
        'input,
    >(
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
        let ___nt = super::___action66::<>(___sym0, ___sym1);
        ___symbols.push((___start, ___Symbol::Variant23(___nt), ___end));
        (2, 33)
    }
    pub(crate) fn ___reduce70<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // StatementBlock = "{", "}" => ActionFn(107);
        assert!(___symbols.len() >= 2);
        let ___sym1 = ___pop_Variant0(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym1.2;
        let ___nt = super::___action107::<>(___sym0, ___sym1);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (2, 34)
    }
    pub(crate) fn ___reduce71<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // StatementBlock = "{", Statement+, "}" => ActionFn(108);
        assert!(___symbols.len() >= 3);
        let ___sym2 = ___pop_Variant0(___symbols);
        let ___sym1 = ___pop_Variant23(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym2.2;
        let ___nt = super::___action108::<>(___sym0, ___sym1, ___sym2);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (3, 34)
    }
    pub(crate) fn ___reduce72<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Variable = "Var" => ActionFn(38);
        let ___sym0 = ___pop_Variant1(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym0.2;
        let ___nt = super::___action38::<>(___sym0);
        ___symbols.push((___start, ___Symbol::Variant24(___nt), ___end));
        (1, 35)
    }
    pub(crate) fn ___reduce73<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // VariableComparisonExpr = Variable, ComparisonOp, IntegerExpr => ActionFn(100);
        assert!(___symbols.len() >= 3);
        let ___sym2 = ___pop_Variant20(___symbols);
        let ___sym1 = ___pop_Variant11(___symbols);
        let ___sym0 = ___pop_Variant24(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym2.2;
        let ___nt = super::___action100::<>(___sym0, ___sym1, ___sym2);
        ___symbols.push((___start, ___Symbol::Variant15(___nt), ___end));
        (3, 36)
    }
    pub(crate) fn ___reduce74<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // WhileStatement = "while", "!", Expr, StatementBlock => ActionFn(101);
        assert!(___symbols.len() >= 4);
        let ___sym3 = ___pop_Variant16(___symbols);
        let ___sym2 = ___pop_Variant14(___symbols);
        let ___sym1 = ___pop_Variant0(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym3.2;
        let ___nt = super::___action101::<>(___sym0, ___sym1, ___sym2, ___sym3);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (4, 37)
    }
    pub(crate) fn ___reduce75<
        'input,
    >(
        ___lookahead_start: Option<&usize>,
        ___symbols: &mut alloc::vec::Vec<(usize,___Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // WhileStatement = "while", Expr, StatementBlock => ActionFn(102);
        assert!(___symbols.len() >= 3);
        let ___sym2 = ___pop_Variant16(___symbols);
        let ___sym1 = ___pop_Variant14(___symbols);
        let ___sym0 = ___pop_Variant0(___symbols);
        let ___start = ___sym0.0;
        let ___end = ___sym2.2;
        let ___nt = super::___action102::<>(___sym0, ___sym1, ___sym2);
        ___symbols.push((___start, ___Symbol::Variant16(___nt), ___end));
        (3, 37)
    }
}
pub use self::___parse___Program::ProgramParser;

#[allow(clippy::too_many_arguments)]
fn ___action0<'input>((_, ___0, _): (usize, ast::Program, usize)) -> ast::Program {
    ___0
}

#[allow(clippy::too_many_arguments)]
fn ___action1<'input>(
    (_, definitions, _): (usize, alloc::vec::Vec<ast::Definition>, usize),
) -> ast::Program {
    ast::Program { definitions }
}

#[allow(clippy::too_many_arguments)]
fn ___action2<'input>((_, ___0, _): (usize, ast::Definition, usize)) -> ast::Definition {
    ___0
}

#[allow(clippy::too_many_arguments)]
fn ___action3<'input>((_, ___0, _): (usize, ast::Definition, usize)) -> ast::Definition {
    ___0
}

#[allow(clippy::too_many_arguments)]
fn ___action4<'input>((_, i, _): (usize, &'input str, usize)) -> ast::Ident {
    i.into()
}

#[allow(clippy::too_many_arguments)]
fn ___action5<'input>(
    (_, location, _): (usize, usize, usize),
    (_, is_inline, _): (usize, core::option::Option<Tok<'input>>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, name, _): (usize, ast::Ident, usize),
    (_, arg, _): (usize, ast::ArgumentType, usize),
    (_, returns, _): (
        usize,
        core::option::Option<(Tok<'input>, ast::ReturnType)>,
        usize,
    ),
    (_, block, _): (usize, ast::Statement, usize),
) -> ast::Definition {
    {
        let returns = match returns {
            None => ast::ReturnType::Nothing,
            Some((_, rt)) => rt,
        };
        let node = if is_inline.is_none() {
            ast::DefinitionKind::Function {
                name,
                arg,
                returns,
                body: vec![block],
            }
        } else {
            // TODO: error if there are any args or returns
            ast::DefinitionKind::InlineFunction {
                name,
                body: vec![block],
            }
        };
        ast::Definition::new(location, node)
    }
}

#[allow(clippy::too_many_arguments)]
fn ___action6<'input>(
    (_, location, _): (usize, usize, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, name, _): (usize, ast::Ident, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, value, _): (usize, ast::Integer, usize),
    (_, _, _): (usize, Tok<'input>, usize),
) -> ast::Definition {
    {
        let node = ast::DefinitionKind::Const { name, value };
        ast::Definition::new(location, node)
    }
}

#[allow(clippy::too_many_arguments)]
fn ___action7<'input>(
    (_, _, _): (usize, Tok<'input>, usize),
    (_, arg, _): (usize, core::option::Option<ast::ArgumentType>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
) -> ast::ArgumentType {
    {
        arg.unwrap_or(ast::ArgumentType::Nothing)
    }
}

#[allow(clippy::too_many_arguments)]
fn ___action8<'input>((_, ___0, _): (usize, Tok<'input>, usize)) -> ast::ReturnType {
    ast::ReturnType::Argument
}

#[allow(clippy::too_many_arguments)]
fn ___action9<'input>((_, ___0, _): (usize, Tok<'input>, usize)) -> ast::ReturnType {
    ast::ReturnType::State
}

#[allow(clippy::too_many_arguments)]
fn ___action10<'input>((_, ___0, _): (usize, Tok<'input>, usize)) -> ast::ArgumentType {
    ast::ArgumentType::Argument
}

#[allow(clippy::too_many_arguments)]
fn ___action11<'input>((_, ___0, _): (usize, Tok<'input>, usize)) -> ast::ArgumentType {
    ast::ArgumentType::State
}

#[allow(clippy::too_many_arguments)]
fn ___action12<'input>(
    (_, location, _): (usize, usize, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, body, _): (usize, alloc::vec::Vec<ast::Statement>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
) -> ast::Statement {
    {
        ast::Statement::new(location, ast::StatementKind::StatementBlock { body })
    }
}

#[allow(clippy::too_many_arguments)]
fn ___action13<'input>(
    (_, s, _): (usize, ast::Statement, usize),
    (_, _, _): (usize, Tok<'input>, usize),
) -> ast::Statement {
    s
}

#[allow(clippy::too_many_arguments)]
fn ___action14<'input>((_, ___0, _): (usize, ast::Statement, usize)) -> ast::Statement {
    ___0
}

#[allow(clippy::too_many_arguments)]
fn ___action15<'input>((_, ___0, _): (usize, ast::Statement, usize)) -> ast::Statement {
    ___0
}

#[allow(clippy::too_many_arguments)]
fn ___action16<'input>((_, ___0, _): (usize, ast::Statement, usize)) -> ast::Statement {
    ___0
}

#[allow(clippy::too_many_arguments)]
fn ___action17<'input>((_, ___0, _): (usize, ast::Statement, usize)) -> ast::Statement {
    ___0
}

#[allow(clippy::too_many_arguments)]
fn ___action18<'input>((_, ___0, _): (usize, ast::Statement, usize)) -> ast::Statement {
    ___0
}

#[allow(clippy::too_many_arguments)]
fn ___action19<'input>((_, ___0, _): (usize, ast::Statement, usize)) -> ast::Statement {
    ___0
}

#[allow(clippy::too_many_arguments)]
fn ___action20<'input>((_, ___0, _): (usize, ast::Statement, usize)) -> ast::Statement {
    ___0
}

#[allow(clippy::too_many_arguments)]
fn ___action21<'input>(
    (_, location, _): (usize, usize, usize),
    (_, body, _): (usize, ast::InlineCode, usize),
) -> ast::Statement {
    {
        let node = ast::StatementKind::InlineCode { body };
        ast::Statement::new(location, node)
    }
}

#[allow(clippy::too_many_arguments)]
fn ___action22<'input>(
    (_, _, _): (usize, Tok<'input>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, code, _): (usize, &'input str, usize),
    (_, _, _): (usize, Tok<'input>, usize),
) -> ast::InlineCode {
    ast::InlineCode { code: code.into() }
}

#[allow(clippy::too_many_arguments)]
fn ___action23<'input>(
    (_, location, _): (usize, usize, usize),
    (_, _, _): (usize, Tok<'input>, usize),
) -> ast::Statement {
    {
        ast::Statement::new(location, ast::StatementKind::Return { expr: None })
    }
}

#[allow(clippy::too_many_arguments)]
fn ___action24<'input>(
    (_, location, _): (usize, usize, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, expr, _): (usize, Box<ast::Expr>, usize),
) -> ast::Statement {
    {
        ast::Statement::new(location, ast::StatementKind::Return { expr: Some(expr) })
    }
}

#[allow(clippy::too_many_arguments)]
fn ___action25<'input>(
    (_, location, _): (usize, usize, usize),
    (_, name, _): (usize, ast::Ident, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
) -> ast::Statement {
    {
        let node = ast::StatementKind::FunctionCall { name };
        ast::Statement { location, node }
    }
}

#[allow(clippy::too_many_arguments)]
fn ___action26<'input>(
    (_, location, _): (usize, usize, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, block, _): (usize, ast::Statement, usize),
) -> ast::Statement {
    {
        let node = ast::StatementKind::Loop { body: vec![block] };
        ast::Statement::new(location, node)
    }
}

#[allow(clippy::too_many_arguments)]
fn ___action27<'input>(
    (_, location, _): (usize, usize, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, negative, _): (usize, core::option::Option<Tok<'input>>, usize),
    (_, condition, _): (usize, Box<ast::Expr>, usize),
    (_, block, _): (usize, ast::Statement, usize),
) -> ast::Statement {
    {
        let negative = negative.is_some();
        let node = ast::StatementKind::While {
            condition,
            body: vec![block],
            negative,
        };
        ast::Statement::new(location, node)
    }
}

#[allow(clippy::too_many_arguments)]
fn ___action28<'input>(
    (_, location, _): (usize, usize, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, negative, _): (usize, core::option::Option<Tok<'input>>, usize),
    (_, condition, _): (usize, Box<ast::Expr>, usize),
    (_, block, _): (usize, ast::Statement, usize),
    (_, orelse, _): (
        usize,
        core::option::Option<(Tok<'input>, ast::Statement)>,
        usize,
    ),
) -> ast::Statement {
    {
        let orelse = match orelse {
            Some((_, stmt)) => vec![stmt],
            None => vec![],
        };
        let negative = negative.is_some();
        let node = ast::StatementKind::If {
            condition,
            body: vec![block],
            orelse,
            negative,
        };
        ast::Statement::new(location, node)
    }
}

#[allow(clippy::too_many_arguments)]
fn ___action29<'input>((_, ___0, _): (usize, ast::Expr, usize)) -> Box<ast::Expr> {
    Box::new(___0)
}

#[allow(clippy::too_many_arguments)]
fn ___action30<'input>((_, ___0, _): (usize, ast::Expr, usize)) -> Box<ast::Expr> {
    Box::new(___0)
}

#[allow(clippy::too_many_arguments)]
fn ___action31<'input>((_, ___0, _): (usize, ast::Expr, usize)) -> Box<ast::Expr> {
    Box::new(___0)
}

#[allow(clippy::too_many_arguments)]
fn ___action32<'input>(
    (_, _, _): (usize, Tok<'input>, usize),
    (_, c, _): (usize, Box<ast::Expr>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
) -> Box<ast::Expr> {
    c
}

#[allow(clippy::too_many_arguments)]
fn ___action33<'input>(
    (_, location, _): (usize, usize, usize),
    (_, body, _): (usize, ast::InlineCode, usize),
) -> ast::Expr {
    {
        let node = ast::ExprKind::InlineCode { body };
        ast::Expr::new(location, node)
    }
}

#[allow(clippy::too_many_arguments)]
fn ___action34<'input>(
    (_, location, _): (usize, usize, usize),
    (_, name, _): (usize, ast::Ident, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
) -> ast::Expr {
    {
        // TODO: check that function returns an argument or a state.
        let node = ast::ExprKind::FunctionCallResult { name };
        ast::Expr::new(location, node)
    }
}

#[allow(clippy::too_many_arguments)]
fn ___action35<'input>(
    (_, location, _): (usize, usize, usize),
    (_, var, _): (usize, ast::Variable, usize),
    (_, op, _): (usize, ast::ComparisonOp, usize),
    (_, value, _): (usize, ast::IntegerExpr, usize),
) -> ast::Expr {
    {
        let node = ast::ExprKind::VariableComparison { var, value, op };
        ast::Expr::new(location, node)
    }
}

#[allow(clippy::too_many_arguments)]
fn ___action36<'input>((_, value, _): (usize, ast::Integer, usize)) -> ast::IntegerExpr {
    ast::IntegerExpr::Literal { value }
}

#[allow(clippy::too_many_arguments)]
fn ___action37<'input>((_, name, _): (usize, ast::Ident, usize)) -> ast::IntegerExpr {
    ast::IntegerExpr::Const { name }
}

#[allow(clippy::too_many_arguments)]
fn ___action38<'input>((_, v, _): (usize, &'input str, usize)) -> ast::Variable {
    ast::Variable { name: v.into() }
}

#[allow(clippy::too_many_arguments)]
fn ___action39<'input>((_, ___0, _): (usize, Tok<'input>, usize)) -> ast::ComparisonOp {
    ast::ComparisonOp::Eq
}

#[allow(clippy::too_many_arguments)]
fn ___action40<'input>((_, ___0, _): (usize, Tok<'input>, usize)) -> ast::ComparisonOp {
    ast::ComparisonOp::NotEq
}

#[allow(clippy::too_many_arguments)]
fn ___action41<'input>((_, ___0, _): (usize, Tok<'input>, usize)) -> ast::ComparisonOp {
    ast::ComparisonOp::GreaterThan
}

#[allow(clippy::too_many_arguments)]
fn ___action42<'input>((_, ___0, _): (usize, Tok<'input>, usize)) -> ast::ComparisonOp {
    ast::ComparisonOp::LessThan
}

#[allow(clippy::too_many_arguments)]
fn ___action43<'input>((_, ___0, _): (usize, Tok<'input>, usize)) -> ast::ComparisonOp {
    ast::ComparisonOp::GreaterOrEqual
}

#[allow(clippy::too_many_arguments)]
fn ___action44<'input>((_, ___0, _): (usize, Tok<'input>, usize)) -> ast::ComparisonOp {
    ast::ComparisonOp::LessOrEqual
}

#[allow(clippy::too_many_arguments)]
fn ___action45<'input>((_, i, _): (usize, u32, usize)) -> ast::Integer {
    i
}

#[allow(clippy::too_many_arguments)]
fn ___action46<'input>(
    (_, ___0, _): (usize, (Tok<'input>, ast::Statement), usize),
) -> core::option::Option<(Tok<'input>, ast::Statement)> {
    Some(___0)
}

#[allow(clippy::too_many_arguments)]
fn ___action47<'input>(
    ___lookbehind: &usize,
    ___lookahead: &usize,
) -> core::option::Option<(Tok<'input>, ast::Statement)> {
    None
}

#[allow(clippy::too_many_arguments)]
fn ___action48<'input>(
    (_, ___0, _): (usize, Tok<'input>, usize),
    (_, ___1, _): (usize, ast::Statement, usize),
) -> (Tok<'input>, ast::Statement) {
    (___0, ___1)
}

#[allow(clippy::too_many_arguments)]
fn ___action49<'input>(
    (_, ___0, _): (usize, Tok<'input>, usize),
) -> core::option::Option<Tok<'input>> {
    Some(___0)
}

#[allow(clippy::too_many_arguments)]
fn ___action50<'input>(
    ___lookbehind: &usize,
    ___lookahead: &usize,
) -> core::option::Option<Tok<'input>> {
    None
}

#[allow(clippy::too_many_arguments)]
fn ___action51<'input>(
    ___lookbehind: &usize,
    ___lookahead: &usize,
) -> alloc::vec::Vec<ast::Statement> {
    alloc::vec![]
}

#[allow(clippy::too_many_arguments)]
fn ___action52<'input>(
    (_, v, _): (usize, alloc::vec::Vec<ast::Statement>, usize),
) -> alloc::vec::Vec<ast::Statement> {
    v
}

#[allow(clippy::too_many_arguments)]
fn ___action53<'input>(
    (_, ___0, _): (usize, ast::ArgumentType, usize),
) -> core::option::Option<ast::ArgumentType> {
    Some(___0)
}

#[allow(clippy::too_many_arguments)]
fn ___action54<'input>(
    ___lookbehind: &usize,
    ___lookahead: &usize,
) -> core::option::Option<ast::ArgumentType> {
    None
}

#[allow(clippy::too_many_arguments)]
fn ___action55<'input>(
    (_, ___0, _): (usize, (Tok<'input>, ast::ReturnType), usize),
) -> core::option::Option<(Tok<'input>, ast::ReturnType)> {
    Some(___0)
}

#[allow(clippy::too_many_arguments)]
fn ___action56<'input>(
    ___lookbehind: &usize,
    ___lookahead: &usize,
) -> core::option::Option<(Tok<'input>, ast::ReturnType)> {
    None
}

#[allow(clippy::too_many_arguments)]
fn ___action57<'input>(
    (_, ___0, _): (usize, Tok<'input>, usize),
    (_, ___1, _): (usize, ast::ReturnType, usize),
) -> (Tok<'input>, ast::ReturnType) {
    (___0, ___1)
}

#[allow(clippy::too_many_arguments)]
fn ___action58<'input>(
    (_, ___0, _): (usize, Tok<'input>, usize),
) -> core::option::Option<Tok<'input>> {
    Some(___0)
}

#[allow(clippy::too_many_arguments)]
fn ___action59<'input>(
    ___lookbehind: &usize,
    ___lookahead: &usize,
) -> core::option::Option<Tok<'input>> {
    None
}

fn ___action60<'input>(___lookbehind: &usize, ___lookahead: &usize) -> usize {
    *___lookahead
}

#[allow(clippy::too_many_arguments)]
fn ___action61<'input>(
    ___lookbehind: &usize,
    ___lookahead: &usize,
) -> alloc::vec::Vec<ast::Definition> {
    alloc::vec![]
}

#[allow(clippy::too_many_arguments)]
fn ___action62<'input>(
    (_, v, _): (usize, alloc::vec::Vec<ast::Definition>, usize),
) -> alloc::vec::Vec<ast::Definition> {
    v
}

#[allow(clippy::too_many_arguments)]
fn ___action63<'input>(
    (_, ___0, _): (usize, ast::Definition, usize),
) -> alloc::vec::Vec<ast::Definition> {
    alloc::vec![___0]
}

#[allow(clippy::too_many_arguments)]
fn ___action64<'input>(
    (_, v, _): (usize, alloc::vec::Vec<ast::Definition>, usize),
    (_, e, _): (usize, ast::Definition, usize),
) -> alloc::vec::Vec<ast::Definition> {
    {
        let mut v = v;
        v.push(e);
        v
    }
}

#[allow(clippy::too_many_arguments)]
fn ___action65<'input>(
    (_, ___0, _): (usize, ast::Statement, usize),
) -> alloc::vec::Vec<ast::Statement> {
    alloc::vec![___0]
}

#[allow(clippy::too_many_arguments)]
fn ___action66<'input>(
    (_, v, _): (usize, alloc::vec::Vec<ast::Statement>, usize),
    (_, e, _): (usize, ast::Statement, usize),
) -> alloc::vec::Vec<ast::Statement> {
    {
        let mut v = v;
        v.push(e);
        v
    }
}

#[allow(clippy::too_many_arguments)]
fn ___action67<'input>(
    ___0: (usize, usize, usize),
    ___1: (usize, Tok<'input>, usize),
    ___2: (usize, Tok<'input>, usize),
    ___3: (usize, Box<ast::Expr>, usize),
    ___4: (usize, ast::Statement, usize),
    ___5: (
        usize,
        core::option::Option<(Tok<'input>, ast::Statement)>,
        usize,
    ),
) -> ast::Statement {
    let ___start0 = ___2.0;
    let ___end0 = ___2.2;
    let ___temp0 = ___action49(___2);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action28(___0, ___1, ___temp0, ___3, ___4, ___5)
}

#[allow(clippy::too_many_arguments)]
fn ___action68<'input>(
    ___0: (usize, usize, usize),
    ___1: (usize, Tok<'input>, usize),
    ___2: (usize, Box<ast::Expr>, usize),
    ___3: (usize, ast::Statement, usize),
    ___4: (
        usize,
        core::option::Option<(Tok<'input>, ast::Statement)>,
        usize,
    ),
) -> ast::Statement {
    let ___start0 = ___1.2;
    let ___end0 = ___2.0;
    let ___temp0 = ___action50(&___start0, &___end0);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action28(___0, ___1, ___temp0, ___2, ___3, ___4)
}

#[allow(clippy::too_many_arguments)]
fn ___action69<'input>(
    ___0: (usize, usize, usize),
    ___1: (usize, Tok<'input>, usize),
    ___2: (usize, Tok<'input>, usize),
    ___3: (usize, Box<ast::Expr>, usize),
    ___4: (usize, ast::Statement, usize),
) -> ast::Statement {
    let ___start0 = ___2.0;
    let ___end0 = ___2.2;
    let ___temp0 = ___action49(___2);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action27(___0, ___1, ___temp0, ___3, ___4)
}

#[allow(clippy::too_many_arguments)]
fn ___action70<'input>(
    ___0: (usize, usize, usize),
    ___1: (usize, Tok<'input>, usize),
    ___2: (usize, Box<ast::Expr>, usize),
    ___3: (usize, ast::Statement, usize),
) -> ast::Statement {
    let ___start0 = ___1.2;
    let ___end0 = ___2.0;
    let ___temp0 = ___action50(&___start0, &___end0);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action27(___0, ___1, ___temp0, ___2, ___3)
}

#[allow(clippy::too_many_arguments)]
fn ___action71<'input>(
    ___0: (usize, usize, usize),
    ___1: (usize, Tok<'input>, usize),
    ___2: (usize, Tok<'input>, usize),
    ___3: (usize, ast::Ident, usize),
    ___4: (usize, ast::ArgumentType, usize),
    ___5: (
        usize,
        core::option::Option<(Tok<'input>, ast::ReturnType)>,
        usize,
    ),
    ___6: (usize, ast::Statement, usize),
) -> ast::Definition {
    let ___start0 = ___1.0;
    let ___end0 = ___1.2;
    let ___temp0 = ___action58(___1);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action5(___0, ___temp0, ___2, ___3, ___4, ___5, ___6)
}

#[allow(clippy::too_many_arguments)]
fn ___action72<'input>(
    ___0: (usize, usize, usize),
    ___1: (usize, Tok<'input>, usize),
    ___2: (usize, ast::Ident, usize),
    ___3: (usize, ast::ArgumentType, usize),
    ___4: (
        usize,
        core::option::Option<(Tok<'input>, ast::ReturnType)>,
        usize,
    ),
    ___5: (usize, ast::Statement, usize),
) -> ast::Definition {
    let ___start0 = ___0.2;
    let ___end0 = ___1.0;
    let ___temp0 = ___action59(&___start0, &___end0);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action5(___0, ___temp0, ___1, ___2, ___3, ___4, ___5)
}

#[allow(clippy::too_many_arguments)]
fn ___action73<'input>(
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, ast::ReturnType, usize),
) -> core::option::Option<(Tok<'input>, ast::ReturnType)> {
    let ___start0 = ___0.0;
    let ___end0 = ___1.2;
    let ___temp0 = ___action57(___0, ___1);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action55(___temp0)
}

#[allow(clippy::too_many_arguments)]
fn ___action74<'input>(
    ___0: (usize, usize, usize),
    ___1: (usize, Tok<'input>, usize),
    ___2: (usize, Tok<'input>, usize),
    ___3: (usize, ast::Ident, usize),
    ___4: (usize, ast::ArgumentType, usize),
    ___5: (usize, Tok<'input>, usize),
    ___6: (usize, ast::ReturnType, usize),
    ___7: (usize, ast::Statement, usize),
) -> ast::Definition {
    let ___start0 = ___5.0;
    let ___end0 = ___6.2;
    let ___temp0 = ___action73(___5, ___6);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action71(___0, ___1, ___2, ___3, ___4, ___temp0, ___7)
}

#[allow(clippy::too_many_arguments)]
fn ___action75<'input>(
    ___0: (usize, usize, usize),
    ___1: (usize, Tok<'input>, usize),
    ___2: (usize, Tok<'input>, usize),
    ___3: (usize, ast::Ident, usize),
    ___4: (usize, ast::ArgumentType, usize),
    ___5: (usize, ast::Statement, usize),
) -> ast::Definition {
    let ___start0 = ___4.2;
    let ___end0 = ___5.0;
    let ___temp0 = ___action56(&___start0, &___end0);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action71(___0, ___1, ___2, ___3, ___4, ___temp0, ___5)
}

#[allow(clippy::too_many_arguments)]
fn ___action76<'input>(
    ___0: (usize, usize, usize),
    ___1: (usize, Tok<'input>, usize),
    ___2: (usize, ast::Ident, usize),
    ___3: (usize, ast::ArgumentType, usize),
    ___4: (usize, Tok<'input>, usize),
    ___5: (usize, ast::ReturnType, usize),
    ___6: (usize, ast::Statement, usize),
) -> ast::Definition {
    let ___start0 = ___4.0;
    let ___end0 = ___5.2;
    let ___temp0 = ___action73(___4, ___5);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action72(___0, ___1, ___2, ___3, ___temp0, ___6)
}

#[allow(clippy::too_many_arguments)]
fn ___action77<'input>(
    ___0: (usize, usize, usize),
    ___1: (usize, Tok<'input>, usize),
    ___2: (usize, ast::Ident, usize),
    ___3: (usize, ast::ArgumentType, usize),
    ___4: (usize, ast::Statement, usize),
) -> ast::Definition {
    let ___start0 = ___3.2;
    let ___end0 = ___4.0;
    let ___temp0 = ___action56(&___start0, &___end0);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action72(___0, ___1, ___2, ___3, ___temp0, ___4)
}

#[allow(clippy::too_many_arguments)]
fn ___action78<'input>(
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, ast::Statement, usize),
) -> core::option::Option<(Tok<'input>, ast::Statement)> {
    let ___start0 = ___0.0;
    let ___end0 = ___1.2;
    let ___temp0 = ___action48(___0, ___1);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action46(___temp0)
}

#[allow(clippy::too_many_arguments)]
fn ___action79<'input>(
    ___0: (usize, usize, usize),
    ___1: (usize, Tok<'input>, usize),
    ___2: (usize, Tok<'input>, usize),
    ___3: (usize, Box<ast::Expr>, usize),
    ___4: (usize, ast::Statement, usize),
    ___5: (usize, Tok<'input>, usize),
    ___6: (usize, ast::Statement, usize),
) -> ast::Statement {
    let ___start0 = ___5.0;
    let ___end0 = ___6.2;
    let ___temp0 = ___action78(___5, ___6);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action67(___0, ___1, ___2, ___3, ___4, ___temp0)
}

#[allow(clippy::too_many_arguments)]
fn ___action80<'input>(
    ___0: (usize, usize, usize),
    ___1: (usize, Tok<'input>, usize),
    ___2: (usize, Tok<'input>, usize),
    ___3: (usize, Box<ast::Expr>, usize),
    ___4: (usize, ast::Statement, usize),
) -> ast::Statement {
    let ___start0 = ___4.2;
    let ___end0 = ___4.2;
    let ___temp0 = ___action47(&___start0, &___end0);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action67(___0, ___1, ___2, ___3, ___4, ___temp0)
}

#[allow(clippy::too_many_arguments)]
fn ___action81<'input>(
    ___0: (usize, usize, usize),
    ___1: (usize, Tok<'input>, usize),
    ___2: (usize, Box<ast::Expr>, usize),
    ___3: (usize, ast::Statement, usize),
    ___4: (usize, Tok<'input>, usize),
    ___5: (usize, ast::Statement, usize),
) -> ast::Statement {
    let ___start0 = ___4.0;
    let ___end0 = ___5.2;
    let ___temp0 = ___action78(___4, ___5);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action68(___0, ___1, ___2, ___3, ___temp0)
}

#[allow(clippy::too_many_arguments)]
fn ___action82<'input>(
    ___0: (usize, usize, usize),
    ___1: (usize, Tok<'input>, usize),
    ___2: (usize, Box<ast::Expr>, usize),
    ___3: (usize, ast::Statement, usize),
) -> ast::Statement {
    let ___start0 = ___3.2;
    let ___end0 = ___3.2;
    let ___temp0 = ___action47(&___start0, &___end0);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action68(___0, ___1, ___2, ___3, ___temp0)
}

#[allow(clippy::too_many_arguments)]
fn ___action83<'input>(
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, ast::Ident, usize),
    ___2: (usize, Tok<'input>, usize),
    ___3: (usize, ast::Integer, usize),
    ___4: (usize, Tok<'input>, usize),
) -> ast::Definition {
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(&___start0, &___end0);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action6(___temp0, ___0, ___1, ___2, ___3, ___4)
}

#[allow(clippy::too_many_arguments)]
fn ___action84<'input>(
    ___0: (usize, ast::Ident, usize),
    ___1: (usize, Tok<'input>, usize),
    ___2: (usize, Tok<'input>, usize),
) -> ast::Expr {
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(&___start0, &___end0);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action34(___temp0, ___0, ___1, ___2)
}

#[allow(clippy::too_many_arguments)]
fn ___action85<'input>(
    ___0: (usize, ast::Ident, usize),
    ___1: (usize, Tok<'input>, usize),
    ___2: (usize, Tok<'input>, usize),
) -> ast::Statement {
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(&___start0, &___end0);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action25(___temp0, ___0, ___1, ___2)
}

#[allow(clippy::too_many_arguments)]
fn ___action86<'input>(
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, Tok<'input>, usize),
    ___2: (usize, ast::Ident, usize),
    ___3: (usize, ast::ArgumentType, usize),
    ___4: (usize, Tok<'input>, usize),
    ___5: (usize, ast::ReturnType, usize),
    ___6: (usize, ast::Statement, usize),
) -> ast::Definition {
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(&___start0, &___end0);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action74(___temp0, ___0, ___1, ___2, ___3, ___4, ___5, ___6)
}

#[allow(clippy::too_many_arguments)]
fn ___action87<'input>(
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, Tok<'input>, usize),
    ___2: (usize, ast::Ident, usize),
    ___3: (usize, ast::ArgumentType, usize),
    ___4: (usize, ast::Statement, usize),
) -> ast::Definition {
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(&___start0, &___end0);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action75(___temp0, ___0, ___1, ___2, ___3, ___4)
}

#[allow(clippy::too_many_arguments)]
fn ___action88<'input>(
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, ast::Ident, usize),
    ___2: (usize, ast::ArgumentType, usize),
    ___3: (usize, Tok<'input>, usize),
    ___4: (usize, ast::ReturnType, usize),
    ___5: (usize, ast::Statement, usize),
) -> ast::Definition {
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(&___start0, &___end0);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action76(___temp0, ___0, ___1, ___2, ___3, ___4, ___5)
}

#[allow(clippy::too_many_arguments)]
fn ___action89<'input>(
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, ast::Ident, usize),
    ___2: (usize, ast::ArgumentType, usize),
    ___3: (usize, ast::Statement, usize),
) -> ast::Definition {
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(&___start0, &___end0);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action77(___temp0, ___0, ___1, ___2, ___3)
}

#[allow(clippy::too_many_arguments)]
fn ___action90<'input>(
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, Tok<'input>, usize),
    ___2: (usize, Box<ast::Expr>, usize),
    ___3: (usize, ast::Statement, usize),
    ___4: (usize, Tok<'input>, usize),
    ___5: (usize, ast::Statement, usize),
) -> ast::Statement {
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(&___start0, &___end0);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action79(___temp0, ___0, ___1, ___2, ___3, ___4, ___5)
}

#[allow(clippy::too_many_arguments)]
fn ___action91<'input>(
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, Tok<'input>, usize),
    ___2: (usize, Box<ast::Expr>, usize),
    ___3: (usize, ast::Statement, usize),
) -> ast::Statement {
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(&___start0, &___end0);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action80(___temp0, ___0, ___1, ___2, ___3)
}

#[allow(clippy::too_many_arguments)]
fn ___action92<'input>(
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, Box<ast::Expr>, usize),
    ___2: (usize, ast::Statement, usize),
    ___3: (usize, Tok<'input>, usize),
    ___4: (usize, ast::Statement, usize),
) -> ast::Statement {
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(&___start0, &___end0);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action81(___temp0, ___0, ___1, ___2, ___3, ___4)
}

#[allow(clippy::too_many_arguments)]
fn ___action93<'input>(
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, Box<ast::Expr>, usize),
    ___2: (usize, ast::Statement, usize),
) -> ast::Statement {
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(&___start0, &___end0);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action82(___temp0, ___0, ___1, ___2)
}

#[allow(clippy::too_many_arguments)]
fn ___action94<'input>(___0: (usize, ast::InlineCode, usize)) -> ast::Expr {
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(&___start0, &___end0);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action33(___temp0, ___0)
}

#[allow(clippy::too_many_arguments)]
fn ___action95<'input>(___0: (usize, ast::InlineCode, usize)) -> ast::Statement {
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(&___start0, &___end0);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action21(___temp0, ___0)
}

#[allow(clippy::too_many_arguments)]
fn ___action96<'input>(
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, ast::Statement, usize),
) -> ast::Statement {
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(&___start0, &___end0);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action26(___temp0, ___0, ___1)
}

#[allow(clippy::too_many_arguments)]
fn ___action97<'input>(___0: (usize, Tok<'input>, usize)) -> ast::Statement {
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(&___start0, &___end0);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action23(___temp0, ___0)
}

#[allow(clippy::too_many_arguments)]
fn ___action98<'input>(
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, Box<ast::Expr>, usize),
) -> ast::Statement {
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(&___start0, &___end0);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action24(___temp0, ___0, ___1)
}

#[allow(clippy::too_many_arguments)]
fn ___action99<'input>(
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, alloc::vec::Vec<ast::Statement>, usize),
    ___2: (usize, Tok<'input>, usize),
) -> ast::Statement {
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(&___start0, &___end0);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action12(___temp0, ___0, ___1, ___2)
}

#[allow(clippy::too_many_arguments)]
fn ___action100<'input>(
    ___0: (usize, ast::Variable, usize),
    ___1: (usize, ast::ComparisonOp, usize),
    ___2: (usize, ast::IntegerExpr, usize),
) -> ast::Expr {
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(&___start0, &___end0);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action35(___temp0, ___0, ___1, ___2)
}

#[allow(clippy::too_many_arguments)]
fn ___action101<'input>(
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, Tok<'input>, usize),
    ___2: (usize, Box<ast::Expr>, usize),
    ___3: (usize, ast::Statement, usize),
) -> ast::Statement {
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(&___start0, &___end0);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action69(___temp0, ___0, ___1, ___2, ___3)
}

#[allow(clippy::too_many_arguments)]
fn ___action102<'input>(
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, Box<ast::Expr>, usize),
    ___2: (usize, ast::Statement, usize),
) -> ast::Statement {
    let ___start0 = ___0.0;
    let ___end0 = ___0.0;
    let ___temp0 = ___action60(&___start0, &___end0);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action70(___temp0, ___0, ___1, ___2)
}

#[allow(clippy::too_many_arguments)]
fn ___action103<'input>(
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, ast::ArgumentType, usize),
    ___2: (usize, Tok<'input>, usize),
) -> ast::ArgumentType {
    let ___start0 = ___1.0;
    let ___end0 = ___1.2;
    let ___temp0 = ___action53(___1);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action7(___0, ___temp0, ___2)
}

#[allow(clippy::too_many_arguments)]
fn ___action104<'input>(
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, Tok<'input>, usize),
) -> ast::ArgumentType {
    let ___start0 = ___0.2;
    let ___end0 = ___1.0;
    let ___temp0 = ___action54(&___start0, &___end0);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action7(___0, ___temp0, ___1)
}

#[allow(clippy::too_many_arguments)]
fn ___action105<'input>(___lookbehind: &usize, ___lookahead: &usize) -> ast::Program {
    let ___start0 = *___lookbehind;
    let ___end0 = *___lookahead;
    let ___temp0 = ___action61(&___start0, &___end0);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action1(___temp0)
}

#[allow(clippy::too_many_arguments)]
fn ___action106<'input>(___0: (usize, alloc::vec::Vec<ast::Definition>, usize)) -> ast::Program {
    let ___start0 = ___0.0;
    let ___end0 = ___0.2;
    let ___temp0 = ___action62(___0);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action1(___temp0)
}

#[allow(clippy::too_many_arguments)]
fn ___action107<'input>(
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, Tok<'input>, usize),
) -> ast::Statement {
    let ___start0 = ___0.2;
    let ___end0 = ___1.0;
    let ___temp0 = ___action51(&___start0, &___end0);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action99(___0, ___temp0, ___1)
}

#[allow(clippy::too_many_arguments)]
fn ___action108<'input>(
    ___0: (usize, Tok<'input>, usize),
    ___1: (usize, alloc::vec::Vec<ast::Statement>, usize),
    ___2: (usize, Tok<'input>, usize),
) -> ast::Statement {
    let ___start0 = ___1.0;
    let ___end0 = ___1.2;
    let ___temp0 = ___action52(___1);
    let ___temp0 = (___start0, ___temp0, ___end0);
    ___action99(___0, ___temp0, ___2)
}
#[allow(clippy::type_complexity)]

pub trait ___ToTriple<'input> {
    fn to_triple(
        value: Self,
    ) -> Result<
        (usize, Tok<'input>, usize),
        ___lalrpop_util::ParseError<usize, Tok<'input>, tok::Error>,
    >;
}

impl<'input> ___ToTriple<'input> for (usize, Tok<'input>, usize) {
    fn to_triple(
        value: Self,
    ) -> Result<
        (usize, Tok<'input>, usize),
        ___lalrpop_util::ParseError<usize, Tok<'input>, tok::Error>,
    > {
        Ok(value)
    }
}
impl<'input> ___ToTriple<'input> for Result<(usize, Tok<'input>, usize), tok::Error> {
    fn to_triple(
        value: Self,
    ) -> Result<
        (usize, Tok<'input>, usize),
        ___lalrpop_util::ParseError<usize, Tok<'input>, tok::Error>,
    > {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(___lalrpop_util::ParseError::User { error }),
        }
    }
}
