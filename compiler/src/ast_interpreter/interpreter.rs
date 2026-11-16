use crate::{
    api::SmolAPI,
    ast::{AST, Constant, ExprId, ExprNode, Method},
    ast_interpreter::{memory::Memory, value::Value},
};
use alloc::{collections::VecDeque, string::ToString, vec::Vec};

#[derive(Debug, Clone)]
struct CheckPoint<'src> {
    prev_program_counter: ExprId,
    program_counter: ExprId,
    call_stack: Vec<ExprId>,
    values_stack: Vec<Value>,
    fun_decl: Vec<ExprId>,
    memory: Memory<'src>,
}

#[derive(Debug, Clone)]
pub struct Interpreter<'src, T: SmolAPI> {
    api: T,

    keep_history: bool,

    history: Vec<CheckPoint<'src>>,
    pub step: usize,

    pub ast: AST<'src>,

    // All the data needed to be stored to do a checkpoint.
    pub prev_program_counter: ExprId,
    pub program_counter: ExprId,
    pub call_stack: Vec<ExprId>,
    pub values_stack: Vec<Value>,
    pub fun_decl: Vec<ExprId>,
    pub memory: Memory<'src>,
}

impl<'src, T: SmolAPI> Interpreter<'src, T> {
    pub fn new(ast: AST<'src>, keep_history: bool, api: T) -> Self {
        let program_counter = ast.root;
        Self {
            keep_history,
            history: Vec::new(),

            ast,
            step: 0,

            prev_program_counter: program_counter,
            program_counter,
            call_stack: Vec::new(),
            values_stack: Vec::new(),

            fun_decl: Vec::new(),

            memory: Memory::EMPTY,

            api,
        }
    }
    pub fn checkpoint(&self) -> CheckPoint<'src> {
        CheckPoint {
            prev_program_counter: self.prev_program_counter,
            program_counter: self.program_counter,
            call_stack: self.call_stack.clone(),
            values_stack: self.values_stack.clone(),
            fun_decl: self.fun_decl.clone(),
            memory: self.memory.clone(),
        }
    }
    pub fn restore(&mut self, checkpoint: CheckPoint<'src>) {
        self.prev_program_counter = checkpoint.prev_program_counter;
        self.program_counter = checkpoint.program_counter;
        self.call_stack = checkpoint.call_stack;
        self.values_stack = checkpoint.values_stack;
        self.fun_decl = checkpoint.fun_decl;
        self.memory = checkpoint.memory;
    }
    pub fn get_value(&mut self) -> Option<Value> {
        if self.values_stack.len() == 1 {
            Some(self.values_stack.pop().unwrap())
        } else {
            None
        }
    }
    pub fn jump(&mut self, pc: ExprId) {
        self.prev_program_counter = self.program_counter;
        self.program_counter = pc;
    }
    pub fn call(&mut self, pc: ExprId) {
        self.call_stack.push(self.program_counter);
        self.jump(pc);
    }
    pub fn ret(&mut self) -> Option<()> {
        if self.call_stack.is_empty() {
            None
        } else {
            self.prev_program_counter = self.program_counter;
            self.program_counter = self.call_stack.pop().unwrap();
            Some(())
        }
    }

    fn get_fun(&self, function_name: ExprId) -> ExprId {
        let function_name = self.ast.get_slice(function_name);
        let function_id = self
            .fun_decl
            .iter()
            .rev()
            .find(|fun_id| {
                let ExprNode::Decl { name, .. } = self.ast.exprs[**fun_id] else {
                    panic!("Should have been prevented by type checking.")
                };
                let name = self.ast.get_slice(name);
                name == function_name
            })
            .unwrap();
        *function_id
    }

    fn get_var_from_index(&self, mut index: ExprId) -> &'src str {
        while let ExprNode::Index { body, .. } = self.ast.exprs[index] {
            index = body;
        }
        self.ast.get_slice(index)
    }
}

impl<T: SmolAPI> DoubleEndedIterator for Interpreter<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if !self.keep_history {
            panic!("We choosed to not keep history here.");
        }
        let checkpoint = self.history.pop()?;
        self.restore(checkpoint);
        self.step -= 1;
        Some(())
    }
}

