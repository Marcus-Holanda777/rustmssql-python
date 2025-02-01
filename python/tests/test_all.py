import pytest
import rustmssql_python

def test_sum_as_string():
    assert rustmssql_python.py_export_to_parquet()