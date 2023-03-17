use std::collections::HashMap; 
use std::any::Any;


#[macro_export]
macro_rules! val {
    ($value: expr, $type: ty) => {
        if let Some(val) = $value.downcast_ref::<$type>() {
            val
        } else {
            panic!("invalid type")
        }
    };
}


pub struct Dict { 
    pub data: HashMap<String, Box<dyn Any>>, 
}


impl Dict { 
    pub fn new() -> Self { 
        Self { 
            data: HashMap::new(), 
        } 
    } 
    pub fn set<T>(&mut self, key:&str, value:T) where T:Any { 
        self.data.insert(key.to_string(), Box::new(value)); 
    } 
    pub fn get(&self, key:&str) -> Option<&dyn Any> {
        self.data.get(&key.to_string()).map(|v| v.as_ref())
    }
    pub fn keys(&self) -> Vec<String> { 
        self.data.keys().cloned().collect() 
    } 
}

