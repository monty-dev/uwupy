#![allow(unused)]
fn main() {
    use pyo3::prelude::*;
    use uwuifier::uwuify_str_sse;

    /// Uwuify a string
    #[pyfunction]
    fn uwuify_str(text: &str) -> PyResult<String> {
        Ok(uwuify_str_sse(text))
    }

    /// A Python binding around the library https://github.com/Daniel-Liu-c0deb0t/uwu
    #[pymodule]
    fn uwupy(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(uwuify_str, m)?)?;

        Ok(())
    }
}
