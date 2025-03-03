use qudit_expr::UnitaryExpression;
use qudit_expr::UnitaryExpressionGenerator;

// TODO: remove
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct CPGate;

impl UnitaryExpressionGenerator for CPGate {
    fn gen_expr(&self) -> UnitaryExpression {
        todo!()
    }
}

