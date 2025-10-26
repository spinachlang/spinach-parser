
use pest::Parser;
use pest_derive::Parser;
use pyo3::prelude::*;

/// ======= RUST PARSER =======
#[derive(Parser)]
#[grammar = "spinach.pest"]
pub struct SpinachParser;

pub fn parse_spinach(code: &str) -> Result<String, String> {
    match SpinachParser::parse(Rule::start, code) {
        Ok(pairs) => {
            let mut out = String::new();
            for pair in pairs {
                out.push_str(&format!("{:?}\n", pair));
            }
            Ok(out)
        }
        Err(e) => Err(e.to_string()),
    }
}

/// ======= PYTHON WRAPPER =======
#[pyclass]
pub struct ParserWrapper;

#[pymethods]
impl ParserWrapper {
    #[new]
    fn new() -> Self {
        ParserWrapper
    }

    fn get_tree(&self, code: &str) -> PyResult<String> {
        match parse_spinach(code) {
            Ok(tree) => Ok(tree),
            Err(err) => Err(pyo3::exceptions::PyValueError::new_err(err)),
        }
    }
}

#[pymodule]
fn spinach_parser(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ParserWrapper>()?;
    Ok(())
}
