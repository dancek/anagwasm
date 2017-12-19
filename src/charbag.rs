use CharMap;
use filter_alphabetic;
use fixedbitset::FixedBitSet;
use std::ops::Sub;

pub type CharIdx = u8;
pub const MAX_CHARIDX: u8 = ::std::u8::MAX;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CharBag {
    svec: Vec<CharIdx>,
    bitset: FixedBitSet,
}

impl CharBag {
    pub fn new(svec: Vec<CharIdx>) -> CharBag {
        let mut bitset = FixedBitSet::with_capacity(MAX_CHARIDX as usize + 1);
        for c in svec.iter() {
            bitset.insert(*c as usize);
        }
        CharBag { svec, bitset }
    }

    pub fn empty(&self) -> bool {
        self.svec.is_empty()
    }

    pub fn from_str(input: &str, charmap: &CharMap) -> Option<CharBag> {
        let lowercased = input.to_lowercase();
        let mut v = Vec::new();
        for c in filter_alphabetic(&lowercased[..]) {
            if let Some(i) = charmap.get(&c) {
                v.push(*i);
            } else {
                return None;
            }
        }
        v.sort();
        Some(CharBag::new(v))
    }
}

impl<'a> Sub for &'a CharBag {
    type Output = Option<CharBag>;
    fn sub(self, rhs: Self) -> Self::Output {
        let bits = self.bitset.as_slice()[0];
        let rhs_bits = rhs.bitset.as_slice()[0];
        if (rhs_bits & !bits) != 0 {
            return None;
        }
        if rhs.svec.len() > self.svec.len() {
            return None;
        }
        if rhs.svec.len() == self.svec.len() {
            return if rhs.svec == self.svec {
                Some(CharBag::new(Vec::new()))
            } else {
                None
            };
        }

        let mut new_vec = Vec::new();
        let mut lhs_it = self.svec.iter();
        let mut rhs_it = rhs.svec.iter();
        let mut l = lhs_it.next();
        let mut r = rhs_it.next();

        while l.is_some() && r.is_some() {
            let lv = l.unwrap();
            let rv = r.unwrap();
            if lv < rv {
                new_vec.push(*lv);
                l = lhs_it.next();
            } else if lv == rv {
                l = lhs_it.next();
                r = rhs_it.next();
            } else {
                return None;
            }
        }
        if l.is_some() {
            new_vec.push(*l.unwrap());
        } else if l.is_none() && r.is_some() {
            return None;
        }

        for val in lhs_it {
            new_vec.push(*val);
        }

        assert!(rhs_it.next().is_none());
        assert_eq!(new_vec.len(), self.svec.len() - rhs.svec.len());

        Some(CharBag::new(new_vec))
    }
}
