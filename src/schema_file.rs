use crate::MSchema;
use crate::converter::{Converter, parse_rows};
use parquet::basic::{Compression, ZstdLevel};
use parquet::file::{properties::WriterProperties, writer::SerializedFileWriter};
use parquet::format::NanoSeconds;
use parquet::{
    basic::{LogicalType, Repetition, TimeUnit, Type as PhysicalType},
    data_type::{ByteArray, FixedLenByteArray},
    format::{MicroSeconds, MilliSeconds},
    schema::types::Type,
};
use std::collections::HashMap;
use std::io::Write;
use std::sync::Arc;
use std::{fs, path::Path};
use tiberius::{ColumnData, QueryItem, QueryStream};
use tokio_stream::StreamExt;

const MAX_GROUP_SIZE: i32 = 100_000;

fn get_type(col: &str, types: PhysicalType, logical: Option<LogicalType>) -> Type {
    //! Retorna um tipo de dado para o parquet.

    Type::primitive_type_builder(&col, types)
        .with_logical_type(logical)
        .with_repetition(Repetition::OPTIONAL)
        .build()
        .unwrap()
}

fn to_type_column(schema: &MSchema) -> Type {
    //! Converte um MSchema para um Type.
    //! Verifica o tipo de dado e retorna um Type.
    //! Se o tipo não for reconhecido, retorna um BYTE_ARRAY.

    let col = schema
        .column_name
        .as_ref()
        .unwrap()
        .trim()
        .to_lowercase()
        .split_whitespace()
        .map(|f| f.trim())
        .collect::<Vec<_>>()
        .join("_");

    // converter para o tipo Option<&str> e depos para &str
    let mut opt = schema.data_type.as_deref().unwrap();

    if let Some(indice) = opt.find("(") {
        opt = &opt[..indice];
    }

    let scale = schema.numeric_scale.unwrap_or(0) as i32;
    let precision = schema.numeric_precision.unwrap_or(0) as i32;

    // definir a precisao do tempo
    let datetime_precision = match schema.datetime_precision.unwrap_or(0) {
        0..=3 => TimeUnit::MILLIS(MilliSeconds {}),
        4..=6 => TimeUnit::MICROS(MicroSeconds {}),
        7.. => TimeUnit::NANOS(NanoSeconds {}),
    };

    let num_binary_digits = precision as f64 * 10f64.log2();
    // Plus one bit for the sign (+/-)
    let length_in_bits = num_binary_digits + 1.0;
    let length_in_bytes = (length_in_bits / 8.0).ceil() as usize;

    match opt {
        "tinyint" => get_type(
            &col,
            PhysicalType::INT32,
            Some(LogicalType::Integer {
                bit_width: 8,
                is_signed: false,
            }),
        ),
        "smallint" => get_type(
            &col,
            PhysicalType::INT32,
            Some(LogicalType::Integer {
                bit_width: 16,
                is_signed: true,
            }),
        ),
        "int" => get_type(&col, PhysicalType::INT32, None),
        "bigint" => get_type(&col, PhysicalType::INT64, None),
        "float" => get_type(&col, PhysicalType::DOUBLE, None),
        "real" => get_type(&col, PhysicalType::FLOAT, None),
        "decimal" | "numeric" => {
            Type::primitive_type_builder(&col, PhysicalType::FIXED_LEN_BYTE_ARRAY)
                .with_length(length_in_bytes.try_into().unwrap())
                .with_logical_type(Some(LogicalType::Decimal { scale, precision }))
                .with_precision(precision)
                .with_scale(scale)
                .with_repetition(Repetition::OPTIONAL)
                .build()
                .unwrap()
        }
        "bit" => get_type(&col, PhysicalType::BOOLEAN, None),
        "char" | "varchar" | "text" | "nchar" | "nvarchar" | "ntext" | "xml" => {
            get_type(&col, PhysicalType::BYTE_ARRAY, Some(LogicalType::String))
        }
        "datetime" | "datetime2" | "smalldatetime" => get_type(
            &col,
            PhysicalType::INT64,
            Some(LogicalType::Timestamp {
                is_adjusted_to_u_t_c: false,
                unit: datetime_precision,
            }),
        ),
        "date" => get_type(&col, PhysicalType::INT32, Some(LogicalType::Date)),
        "time" => get_type(
            &col,
            PhysicalType::INT64,
            Some(LogicalType::Time {
                is_adjusted_to_u_t_c: false,
                unit: datetime_precision,
            }),
        ),
        "binary" | "varbinary" | "image" | "timestamp" => {
            get_type(&col, PhysicalType::BYTE_ARRAY, None)
        }
        _ => get_type(&col, PhysicalType::BYTE_ARRAY, Some(LogicalType::String)),
    }
}

