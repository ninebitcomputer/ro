use std::fmt;

pub struct BoolStrMap<'a> {
    pub bits: &'a [bool],
    pub t: &'a str,
    pub f: &'a str,
}

impl<'a> BoolStrMap<'a> {
    pub fn new(bits: &'a [bool], t: &'a str, f: &'a str) -> Self {
        Self { bits, t, f }
    }
}

impl fmt::Display for BoolStrMap<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &b in self.bits {
            f.write_str(if b { self.t } else { self.f })?;
        }
        Ok(())
    }
}

// pretty print tree
pub trait TPrint {
    fn label(&self) -> String;
    fn children<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn TPrint> + 'a>;

    fn tprint(&self) {
        let mut v: Vec<bool> = Vec::new();
        self._tprint(&mut v, true);
    }

    fn _tprint(&self, stack: &mut Vec<bool>, last: bool) {
        stack.push(last);

        let (end, pfx) = stack.split_last().unwrap();
        let s = if *end { "└─" } else { "├─" };

        println!("{}{}{}", BoolStrMap::new(pfx, "  ", "│ "), s, self.label());

        let kids: Vec<&dyn TPrint> = self.children().collect();
        for (i, k) in kids.iter().enumerate() {
            let last = i + 1 == kids.len();
            k._tprint(stack, last);
        }

        stack.pop();
    }
}
