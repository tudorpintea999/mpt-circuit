use super::{BinaryQuery, ConstraintBuilder, Query};
use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::{Region, Value},
    plonk::ConstraintSystem,
    plonk::{Advice, Column},
};

#[derive(Clone, Copy)]
pub struct BinaryColumn(pub Column<Advice>);

impl BinaryColumn {
    pub fn rotation<F: FieldExt>(&self, i: i32) -> BinaryQuery<F> {
        BinaryQuery(Query::Advice(self.0, i))
    }

    pub fn current<F: FieldExt>(&self) -> BinaryQuery<F> {
        self.rotation(0)
    }

    pub fn previous<F: FieldExt>(&self) -> BinaryQuery<F> {
        self.rotation(-1)
    }

    pub fn next<F: FieldExt>(&self) -> BinaryQuery<F> {
        self.rotation(1)
    }

    pub fn configure<F: FieldExt>(
        cs: &mut ConstraintSystem<F>,
        cb: &mut ConstraintBuilder<F>,
    ) -> Self {
        let binary_column = Self(cs.advice_column());
        cb.assert(
            "binary column is 0 or 1",
            binary_column.current().or(!binary_column.current()),
        );
        binary_column
    }

    pub fn assign<F: FieldExt>(&self, region: &mut Region<'_, F>, offset: usize, value: bool) {
        region
            .assign_advice(|| "binary", self.0, offset, || Value::known(F::from(value)))
            .expect("failed assign_advice");
    }
}
