use super::*;
use super::case::*;

#[repr(C)]
pub enum CaseType {
    CAMEL,
    SNAKE,
    KEBAB
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

impl<'a> DefiningCase<'a, CaseTypeIterator<'a>> for CaseType {
    fn components_iter(self, src: &'a str) -> CaseTypeIterator<'a> {
        match self {
            CaseType::CAMEL => CaseTypeIterator::Camel(CAMEL.components_iter(src)),
            CaseType::SNAKE => CaseTypeIterator::Delimeter(SNAKE.components_iter(src)),
            CaseType::KEBAB => CaseTypeIterator::Delimeter(KEBAB.components_iter(src)),
        }
    }
}

impl Case for CaseType {
    fn build_identifier<'a, It: Iterator<Item=&'a str>>(self, components: It) -> String {
        match self {
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
        let id = convert("simpleCamelCase", CaseType::CAMEL, CaseType::KEBAB);
        assert_eq!(id, "simple-camel-case");
    }
}
