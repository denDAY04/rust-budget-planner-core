//! The core modules for the rust budget planner.
//!
//! This crate contains all the business logic for the simple rust budget planner and is meant to
//! be integrated with external UI executables or libraries.

/// Module holding the core budget item type that acts as entries in a budget group.
pub mod budget_item;

/// Module holding the core budget group types that manages a collection of budget items.
pub mod budget_group;
