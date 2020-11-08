// extern crate llvm_sys as llvm;

// use self::llvm::core::*;
// use self::llvm::prelude::*;

// use crate::ast::*;
// use llvm::LLVMIntPredicate::*;
// use llvm::LLVMRealPredicate::*;
// use std::collections::HashMap;
// use std::ffi::CString;
// use std::ptr;

// type SymbolTable = HashMap<String, IRValue>;
// type TypeTable = HashMap<String, LLVMTypeRef>;

// pub struct LLVMGenerator {
//     pub ctx: LLVMContextRef,
//     pub module: LLVMModuleRef,
//     pub builder: LLVMBuilderRef,

//     pub structs: TypeTable,
//     pub functions: SymbolTable,
//     pub loops: Vec<(LLVMBasicBlockRef, LLVMBasicBlockRef)>,
//     pub global: SymbolTable,
//     pub locals: Vec<SymbolTable>,
// }

// #[derive(Debug, Clone)]
// pub struct IRValue {
//     pub val: LLVMValueRef,
//     pub kind: ValueKind,
// }

// #[derive(Debug, Clone)]
// pub enum ValueKind {
//     Ref,
//     Const,
// }

// // type arithmetic_prototype = unsafe extern "C" fn(LLVMBuilderRef, LLVMValueRef, LLVMValueRef, *const i8) -> LLVMValueRef;

// macro_rules! c_str {
//     ($s:expr) => {
//         concat!($s, "\0").as_ptr() as *const i8
//     };
// }

// macro_rules! ir_ref {
//     ($s:expr) => {
//         IRValue::new_ref($s)
//     };
// }

// unsafe fn convert_cstring(cstr: *mut i8) -> String {
//     let _str = CString::from_raw(cstr);
//     _str.into_string().unwrap()
// }

// macro_rules! ir_const {
//     ($s:expr) => {
//         IRValue::new_const($s)
//     };
// }

// impl IRValue {
//     pub fn new_const(v: LLVMValueRef) -> Self {
//         IRValue {
//             val: v,
//             kind: ValueKind::Const,
//         }
//     }
//     pub fn new_ref(v: LLVMValueRef) -> Self {
//         IRValue {
//             val: v,
//             kind: ValueKind::Ref,
//         }
//     }
// }

// impl LLVMGenerator {
//     pub unsafe fn new() -> Self {
//         let _ctx = LLVMContextCreate();
//         let _mod = LLVMModuleCreateWithNameInContext(b"__module\0".as_ptr() as *const _, _ctx);
//         LLVMGenerator {
//             ctx: _ctx,
//             module: _mod,
//             builder: LLVMCreateBuilderInContext(_ctx),

//             structs: HashMap::new(),
//             functions: HashMap::new(),
//             global: HashMap::new(),
//             locals: Vec::new(),
//             loops: Vec::new(),
//         }
//     }

//     pub unsafe fn run(&mut self, name: &String, module: &Vec<AstNode>) {
//         for item in module {
//             match item {
//                 AstNode::FnDecl(_, _, _) => self.gen_fndecl(item.clone()),
//                 AstNode::StructDecl(_, _) => self.gen_struct(item.clone()),
//                 // AstNode::VarDecl(_, _, _) => self.gen_vardecl(&item, true),
//                 _ => (),
//             }
//         }

//         let mut module_ir = convert_cstring(LLVMPrintModuleToString(self.module));

//         for (_, ty_ref) in &self.structs {
//             let sty = convert_cstring(LLVMPrintTypeToString(*ty_ref));
//             module_ir.push_str(&format!("{}\n", sty));
//             // LLVMDisposeMessage(sty);
//         }

//         println!("{}", module_ir);

//         std::fs::write(format!("{}.ll", name), module_ir).expect(&format!("[err] write {}", name));
//         // let out_file = CString::new(format!("{}.ll", name)).unwrap();
//         // LLVMPrintModuleToFile(self.module, out_file.as_ptr(), ptr::null_mut());
//         LLVMDisposeBuilder(self.builder);
//         LLVMDisposeModule(self.module);
//         LLVMContextDispose(self.ctx);
//     }

