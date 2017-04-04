use std::default::Default;
use super::*;
use super::case::*;

#[repr(C)]
#[derive(Clone)]
/// Runtime data type representing a case
pub enum CaseType {
    CAMEL,
    SNAKE,
    KEBAB
}

impl Default for CaseType {
    fn default() -> CaseType {
        CaseType::CAMEL
    }
}

const ALL_DEFINING_CASE_TYPES: [CaseType; 3] = [
    CaseType::CAMEL,
    CaseType::SNAKE,
    CaseType::KEBAB
];

pub const JUMBLED: &'static [CaseType] = &ALL_DEFINING_CASE_TYPES;

struct CombinedCaseIterator<'s, C: AsRef<[CaseType]>> {
    cases: C,
    buffer: &'s str
}

impl<'s, C: AsRef<[CaseType]>> Iterator for CombinedCaseIterator<'s, C> {
    type Item = &'s str;

    fn next(&mut self) -> Option<&'s str> {
        let maybe_next = {
            let cases: &[CaseType] = self.cases.as_ref();
            cases.iter()
                .filter_map(|case_type| case_type.components_iter(self.buffer).next().map(|s| (case_type.clone(), s)))
                .min_by_key(|&(_, ref item)| item.len())
        };
        maybe_next.map(|(case_type, item)| unsafe {
            self.buffer = case_type.slice_buffer(self.buffer, item);
            item
        })
    }
}



impl CaseType {
    /// Attempts to guess the case of a given string by counting the # of
    /// components parsed for each case, and returning the one that finds the
    /// most.
    pub fn guess(s: &str) -> CaseType {
        let maybe_type = ALL_DEFINING_CASE_TYPES.iter()
            .map(|ty| (ty.clone(), ty.components_iter(s).count()))
            .max_by_key(|&(_, count)| count);
        match maybe_type {
            Some((ref ty, _)) => ty.clone(),
            None => Default::default(),
        }
    }

    unsafe fn slice_buffer<'a>(&self, buf: &'a str, item: &str) -> &'a str {
        let buffer_skip_length = item.len() + match *self {
            CaseType::CAMEL => 0,
            _ => 1
        };
        buf.slice_unchecked(buffer_skip_length, buf.len())
    }
}

/// Runtime iterator for components of a string who's case is only known at
/// runtime.
pub enum CaseTypeIterator<'a> {
    Camel(CamelIterator<'a>),
    Delimeter(DelimeterIterator<'a>),
}

impl<'a> Iterator for CaseTypeIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        match self {
            &mut CaseTypeIterator::Camel(ref mut it) => it.next(),
            &mut CaseTypeIterator::Delimeter(ref mut it) => it.next(),
        }
    }
}

impl<'a, 'b> DefiningCase<'a, CaseTypeIterator<'a>> for &'b CaseType {
    fn components_iter(self, src: &'a str) -> CaseTypeIterator<'a> {
        match *self {
            CaseType::CAMEL => CaseTypeIterator::Camel(CAMEL.components_iter(src)),
            CaseType::SNAKE => CaseTypeIterator::Delimeter(SNAKE.components_iter(src)),
            CaseType::KEBAB => CaseTypeIterator::Delimeter(KEBAB.components_iter(src)),
        }
    }
}

impl<'s, C: AsRef<[CaseType]>> DefiningCase<'s, CombinedCaseIterator<'s, C>> for C {
    fn components_iter(self, src: &'s str) -> CombinedCaseIterator<'s, C> {
        CombinedCaseIterator {
            cases: self,
            buffer: src
        }
    }
}

impl<'b> Case for &'b CaseType {
    fn build_identifier<'a, It: Iterator<Item=&'a str>>(self, components: It) -> String {
        match *self {
            CaseType::CAMEL => CAMEL.build_identifier(components),
            CaseType::SNAKE => SNAKE.build_identifier(components),
            CaseType::KEBAB => KEBAB.build_identifier(components),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dynamic_camel_to_kebab() {
        let id = convert("simpleCamelCase", &CaseType::CAMEL, &CaseType::KEBAB);
        assert_eq!(id, "simple-camel-case");
    }

    #[test]
    fn dynamic_jumbled_to_kebab() {
        let id = convert("simple_jumbledCase", JUMBLED, &CaseType::KEBAB);
        assert_eq!(id, "simple-jumbled-case");
    }
}
