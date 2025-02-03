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

## Licença

Este projeto está licenciado sob a [MIT License](LICENSE).