use pyo3::prelude::*;

use crate::core::config::ProcessorConfig;
use crate::core::processor::Processor;

pub fn py_process(
    processor: Box<dyn Processor>,
    config: serde_json::Value,
    data: ndarray::ArrayViewD<'_, u8>,
) -> ndarray::ArrayD<u8> {
    let image: ndarray_image::NdColor<'_, u8> = data.into_dimensionality::<ndarray::Ix3>().unwrap();

    let image: Option<ndarray_image::ImgRgb> = ndarray_image::NdImage(image.view()).into();
    let image = image.unwrap().to_owned();

    let config: ProcessorConfig = ProcessorConfig {
        id: processor.id(),
        config: config,
    };
    let s: &[u8] = image.as_raw();
    let image =
        image::ImageBuffer::from_raw(image.width() as u32, image.height() as u32, Vec::from(s))
            .unwrap();
    let image = image::DynamicImage::ImageRgb8(image);

    let image = processor.process(config, image);
    let image = image.to_rgb8();
    let image: ndarray_image::NdColor = ndarray_image::NdImage(&image).into();

    image.to_owned().into_dimensionality::<_>().unwrap()
}

#[macro_export]
macro_rules! export_python_plugin {
    ($processor:ident) => {
        #[doc(hidden)]
        #[pyfunction]
        pub fn default_config() -> PyResult<PyObject> {
            let processor = $processor::default();
            let gil = Python::acquire_gil();
            let py = gil.python();
            Ok(pythonize::pythonize(py, &processor.default_config()).unwrap())
        }

        #[doc(hidden)]
        #[pyfunction]
        pub fn id() -> String {
            let processor = $processor::default();
            processor.id()
        }

        #[doc(hidden)]
        #[pyfunction]
        pub fn name() -> String {
            let processor = $processor::default();
            processor.name()
        }

        #[doc(hidden)]
        #[pyfunction]
        pub fn description() -> String {
            let processor = $processor::default();
            processor.description()
        }

        #[allow(non_snake_case)]
        #[pymodule]
        pub fn $processor(_py: Python, m: &PyModule) -> PyResult<()> {
            m.add_function(pyo3::wrap_pyfunction!(id, m)?)?;
            m.add_function(pyo3::wrap_pyfunction!(name, m)?)?;
            m.add_function(pyo3::wrap_pyfunction!(description, m)?)?;
            m.add_function(pyo3::wrap_pyfunction!(default_config, m)?)?;

            #[pyfn(m, "process")]
            fn process_py<'py>(
                py: Python<'py>,
                config: PyObject,
                data: numpy::PyReadonlyArrayDyn<u8>,
            ) -> &'py numpy::PyArrayDyn<u8> {
                use numpy::IntoPyArray;
                let config = pythonize::depythonize(config.as_ref(py)).unwrap();
                $crate::plugins::python::py_process(
                    Box::new($processor::default()),
                    config,
                    data.as_array(),
                )
                .into_pyarray(py)
            }
            Ok(())
        }
    };
    ($processor:ty, $name:expr, $python:ident, $pymod:ident) => {
        paste::paste! {

            mod [< $name >] {
                use pyo3::prelude::*;
                use $crate::core::processor::Processor;

                #[doc(hidden)]
                #[pyfunction]
                pub fn default_config() -> PyResult<PyObject> {
                    let processor = $processor::default();
                    let gil = Python::acquire_gil();
                    let py = gil.python();
                    Ok(pythonize::pythonize(py, &processor.default_config()).unwrap())
                }

                #[doc(hidden)]
                #[pyfunction]
                pub fn id() -> String {
                    let processor = $processor::default();
                    processor.id()
                }

                #[doc(hidden)]
                #[pyfunction]
                pub fn name() -> String {
                    let processor = $processor::default();
                    processor.name()
                }

                #[doc(hidden)]
                #[pyfunction]
                pub fn description() -> String {
                    let processor = $processor::default();
                    processor.description()
                }

                #[doc(hidden)]
                #[pyfunction]
                fn process_py<'py>(
                    py: Python<'py>,
                    config: PyObject,
                    data: numpy::PyReadonlyArrayDyn<u8>,
                ) -> &'py numpy::PyArrayDyn<u8> {
                    use numpy::IntoPyArray;
                    let config = pythonize::depythonize(config.as_ref(py)).unwrap();
                    $crate::plugins::python::py_process(
                        Box::new($processor::default()),
                        config,
                        data.as_array(),
                    )
                    .into_pyarray(py)
                }

                pub fn init_submodule(m: &PyModule) -> PyResult<()> {
                    m.add_function(pyo3::wrap_pyfunction!(id, m)?)?;
                    m.add_function(pyo3::wrap_pyfunction!(name, m)?)?;
                    m.add_function(pyo3::wrap_pyfunction!(description, m)?)?;
                    m.add_function(pyo3::wrap_pyfunction!(default_config, m)?)?;
                    m.add("process", pyo3::wrap_pyfunction!(process_py, m)?)?;
                    Ok(())
                }
            }

            let submod = PyModule::new($python, $name)?;
            [< $name >]::init_submodule(submod)?;
            $pymod.add_submodule(submod)?;
        };
    };
}

#[cfg(feature = "resize-plugin")]
#[pymodule]
fn lenna_core(py: Python, module: &PyModule) -> PyResult<()> {
    export_python_plugin!(crate::core::resize::Resize, "resize", py, module);
    Ok(())
}

#[cfg(test)]
mod tests {
    use pyo3::prelude::*;

    #[test]
    fn module() {
        use crate::core::processor::Processor;
        use crate::core::resize::Resize;
        #[allow(non_camel_case_types)]
        type resize = Resize;
        export_python_plugin!(resize);
    }

    #[test]
    fn submodule() {
        #[pymodule]
        fn lenna_test_core(py: Python, module: &PyModule) -> PyResult<()> {
            export_python_plugin!(crate::core::resize::Resize, "resize", py, module);
            Ok(())
        }
    }
}