pub fn create_schema_parquet(sql_types: &Vec<MSchema>) -> Type {
    //! Cria um schema parquet a partir de um MSchema.
    //! Recebe um MSchema e retorna um Type.
    //! O Type é um schema parquet.

    let mut fields = vec![];

    for mssql in sql_types {
        let data = to_type_column(mssql);
        let tp = Arc::new(data);

        fields.push(tp);
    }

    Type::group_type_builder("schema_mvsh")
        .with_fields(fields)
        .build()
        .unwrap()
}

async fn process_rows<W: Write>(
    schema_sql: &Vec<MSchema>,
    data: &mut HashMap<usize, Vec<ColumnData<'_>>>,
    writer: &mut SerializedFileWriter<W>,
) -> anyhow::Result<()>
where
    W: Send,
{
    let mut col_key: usize = 0;
    let mut row_group_writer = writer.next_row_group()?;
    while let Some(col_write) = row_group_writer.next_column()? {
        let col_data = data.get(&col_key).unwrap();
        let mssql = schema_sql.get(col_key).unwrap();

        let conv = Some(Converter {
            col_data,
            col_write: Some(col_write),
            mssql: Some(mssql),
        });

        match &col_data[0] {
            ColumnData::I32(_) => parse_rows::<i32>(conv)?,
            ColumnData::String(_) => parse_rows::<ByteArray>(conv)?,
            ColumnData::Binary(_) => parse_rows::<ByteArray>(conv)?,
            ColumnData::U8(_) => parse_rows::<i32>(conv)?,
            ColumnData::I16(_) => parse_rows::<i32>(conv)?,
            ColumnData::I64(_) => parse_rows::<i64>(conv)?,
            ColumnData::F32(_) => parse_rows::<f32>(conv)?,
            ColumnData::F64(_) => parse_rows::<f64>(conv)?,
            ColumnData::Numeric(_) => parse_rows::<FixedLenByteArray>(conv)?,
            ColumnData::Bit(_) => parse_rows::<bool>(conv)?,
            ColumnData::DateTime(_) => parse_rows::<i64>(conv)?,
            ColumnData::DateTime2(_) => parse_rows::<i64>(conv)?,
            ColumnData::Time(_) => parse_rows::<i64>(conv)?,
            ColumnData::Date(_) => parse_rows::<i32>(conv)?,
            ColumnData::Xml(_) => parse_rows::<ByteArray>(conv)?,
            _ => {
                println!("Tipo de dado não reconhecido, {:?}", col_data[0]);
                unimplemented!()
            }
        };
        col_key += 1;
    }

    data.clear();
    row_group_writer.close()?;

    Ok(())
}

pub async fn write_parquet_from_stream(
    mut stream: QueryStream<'_>,
    schema: Arc<Type>,
    schema_sql: &Vec<MSchema>,
    path: &str,
) -> anyhow::Result<()> {
    //! Escreve um arquivo parquet a partir de um QueryStream.
    //! Recebe um QueryStream, um Arc<Type> e um &str.
    //! O Arc<Type> é o schema parquet.
    //! O &str é o caminho do arquivo parquet.
    //! Retorna um Result<()>.

    let path_new = Path::new(path);
    let file = fs::File::create(&path_new).unwrap();

    let props = WriterProperties::builder()
        .set_compression(Compression::ZSTD(ZstdLevel::try_new(1)?))
        .build()
        .into();

    let mut writer = SerializedFileWriter::new(file, schema, props)?;
    let mut data: HashMap<usize, Vec<ColumnData>> = HashMap::new();

    // armazena os dados
    let mut rows_batch: i32 = 1;

    while let Some(row) = stream.try_next().await? {
        if let QueryItem::Row(r) = row {
            for (p, col_data) in r.into_iter().enumerate() {
                data.entry(p).or_insert_with(Vec::new).push(col_data);
            }

            if rows_batch % MAX_GROUP_SIZE == 0 {
                process_rows(&schema_sql, &mut data, &mut writer).await?;
            }
            rows_batch += 1;
        }
    }

    if !data.is_empty() {
        process_rows(&schema_sql, &mut data, &mut writer).await?;
    }

    writer.close()?;

    Ok(())
}
