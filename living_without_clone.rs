// Performance Note: Living without clone:
// https://www.youtube.com/watch?v=AZsgdCEQjFo



struct Expr {
    self.dialect: u32,
}
impl Expr {
    fn take(&mut self) -> Expr {
        let mut taken = Self::new_placeholder();
        std::mem::swap(self, &mut taken);
        taken
    }
}