//     fn enter_scope(&mut self) {
//         self.locals.push(HashMap::new());
//     }

//     fn leave_scope(&mut self) {
//         self.locals.pop();
//     }

//     fn get(&self, var: &String) -> Option<IRValue> {
//         let mut stk = self.locals.clone();
//         stk.reverse();
//         for s in stk.iter() {
//             if s.contains_key(var) {
//                 return s.get(var).cloned();
//             }
//         }
//         self.global.get(var).cloned()
//     }

//     unsafe fn gen_fndecl(&mut self, n: AstNode) {
//         if let AstNode::FnDecl(ident, param, block) = n {
//             let function_name = ident_name(&ident);
//             let function_type = {
//                 let return_type = self.typeof_llvm(ident_type(&ident.clone()));
//                 let mut param_types = self.gen_param_type(&param);
//                 LLVMFunctionType(
//                     return_type,
//                     param_types.as_mut_ptr(),
//                     param_types.len() as u32,
//                     0,
//                 )
//             };
//             let function = LLVMAddFunction(
//                 self.module,
//                 function_name.into_bytes().as_ptr() as *const _,
//                 function_type,
//             );
//             let entry = CString::new("entry").unwrap();
//             self.functions.insert(ident_name(&ident), ir_ref!(function));
//             self.enter_scope();
//             let bb = LLVMAppendBasicBlockInContext(self.ctx, function, entry.as_ptr());
//             LLVMPositionBuilderAtEnd(self.builder, bb);
//             self.alloc_param(function, &param);
//             self.gen_block(&block);
//             if LLVMGetBasicBlockTerminator(bb).is_null() {
//                 self.gen_default_return(ident_type(&ident.clone()));
//             }
//             self.leave_scope();
//         }
//     }

//     unsafe fn gen_struct(&mut self, n: AstNode) {
//         if let AstNode::StructDecl(ident, block) = n {
//             let cname = CString::new(ident_name(&ident)).unwrap();
//             let mut member: Vec<LLVMTypeRef> = block
//                 .into_iter()
//                 .map(|e| self.typeof_llvm(ident_type(&e)))
//                 .collect();
//             let sty = LLVMStructCreateNamed(self.ctx, cname.as_ptr());
//             LLVMStructSetBody(sty, member.as_mut_ptr(), member.len() as u32, 0);

//             // let fptr = LLVMGetTypeByName(self.module, cname.as_ptr());
//             // println!("cname:{:?} member: {:?} sty {:?} p:{:?}", cname, member, sty, fptr);
//             self.push_struct(ident_name(&ident), sty);

//             return;
//         }
//         unreachable!("[gen_struct]: {:?}", n);
//     }

//     unsafe fn alloc_param(&mut self, func: LLVMValueRef, p: &Vec<AstNode>) {
//         for (idx, var) in p.iter().enumerate() {
//             let cname = CString::new(ident_name(&var)).unwrap();
//             let ty = self.typeof_llvm(ident_type(&var));
//             let _var = LLVMBuildAlloca(self.builder, ty, cname.as_ptr());
//             self.push_var(ident_name(&var), ir_ref!(_var));
//             let val = LLVMGetParam(func, idx as u32);
//             LLVMBuildStore(self.builder, val, _var);
//         }
//     }

//     fn push_var(&mut self, var: String, val: IRValue) {
//         let idx = self.locals.len();
//         self.locals[idx - 1].insert(var, val);
//     }

//     fn push_global_var(&mut self, var: String, val: IRValue) {
//         self.global.insert(var, val);
//     }

//     fn push_struct(&mut self, var: String, val: LLVMTypeRef) {
//         self.structs.insert(var, val);
//     }

