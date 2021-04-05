use crate::core::processor::Processor;
use crate::core::resize::Resize;

#[macro_export]
macro_rules! export_python_plugin {
    ($processor:ident) => {
        use pyo3::prelude::*;
        use pythonize::{depythonize, pythonize};

        #[doc(hidden)]
        #[pyfunction]
        pub fn default_config() -> PyResult<PyObject> {
            let processor = $processor::default();
            let gil = Python::acquire_gil();
            let py = gil.python();
            Ok(pythonize(py, &processor.default_config()).unwrap())
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

        use image::DynamicImage;
        use ndarray::{ArrayD, ArrayViewD, Ix3};
        use ndarray_image;
        use ndarray_image::{ImgRgb, NdColor, NdImage};
        use numpy::{IntoPyArray, PyArrayDyn, PyReadonlyArrayDyn};

        pub fn process(config: serde_json::Value, data: ArrayViewD<'_, u8>) -> ArrayD<u8> {
            let image: NdColor<'_, u8> = data.into_dimensionality::<Ix3>().unwrap();

            let image: Option<ImgRgb> = NdImage(image.view()).into();
            let image = image.unwrap().to_owned();

            let config: $crate::core::config::ProcessorConfig =
                $crate::core::config::ProcessorConfig {
                    id: id(),
                    config: config,
                };
            let s: &[u8] = image.as_raw();
            let image = image::ImageBuffer::from_raw(
                image.width() as u32,
                image.height() as u32,
                Vec::from(s),
            )
            .unwrap();
            let image = DynamicImage::ImageRgb8(image);

            let image = $processor::default().process(config, image);
            let image = image.to_rgb8();
            let image: NdColor = NdImage(&image).into();

            image.to_owned().into_dimensionality::<_>().unwrap()
        }

        use pyo3::wrap_pyfunction;

        #[allow(non_snake_case)]
        #[pymodule]
        fn $processor(_py: Python, m: &PyModule) -> PyResult<()> {
            m.add_function(wrap_pyfunction!(id, m)?)?;
            m.add_function(wrap_pyfunction!(name, m)?)?;
            m.add_function(wrap_pyfunction!(description, m)?)?;
            m.add_function(wrap_pyfunction!(default_config, m)?)?;

            #[pyfn(m, "process")]
            fn process_py<'py>(
                py: Python<'py>,
                config: PyObject,
                data: PyReadonlyArrayDyn<u8>,
            ) -> &'py PyArrayDyn<u8> {
                let config = depythonize(config.as_ref(py)).unwrap();
                process(config, data.as_array()).into_pyarray(py)
            }
            Ok(())
        }
    };
}

export_python_plugin!(Resize);
