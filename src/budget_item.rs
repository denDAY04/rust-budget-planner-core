use std::cmp::Ordering;
use std::cmp::Ordering::Equal;

#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub enum Period {
    Every1Month,
    Every2Months,
    Every3Months,
    Every6Months,
    Every12Months,
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
enum Type {
    Income,
    Expense,
}

pub struct BudgetItem {
    name: String,
    period: Period,
    item_type: Type,
    amount: f32,
}

impl BudgetItem {

    pub fn new_income(name: &str, amount: f32, period: Period) -> BudgetItem {
        BudgetItem{
            name: name.to_owned(),
            period,
            item_type: Type::Income,
            amount
        }
    }

    pub fn new_expense(name: &str, amount: f32, period: Period) -> BudgetItem {
        BudgetItem{
            name: name.to_owned(),
            period,
            item_type: Type::Expense,
            amount
        }
    }

    pub fn monthly_contribution(&self) -> f32 {
        let num = match self.period {
            Period::Every1Month => self.amount,
            Period::Every2Months => self.amount / 2.0,
            Period::Every3Months => self.amount / 3.0,
            Period::Every6Months => self.amount / 6.0,
            Period::Every12Months => self.amount / 12.0,
        };

        match self.item_type {
            Type::Income => num,
            Type::Expense => -num,
        }
    }
}

impl PartialOrd for BudgetItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let name_cmp = self.name.partial_cmp(&other.name)?;
        if name_cmp != Equal {
            return Option::Some(name_cmp);
        }

        let period_cmp = self.period.partial_cmp(&other.period)?;
        if period_cmp != Equal {
            return Option::Some(period_cmp);
        }

        self.item_type.partial_cmp(&other.item_type)
    }
}

impl Ord for BudgetItem {
    fn cmp(&self, other: &Self) -> Ordering {
        let name_cmp = self.name.cmp(&other.name);
        if name_cmp != Equal {
            return name_cmp;
        }

        let period_cmp = self.period.cmp(&other.period);
        if period_cmp != Equal {
            return period_cmp;
        }

        self.item_type.cmp(&other.item_type)
    }
}

impl PartialEq for BudgetItem {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.period == other.period && self.item_type == other.item_type
    }
}

impl Eq for BudgetItem {}


#[cfg(test)]
mod tests {
    use crate::budget_item::{BudgetItem, Type, Period};
    use crate::budget_item::Period::{Every1Month, Every12Months, Every2Months, Every3Months, Every6Months};

    #[test]
    fn monthly_contribution_for_income() {
        let mut amount = 42.0;
        let mut months = 1.0;
        let monthly_income = BudgetItem::new_income("1m", amount, Every1Month);
        assert_eq!(amount / months, monthly_income.monthly_contribution(), "Unexpected monthly contribution from monthly income");

        amount = 10.0;
        months = 2.0;
        let yearly_income = BudgetItem::new_income("2m", amount, Every2Months);
        assert_eq!(amount / months, yearly_income.monthly_contribution(), "Unexpected monthly contribution from bimonthly income");

        amount = 15.0;
        months = 3.0;
        let yearly_income = BudgetItem::new_income("3m", amount, Every3Months);
        assert_eq!(amount / months, yearly_income.monthly_contribution(), "Unexpected monthly contribution from quarterly income");

        amount = 30.0;
        months = 6.0;
        let yearly_income = BudgetItem::new_income("6m", amount, Every6Months);
        assert_eq!(amount / months, yearly_income.monthly_contribution(), "Unexpected monthly contribution from 4-month income");

        amount = 12.0;
        months = 12.0;
        let yearly_income = BudgetItem::new_income("12m", amount, Every12Months);
        assert_eq!(amount / months, yearly_income.monthly_contribution(), "Unexpected monthly contribution from yearly income");
    }

    #[test]
    fn monthly_contribution_for_expense() {
        let mut amount = 42.0;
        let mut months = 1.0;
        let monthly_expense = BudgetItem::new_expense("1m", amount, Every1Month);
        assert_eq!(-amount / months, monthly_expense.monthly_contribution(), "Unexpected monthly contribution from monthly expense");

        amount = 10.0;
        months = 2.0;
        let yearly_expense = BudgetItem::new_expense("2m", amount, Every2Months);
        assert_eq!(-amount / months, yearly_expense.monthly_contribution(), "Unexpected monthly contribution from bimonthly expense");

        amount = 15.0;
        months = 3.0;
        let yearly_expense = BudgetItem::new_expense("3m", amount, Every3Months);
        assert_eq!(-amount / months, yearly_expense.monthly_contribution(), "Unexpected monthly contribution from quarterly expense");

        amount = 30.0;
        months = 6.0;
        let yearly_expense = BudgetItem::new_expense("6m", amount, Every6Months);
        assert_eq!(-amount / months, yearly_expense.monthly_contribution(), "Unexpected monthly contribution from 4-month expense");

        amount = 12.0;
        months = 12.0;
        let yearly_expense = BudgetItem::new_expense("12m", amount, Every12Months);
        assert_eq!(-amount / months, yearly_expense.monthly_contribution(), "Unexpected monthly contribution from yearly expense");
    }
}