//     unsafe fn gen_vardecl(&mut self, var: &AstNode, global: bool) {
//         if let AstNode::VarDecl(ident, val, _) = var {
//             let cname = CString::new(ident_name(&ident)).unwrap();
//             let ty = self.typeof_llvm(ident_type(&ident));
//             let pvar = LLVMBuildAlloca(self.builder, ty, cname.as_ptr());
//             let _var = ir_ref!(pvar);
//             if global {
//                 self.push_global_var(ident_name(&ident), _var);
//             } else {
//                 self.push_var(ident_name(&ident), _var);
//             }

//             if !nil_node(val) {
//                 LLVMBuildStore(self.builder, self.gen_initializer(val), pvar);
//             }
//         }
//     }

//     unsafe fn gen_initializer(&mut self, expr: &AstNode) -> LLVMValueRef {
//         let irv = match expr {
//             AstNode::BinaryOp(_, _, _, _) => self.gen_op(expr),
//             _ => self.gen_value(expr),
//         };
//         return self.load(&irv);
//     }

//     unsafe fn gen_return(&mut self, expr: &AstNode) {
//         if let AstNode::ReturnStmt(var, _) = expr {
//             let irv = match *var.clone() {
//                 AstNode::BinaryOp(_, _, _, _) => self.gen_op(var),
//                 _ => self.gen_value(var),
//             };
//             LLVMBuildRet(self.builder, self.load(&irv));
//             return;
//         }
//         unreachable!("[gen_return] {:?}", expr);
//     }

//     unsafe fn gen_default_return(&mut self, ty: AstType) {
//         let irv = self.llvm_default_value(ty);
//         LLVMBuildRet(self.builder, irv);
//     }

//     unsafe fn gen_value(&mut self, val: &AstNode) -> IRValue {
//         match val {
//             AstNode::Int(v) => ir_const!(LLVMConstInt(self.i64_type(), *v as u64, 1)),
//             AstNode::Float(v) => ir_const!(LLVMConstReal(self.f64_type(), *v as f64)),
//             AstNode::Call(_, _) => self.gen_call(val),
//             AstNode::Id(name, _) => self.get(name).unwrap(),
//             AstNode::BinaryOp(_, _, _, _) => self.gen_op(val),
//             // TODO: supports String
//             _ => unreachable!("{:?}", val),
//         }
//     }

//     unsafe fn gen_call(&mut self, func: &AstNode) -> IRValue {
//         if let AstNode::Call(ident, args) = func {
//             let name = ident_name(&ident);
//             let fnptr = self.functions[&name].val;
//             let mut _args: Vec<LLVMValueRef> =
//                 args.into_iter().map(|n| self.gen_initializer(n)).collect();
//             return ir_const!(LLVMBuildCall(
//                 self.builder,
//                 fnptr,
//                 _args.as_mut_ptr(),
//                 _args.len() as u32,
//                 c_str!("")
//             ));
//         }
//         unreachable!();
//     }

//     unsafe fn gen_conditional(&mut self, expr: &AstNode) -> LLVMValueRef {
//         match expr {
//             AstNode::BinaryOp(lhs, _, _, _) => match *lhs.clone() {
//                 AstNode::BinaryOp(_, _, _, _) => self.gen_conditional(lhs),
//                 _ => self.gen_expr_cmp(expr).val,
//             },
//             _ => self.gen_value(expr).val,
//         }
//     }

//     fn llvm_int_op(&mut self, op: &Op) -> llvm::LLVMIntPredicate {
//         match op {
//             Op::EQ => LLVMIntEQ,
//             Op::GT => LLVMIntSGT,
//             Op::LT => LLVMIntSLT,
//             _ => unreachable!(),
//         }
//     }

//     fn llvm_float_op(&mut self, op: &Op) -> llvm::LLVMRealPredicate {
//         match op {
//             Op::EQ => LLVMRealOEQ,
//             Op::GT => LLVMRealOGT,
//             Op::LT => LLVMRealOLT,
//             _ => unreachable!(),
//         }
//     }

