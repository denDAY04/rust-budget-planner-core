use std::slice::Iter;
use std::iter::Enumerate;
use crate::budget_item::BudgetItem;

/// An ordered collection of budget items gathered under a common name.
///
/// Note that the group does not impose a unique restriction on its entries, so there is no checks
/// for duplicate entries.
pub struct BudgetGroup {
    name: String,
    items: Vec<BudgetItem>
}

/// Error thrown when trying to index the group of budget items with an invalid index.
pub struct InvalidIndex { }

impl BudgetGroup {

    /// Create a new budget group with a name.
    ///
    /// # Parameters
    /// * `name` - the desired name of the group.
    pub fn new(name: &str) -> BudgetGroup {
        BudgetGroup {
            name: name.to_owned(),
            items: Vec::new()
        }
    }

    /// Get a reference to the group's name.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get an enumeration iterator to the items in the group.
    ///
    /// This is useful for two reasons:
    /// 1. To iterate over all the elements in the group, since the underlying container is not
    /// exposed through the public API.
    /// 2. To index the elements as they are iterated. This is needed for removing elements using
    /// [`remove`](#method.remove), as the group is ordered.
    pub fn enumerate(&self) -> Enumerate<Iter<BudgetItem>> {
        self.items.iter().enumerate()
    }

    /// Add a budget item to the group.
    ///
    /// Since the group is ordered, adding an item to will trigger a re-ordering of the items in
    /// the group.
    ///
    /// # Parameters
    /// * `item` - the budget item that should be added to the group.
    pub fn add(&mut self, item: BudgetItem) {
        self.items.push(item);
        self.items.sort_unstable();
    }

    /// Remove an item from the group, based on its index.
    ///
    /// As the budget items do not contain a notion of identity, the removal has to be done based
    /// on the item's position in the group.
    ///
    /// # Parameters
    /// * `idx` - the index of the item to be removed, as discovered using [`enumerate`](#method.enumerate).
    ///
    /// # Returns
    /// `Result::Ok()` if the index is valid and an item was therefore removed, or `Result::Err` if
    /// the index is invalid.
    pub fn remove(&mut self, idx: usize) -> Result<(), ()> {
        if idx > self.items.len() {
            return Err(());
        }
        self.items.remove(idx);
        self.items.sort_unstable();
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::budget_group::BudgetGroup;
    use crate::budget_item::{BudgetItem, Period};

    #[test]
    fn new() {
        let group = BudgetGroup::new("foo");
        assert_eq!("foo", group.name());
    }

    #[test]
    fn add() {
        let mut item_group = BudgetGroup::new("foo");
        assert_eq!(0, item_group.enumerate().len());

        let item = BudgetItem::with_income("bar", 10.0, Period::Every1Month);
        item_group.add(item);

        assert_eq!(1, item_group.enumerate().len())
    }

    #[test]
    fn list_is_ordered() {
        let mut item_group = BudgetGroup::new("foo");

        let item1 = BudgetItem::with_income("qq", 10.0, Period::Every1Month);
        let item2 = BudgetItem::with_income("ab", 10.0, Period::Every1Month);
        let expected_first = item2.clone();
        item_group.add(item1);
        item_group.add(item2);

        assert_eq!(&expected_first, item_group.enumerate().next().unwrap().1, "Unexpected first budget item")
    }

    #[test]
    fn remove() {
        let mut item_group = BudgetGroup::new("foo");
        let item = BudgetItem::with_income("bar", 10.0, Period::Every1Month);
        item_group.add(item);

        let removed = item_group.remove(0);
        assert!(removed.is_ok());
        assert_eq!(0, item_group.enumerate().len())
    }
}