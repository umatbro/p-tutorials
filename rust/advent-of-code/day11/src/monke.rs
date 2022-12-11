use std::{collections::VecDeque, ops::{Mul, Add}};

pub struct Monke {
    items: VecDeque<u32>,
    divisible_by: u32,
    if_true_throw_to_monkey: u32,
    if_false_throw_to_monkey: u32,    
}

pub enum DivisionElement<T: Add + Mul> {
    Old,
    Val(T),
}

pub enum OpType {
    Add,
    Mul,
}

pub fn perform_inspection<T>(old_val: &T, element1: &DivisionElement<T>, element2: &DivisionElement<T>, op_type: &OpType) -> T
where T: Add<Output = T> + Mul<Output = T> + Clone {
    use DivisionElement::*;

    let val1 = match element1 {
        Old => old_val.clone(),
        Val(v) => v.clone(),
    };
    let val2 = match element2 {
        Old => old_val.clone(),
        Val(v) => v.clone(),
    };

    match op_type {
        OpType::Add => {
            val1 + val2
        },
        OpType::Mul => {
            val1 * val2
        }
    }
}


#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{DivisionElement, perform_inspection, OpType};
    use DivisionElement::*;

    #[rstest]
    #[case(79, Old, Val(19), 1501, OpType::Mul)]
    #[case(60, Old, Old, 60*60, OpType::Mul)]
    #[case(54, Old, Val(6), 60, OpType::Add)]
    fn test_perform_inspection(
        #[case] old_val: u32,
        #[case] el1: DivisionElement<u32>, 
        #[case] el2: DivisionElement<u32>, 
        #[case] expected_result: u32,
        #[case] op_type: OpType,
    ) {
        let result = perform_inspection(&old_val, &el1, &el2, &op_type);
        assert_eq!(result, expected_result);
    }

}