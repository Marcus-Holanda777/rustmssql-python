use anyhow::Ok;
use tiberius::{AuthMethod, Client, Config};
use tiberius::{Query, QueryItem, QueryStream};
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::compat::Compat;
use tokio_util::compat::TokioAsyncWriteCompatExt;

#[derive(Debug)]
pub struct MSchema {
    pub column_name: Option<String>,
    pub data_type: Option<String>,
    pub is_nullable: Option<String>,
    pub numeric_precision: Option<u8>,
    pub numeric_scale: Option<u8>,
    pub datetime_precision: Option<u8>,
}

pub async fn connect_server(
    server: &str,
    user: Option<&str>,
    password: Option<&str>,
) -> anyhow::Result<Client<Compat<TcpStream>>> {
    //! Conecta ao servidor SQL Server.
    //! Retorna um cliente para realizar consultas.

    let mut config: Config = Config::new();
    config.host(server);
    config.port(1433);

    if let (Some(user), Some(password)) = (user, password) {
        config.authentication(AuthMethod::sql_server(user, password));
    } else {
        #[cfg(windows)]
        config.authentication(AuthMethod::Integrated);

        #[cfg(not(windows))]
        panic!("Autenticacao integrada somente no windows !");
    }
    config.trust_cert();

    let tcp_stream: TcpStream = TcpStream::connect(config.get_addr()).await?;
    tcp_stream.set_nodelay(true)?;

    let client: Client<Compat<TcpStream>> =
        Client::connect(config, tcp_stream.compat_write()).await?;

    Ok(client)
}

pub async fn schema_mssql(
    database: &str,
    table_name: &str,
    server: &str,
    user: Option<&str>,
    password: Option<&str>,
) -> anyhow::Result<Vec<MSchema>> {
    //! Retorna os metadados de uma tabela do banco.
    //! Utiliza a tabela `INFORMATION_SCHEMA.columns` para obter os metadados.

    let mut schema: Vec<MSchema> = Vec::new();
    let mut client: Client<Compat<TcpStream>> = connect_server(server, user, password).await?;

    let sql: String = format!(
        r#"
        select
             column_name
            ,data_type
            ,is_nullable
            ,cast(numeric_precision as tinyint)  as numeric_precision
            ,cast(numeric_scale as tinyint)      as numeric_scale
            ,cast(datetime_precision as tinyint) as datetime_precision
        from {}.INFORMATION_SCHEMA.columns
        where TABLE_NAME = '{}'
       "#,
        database, table_name
    );

    let select: Query<'_> = Query::new(sql);
    let mut stream: QueryStream<'_> = select.query(&mut client).await?;

    while let Some(row) = stream.try_next().await? {
        if let QueryItem::Row(r) = row {
            let ms_schema: MSchema = MSchema {
                column_name: r.get(0).map(|f: &str| f.to_string()),
                data_type: r.get(1).map(|f: &str| f.to_string()),
                is_nullable: r.get(2).map(|f: &str| f.to_string()),
                numeric_precision: r.get(3),
                numeric_scale: r.get(4),
                datetime_precision: r.get(5),
            };
            schema.push(ms_schema);
        }
    }

    Ok(schema)
}

pub async fn schema_mssql_query(
    query: &str,
    server: &str,
    user: Option<&str>,
    password: Option<&str>,
) -> anyhow::Result<Vec<MSchema>> {
    //! Retorna os metadados da consulta,
    //! como nome da coluna, tipo de dado, se é nulo,
    //! precisão numérica, escala numérica e precisão de data e hora.
    //! Utiliza a `procedure sp_describe_first_result_set` para obter os metadados.

    let mut schema: Vec<MSchema> = Vec::new();
    let mut client: Client<Compat<TcpStream>> = connect_server(server, user, password).await?;

    let sql: String = format!(
        r#"
        EXEC sp_describe_first_result_set @tsql = N'{}'
       "#,
        query.replace("'", "''") // scape
    );

    let select: Query<'_> = Query::new(sql);
    let mut stream: QueryStream<'_> = select.query(&mut client).await?;

    while let Some(row) = stream.try_next().await? {
        if let QueryItem::Row(r) = row {
            let is_nullable: &str = if r.get::<bool, _>(3).unwrap() {
                "YES"
            } else {
                "NO"
            };

            let ms_schema: MSchema = MSchema {
                column_name: r.get(2).map(|f: &str| f.to_string()),
                data_type: r.get(5).map(|f: &str| f.to_string()),
                is_nullable: Some(is_nullable.to_string()),
                numeric_precision: r.get(7),
                numeric_scale: r.get(8),
                datetime_precision: r.get(8),
            };
            schema.push(ms_schema);
        }
    }

    Ok(schema)
}
