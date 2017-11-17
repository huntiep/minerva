use object::{Object, Primitive};

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

macro_rules! init_env {
    ($($key:expr),*) => {
        hashmap!{$(
            $key.0.to_string() =>
                Object::cons(Object::Symbol("primitive".to_string()),
                             Object::cons(
                                 Object::Primitive(Primitive::new($key.0.to_string(), $key.1)),
                                 Object::Nil)),
        )*}
    };
}

pub fn init_env() -> Environment {
    let bindings = init_env!{
        ("cons", Some(2)),
        ("car", Some(1)),
        ("cdr", Some(1)),
        ("=", None::<usize>),
        ("+", None::<usize>),
        ("-", None::<usize>),
        ("*", None::<usize>),
        ("/", None::<usize>)
    };
    Environment::from_hashmap(bindings)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Environment {
    env: Rc<RefCell<_Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            env: Rc::new(RefCell::new(_Environment::new())),
        }
    }

    pub fn from_hashmap(map: HashMap<String, Object>) -> Self {
        let env = _Environment {
            bindings: map,
            parent: None,
        };

        Environment {
            env: Rc::new(RefCell::new(env)),
        }
    }

    pub fn extend(&self) -> Self {
        let mut env = _Environment::new();
        env.parent = Some(self.clone());
        Environment {
            env: Rc::new(RefCell::new(env)),
        }
    }

    pub fn lookup_variable_value(&self, name: &str) -> Option<Object> {
        let env = self.env.borrow();
        env.lookup_variable_value(name)
    }

    pub fn define_variable(&self, name: String, value: Object) {
        let mut env = self.env.borrow_mut();
        env.define_variable(name, value);
    }

    pub fn procedure_local(&self) -> Self {
        let env = self.env.borrow();
        let local = _Environment {
            bindings: env.bindings.clone(),
            parent: env.parent.clone(),
        };
        Environment {
            env: Rc::new(RefCell::new(local)),
        }
    }
}

#[derive(Debug, Default)]
pub struct _Environment {
    bindings: HashMap<String, Object>,
    parent: Option<Environment>,
}

impl PartialEq for _Environment {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl _Environment {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn lookup_variable_value(&self, name: &str) -> Option<Object> {
        if let Some(val) = self.bindings.get(name) {
            Some(val.clone())
        } else if let Some(ref env) = self.parent {
            env.lookup_variable_value(name)
        } else {
            None
        }
    }

    pub fn define_variable(&mut self, name: String, value: Object) {
        self.bindings.insert(name, value);
    }

    /*
    pub fn set_variable_value(&mut self, name: String, value: Object) -> Option<Object> {
        self.bindings.insert(name, value)
    }
    */
}