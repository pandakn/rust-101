use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString {
    data: HashMap<String, String>,
}

impl QueryString {
    pub fn get(&self, key: &String) -> Option<&String> {
        self.data.get(key)
    }
}

impl From<&str> for QueryString {
    fn from(s: &str) -> Self {
        let mut data = HashMap::new();

        // query string -> a=1&b=2&
        for pair in s.split("&") {
            if let Some(i) = pair.find("=") {
                let key = &pair[..i];
                let value = &pair[i + 1..];
                data.insert(key.to_string(), value.to_string());
            }
        }

        QueryString { data }
    }
}
