import rustmssql_python

def test_sum_as_string():
    rustmssql_python.py_export_to_parquet(
        name_server='teste',
        query='select 1',
        file_parquet='teste.parquet'
    )

test_sum_as_string()