extern crate libc;

pub trait DefiningCase<'a, T: Iterator<Item=&'a str>> {
    fn components_iter(self, &'a str) -> T;
}

pub trait Case {
    fn build_identifier<'a, It: Iterator<Item=&'a str>>(self, components: It) -> String;
}

pub fn convert<'a, It: Iterator<Item=&'a str>, A: DefiningCase<'a, It>, B: Case>(src: &'a str, src_case: A, dst_case: B) -> String {
    dst_case.build_identifier(src_case.components_iter(src))
}

pub mod dynamic;

pub mod case {
    use super::{ Case, DefiningCase };

    pub trait DelimetedCase {
        fn delimeter() -> char;
    }

    pub struct Camel {}

    pub const CAMEL: Camel = Camel {};

    pub struct CamelIterator<'a> {
        src: &'a str
    }

    impl<'a> CamelIterator<'a> {
        fn new(src: &'a str) -> CamelIterator<'a> {
            CamelIterator {
                src: src
            }
        }
    }

    impl<'a> Iterator for CamelIterator<'a> {
        type Item = &'a str;

        fn next(&mut self) -> Option<&'a str> {
            match self.src.char_indices().skip(1).find(|&(_, c)| c.is_uppercase()) {
                Some((idx, _)) => {
                    let (component, new_src) = self.src.split_at(idx);
                    self.src = new_src;
                    Some(component)
                }
                None => if self.src.is_empty() {
                    None
                } else {
                    let ret = self.src;
                    self.src = "";
                    Some(ret)
                }
            }
        }
    }

    impl<'a> DefiningCase<'a, CamelIterator<'a>> for Camel {
        fn components_iter(self, src: &'a str) -> CamelIterator<'a> {
            CamelIterator::new(src)
        }
    }

    impl Case for Camel {
        fn build_identifier<'a, It: Iterator<Item=&'a str>>(self, mut components: It) -> String {
            components.next().map_or(Default::default(), |first_component| {
                let mut buf = first_component.to_lowercase();
                for comp in components {
                    let (first_letter, remainder) = comp.split_at(1);
                    let beginning = first_letter.to_uppercase();
                    buf.push_str(beginning.as_ref());
                    buf.push_str(remainder);
                }
                buf
            })
        }
    }

    pub struct Snake {}

    pub const SNAKE: Snake = Snake {};

    pub struct DelimeterIterator<'a> {
        delimeter: char,
        src: &'a str
    }

    impl<'a> DelimeterIterator<'a> {
        fn new(src: &'a str, delimeter: char) -> DelimeterIterator<'a> {
            DelimeterIterator {
                delimeter: delimeter,
                src: src
            }
        }
    }

    impl<'a> Iterator for DelimeterIterator<'a> {
        type Item = &'a str;

        fn next(&mut self) -> Option<&'a str> {
            if self.src.is_empty() {
                None
            } else {
                match self.src.char_indices().find(|&(_, c)| c.eq(&self.delimeter)) {
                    Some((idx, _)) => {
                        let (component, new_src_prefixed) = self.src.split_at(idx);
                        let (_, new_src) = new_src_prefixed.split_at(1);
                        self.src = new_src;
                        Some(component)
                    }
                    None => {
                        let ret = self.src;
                        self.src = "";
                        Some(ret)
                    }
                }
            }
        }
    }

    impl DelimetedCase for Snake {
        fn delimeter() -> char {
            '_'
        }
    }

    pub struct Kebab {}

    pub const KEBAB: Kebab = Kebab {};

    impl DelimetedCase for Kebab {
        fn delimeter() -> char {
            '-'
        }
    }

    impl<'a, T: DelimetedCase> DefiningCase<'a, DelimeterIterator<'a>> for T {
        fn components_iter(self, src: &'a str) -> DelimeterIterator<'a> {
            DelimeterIterator::new(src, T::delimeter())
        }
    }

    impl<T: DelimetedCase> Case for T {
        fn build_identifier<'a, It: Iterator<Item=&'a str>>(self, components: It) -> String {
            let components = components.map(|c| c.to_lowercase());
            let mut is_first = false;
            let mut buf = String::from("");
            for c in components {
                if is_first {
                    buf.push(T::delimeter());
                } else {
                    is_first = true;
                }
                buf.push_str(c.as_ref());
            }
            buf
        }
    }
}

pub mod ffi;

#[cfg(test)]
mod tests {
    use super::*;
    use super::case::*;

    const CAMEL_CASE_TEST_VAL: &'static str = "simpleCamelCase";
    const SNAKE_CASE_TEST_VAL: &'static str = "simple_snake_case";
    const KEBAB_CASE_TEST_VAL: &'static str = "simple-kebab-case";

    #[test]
    fn simple_camel_case_counted_correctly() {
        let it = CAMEL.components_iter(CAMEL_CASE_TEST_VAL);
        assert_eq!(it.count(), 3);
    }

    #[test]
    fn simple_camel_case_defined_correctly() {
        let it = CAMEL.components_iter(CAMEL_CASE_TEST_VAL);
        let components: Vec<&'static str> = it.collect();
        assert_eq!(components, vec!["simple", "Camel", "Case"]);
    }

    #[test]
    fn simple_camel_case_converts_self() {
        let id = convert(CAMEL_CASE_TEST_VAL, CAMEL, CAMEL);
        assert_eq!(id, "simpleCamelCase");
    }

    #[test]
    fn simple_snake_case_converts_camel() {
        let id = convert(SNAKE_CASE_TEST_VAL, SNAKE, CAMEL);
        assert_eq!(id, "simpleSnakeCase");
    }

    #[test]
    fn simple_kebab_case_converts_camel() {
        let id = convert(KEBAB_CASE_TEST_VAL, KEBAB, CAMEL);
        assert_eq!(id, "simpleKebabCase");
    }

    #[test]
    fn simple_camel_case_converts_kebab() {
        let id = convert(CAMEL_CASE_TEST_VAL, CAMEL, KEBAB);
        assert_eq!(id, "simple-camel-case");
    }

    #[test]
    fn simple_snake_case_converts_kebab() {
        let id = convert(SNAKE_CASE_TEST_VAL, SNAKE, KEBAB);
        assert_eq!(id, "simple-snake-case");
    }

    #[test]
    fn simple_kebab_case_converts_snake() {
        let id = convert(KEBAB_CASE_TEST_VAL, KEBAB, SNAKE);
        assert_eq!(id, "simple_kebab_case");
    }

    #[test]
    fn simple_camel_case_converts_snake() {
        let id = convert(CAMEL_CASE_TEST_VAL, CAMEL, SNAKE);
        assert_eq!(id, "simple_camel_case");
    }

    #[test]
    fn simple_snake_case_defined_correctly() {
        let it = SNAKE.components_iter(SNAKE_CASE_TEST_VAL);
        let components: Vec<&'static str> = it.collect();
        assert_eq!(components, vec!["simple", "snake", "case"]);
    }

    #[test]
    fn simple_kebab_case_defined_correctly() {
        let it = KEBAB.components_iter(KEBAB_CASE_TEST_VAL);
        let components: Vec<&'static str> = it.collect();
        assert_eq!(components, vec!["simple", "kebab", "case"]);
    }
}