impl<T: SmolAPI> Iterator for Interpreter<'_, T> {
    type Item = ();

    // todo break this function into multiple function
    fn next(&mut self) -> Option<()> {
        let checkpoint = self.checkpoint();
        self.history.push(checkpoint);
        self.step += 1;
        use crate::ast::ExprNode::*;
        match self.ast.exprs[self.program_counter] {
            Decl { .. } => {
                self.fun_decl.push(self.program_counter);
                self.values_stack.push(Value::Constant(Constant::Unit));
                self.ret()
            }
            Var => {
                // TODO change this if
                let name = self.ast.get_slice(self.program_counter);
                self.values_stack.push(self.memory.get_value(name));
                self.ret()
            }
            Seq { fst, snd } => {
                if self.prev_program_counter == fst {
                    self.call(snd);
                    Some(())
                } else if self.prev_program_counter == snd {
                    let snd_value = self.values_stack.pop().unwrap();
                    let _fst_value = self.values_stack.pop().unwrap();

                    self.values_stack.push(snd_value);
                    if let Let { .. } = self.ast.exprs[fst] {
                        self.memory.pop_var();
                    } else if let Decl { .. } = self.ast.exprs[fst] {
                        self.fun_decl.pop();
                    }
                    self.ret()
                } else {
                    self.call(fst);
                    Some(())
                }
            }
            Array { head, tail } => {
                if self.prev_program_counter == head {
                    self.call(tail);
                    Some(())
                } else if self.prev_program_counter == tail {
                    let tail_value = self.values_stack.pop().unwrap();
                    let head_value = self.values_stack.pop().unwrap();

                    if let Value::Array(mut arr) = tail_value {
                        arr.push_front(head_value);
                        self.values_stack.push(Value::Array(arr));
                    } else {
                        self.values_stack
                            .push(Value::Array(VecDeque::from([head_value, tail_value])));
                    }
                    self.ret()
                } else {
                    self.call(head);
                    Some(())
                }
            }
            Tuple { head, tail } => {
                if self.prev_program_counter == head {
                    self.call(tail);
                    Some(())
                } else if self.prev_program_counter == tail {
                    let tail_value = self.values_stack.pop().unwrap();
                    let head_value = self.values_stack.pop().unwrap();

                    if let Value::Array(mut arr) = tail_value {
                        arr.push_front(head_value);
                        self.values_stack.push(Value::Array(arr));
                    } else {
                        self.values_stack
                            .push(Value::Array(VecDeque::from([head_value, tail_value])));
                    }
                    self.ret()
                } else {
                    self.call(head);
                    Some(())
                }
            }
            FieldAccess { body, field } => {
                assert_eq!(Var, self.ast.exprs[body]);
                assert_eq!(self.ast.get_slice(field), "text");
                let widget_name = self.ast.get_slice(body);
                let widget_handle = self.api.get_handle(widget_name);
                let Some(widget_text) = self.api.get_text(widget_handle) else {
                    panic!("This should be prevented by type checking.")
                };
                self.values_stack.push(Value::Text(widget_text.to_string()));
                self.ret()
            }
            Index { body, index } => {
                if Var == self.ast.exprs[index] {
                    if self.prev_program_counter == index {
                        let index_value = self.values_stack.pop().unwrap();
                        self.values_stack
                            .push(Value::Array(VecDeque::from([index_value])));
                        self.ret()
                    } else {
                        self.call(index);
                        Some(())
                    }
                } else {
                    if self.prev_program_counter == body {
                        self.call(index);
                        Some(())
                    } else if self.prev_program_counter == index {
                        let index_value = self.values_stack.pop().unwrap();
                        let body_values = self.values_stack.pop().unwrap();
                        let Value::Array(mut body_values) = body_values else {
                            panic!("This should be prevented by type checking.")
                        };
                        body_values.push_back(index_value);
                        self.values_stack.push(Value::Array(body_values));
                        self.ret()
                    } else {
                        self.call(body);
                        Some(())
                    }
                }
            }
            MethodCall {
                index,
                method,
                args,
            } => {
                if self.prev_program_counter == index {
                    self.call(index);
                    Some(())
                } else if self.prev_program_counter == args {
                    // TODO fix this
                    let var = ExprId(0);
                    match method {
                        Method::Push => {
                            self.memory.push_method(index, var);
                            self.values_stack.push(Value::Constant(Constant::Unit));
                        }
                        Method::Pop => {
                            self.memory.pop_method(index);
                        }
                        Method::Replace => {
                            todo!()
                            // self.memory.replace_method(index, var, todo!());
                        }
                        Method::Len => {
                            let len_value = self.memory.len_method(index);
                            self.values_stack
                                .push(Value::Constant(Constant::Int(len_value as u32)));
                        }
                    };
                    self.ret()
                } else {
                    self.call(args);
                    Some(())
                }
            }
            APICall { name, args } => {
                if self.prev_program_counter == args {
                    let name = self.ast.get_slice(name);
                    match name {
                        "get_fun" => {}
                        _ => {
                            panic!("")
                        }
                    }
                    self.ret()
                } else {
                    self.call(args);
                    Some(())
                }
            }
            FunCall { name, args } => {
                let fun_id = self.get_fun(name);
                let Decl { mut args, expr, .. } = self.ast.exprs[fun_id] else {
                    panic!("This should be prevented by type checking.")
                };
                if self.prev_program_counter == expr {
                    while let Tuple { head: _, tail, .. } = self.ast.exprs[args] {
                        self.memory.pop_var();
                        args = tail;
                    }
                    self.ret()
                } else if self.prev_program_counter == args {
                    let Value::Array(args_value) = self.values_stack.pop().unwrap() else {
                        panic!("This should be prevented by type checking.")
                    };
                    for value in args_value.into_iter() {
                        if let Tuple { head, tail, .. } = self.ast.exprs[args]
                            && let TypeInfo { expr, t: _, .. } = self.ast.exprs[head]
                            && Var == self.ast.exprs[expr]
                        {
                            let var = self.ast.get_slice(expr);
                            self.memory.bind_var(var, value);
                            args = tail;
                        } else {
                            panic!("This should be prevented by type checking.")
                        };
                    }
                    self.call(expr);
                    Some(())
                } else {
                    self.call(args);
                    Some(())
                }
            }
            Let { var, t, expr } => {
                if self.prev_program_counter == expr {
                    let expr_value = self.values_stack.pop().unwrap();
                    let var = self.ast.get_slice(var);
                    self.memory.bind_var(var, expr_value);
                    self.values_stack.push(Value::Constant(Constant::Unit));
                    self.ret()
                } else {
                    self.call(expr);
                    Some(())
                }
            }
            Assign { index, expr } => {
                if self.prev_program_counter == index {
                    self.call(expr);
                    Some(())
                } else if self.prev_program_counter == expr {
                    let expr_value = self.values_stack.pop().unwrap();
                    let index_values = self.values_stack.pop().unwrap();
                    let Value::Array(index_values) = index_values else {
                        panic!("This should be prevented by type checking.")
                    };
                    let var = self.get_var_from_index(index);
                    self.memory.replace_method(var, index_values, expr_value);
                    self.ret()
                } else {
                    self.call(index);
                    Some(())
                }
            }
            While { cond, body } => {
                if self.prev_program_counter == cond {
                    let cond_value = self.values_stack.pop().unwrap();
                    if let Value::Constant(Constant::Bool(true)) = cond_value {
                        self.call(body);
                        Some(())
                    } else if let Value::Constant(Constant::Bool(false)) = cond_value {
                        self.values_stack.push(Value::Constant(Constant::Unit));
                        self.ret()
                    } else {
                        panic!("Expecting a boolean but got '{}'.", cond_value)
                    }
                } else if self.prev_program_counter == body {
                    let _ = self.values_stack.pop().unwrap();
                    self.call(cond);
                    Some(())
                } else {
                    self.call(cond);
                    Some(())
                }
            }
            If { cond, yes, no } => {
                if self.prev_program_counter == cond {
                    let cond_value = self.values_stack.pop().unwrap();
                    if let Value::Constant(Constant::Bool(true)) = cond_value {
                        self.call(yes);
                        Some(())
                    } else if let Value::Constant(Constant::Bool(false)) = cond_value {
                        self.call(no);
                        Some(())
                    } else {
                        panic!("Expecting a boolean but got '{}'.", cond_value)
                    }
                } else if self.prev_program_counter == yes || self.prev_program_counter == no {
                    self.ret()
                } else {
                    self.call(cond);
                    Some(())
                }
            }
            Text => {
                let s = self.ast.get_slice(self.program_counter);
                self.values_stack.push(Value::Text(s.to_string()));
                self.ret()
            }
            Const(value) => {
                self.values_stack.push(Value::Constant(value));
                self.ret()
            }
            BinaryOp { lhs, op, rhs } => {
                if self.prev_program_counter == lhs {
                    self.call(rhs);
                    Some(())
                } else if self.prev_program_counter == rhs {
                    let rhs_value = self.values_stack.pop().unwrap();
                    let lhs_value = self.values_stack.pop().unwrap();
                    self.values_stack
                        .push(lhs_value.compute_binary(op, rhs_value));
                    self.ret()
                } else {
                    self.call(lhs);
                    Some(())
                }
            }
            UnaryOp { lhs, op } => {
                if self.prev_program_counter == lhs {
                    let lhs_value = self.values_stack.pop().unwrap();
                    self.values_stack.push(lhs_value.compute_unary(op));
                    self.ret()
                } else {
                    self.call(lhs);
                    Some(())
                }
            }
            TypeInfo { expr, .. } => {
                if self.prev_program_counter == expr {
                    self.ret()
                } else {
                    self.call(expr);
                    Some(())
                }
            }
            EmptyTuple => {
                self.values_stack.push(Value::Array(VecDeque::new()));
                self.ret()
            }
            EmptyVec => {
                self.values_stack.push(Value::Array(VecDeque::new()));
                self.ret()
            }
            Unknown => {
                panic!("This should be prevented by type checking.")
            }
        }
    }
}