//     unsafe fn gen_expr_cmp(&mut self, expr: &AstNode) -> IRValue {
//         if let AstNode::BinaryOp(lhs, op, rhs, ty) = expr {
//             let lval = self.gen_value(lhs);
//             let rval = self.gen_value(rhs);
//             let val = match ty {
//                 AstType::Float => LLVMBuildFCmp(
//                     self.builder,
//                     self.llvm_float_op(op),
//                     self.load(&lval),
//                     self.load(&rval),
//                     c_str!(""),
//                 ),
//                 AstType::Int => LLVMBuildICmp(
//                     self.builder,
//                     self.llvm_int_op(op),
//                     self.load(&lval),
//                     self.load(&rval),
//                     c_str!(""),
//                 ),
//                 _ => unreachable!(),
//             };
//             return ir_const!(val);
//         }
//         unreachable!();
//     }

//     unsafe fn load(&mut self, var: &IRValue) -> LLVMValueRef {
//         match var.kind {
//             ValueKind::Ref => LLVMBuildLoad(self.builder, var.val, c_str!("")),
//             ValueKind::Const => var.val,
//         }
//     }

//     unsafe fn gen_op(&mut self, expr: &AstNode) -> IRValue {
//         if let AstNode::BinaryOp(var, op, val, ty) = expr {
//             let lhs = match *var.clone() {
//                 AstNode::BinaryOp(_, _, _, _) => self.gen_op(&var),
//                 _ => self.gen_value(var),
//             };
//             let rhs = self.gen_value(val);
//             match op {
//                 Op::PLUS => match ty {
//                     AstType::Float => {
//                         return ir_const!(LLVMBuildFAdd(
//                             self.builder,
//                             self.load(&lhs),
//                             self.load(&rhs),
//                             c_str!("")
//                         ));
//                     }
//                     AstType::Int => {
//                         return ir_const!(LLVMBuildAdd(
//                             self.builder,
//                             self.load(&lhs),
//                             self.load(&rhs),
//                             c_str!("")
//                         ));
//                     }
//                     _ => unreachable!("[gen_op] {:?}", ty),
//                 },
//                 Op::SUB => match ty {
//                     AstType::Float => {
//                         return ir_const!(LLVMBuildFSub(
//                             self.builder,
//                             self.load(&lhs),
//                             self.load(&rhs),
//                             c_str!("")
//                         ));
//                     }
//                     AstType::Int => {
//                         return ir_const!(LLVMBuildSub(
//                             self.builder,
//                             self.load(&lhs),
//                             self.load(&rhs),
//                             c_str!("")
//                         ));
//                     }
//                     _ => unreachable!("[gen_op] {:?}", ty),
//                 },
//                 Op::EQ => {
//                     return self.gen_expr_cmp(expr);
//                 }
//                 Op::MUL => match ty {
//                     AstType::Float => {
//                         return ir_const!(LLVMBuildFMul(
//                             self.builder,
//                             self.load(&lhs),
//                             self.load(&rhs),
//                             c_str!("")
//                         ));
//                     }
//                     AstType::Int => {
//                         return ir_const!(LLVMBuildMul(
//                             self.builder,
//                             self.load(&lhs),
//                             self.load(&rhs),
//                             c_str!("")
//                         ));
//                     }
//                     _ => unreachable!("[gen_op] {:?}", ty),
//                 },
//                 _ => unreachable!("[gen_op]: {:?} -> Op: {:?}", expr, op),
//             }
//         }
//         unreachable!("{:?}", expr);
//     }

//     unsafe fn gen_param_type(&mut self, n: &Vec<AstNode>) -> Vec<LLVMTypeRef> {
//         let mut ty = Vec::new();
//         for ident in n {
//             ty.push(self.typeof_llvm(ident_type(ident)));
//         }
//         return ty;
//     }

