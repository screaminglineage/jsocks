use indexmap::IndexMap;

#[derive(Debug)]
pub enum JsonBool {
    True,
    False,
}

#[derive(Debug)]
pub enum JsonValue {
    Object(IndexMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    String(String),
    Number(f64),
    JsonBool(JsonBool),
    Null,
}

impl JsonValue {
    pub fn dump(&self) -> String {
        match self {
            Self::Null => format!("null"),
            Self::JsonBool(JsonBool::True) => format!("true"),
            Self::JsonBool(JsonBool::False) => format!("false"),
            Self::String(str) => format!("\"{str}\""),
            Self::Number(x) => format!("{x}"),
            Self::Array(v) => {
                let mut str = String::new();
                str += "[";
                for (_, i) in v.iter().enumerate().take_while(|(k, _)| k + 1 < v.len()) {
                    str += &format!("{}, ", i.dump());
                }
                let last = match v.last() {
                    Some(n) => n.dump(),
                    None => String::new(),
                };
                str += &format!("{}]", last);
                str
            }
            Self::Object(obj) => {
                let mut str = String::new();
                str += "{";
                for (_, (s, val)) in obj
                    .iter()
                    .enumerate()
                    .take_while(|(k, _)| k + 1 < obj.len())
                {
                    str += &format!("\"{s}\" : {}, ", val.dump());
                }
                let last = match obj.last() {
                    Some((s, val)) => format!("\"{s}\" : {}", val.dump()),
                    None => String::new(),
                };
                str += &format!("{}}}", last);
                str
            }
        }
    }
}
