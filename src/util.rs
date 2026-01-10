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
