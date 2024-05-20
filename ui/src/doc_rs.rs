use std::collections::BTreeMap;

#[macro_export]
macro_rules! doc {
    () => {
        $crate::Doc::Val("".to_string())
    };
    ( $val:literal ) => {
        $crate::Doc::Val($val.to_string())
    };
    ( $val:ident ) => {
        $crate::Doc::Val($val.to_string())
    };
    ( {$($tt:tt)+} ) => {
        $crate::Doc::Map({
            let mut map = std::collections::BTreeMap::new();
            doc!(@map map $($tt)+);
            map
        })
    };
    (@map $val:literal) => {
        $crate::Doc::Val($val.into())
    };
    (@map $val:ident) => {
        $crate::Doc::Val($val.into())
    };
    (@map $map:ident $key:literal : $val:tt) => {
        let _ = $map.insert($key.into(), doc!($val));
    };
    (@map $map:ident $key:literal : $val:tt ,) => {
        let _ = $map.insert($key.into(), doc!($val));
    };
    (@map $map:ident $key:literal : $val:tt , $($rest:tt)+) => {
        let _ = $map.insert($key.into(), doc!($val));
        doc!(@map $map $($rest)+)
    };
    (@map $map:ident $key:literal : $val:expr) => {
        let _ = $map.insert($key.into(), $crate::Doc::Val($val.to_string()));
    };
    (@map $map:ident $key:literal : $val:expr ,) => {
        let _ = $map.insert($key.into(), $crate::Doc::Val($val.to_string()));
    };
    (@map $map:ident $key:literal : $val:expr , $($rest:tt)+) => {
        let _ = $map.insert($key.into(), $crate::Doc::Val($val.to_string()));
        doc!(@map $map $($rest)+)
    };
    (@map ()) => {}
}

#[derive(Debug, PartialEq, Clone)]
pub enum Doc {
    Val(String),
    Map(BTreeMap<String, Doc>),
}

impl Doc {

    pub fn get<'a>(&'a self, path: &str) -> Option<&'a Self> {
        if path.len() == 0 {
            return Some(self);
        }
        match self {
            Self::Val(_) => None,
            Self::Map(m) => match path.split_once(".") {
                Some((lead, rest)) => match m.get(lead) {
                    None => None,
                    Some(doc) => doc.get(rest),
                },
                None => match m.get(path) {
                    None => None,
                    Some(doc) => Some(doc),
                },
            },
        }
    }

    // pub fn parse<'a>(s: &'a str) -> Result<Self, ParseError<usize, Token<'a>, &'a str>> {
    //     let parser = DocParser::new();
    //     parser.parse(s)
    // }

    pub fn emit(&self) -> String {
        match self {
            Self::Val(val) => format!("\"{}\"", val),
            Self::Map(map) => {
                let mut s = String::from("{");
                for (k, v) in map {
                    s.push_str(format!("\"{}\":{},", k, v.emit()).as_str());
                }
                s.push('}');
                s
            }
        }
    }

    pub fn emit_pretty(&self) -> String {
        match self {
            Self::Val(val) => format!("\"{}\"", val),
            Self::Map(map) => {
                let mut s = String::from("{\n");
                for (k, v) in map {
                    s.push_str(format!("  \"{}\": {},\n", k, v.emit()).as_str());
                }
                s.push('}');
                s
            }
        }
    }
}
