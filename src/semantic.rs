// use crate::ast::*;
// use crate::env::*;

// pub fn semantic_check(stmt: Vec<AstNode>) -> Vec<AstNode> {
//     let mut ev = Env::new();
//     let mut _stmt = stmt.clone();
//     for (_, e) in _stmt.iter_mut().enumerate() {
//         match e {
//             AstNode::FnDecl(_, _, _) => check_fndecl(&mut ev, e),
//             AstNode::VarDecl(_, _, _) => check_vardecl(&mut ev, e, true),
//             _ => (),
//         }
//     }
//     _stmt
// }

// fn prototype_fn(ev: &mut Env, ident: String, p: &mut Vec<AstNode>) -> String {
//     ident + &join_param(ev, p)
// }

// fn check_fndecl(ev: &mut Env, n: &mut AstNode) {
//     if let AstNode::FnDecl(ident, ref mut param, block) = n {
//         let proto = prototype_fn(ev, ident_name(&ident), param);
//         if ev.global_defined(&proto) {
//             unreachable!("redefine function:{}", proto)
//         }
//         if let AstNode::Id(_, typ) = *ident.clone() {
//             ev.global_def(&proto, typ);
//             ev.enter_scope();
//             define_local_var(ev, &param);
//         }
//         check_stmtblock(ev, block);
//         // println!("[leave_scope]:\n{}", ev);
//         ev.leave_scope();
//     }
// }

// fn check_stmtblock(ev: &mut Env, block: &mut Vec<AstNode>) {
//     for stmt in block {
//         check_expr(ev, stmt)
//     }
// }

// fn check_expr(ev: &mut Env, stmt: &mut AstNode) {
//     match stmt {
//         AstNode::VarDecl(_, _, _) => {
//             check_vardecl(ev, stmt, false);
//         }
//         AstNode::Assignment(_, _) => {
//             check_assignstmt(ev, stmt);
//         }
//         AstNode::IfStmt(ref mut cond, ref mut tblock, ref mut fblock) => {
//             assert!(typeof_bool_expr(ev, cond) != AstType::Undef);
//             check_stmtblock(ev, tblock);
//             check_stmtblock(ev, fblock);
//         }
//         AstNode::WhileStmt(ref mut cond, ref mut block) => {
//             assert!(typeof_bool_expr(ev, cond) != AstType::Undef);
//             check_stmtblock(ev, block);
//         }
//         AstNode::ReturnStmt(ref mut expr, ref mut typ) => {
//             *typ = typeof_value_expr(ev, expr);
//         }
//         _ => {
//             if typeof_bool_expr(ev, stmt) == AstType::Undef {
//                 panic!("typeof_bool_expr");
//             }
//         }
//     }
// }

// fn check_assignstmt(ev: &mut Env, n: &mut AstNode) {
//     if let AstNode::Assignment(ref mut var, ref mut valexpr) = n {
//         let vname = ident_name(var);
//         let ltyp = ev.lookup(&vname).unwrap();
//         let rtyp = typeof_value_expr(ev, valexpr);
//         match ltyp {
//             AstType::Ext(_) | AstType::Undef => {
//                 update_ident_type(var, rtyp.clone());
//                 ev.update(var, rtyp);
//             }
//             _ => {
//                 if ltyp != rtyp {
//                     unreachable!("unmatch {} {}", ltyp, rtyp);
//                 }
//             }
//         }
//     }
// }

// fn check_vardecl(ev: &mut Env, n: &mut AstNode, global: bool) {
//     if let AstNode::VarDecl(ref mut var, ref mut val, ref mut typ) = n {
//         let vname = ident_name(&var);
//         if ev.can_lookup(&vname) {
//             panic!("redefine '{}'", vname);
//         }
//         if global {
//             ev.global_def(&vname, typ.clone());
//         } else {
//             ev.local_def(&vname, typ.clone());
//         }
//         let valty = typeof_value_expr(ev, val);
//         let ty = if valty != AstType::Nil {
//             valty.clone()
//         } else {
//             typ.clone()
//         };
//         ev.update(var, ty);
//     }
// }

