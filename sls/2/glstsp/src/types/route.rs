use crate::types::path::Path;

#[derive(Eq, PartialEq, Debug)]
pub struct Route {
    path: Path,
    cost: i32,
}

#[derive(Eq, PartialEq, Debug)]
pub enum HamiltonianResult {
    Ok,
    VisitedTwice(usize),
}

impl Route
{
    pub fn new(path: Path, cost: i32) -> Route {
        Route { path, cost }
    }

    pub fn cost(&self) -> i32 {
        self.cost
    }
}
