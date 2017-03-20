use std::default::Default;
use super::*;
use super::case::*;

#[repr(C)]
#[derive(Clone)]
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

impl CaseType {
    pub fn guess(s: &str) -> CaseType {
        let maybe_type = ALL_DEFINING_CASE_TYPES.iter()
            .map(|ty| (ty.clone(), ty.components_iter(s).count()))
            .max_by_key(|&(_, count)| count);
        match maybe_type {
            Some((ref ty, _)) => ty.clone(),
            None => Default::default(),
        }
    }
}

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
}
