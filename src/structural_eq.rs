pub trait StructuralEq<Rhs = Self> {
    fn structural_eq(&self, rhs: &Rhs) -> bool;
}
