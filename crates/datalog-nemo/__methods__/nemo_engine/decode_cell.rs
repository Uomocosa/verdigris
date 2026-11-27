use nemo::datavalues::AnyDataValue;
use nemo::datavalues::DataValue;
use verdigris_datalog_ir::EngineError;
use verdigris_datalog_ir::Value;

pub fn decode_cell(cell: &AnyDataValue) -> Result<Value, EngineError> {
    if let Some(number) = cell.to_i64() {
        return Ok(Value::Int(number));
    }
    if let Some(text) = cell.to_plain_string() {
        return Ok(Value::Sym(Box::leak(text.into_boxed_str())));
    }
    Err(EngineError::Backend(format!(
        "unsupported nemo value {cell}"
    )))
}

#[cfg(test)]
mod tests {
    use nemo::datavalues::AnyDataValue;

    use super::decode_cell;

    #[test]
    fn test_usage() {
        assert_eq!(
            decode_cell(&AnyDataValue::new_integer_from_i64(7)).unwrap(),
            verdigris_datalog_ir::Value::Int(7)
        );
        assert_eq!(
            decode_cell(&AnyDataValue::new_plain_string("ada".to_string())).unwrap(),
            verdigris_datalog_ir::Value::Sym("ada")
        );
    }

    #[test]
    fn rejects_values_outside_the_ir() {
        let outcome = decode_cell(&AnyDataValue::new_double_from_f64(0.5).unwrap());
        assert!(matches!(
            outcome,
            Err(verdigris_datalog_ir::EngineError::Backend(_))
        ));
    }
}
