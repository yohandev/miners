use std::{any::{ Any, TypeId }, collections::HashMap};


pub trait Block: Any
{
    fn name(&self) -> String;
}

pub struct BlockMeta
{
    destructor: fn(*mut ()),
    size: usize,
    align: usize,
    type_id: TypeId,

    
}