// fn typeof_value_expr(ev: &mut Env, n: &mut AstNode) -> AstType {
//     match n {
//         AstNode::BinaryOp(_, _, _, _) => {
//             // if !is_math_op(*op) { unreachable!("unmatch math Op{}", op); }
//             typeof_binary_op(ev, n)
//         }
//         _ => typeof_valobj(ev, n),
//     }
// }

// fn typeof_bool_expr(ev: &mut Env, n: &mut AstNode) -> AstType {
//     match n {
//         AstNode::BinaryOp(_, op, _, _) => {
//             if !is_logic_op(*op) {
//                 unreachable!("unmatch logic Op{}", op);
//             }
//             typeof_binary_op(ev, n)
//         }
//         _ => typeof_valobj(ev, n),
//     }
// }

// fn typeof_valobj(ev: &mut Env, n: &mut AstNode) -> AstType {
//     match n {
//         AstNode::Int(_) => AstType::Int,
//         AstNode::Float(_) => AstType::Float,
//         AstNode::Str(_) => AstType::Str,
//         AstNode::Id(var, _) => {
//             if !ev.can_lookup(&var) {
//                 panic!("cann't resolve {}", var);
//             }
//             ev.lookup(&var).unwrap()
//         }
//         AstNode::Call(ident, param) => {
//             let proto = prototype_fn(ev, ident_name(&ident), param);
//             match ev.global_resolve(&proto) {
//                 Some(typ) => typ.clone(),
//                 None => unreachable!("cann't resolve fn proto:{}", proto),
//             }
//         }
//         AstNode::BinaryOp(_, _, _, _) => typeof_value_expr(ev, n),
//         AstNode::Nil => AstType::Nil,
//         _ => unreachable!(),
//     }
// }

// fn typeof_binary_op(ev: &mut Env, n: &mut AstNode) -> AstType {
//     if let AstNode::BinaryOp(ref mut lhs, _, ref mut rhs, ref mut typ) = n {
//         let rtyp = typeof_valobj(ev, rhs);
//         let ltyp = match *lhs.clone() {
//             AstNode::BinaryOp(_, _, _, _) => typeof_binary_op(ev, lhs),
//             _ => typeof_valobj(ev, lhs),
//         };
//         if rtyp != ltyp {
//             unreachable!("unexpected {} == {}", ltyp, rtyp);
//         }
//         *typ = rtyp.clone();
//         return rtyp;
//     }
//     unreachable!();
// }

// fn _is_math_op(op: Op) -> bool {
//     match op {
//         Op::PLUS | Op::SUB | Op::MUL | Op::DIV => true,
//         _ => false,
//     }
// }

// fn define_local_var(ev: &mut Env, p: &Vec<AstNode>) {
//     for var in p {
//         if let AstNode::Id(name, typ) = var {
//             ev.local_def(&name, typ.clone());
//         }
//     }
// }

// fn typeof_param(ev: &mut Env, n: AstNode) -> AstType {
//     let mut rtyp = AstType::Undef;
//     if let AstNode::Id(_, typ) = n {
//         match typ {
//             AstType::Ext(name) => {
//                 rtyp = ev.lookup(&name).unwrap();
//             }
//             _ => rtyp = typ,
//         }
//     }
//     rtyp
// }

// fn join_param(ev: &mut Env, p: &mut Vec<AstNode>) -> String {
//     let mut typs = vec![":".to_string()];
//     let mut typ: AstType;
//     for item in p {
//         match item {
//             AstNode::Id(_, _) => {
//                 typ = typeof_param(ev, item.clone());
//             }
//             _ => {
//                 typ = typeof_value_expr(ev, item);
//             }
//         }
//         typs.push(typ.to_string());
//     }
//     return typs.join("-");
// }
