use std::fmt;
use std::cell::RefCell;

use fnv::FnvHashMap;
use internal_macro;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Symbol(u32);

impl fmt::Debug for Symbol {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}{}", self, self.0)
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.as_str(), fmt)
    }
}

impl Symbol {
    pub fn as_str<'a>(self) -> &'a str {
        resolve(self)
    }
}


#[derive(Default)]
pub struct Interner {
    names: FnvHashMap<Box<str>, Symbol>,
    strings: Vec<Box<str>>,
}

impl Interner {
    fn prefill(init: &[&str]) -> Self {
        let mut this = Interner::default();
        for &string in init {
            this.intern(string);
        }
        this
    }

    pub fn intern(&mut self, string: &str) -> Symbol {
        if let Some(&name) = self.names.get(string) {
            return name;
        }

        let name = Symbol(self.strings.len() as u32);
        let string = string.to_string().into_boxed_str();
        self.strings.push(string.clone());
        self.names.insert(string, name);
        name
    }

    pub fn get(&self, symbol: Symbol) -> &str {
        unsafe { self.strings.get_unchecked(symbol.0 as usize) }
    }
}

internal_macro::declare_keywords! {
    (0,      NIL_KW,           "nil")
        (1,  IF_KEYWORD,       "if")
        (2,  ELSE_KEYWORD,     "else")
        (3,  FUNCTION_KEYWORD, "fn")
        (4,  TRUE_KEYWORD,     "true")
        (5,  FALSE_KEYWORD,    "false")
        (6,  VARIABLE_KEYWORD, "var")
        (7,  ENUM_RESERVED,    "enum")
        (8,  USE_RESERVED,     "use")
        (9,  PUB_RESERVED,     "pub")
        (10, STRUCT_RESERVED,  "struct")
        (11, CONST_RESERVED,   "const")
}

fn with_interner<T, F: FnOnce(&mut Interner) -> (f: F) -> T {
    // Declares a new thread local storage key of type std::thread::LocalKey.
    // LocalKey is a thread local storage key which owns its own contents.
    thread_local!(static INTERNER: RefCell<Interner> = {
        RefCell::new(Interner::fresh())
    });
    INTERNER.with(|interner| f(&mut *interner.borrow_mut()))
}

pub fn intern(val: &str) -> Symbol {
    with_interner(|interner| interner.intern(val))
}

pub fn resolve<'a>(key: Symbol) -> &'a str {
    with_interner(|interner| {
        unsafe { ::std::mem::transmute::<&str, &str>(interner.get(key)) }
    })
}
