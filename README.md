# Rustmssql-Python 🐍🦀

**Rustmssql-Python** é um pacote Python desenvolvido em Rust que permite exportar os resultados de consultas SQL do SQL Server diretamente para arquivos Parquet de forma eficiente.

## Instalação

Para instalar o pacote, utilize:

```sh
pip install rustmssql-python
```

## Uso

### Exemplo de uso

#### Com autenticação integrada (Windows)
```python
import rustmssql_python

rustmssql_python.py_export_to_parquet(
    name_server="meu_servidor",
    query="SELECT * FROM minha_tabela",
    file_parquet="saida.parquet"
)
```

#### Com usuário e senha
```python
import rustmssql_python

rustmssql_python.py_export_to_parquet(
    name_server="meu_servidor",
    query="SELECT * FROM minha_tabela",
    file_parquet="saida.parquet",
    user="meu_usuario",
    secret="minha_senha"
)
```

#### Usando um arquivo `.sql`
```python
import rustmssql_python

rustmssql_python.py_export_to_parquet(
    name_server="meu_servidor",
    path_file="consulta.sql",
    file_parquet="saida.parquet"
)
```

## Função principal

### `py_export_to_parquet`

```python
def py_export_to_parquet(
    name_server: str, 
    query: str = None, 
    path_file: str = None, 
    file_parquet: str ="default.parquet", 
    user: str = None, 
    secret: str = None, 
    parameters: list[str] = None
) -> None:
    """
    ## Exporta os resultados de uma consulta SQL do SQL Server para um arquivo Parquet.
    
    ### Argumentos

    - `name_server` (str): O nome ou IP do servidor SQL Server.
    - `file_parquet` (str): O nome do arquivo Parquet de saída (padrão: "default.parquet").
    - `query` (str): A consulta SQL a ser executada (opcional).
    - `path_file` (str): Caminho para um arquivo contendo a consulta SQL (opcional).
    - `user` (str): Nome de usuário para autenticação no SQL Server (opcional).
    - `secret` (str): Senha para autenticação no SQL Server (opcional).
    - `parameters` (list[str]): Lista de parâmetros para a consulta SQL (opcional).
    
    ### Retorno
    
    Esta função não retorna nenhum valor.
    """
```
## Correspondência entre os tipos

| SQL Server Type       | Parquet Type           | Logical Type               |
|-----------------------|------------------------|----------------------------|
| `INT`                 | `INT32`                |                            |
| `BIGINT`              | `INT64`                |                            |
| `SMALLINT`            | `INT32`                | `Int 16 bit width signed`  |
| `TINYINT`             | `INT32`                | `Int 8 bit width unsigned` | 
| `BIT`                 | `BOOLEAN`              |                            |
| `FLOAT`               | `DOUBLE`               |                            |
| `REAL`                | `FLOAT`                |                            |
| `DECIMAL`             | `FIXED_LEN_BYTE_ARRAY` | `Decimal precision, scale` |  
| `NUMERIC`             | `FIXED_LEN_BYTE_ARRAY` | `Decimal precision, scale` |
| `CHAR`                | `BYTE_ARRAY`           | `String`                   |
| `VARCHAR`             | `BYTE_ARRAY`           | `String`                   |
| `NCHAR`               | `BYTE_ARRAY`           | `String`                   |
| `NVARCHAR`            | `BYTE_ARRAY`           | `String`                   |
| `TEXT`                | `BYTE_ARRAY`           | `String`                   |
| `NTEXT`               | `BYTE_ARRAY`           | `String`                   |
| `XML`                 | `BYTE_ARRAY`           | `String`                   |
| `BINARY`              | `BYTE_ARRAY`           |                            |
| `VARBINARY`           | `BYTE_ARRAY`           |                            |
| `IMAGE`               | `BYTE_ARRAY`           |                            |
| `TIMESTAMP`           | `BYTE_ARRAY`           |                            |
| `DATE`                | `INT32`                | `Date`                     |
| `DATETIME`            | `INT64`                | `Timestamp`                |
| `DATETIME2`           | `INT64`                | `Timestamp`                |
| `TIME`                | `INT64`                | `Time`                     |

## Licença

Este projeto está licenciado sob a [MIT License](LICENSE).