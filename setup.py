from setuptools import setup
from setuptools_rust import RustExtension

setup(name="rust-implementation",
      version="1.0",
      rust_extensions=[RustExtension('optimized_rust_module._rustbridge', 'rust_extension/Cargo.toml')],
      packages=['optimized_rust_module'],
      zip_safe=False
      )

