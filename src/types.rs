
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub enum Period {
    Every1Month,
    Every2Months,
    Every3Months,
    Every6Months,
    Every12Months
}


pub trait BudgetItem: Ord {
    fn monthly_contribution(&self) -> f32;
}

