from typing import Literal


def py_export_to_parquet(
    name_server: str, 
    query: str = None, 
    path_file: str = None, 
    file_parquet: str ="default.parquet", 
    user: str = None, 
    secret: str = None, 
    parameters: list[str] = None,
    compression: Literal["UNCOMPRESSED", "SNAPPY", "GZIP", "BROTLI", "LZ4", "LZO", "LZ4_RAW", "ZSTD"] = "ZSTD"
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
    - `compression` (Literal["UNCOMPRESSED", "SNAPPY", "GZIP", "BROTLI", "LZ4", "LZO", "LZ4_RAW", "ZSTD"]): Tipo de compressão para o arquivo Parquet (padrão: "ZSTD").
    
    ### Retorno
    
    Esta função não retorna nenhum valor.

    """
    ...