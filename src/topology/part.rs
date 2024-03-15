pub enum Topology { TRIANGLE }

pub struct Part
{
    pub index: usize,
    pub count: usize,
    pub topology: Topology,
}

impl Part
{
    pub fn create(topology: Topology, index: usize, count: usize) -> Self
    {
        Self { index, count, topology }
    }
}