//     unsafe fn gen_block(&mut self, stmts: &Vec<AstNode>) -> bool {
//         let mut ret = false;
//         for stmt in stmts {
//             match stmt {
//                 AstNode::VarDecl(_, _, _) => self.gen_vardecl(stmt, false),
//                 AstNode::IfStmt(_, _, _) => self.gen_ifstmt(stmt),
//                 AstNode::Assignment(_, _) => self.gen_assign(stmt),
//                 AstNode::ReturnStmt(_, _) => {
//                     self.gen_return(stmt);
//                     ret = true;
//                 }
//                 AstNode::WhileStmt(_, _) => self.gen_while(stmt),
//                 // AstNode::StructDecl(_, _,) => self.gen_struct(stmt),
//                 _ => (),
//             }
//         }
//         return ret;
//     }

//     unsafe fn gen_assign(&mut self, stmt: &AstNode) {
//         if let AstNode::Assignment(var, val) = stmt {
//             let _var = self.get(&ident_name(var)).unwrap();
//             LLVMBuildStore(self.builder, self.gen_initializer(val), _var.val);
//             return;
//         }
//         unreachable!();
//     }

//     unsafe fn gen_while(&mut self, stmt: &AstNode) {
//         if let AstNode::WhileStmt(cond, body) = stmt {
//             let parent = LLVMGetBasicBlockParent(LLVMGetInsertBlock(self.builder));
//             let cond_block = LLVMAppendBasicBlock(parent, c_str!("while:cond"));
//             let body_block = LLVMAppendBasicBlock(parent, c_str!("while:body"));
//             let merge_block = LLVMAppendBasicBlock(parent, c_str!("while:merge"));

//             LLVMBuildBr(self.builder, cond_block);
//             LLVMPositionBuilderAtEnd(self.builder, cond_block);
//             let condval = self.gen_conditional(cond);
//             LLVMBuildCondBr(self.builder, condval, body_block, merge_block);
//             self.loops.push((cond_block, merge_block));
//             // move to body block
//             LLVMMoveBasicBlockAfter(body_block, LLVMGetInsertBlock(self.builder));
//             LLVMPositionBuilderAtEnd(self.builder, body_block);
//             self.gen_block(body);
//             if LLVMGetBasicBlockTerminator(LLVMGetInsertBlock(self.builder)).is_null() {
//                 LLVMBuildBr(self.builder, cond_block);
//             }
//             self.loops.pop();
//             // move to merge block
//             // TODO: fix empty merge block
//             LLVMMoveBasicBlockAfter(merge_block, LLVMGetInsertBlock(self.builder));
//             LLVMPositionBuilderAtEnd(self.builder, merge_block);
//             LLVMGetUndef(LLVMVoidType());
//             return;
//         }
//         unreachable!("[gen_while]: {:?}", stmt);
//     }

//     unsafe fn gen_ifstmt(&mut self, stmt: &AstNode) {
//         if let AstNode::IfStmt(cond, tstmt, fstmt) = stmt {
//             let condval = self.gen_conditional(cond);

//             let current = LLVMGetInsertBlock(self.builder);
//             let parent = LLVMGetBasicBlockParent(current);

//             let tblock = LLVMAppendBasicBlock(parent, c_str!("if:then"));
//             let eblock = LLVMAppendBasicBlock(parent, c_str!("if:else"));
//             let mblock = LLVMAppendBasicBlock(parent, c_str!("if:merge"));

//             LLVMBuildCondBr(self.builder, condval, tblock, eblock);
//             LLVMMoveBasicBlockAfter(tblock, LLVMGetInsertBlock(self.builder));
//             LLVMPositionBuilderAtEnd(self.builder, tblock);
//             let mut then_term = true;
//             self.gen_block(tstmt);
//             if LLVMGetBasicBlockTerminator(tblock).is_null() {
//                 LLVMBuildBr(self.builder, mblock);
//                 then_term = false;
//             }
//             // if !self.gen_block(tstmt) { LLVMBuildBr(self.builder, mblock); }

