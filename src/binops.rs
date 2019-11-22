/// Helper functions for translating binops.

use binaryen::sys;
use rustc::mir::BinOp;
use rustc::ty;
use rustc::ty::Ty;
use syntax::ast::IntTy;

pub fn binaryen_op_for<'tcx>(op: BinOp, ty: Ty<'tcx>) -> sys::BinaryenOp {
    match ty.sty {
        ty::TyInt(IntTy::I32) |
        ty::TyInt(IntTy::I16) => {
            match op {
                BinOp::Add => sys::BinaryenAddInt32(),
                BinOp::Sub => sys::BinaryenSubInt32(),
                BinOp::Mul => sys::BinaryenMulInt32(),
                BinOp::Div => sys::BinaryenDivSInt32(),
                BinOp::BitAnd => sys::BinaryenAndInt32(),
                BinOp::BitOr => sys::BinaryenOrInt32(),
                BinOp::BitXor => sys::BinaryenXorInt32(),
                BinOp::Eq => sys::BinaryenEqInt32(),
                BinOp::Ne => sys::BinaryenNeInt32(),
                BinOp::Lt => sys::BinaryenLtSInt32(),
                BinOp::Le => sys::BinaryenLeSInt32(),
                BinOp::Gt => sys::BinaryenGtSInt32(),
                BinOp::Ge => sys::BinaryenGeSInt32(),
                _ => panic!("unimplemented BinOp: {:?}", op),
            }
        }
        ty::TyInt(IntTy::I64) => {
            match op {
                BinOp::Add => sys::BinaryenAddInt64(),
                BinOp::Sub => sys::BinaryenSubInt64(),
                BinOp::Mul => sys::BinaryenMulInt64(),
                BinOp::Div => sys::BinaryenDivSInt64(),
                BinOp::BitAnd => sys::BinaryenAndInt64(),
                BinOp::BitOr => sys::BinaryenOrInt64(),
                BinOp::BitXor => sys::BinaryenXorInt64(),
                BinOp::Eq => sys::BinaryenEqInt64(),
                BinOp::Ne => sys::BinaryenNeInt64(),
                BinOp::Lt => sys::BinaryenLtSInt64(),
                BinOp::Le => sys::BinaryenLeSInt64(),
                BinOp::Gt => sys::BinaryenGtSInt64(),
                BinOp::Ge => sys::BinaryenGeSInt64(),
                _ => panic!("unimplemented BinOp: {:?}", op),
            }
        }
        ref otherwise => panic!("binops on {:?} are not implemented", otherwise),
    }
}
