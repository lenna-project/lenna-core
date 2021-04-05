#!/usr/bin/env python
# coding: utf-8

from setuptools import setup, find_namespace_packages
from setuptools_rust import Binding, RustExtension

from os import path
this_directory = path.abspath(path.dirname(__file__))
with open(path.join(this_directory, 'README.md'), encoding='utf-8') as f:
    long_description = f.read()

setup(
    name="lenna-core-py",
    version="0.1.0",
    long_description=long_description,
    long_description_content_type='text/markdown',
    install_requires=['numpy'],
    packages=["lenna_core_py"],
    rust_extensions=[RustExtension(
        "lenna_core_py.Resize",
        binding=Binding.PyO3, features=["python"])],
    zip_safe=False,
)
