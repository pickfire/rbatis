use std::borrow::BorrowMut;

use serde_json::{json, Value};

use crate::ast::ast::Ast;

use crate::ast::node::node::{create_deep, do_child_nodes, print_child, SqlNodePrint};
use crate::ast::node::node_type::NodeType;
use crate::engine::runtime::RbatisEngine;
use crate::error::RbatisError;

#[derive(Clone,Debug)]
pub struct WhenNode {
    pub childs: Vec<NodeType>,
    pub test: String,

}


impl Ast for WhenNode {
    fn eval(&self, env: &mut Value, engine: &mut RbatisEngine,arg_array:&mut Vec<Value>) -> Result<String, RbatisError> {
        let result_value = engine.eval(self.test.as_str(), env);
        if result_value.is_err() {
            return Result::Err(RbatisError::from(result_value.err().unwrap()));
        }
        let result = result_value.unwrap();
        if !result.is_boolean() {
            return Result::Err(RbatisError::from("[rbatis] test:'".to_owned() + self.test.as_str() + "' is not return bool!"));
        }
        if result.as_bool().unwrap() {
            return do_child_nodes(&self.childs, env,engine,arg_array);
        }
        return Result::Ok("".to_string());
    }
}

impl SqlNodePrint for WhenNode {
    fn print(&self, deep: i32) -> String {
        let mut result = create_deep(deep) + "<when ";
        result = result + " test=\"" + self.test.as_str() + "\" >";
        result = result + print_child(self.childs.as_ref(), deep + 1).as_str();
        result = result + create_deep(deep).as_str() + "</when>";
        return result;
    }
}