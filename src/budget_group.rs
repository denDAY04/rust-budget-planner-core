use crate::errors::CollectionError;
use crate::budget_item::BudgetItem;
use crate::errors::CollectionError::IndexOutOfBounds;

struct BudgetGroup {
    name: String,
    items: Vec<BudgetItem>
}

impl BudgetGroup {

    pub fn new(group_name: String) -> BudgetGroup {
        BudgetGroup {
            name: group_name,
            items: Vec::new()
        }
    }

    pub fn add(&mut self, item: BudgetItem) {
        self.items.push(item);
        self.items.sort_unstable();
    }

    pub fn remove(&mut self, idx: usize) -> Result<(), CollectionError> {
        if idx > self.items.len() {
            return Err(IndexOutOfBounds)
        }
        self.items.remove(idx);
        self.items.sort_unstable();
        Ok(())
    }

    pub fn items(&self) -> &Vec<BudgetItem> {
        &self.items
    }
}
