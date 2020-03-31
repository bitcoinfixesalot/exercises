use std::ops::DerefMut;
use std::{
    cell::{Cell, RefCell},
    collections::HashSet,
};

/// `InputCellID` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct InputCellID(usize);
/// `ComputeCellID` is a unique identifier for a compute cell.
/// Values of type `InputCellID` and `ComputeCellID` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellID = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellID = r.create_compute(&[react::CellID::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ComputeCellID(usize);
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CallbackID(usize);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CellID {
    Input(InputCellID),
    Compute(ComputeCellID),
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

pub struct Reactor<'a, T> {
    input_cells: Vec<T>,
    compute_cells: Vec<ComputeCell<'a, T>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Self {
            input_cells: vec![],
            compute_cells: vec![],
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellID {
        self.input_cells.push(initial);
        InputCellID(self.input_cells.len() - 1)
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'a>(
        &mut self,
        dependencies: &[CellID],
        compute_func: F,
    ) -> Result<ComputeCellID, CellID> {
        let value = compute_func(&self.get_values(dependencies)?);
        self.compute_cells
            .push(ComputeCell::new(value, dependencies, compute_func));
        Ok(ComputeCellID(self.compute_cells.len() - 1))
    }

    fn get_values(&self, dependencies: &[CellID]) -> Result<Vec<T>, CellID> {
        dependencies
            .iter()
            .map(|&id| self.value(id).ok_or(id))
            .collect()
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellID) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellID) -> Option<T> {
        match id {
            CellID::Input(InputCellID(i)) => self.input_cells.get(i).copied(),
            CellID::Compute(ComputeCellID(i)) => self.compute_cells.get(i).map(|c| c.value.get()),
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellID) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellID, new_value: T) -> bool {
        // Update input cell, if it exists.
        match self.input_cells.get_mut(id.0) {
            Some(cell) => *cell = new_value,
            None => return false,
        }

        // Update dependent compute cells.
        let mut changed = HashSet::new();
        changed.insert(CellID::Input(id));
        for (i, cell) in self.compute_cells.iter().enumerate() {
            if cell.dependencies.iter().any(|id| changed.contains(id)) {
                let dependency_values = self.get_values(&cell.dependencies).unwrap();
                let new_value = (cell.compute_func)(&dependency_values);
                if new_value != cell.value.get() {
                    cell.value.set(new_value);
                    changed.insert(CellID::Compute(ComputeCellID(i)));
                    for callback in &cell.callbacks {
                        if let Some(callback) = callback {
                            callback.borrow_mut().deref_mut()(new_value);
                        };
                    }
                }
            }
        }
        true
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) + 'a>(
        &mut self,
        id: ComputeCellID,
        callback: F,
    ) -> Option<CallbackID> {
        self.compute_cells.get_mut(id.0).map(|cell| {
            cell.callbacks.push(Some(Box::new(RefCell::new(callback))));
            CallbackID(cell.callbacks.len() - 1)
        })
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellID,
        callback: CallbackID,
    ) -> Result<(), RemoveCallbackError> {
        self.compute_cells
            .get_mut(cell.0)
            .ok_or(RemoveCallbackError::NonexistentCell)
            .and_then(|cell| {
                cell.callbacks
                    .get_mut(callback.0)
                    .filter(|c| c.is_some()) // Filter out if already deleted
                    .ok_or(RemoveCallbackError::NonexistentCallback)
                    .map(|c| *c = None)
            })
    }
}

type Callback<'a, T> = Box<RefCell<dyn FnMut(T) + 'a>>;

struct ComputeCell<'a, T> {
    value: Cell<T>,
    dependencies: Vec<CellID>,
    compute_func: Box<dyn Fn(&[T]) -> T + 'a>,
    callbacks: Vec<Option<Callback<'a, T>>>,
}

impl<'a, T> ComputeCell<'a, T> {
    fn new<F>(value: T, dependencies: &[CellID], compute_func: F) -> Self
    where
        F: Fn(&[T]) -> T + 'a,
    {
        Self {
            value: Cell::new(value),
            dependencies: dependencies.to_vec(),
            compute_func: Box::new(compute_func),
            callbacks: vec![],
        }
    }
}
