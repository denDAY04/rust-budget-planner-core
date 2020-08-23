use std::cmp::Ordering;
use std::cmp::Ordering::Equal;

/// The repeating period of a budget item, e.g. [`Every3Months`] means in item whose amount is
/// repeated every 3 months.
#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone)]
pub enum Period {
    /// The amount of the budget item is based on a 1-month recurring period.
    Every1Month,
    /// The amount of the budget item is based on a 2-month recurring period.
    Every2Months,
    /// The amount of the budget item is based on a 3-month recurring period.
    Every3Months,
    /// The amount of the budget item is based on a 6-month recurring period.
    Every6Months,
    /// The amount of the budget item is based on a 12-month recurring period.
    Every12Months,
}

/// A singular entry item in a budget.
///
/// This type encompass the basic necessary data for a budget entry: a simple name, the amount the
/// entry contributes with, the recurring period of how often the amount is contributed to the
/// overall budget, and the type of the entry (positive income, or negative express).
///
/// An entry always operate with a positive number as its amount, no matter whether its an expense
/// or an income amount. Instead this influence is handled by the way the entry is created.
///
/// ## Creating an income entry
/// ```
/// use rbp_core::budget_item::{BudgetItem, Period};
/// let monthly_income = BudgetItem::with_income("An income entry", 1_000.0, Period::Every1Month);
/// ```
/// ## Creating an expese entry
/// ```
/// use rbp_core::budget_item::{BudgetItem, Period};
/// let monthly_expense = BudgetItem::with_expense("An income entry", 1_000.0, Period::Every1Month);
/// ```
#[derive(Debug)]
pub struct BudgetItem {
    name: String,
    period: Period,
    item_type: Type,
    amount: f64,
}

// Local type denoting the type of the budget item.
#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone)]
enum Type {
    Income,
    Expense,
}

impl BudgetItem {

    /// Create a new income budget item.
    ///
    /// This type of budget entries will have a positive contribution on an overall budget.
    ///
    /// # Parameters
    /// * `name` - the name of the item.
    /// * `amount` - the amount the entry contributes to an overall budget. This must always be
    /// a positive number.
    /// * `period` - the recurring period of how often the amount contribute to the overall budget.
    ///
    /// # Returns
    /// Always returns a valid budget item.
    ///
    /// # Panics
    /// If the amount is less than 0, the method will panic.
    pub fn with_income(name: &str, amount: f64, period: Period) -> BudgetItem {
        Self::check_amount(&amount);

        BudgetItem{
            name: name.to_owned(),
            period,
            item_type: Type::Income,
            amount
        }
    }

    /// Create a new expense budget item.
    ///
    /// This type of budget entries will have a negative contribution on an overall budget.
    ///
    /// # Parameters
    /// * `name` - the name of the item.
    /// * `amount` - the amount the entry contributes to an overall budget. This must always be
    /// a positive number.
    /// * `period` - the recurring period of how often the amount contribute to the overall budget.
    ///
    /// # Returns
    /// Always returns a valid budget item.
    ///
    /// # Panics
    /// If the amount is less than 0, the method will panic.
    pub fn with_expense(name: &str, amount: f64, period: Period) -> BudgetItem {
        Self::check_amount(&amount);

        BudgetItem{
            name: name.to_owned(),
            period,
            item_type: Type::Expense,
            amount
        }
    }

    /// Calculate the monthly contributions for this item.
    ///
    /// # Returns
    /// The monthly contribution, calculated based on the entry's amount and its period.
    pub fn monthly_contribution(&self) -> f64 {
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

    fn check_amount(amount: &f64) {
        assert!(*amount > 0.0, "Amount must be greater than 0");
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

impl Eq for BudgetItem { }

impl Clone for BudgetItem {
    fn clone(&self) -> Self {
        BudgetItem{
            name: self.name.clone(),
            period: self.period,
            item_type: self.item_type,
            amount: self.amount
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::budget_item::{BudgetItem, Period};
    use crate::budget_item::Period::{Every1Month, Every12Months, Every2Months, Every3Months, Every6Months};

    #[test]
    #[should_panic]
    fn negative_income() {
        BudgetItem::with_income("Foo", -100.0, Period::Every1Month);
    }

    #[test]
    #[should_panic]
    fn negative_expense() {
        BudgetItem::with_expense("Foo", -100.0, Period::Every1Month);
    }

    #[test]
    fn monthly_contribution_for_income() {
        let mut amount = 42.0;
        let mut months = 1.0;
        let monthly_income = BudgetItem::with_income("1m", amount, Every1Month);
        assert_eq!(amount / months, monthly_income.monthly_contribution(), "Unexpected monthly contribution from monthly income");

        amount = 10.0;
        months = 2.0;
        let yearly_income = BudgetItem::with_income("2m", amount, Every2Months);
        assert_eq!(amount / months, yearly_income.monthly_contribution(), "Unexpected monthly contribution from bimonthly income");

        amount = 15.0;
        months = 3.0;
        let yearly_income = BudgetItem::with_income("3m", amount, Every3Months);
        assert_eq!(amount / months, yearly_income.monthly_contribution(), "Unexpected monthly contribution from quarterly income");

        amount = 30.0;
        months = 6.0;
        let yearly_income = BudgetItem::with_income("6m", amount, Every6Months);
        assert_eq!(amount / months, yearly_income.monthly_contribution(), "Unexpected monthly contribution from 4-month income");

        amount = 12.0;
        months = 12.0;
        let yearly_income = BudgetItem::with_income("12m", amount, Every12Months);
        assert_eq!(amount / months, yearly_income.monthly_contribution(), "Unexpected monthly contribution from yearly income");
    }

    #[test]
    fn monthly_contribution_for_expense() {
        let mut amount = 42.0;
        let mut months = 1.0;
        let monthly_expense = BudgetItem::with_expense("1m", amount, Every1Month);
        assert_eq!(-amount / months, monthly_expense.monthly_contribution(), "Unexpected monthly contribution from monthly expense");

        amount = 10.0;
        months = 2.0;
        let yearly_expense = BudgetItem::with_expense("2m", amount, Every2Months);
        assert_eq!(-amount / months, yearly_expense.monthly_contribution(), "Unexpected monthly contribution from bimonthly expense");

        amount = 15.0;
        months = 3.0;
        let yearly_expense = BudgetItem::with_expense("3m", amount, Every3Months);
        assert_eq!(-amount / months, yearly_expense.monthly_contribution(), "Unexpected monthly contribution from quarterly expense");

        amount = 30.0;
        months = 6.0;
        let yearly_expense = BudgetItem::with_expense("6m", amount, Every6Months);
        assert_eq!(-amount / months, yearly_expense.monthly_contribution(), "Unexpected monthly contribution from 4-month expense");

        amount = 12.0;
        months = 12.0;
        let yearly_expense = BudgetItem::with_expense("12m", amount, Every12Months);
        assert_eq!(-amount / months, yearly_expense.monthly_contribution(), "Unexpected monthly contribution from yearly expense");
    }
}