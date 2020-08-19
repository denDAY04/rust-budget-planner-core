use crate::errors::CollectionError;
use crate::types::BudgetItem;
use crate::errors::CollectionError::IndexOutOfBounds;

struct BudgetGroup {
    name: String,
    items: Vec<Box<dyn BudgetItem>>
}

impl BudgetGroup {

    fn new(group_name: String) -> BudgetGroup {
        BudgetGroup {
            name: group_name,
            items: Vec::new()
        }
    }

    fn add(&mut self, item: Box<dyn BudgetItem>) {
        self.items.push(item);
        self.items.sort_unstable();
    }

    fn remove(&mut self, idx: usize) -> Result<(), CollectionError> {
        if idx > self.items.len() {
            return Err(IndexOutOfBounds)
        }
        self.items.remove(idx);
        self.items.sort_unstable();
        Ok(())
    }

    fn items(&self) -> &Vec<Box<dyn BudgetItem>> {
        &self.items
    }
}