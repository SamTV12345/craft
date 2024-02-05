pub trait Version: ToString {
    fn new(version: &str) -> Self;

    fn satisfies(&self, version: &str) -> bool;

    fn is_exact(&self) -> bool;
}

pub trait Satisfies {
    fn satisfies(&self, version: &str) -> bool;
}
