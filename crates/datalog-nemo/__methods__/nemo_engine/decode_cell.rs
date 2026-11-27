use std::collections::BTreeMap;

use nemo::datavalues::AnyDataValue;
use nemo::datavalues::DataValue;
use verdigris_datalog_ir::EngineError;
use verdigris_datalog_ir::Value;

use crate::NemoEngine;

pub fn decode_cell(
    cell: &AnyDataValue,
    symbols: &BTreeMap<String, &'static str>,
) -> Result<Value, EngineError> {
    if let Some(number) = cell.to_i64() {
        return Ok(Value::Int(number));
    }
    if let Some(iri) = cell.to_iri()
        && let Some(number) = iri
            .strip_prefix(NemoEngine::ENTITY_IRI_PREFIX)
            .and_then(|suffix| suffix.parse::<u64>().ok())
    {
        return Ok(Value::Entity(number));
    }
    if let Some(text) = cell.to_plain_string()
        && let Some(symbol) = symbols.get(&text)
    {
        return Ok(Value::Sym(symbol));
    }
    Err(EngineError::Backend(format!(
        "unsupported nemo value {cell}"
    )))
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use nemo::datavalues::AnyDataValue;

    use super::decode_cell;
    use crate::NemoEngine;

    #[test]
    fn test_usage() {
        let symbols: BTreeMap<String, &'static str> = BTreeMap::from([("ada".to_string(), "ada")]);
        assert_eq!(
            decode_cell(&AnyDataValue::new_integer_from_i64(7), &symbols).unwrap(),
            verdigris_datalog_ir::Value::Int(7)
        );
        assert_eq!(
            decode_cell(&AnyDataValue::new_plain_string("ada".to_string()), &symbols).unwrap(),
            verdigris_datalog_ir::Value::Sym("ada")
        );
        assert_eq!(
            decode_cell(
                &AnyDataValue::new_iri(format!("{}7", NemoEngine::ENTITY_IRI_PREFIX)),
                &symbols
            )
            .unwrap(),
            verdigris_datalog_ir::Value::Entity(7)
        );
    }

    #[test]
    fn rejects_values_outside_the_ir() {
        let symbols: BTreeMap<String, &'static str> = BTreeMap::new();
        let outcome = decode_cell(&AnyDataValue::new_double_from_f64(0.5).unwrap(), &symbols);
        assert!(matches!(
            outcome,
            Err(verdigris_datalog_ir::EngineError::Backend(_))
        ));
    }

    #[test]
    fn rejects_unknown_symbols_and_foreign_iris() {
        let symbols: BTreeMap<String, &'static str> = BTreeMap::new();
        let unknown = decode_cell(
            &AnyDataValue::new_plain_string("grace".to_string()),
            &symbols,
        );
        assert!(matches!(
            unknown,
            Err(verdigris_datalog_ir::EngineError::Backend(_))
        ));
        let foreign = decode_cell(
            &AnyDataValue::new_iri("https://example.com/other".to_string()),
            &symbols,
        );
        assert!(matches!(
            foreign,
            Err(verdigris_datalog_ir::EngineError::Backend(_))
        ));
    }
}
