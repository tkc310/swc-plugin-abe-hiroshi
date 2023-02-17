use swc_core::{
    ecma::{
        ast::{Program, CallExpr, Callee, Expr, MemberExpr, MemberProp, Lit, Str},
        visit::{VisitMut, as_folder, FoldWith},
        atoms::{JsWord},
    },
    common::{DUMMY_SP},
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata}
};

pub struct TransformVisitor;

impl VisitMut for TransformVisitor {
    fn visit_mut_call_expr(&mut self, call: &mut CallExpr) {
        if let Callee::Expr(expr) = &call.callee {
            if let Expr::Member(MemberExpr { obj, prop, .. }) = &**expr {
                if let MemberProp::Ident(ident) = prop {
                    if ident.sym == *"log" {
                        if let Expr::Ident(ident) = &**obj {
                            if ident.sym == *"console" {
                                call.args[0].expr = Box::new(Expr::Lit(Lit::Str(Str {
                                    span: DUMMY_SP,
                                    value: JsWord::from("阿部寛"),
                                    raw: None,
                                })));
                            }
                        }
                    }
                }
            }
        }
    }
}

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(TransformVisitor))
}

#[cfg(test)]
mod test;