//             LLVMMoveBasicBlockAfter(eblock, LLVMGetInsertBlock(self.builder));
//             LLVMPositionBuilderAtEnd(self.builder, eblock);
//             let mut else_term = true;
//             self.gen_block(fstmt);
//             if LLVMGetBasicBlockTerminator(eblock).is_null() {
//                 LLVMBuildBr(self.builder, mblock);
//                 else_term = false;
//             }
//             // if !self.gen_block(fstmt) { LLVMBuildBr(self.builder, mblock); }

//             // TODO: fix empty merge block
//             if then_term && else_term {
//                 LLVMDeleteBasicBlock(mblock);
//             } else {
//                 LLVMMoveBasicBlockAfter(mblock, LLVMGetInsertBlock(self.builder));
//                 LLVMPositionBuilderAtEnd(self.builder, mblock);
//             }
//         }
//     }

//     unsafe fn typeof_llvm(&mut self, t: AstType) -> LLVMTypeRef {
//         match t {
//             AstType::Int => LLVMInt64TypeInContext(self.ctx),
//             AstType::Float => LLVMFloatTypeInContext(self.ctx),
//             // TODO: AstType::Str => LLVMConstStringInContext(self.ctx),
//             AstType::Bool => LLVMInt1TypeInContext(self.ctx),
//             AstType::Ext(name) => *self.structs.get(&name).unwrap(),
//             _ => LLVMInt8TypeInContext(self.ctx),
//         }
//     }

//     unsafe fn llvm_default_value(&mut self, t: AstType) -> LLVMValueRef {
//         match t {
//             AstType::Int => LLVMConstInt(self.i64_type(), 0 as u64, 1),
//             AstType::Float => LLVMConstReal(self.f64_type(), 0 as f64),
//             AstType::Bool => LLVMConstInt(self.bool_type(), 0 as u64, 0),
//             _ => LLVMConstInt(self.i64_type(), 0 as u64, 1),
//         }
//     }

//     unsafe fn i64_type(&self) -> LLVMTypeRef {
//         LLVMInt64TypeInContext(self.ctx)
//     }

//     unsafe fn f64_type(&self) -> LLVMTypeRef {
//         LLVMFloatTypeInContext(self.ctx)
//     }

//     unsafe fn bool_type(&self) -> LLVMTypeRef {
//         LLVMInt1TypeInContext(self.ctx)
//     }
// }

// #[test]
// fn codegen_test() {
//     use crate::ast::*;
//     use crate::codegen::*;
//     use crate::grammar::ModuleParser;
//     use crate::semantic::*;
//     let sources = r#"
//         fn foo1(a: int, b: int) -> int {
//             let c = a + 1001;
//             let d: int;
//             let ok = 123.456;
//             if ok > 100.123 {
//                 let val = 123.24;
//                 d = b + 1992 + c + a;
//                 val = val + 0.87;
//             }
//             if c > 100 {
//                 let bv = 1002;
//                 c = bv + c;
//             }
//             return a;
//         }

//         fn foo2(a: int) -> bool {
//             return a == 100;
//         }

//         fn fact(n: int) -> int {
//             if n == 1 { return 1; }
//             else { return fact(n - 1) * n; }
//         }

//         fn main() {
//             let a = 1000 + 10;
//             let b: int;
//             a = foo1(a, 1001) + 123 + foo1(a, 100+101);
//             b = foo1(123, a);
//             while a > b + 100 {
//                 b = a + foo1(a, b);
//             }
//         }
//     "#;
//     let stmts = ModuleParser::new().parse(sources).unwrap();
//     let typed_ast = semantic_check(stmts);
//     unsafe {
//         let mut generator = LLVMGenerator::new();
//         generator.run(&"demo".to_string(), &typed_ast);
//     }
// }
