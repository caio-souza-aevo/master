use crate::types::path::Path;

#[derive(Eq, PartialEq, Debug)]
pub struct Route {
    pub cost: i32,
    pub path: Path,
}

#[derive(Eq, PartialEq, Debug)]
pub enum HamiltonianResult {
    Ok,
    VisitedTwice(usize),
}

impl Route
{
    pub fn new(cost: i32, path: Path) -> Route {
        Route { cost, path }
    }
}
