use crate::core::config::ProcessorConfig;
use crate::core::processor::Processor;
use crate::core::LennaImage;
use image::RgbImage;
use ndarray::{Array3, ArrayView3};
use nshare::ToNdarray3;

#[allow(unused_imports)]
use pyo3::prelude::*;

fn array_to_image(arr: Array3<u8>) -> RgbImage {
    assert!(arr.is_standard_layout());

    let (height, width, _) = arr.dim();
    let raw = arr.into_raw_vec();

    RgbImage::from_raw(width as u32, height as u32, raw)
        .expect("container should have the right size for the image dimensions")
}

pub fn py_process(
    processor: Box<dyn Processor>,
    config: serde_json::Value,
    data: ndarray::ArrayViewD<'_, u8>,
) -> ndarray::ArrayD<u8> {
    let image: ArrayView3<u8> = data.into_dimensionality::<ndarray::Ix3>().unwrap();

    let config: ProcessorConfig = ProcessorConfig {
        id: processor.id(),
        config: config,
    };

    let image: RgbImage = array_to_image(image.to_owned());
    let image: image::DynamicImage = image::DynamicImage::ImageRgb8(image);

    let mut lenna_image = Box::new(LennaImage::default());
    lenna_image.image = Box::new(image);

    let mut processor = processor;

    processor.process(config, &mut lenna_image).unwrap();
    let image = lenna_image.image.to_rgb8();
    let image = image.into_ndarray3();
    // dimension is here (channel, row, col)
    let mut image = image.reversed_axes();
    image.swap_axes(0, 1);
    // dimension is here (row, col, channel)
    image.to_owned().into_dimensionality::<_>().unwrap()
}

pub fn py_icon(processor: Box<dyn Processor>) -> Option<ndarray::ArrayD<u8>> {
    match processor.icon() {
        Some(icon) => {
            let img = image::load_from_memory(&icon).unwrap();
            let image = img.to_rgb8();
            let mut image = image.into_ndarray3().reversed_axes();
            image.swap_axes(0, 1);
            Some(image.to_owned().into_dimensionality::<_>().unwrap())
        }
        _ => None,
    }
}

#[macro_export]
macro_rules! export_python_plugin {
    ($processor:ident) => {
        #[pyfunction]
        pub fn default_config() -> PyResult<PyObject> {
            let processor = $processor::default();
            let gil = Python::acquire_gil();
            let py = gil.python();
            Ok(pythonize::pythonize(py, &processor.default_config()).unwrap())
        }

        #[pyfunction]
        pub fn id() -> String {
            let processor = $processor::default();
            processor.id()
        }

        #[pyfunction]
        pub fn name() -> String {
            let processor = $processor::default();
            processor.name()
        }

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

            #[pyfn(m)]
            #[pyo3(name = "process")]
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

            #[pyfn(m)]
            #[pyo3(name = "icon")]
            fn icon_py<'py>(py: Python<'py>) -> &'py numpy::PyArrayDyn<u8> {
                use numpy::IntoPyArray;
                $crate::plugins::python::py_icon(Box::new($processor::default()))
                    .unwrap()
                    .into_pyarray(py)
            }

            Ok(())
        }
    };
    ($processor:ty, $name:expr, $python:ident, $pymod:ident) => {
        paste::paste! {

            mod [< $name >] {
                use $crate::core::processor::Processor;
                use pyo3::prelude::*;

                #[pyfunction]
                pub fn default_config() -> PyResult<PyObject> {
                    let processor = $processor::default();
                    let gil = Python::acquire_gil();
                    let py = gil.python();
                    Ok(pythonize::pythonize(py, &processor.default_config()).unwrap())
                }

                #[pyfunction]
                pub fn id() -> String {
                    let processor = $processor::default();
                    processor.id()
                }

                #[pyfunction]
                pub fn name() -> String {
                    let processor = $processor::default();
                    processor.name()
                }

                #[pyfunction]
                pub fn description() -> String {
                    let processor = $processor::default();
                    processor.description()
                }

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

                #[pyfunction]
                fn icon_py<'py>(
                    py: Python<'py>,
                ) -> &'py numpy::PyArrayDyn<u8> {
                    use numpy::IntoPyArray;
                    $crate::plugins::python::py_icon(Box::new($processor::default()))
                    .unwrap()
                    .into_pyarray(py)
                }

                pub fn init_submodule(m: &PyModule) -> pyo3::PyResult<()> {
                    m.add_function(pyo3::wrap_pyfunction!(id, m)?)?;
                    m.add_function(pyo3::wrap_pyfunction!(name, m)?)?;
                    m.add_function(pyo3::wrap_pyfunction!(description, m)?)?;
                    m.add_function(pyo3::wrap_pyfunction!(default_config, m)?)?;
                    m.add("process", pyo3::wrap_pyfunction!(process_py, m)?)?;
                    m.add("icon", pyo3::wrap_pyfunction!(icon_py, m)?)?;
                    Ok(())
                }
            }

            let submod = pyo3::types::PyModule::new($python, $name)?;
            [< $name >]::init_submodule(submod)?;
            $pymod.add_submodule(submod)?;
        };
    };
}

#[cfg(feature = "resize-plugin")]
#[pymodule]
fn lenna_core(py: pyo3::Python, module: &pyo3::types::PyModule) -> pyo3::PyResult<()> {
    export_python_plugin!(crate::core::resize::Resize, "resize", py, module);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_py_icon() {
        use crate::core::resize::Resize;
        let processor = Box::new(Resize::default());
        assert_eq!(py_icon(processor).unwrap().shape(), &[976, 980, 3]);
    }
}
