use std::{
    collections::VecDeque,
    ops::{Add, Mul, Rem},
    str::FromStr,
};

use crate::parse::parse_inspection;

#[derive(Debug)]
pub struct Monke {
    pub items: VecDeque<u64>,
    divisible_by: u64,
    inspection_rules: MonkeInspection<u64>,
    if_true_throw_to_monkey: usize,
    if_false_throw_to_monkey: usize,

    number_of_inspections: u64,
}

impl Monke {
    pub fn inspect(&mut self, old_val: &u64, magic_trick: u64) -> (usize, u64) {
        self.number_of_inspections += 1;

        use DivisionElement::*;

        let val1 = match &self.inspection_rules.element1 {
            Old => old_val.clone(),
            Val(v) => v.clone(),
        };
        let val2 = match &self.inspection_rules.element2 {
            Old => old_val.clone(),
            Val(v) => v.clone(),
        };

       let worry_level = match self.inspection_rules.op_type {
            OpType::Add => val1 + val2,
            OpType::Mul => {
                let result = val1 * val2;
                result % magic_trick
            },
        };

        // let worry_level = self.inspection_rules.inspect(old_val, magic_trick);
        // let worry_level_after_bored = worry_level / 3;
        let worry_level_after_bored = worry_level / 1;
        let is_divisible = worry_level_after_bored % self.divisible_by == 0;
        match is_divisible {
            true => (self.if_true_throw_to_monkey, worry_level_after_bored),
            false => (self.if_false_throw_to_monkey, worry_level_after_bored),
        }
    }

    pub fn get_number_of_inspections(&self) -> u64 {
        self.number_of_inspections
    }

    pub fn get_divisible_by(&self) -> u64 {
        self.divisible_by
    }
}

pub struct MonkeBuilder {
    items: Vec<u64>,
    divisible_by: Option<u64>,
    inspection_rules: Option<MonkeInspection<u64>>,
    if_true_throw_to_monke: Option<usize>,
    if_false_throw_to_monke: Option<usize>,
}

impl MonkeBuilder {
    pub fn new() -> Self {
        MonkeBuilder {
            items: vec![],
            divisible_by: None,
            inspection_rules: None,
            if_true_throw_to_monke: None,
            if_false_throw_to_monke: None,
        }
    }

    pub fn with_items(mut self, items: impl Iterator<Item = u64>) -> Self {
        self.items = Vec::from_iter(items);
        self
    }

    pub fn set_divisible_by(mut self, divisible_by: u64) -> Self {
        self.divisible_by = Some(divisible_by);
        self
    }

    pub fn set_inspection_rules(mut self, inspection_rules: MonkeInspection<u64>) -> Self {
        self.inspection_rules = Some(inspection_rules);
        self
    }

    pub fn set_throw_to_monke(mut self, if_true: usize, if_false: usize) -> Self {
        self.if_true_throw_to_monke = Some(if_true);
        self.if_false_throw_to_monke = Some(if_false);
        self
    }

    pub fn build(self) -> Result<Monke, String> {
        if self.divisible_by.is_none() {
            return Err("divisible_by is not set".into());
        }
        if self.inspection_rules.is_none() {
            return Err("inspection_rules is not set".into());
        }
        if self.if_true_throw_to_monke.is_none() {
            return Err("if_true_throw_to_monke is not set".into());
        }
        if self.if_false_throw_to_monke.is_none() {
            return Err("if_false_throw_to_monke is not set".into());
        }

        Ok(Monke {
            items: VecDeque::from_iter(self.items),
            divisible_by: self.divisible_by.unwrap(),
            inspection_rules: self.inspection_rules.unwrap(),
            if_true_throw_to_monkey: self.if_true_throw_to_monke.unwrap(),
            if_false_throw_to_monkey: self.if_false_throw_to_monke.unwrap(),
            number_of_inspections: 0,
        })
    }
}

#[derive(PartialEq, Debug)]
pub enum DivisionElement<T: Add + Mul> {
    Old,
    Val(T),
}

#[derive(PartialEq, Debug)]
pub enum OpType {
    Add,
    Mul,
}

#[derive(Debug)]
pub struct MonkeInspection<T>
where
    T: Add + Mul,
{
    pub(crate) element1: DivisionElement<T>,
    pub(crate) element2: DivisionElement<T>,
    pub(crate) op_type: OpType,
}

impl FromStr for MonkeInspection<u64> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = parse_inspection(s);
        match result {
            Ok((_, mi)) => Ok(mi),
            Err(v) => Err(v.to_string()),
        }
    }
}

impl<T> MonkeInspection<T>
where
    T: Add<Output = T> + Mul<Output = T> + Rem<Output= T> + Clone + std::fmt::Debug,
{
    fn inspect(&self, old_val: &T, magic_trick: T) -> T {
        use DivisionElement::*;

        let val1 = match &self.element1 {
            Old => old_val.clone(),
            Val(v) => v.clone(),
        };
        let val2 = match &self.element2 {
            Old => old_val.clone(),
            Val(v) => v.clone(),
        };

       let result = match self.op_type {
            OpType::Add => val1 + val2,
            OpType::Mul => {
                let result = val1 * val2;
                result
            },
        };
        result % magic_trick
    }
}

#[cfg(test)]
mod tests {
    use super::{DivisionElement, MonkeInspection, OpType};
    use rstest::*;
    use DivisionElement::*;

    #[rstest]
    #[case(79, Old, Val(19), 1501, OpType::Mul)]
    #[case(60, Old, Old, 60*60, OpType::Mul)]
    #[case(54, Old, Val(6), 60, OpType::Add)]
    fn test_perform_inspection(
        #[case] old_val: u64,
        #[case] el1: DivisionElement<u64>,
        #[case] el2: DivisionElement<u64>,
        #[case] expected_result: u64,
        #[case] op_type: OpType,
    ) {
        let mi = MonkeInspection {
            element1: el1,
            element2: el2,
            op_type,
        };
        let magic = 7 * 13 * 17 * 29 * 31;
        let result = mi.inspect(&old_val, magic);
        assert_eq!(result, expected_result);
    }

    #[rstest]
    #[case("  Operation: new = old + 6", Old, Val(6), OpType::Add)]
    #[case("  Operation: new = old * old", Old, Old, OpType::Mul)]
    #[case("  Operation: new = old * 19", Old, Val(19), OpType::Mul)]
    fn test_parse_monke_inspection(
        #[case] input: &str,
        #[case] expected1st: DivisionElement<u64>,
        #[case] expected2nd: DivisionElement<u64>,
        #[case] op_type: OpType,
    ) {
        let mi = input.parse::<MonkeInspection<u64>>().unwrap();

        assert_eq!(mi.element1, expected1st);
        assert_eq!(mi.element2, expected2nd);
        assert_eq!(mi.op_type, op_type);
    }
}
