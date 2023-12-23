use hexagon::{
    iota::{hex_casting::null::NullIota, Iota},
    parser::{AstNode, OpName},
};
use im::vector;
use rhai::{Dynamic, Position, Shared};
use std::rc::Rc;

pub fn translate_dynamic_to_iota(val: Box<Dynamic>, position: Position) -> Rc<dyn Iota> {
    if val.is_array() {
        let mut translated_array = vector![];
        let array = val.into_array().unwrap();
        for var in array {
            translated_array.push_back(translate_dynamic_to_iota(Box::new(var), position));
        }

        //AstNode::Op { location: position_to_location(position), name: OpName::Push, arg: Some(hexagon::parser::OpValue::Iota(())) };

        Rc::new(translated_array)
    } else if val.is_bool() {
        let bool = val.as_bool().unwrap();
        Rc::new(bool)
    } else if val.is_char() {
        let char = val.as_char().unwrap();
        Rc::new(char.to_string())
    } else if val.is_string() {
        let string = val.into_string().unwrap();
        Rc::new(string)
    } else if val.is_int() {
        let int = val.as_int().unwrap();
        Rc::new(int as f64)
    } else if val.is_float() {
        let float = val.as_float().unwrap();
        Rc::new(float)
    } else if val.is_unit() {
        Rc::new(NullIota)
    } else {
        panic!()
    }
}
