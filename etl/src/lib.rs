use std::collections::BTreeMap;

pub fn transform(inp: &BTreeMap<i32, Vec<String>>) -> BTreeMap<String, i32> {
    let mut out = BTreeMap::new();
    for (i, v) in inp {
        for s in v {
            out.insert(s.to_lowercase(), *i);
        }
    }
    out
}
