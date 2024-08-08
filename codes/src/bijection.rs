use bimap::BiMap;

pub struct Bijection {
    pub map: BiMap<String, String>,
    pub err_group: String,
}

impl Default for Bijection {
    fn default() -> Self {
        Self {
            map: Default::default(),
            err_group: String::from("�"),
        }
    }
}

impl Bijection {
    pub fn encode_or_err_group(&self, s: &str) -> &String {
        self.map.get_by_left(s).unwrap_or(&self.err_group)
    }

    pub fn decode_or_err_group(&self, s: &str) -> &String {
        self.map.get_by_right(s).unwrap_or(&self.err_group)
    }

    pub fn maybe_encode(&self, s: &str) -> Option<&String> {
        self.map.get_by_left(s)
    }

    pub fn maybe_decode(&self, s: &str) -> Option<&String> {
        self.map.get_by_right(s)
    }
}
