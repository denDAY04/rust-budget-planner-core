use crate::types::{Period, BudgetItem};
use std::cmp::Ordering;

pub struct Income {
    name: String,
    amount: f32,
    period: Period
}

impl BudgetItem for Income {
    fn monthly_contribution(&self) -> f32 {
        match self.period {
            Period::Every1Month => self.amount,
            Period::Every2Months => self.amount / 2.0,
            Period::Every3Months => self.amount / 3.0,
            Period::Every6Months => self.amount / 6.0,
            Period::Every12Months => self.amount / 12.0,
        }
    }
}

impl PartialOrd for Income {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Ord for Income {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.name.cmp(&other.name) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.period.cmp(&other.period),
            Ordering::Greater => Ordering::Greater,
        }
    }
}

impl PartialEq for Income {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.period == other.period
    }
}

impl Eq for Income { }

