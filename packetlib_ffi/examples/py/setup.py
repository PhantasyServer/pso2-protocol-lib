from setuptools import Extension, setup
from Cython.Build import cythonize

setup(
    ext_modules=cythonize(
        [Extension("packetlib", ["packetlib_py.pyx"],
                   libraries=["packetlib_ffi"],
                   library_dirs=["../../target/release/"])],
        include_path=["../../include/"],
    )
)
