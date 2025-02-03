use pyo3::prelude::*;
use std::fs;
use tokio::runtime::Runtime;

use std::error::Error;
use std::result::Result;
use std::sync::Arc;
use tiberius::{Query, QueryStream};

mod connections;
pub use connections::*;
mod schema_file;
pub use schema_file::*;
mod converter;
pub use converter::*;

pub struct Params {
    pub name_server: String,
    pub query: Option<String>,
    pub path_file: Option<std::path::PathBuf>,
    pub file_parquet: String,
    pub user: Option<String>,
    pub secret: Option<String>,
    pub parameters: Vec<String>,
}

pub async fn export_to_parquet(cli: Params) -> Result<(), Box<dyn Error>> {
    let mut query: String = String::new();

    if let Some(str_query) = cli.query {
        query = str_query;
    } else if let Some(file_query) = cli.path_file {
        query = fs::read_to_string(&file_query)?;
    };

    let schema_sql: Vec<MSchema> = schema_mssql_query(
        query.as_str(),
        cli.name_server.as_str(),
        cli.user.as_deref(),
        cli.secret.as_deref(),
    )
    .await?;
    let schema = create_schema_parquet(&schema_sql);

    let mut client = connect_server(
        cli.name_server.as_str(),
        cli.user.as_deref(),
        cli.secret.as_deref(),
    )
    .await?;

    let mut select: Query<'_> = Query::new(query);
    for param in cli.parameters {
        select.bind(param);
    }

    let stream: QueryStream<'_> = select.query(&mut client).await?;

    write_parquet_from_stream(
        stream,
        Arc::new(schema),
        &schema_sql,
        cli.file_parquet.as_str(),
    )
    .await?;

    Ok(())
}

/// Exporta os resultados de uma consulta SQL do SQL Server para um arquivo Parquet.
///
/// # Argumentos
/// - `name_server`: O nome ou IP do servidor SQL Server.
/// - `file_parquet`: O nome do arquivo Parquet de saída (padrão: "default.parquet").
/// - `query`: A consulta SQL a ser executada (opcional).
/// - `path_file`: Caminho para um arquivo contendo a consulta SQL (opcional).
/// - `user`: Nome de usuário para autenticação no SQL Server (opcional).
/// - `secret`: Senha para autenticação no SQL Server (opcional).
/// - `parameters`: Lista de parâmetros para a consulta SQL (opcional).
///
/// # Retorno
/// Esta função não retorna nenhum valor.
#[pyfunction]
#[pyo3(signature = (name_server, query=None, path_file=None, file_parquet="default.parquet", user=None, secret=None, parameters=None))]
fn py_export_to_parquet(
    name_server: String,
    query: Option<String>,
    path_file: Option<std::path::PathBuf>,
    file_parquet: &str,
    user: Option<String>,
    secret: Option<String>,
    parameters: Option<Vec<String>>,
) -> PyResult<()> {
    // Converter lista Python para Vec<String>
    let param_vec = parameters.unwrap_or_else(|| Vec::new());

    // Criar struct de parâmetros
    let params = Params {
        name_server,
        query,
        path_file,
        file_parquet: file_parquet.to_string(),
        user,
        secret,
        parameters: param_vec,
    };

    // Criar runtime assíncrona para rodar no Python
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        if let Err(e) = export_to_parquet(params).await {
            eprintln!("Erro ao exportar para Parquet: {}", e);
        }
    });

    Ok(())
}

/// Criar o módulo Python
#[pymodule]
fn rustmssql_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_export_to_parquet, m)?)?;
    Ok(())
}
