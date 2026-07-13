use std::{fmt::{Display, Formatter}, ops::Index};

use crate::{Address, Region, value::Value};

pub struct Heap {
    pub data: Vec<Region>,
}

impl Heap {
    pub const EMPTY: Self = Self { data: vec![] };

    pub fn alloc(&mut self) -> Address {
        self.data.push(Region::default());
        self.data.len() - 1
    }

    pub fn remove(&mut self, address: Address) -> Region {
        self.data.remove(address)
    }

    pub fn drop(&mut self, value: Value) {
        match value {
            Value::Address(address) => {
                let region = self.remove(address);
                for value in region {
                    self.drop(value)
                }
            }
            Value::Constant(_) => (),
        }
    }

    pub fn push_method(&mut self, address: Address, value: Value) {
        self.data[address].push(value)
    }

    pub fn copy_value(&mut self, value: &Value) -> Value {
        match value {
            Value::Address(address) => {
                let new_addr = self.alloc();
                let mut future_region = Region::default();

                let region = std::mem::take(&mut self.data[*address]);
                for val in region.iter() {
                    future_region.push(self.copy_value(val))
                }
                self.data[new_addr] = future_region;
                Value::Address(new_addr)
            }
            Value::Constant(constant) => Value::Constant(constant.clone()),
        }
    }

    pub(crate) fn pop_method(&mut self, address: Address) -> Value {
        let region = &mut self.data[address];
        region
            .pop()
            .expect("Pop on a region of size 0. This is a runtime error typecheking cannot avoid.")
    }
}

impl Display for Heap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, region) in self.data.iter().enumerate() {
            write!(f, "{i} :")?;

            let mut iter = region.iter();
            if let Some(val) = iter.next() {
                write!(f, "{val}")?;
                for val in iter {
                    write!(f, " | {val}")?;
                }
            }
        }
        Ok(())
    }
}



impl Index<&Address> for Heap {
    type Output = Region;

    fn index(&self, index: &Address) -> &Self::Output {
        &self.data[*index]
    }
}
