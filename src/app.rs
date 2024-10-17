use arrow::{array::RecordBatchReader, datatypes::SchemaRef};
use parquet::{
    arrow::arrow_reader::{
        ArrowReaderMetadata, ParquetRecordBatchReader, ParquetRecordBatchReaderBuilder,
    },
    file::metadata::ParquetMetaDataReader,
};

use crate::error::Result;
use std::{fs::File, sync::Arc};

pub struct Viewer {
    pub version: String,
    pub num_rows: i64,
    pub num_cols: usize,
    pub created_by: String,
    pub file_kv_data: Vec<(String, String)>,
    pub schema_table_data: Vec<[String; 9]>,
    pub max_col_name_width: usize,
    // parquet_metadata: Arc<ParquetMetaData>,
    reader: ParquetRecordBatchReader,
    pub file_stem: String,
}

impl Viewer {
    pub fn new(file: File, name: Option<String>) -> Result<Self> {
        let parquet_metadata = ParquetMetaDataReader::new().parse_and_finish(&file)?;
        let parquet_metadata = Arc::new(parquet_metadata);
        let version = parquet_metadata.file_metadata().version().to_string();
        let num_rows = parquet_metadata.file_metadata().num_rows();
        let num_cols = parquet_metadata
            .file_metadata()
            .schema_descr()
            .num_columns();
        let created_by = parquet_metadata
            .file_metadata()
            .created_by()
            .unwrap_or("")
            .to_string();

        let mut file_kv_data = vec![];
        if let Some(kv_data) = parquet_metadata.file_metadata().key_value_metadata() {
            for kv in kv_data.iter() {
                file_kv_data.push((kv.key.clone(), kv.value.clone().unwrap_or("".to_string())));
            }
        }
        let mut schema_table_data = vec![];
        let mut max_col_name_width = 0;
        for c in parquet_metadata
            .file_metadata()
            .schema_descr()
            .columns()
            .iter()
        {
            let name = c.name().to_string();
            max_col_name_width = max_col_name_width.max(name.len());
            let sort_order = c.sort_order().to_string();
            let row = match c.self_type() {
                parquet::schema::types::Type::PrimitiveType {
                    basic_info,
                    physical_type,
                    type_length,
                    scale,
                    precision,
                } => {
                    let ctype = basic_info.converted_type().to_string();
                    let ltype = basic_info
                        .logical_type()
                        .map_or("".to_string(), |t| format!("{t:?}"));
                    let ptype = physical_type.to_string();
                    [
                        name,
                        "primitive".to_string(),
                        ltype,
                        ctype,
                        ptype,
                        type_length.to_string(),
                        scale.to_string(),
                        precision.to_string(),
                        sort_order,
                    ]
                }
                parquet::schema::types::Type::GroupType { basic_info, .. } => {
                    let ctype = basic_info.converted_type().to_string();
                    let ltype = basic_info
                        .logical_type()
                        .map_or("".to_string(), |t| format!("{t:?}"));
                    [
                        name,
                        "group".to_string(),
                        ltype,
                        ctype,
                        "".to_string(),
                        "".to_string(),
                        "".to_string(),
                        "".to_string(),
                        sort_order,
                    ]
                }
            };
            schema_table_data.push(row);
        }

        let file_stem = name.unwrap_or("no name".to_string());
        let arrow_metadata =
            ArrowReaderMetadata::try_new(Arc::clone(&parquet_metadata), Default::default())?;
        // arrow_metadata.schema()
        // let metadata = ArrowReaderMetadata::load(&file, Default::default())?;
        // metadata.metadata
        // let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
        let builder = ParquetRecordBatchReaderBuilder::new_with_metadata(file, arrow_metadata);
        let reader = builder.build()?;
        Ok(Self {
            version,
            num_rows,
            num_cols,
            created_by,
            file_kv_data,
            schema_table_data,
            max_col_name_width,
            // parquet_metadata,
            reader,
            file_stem,
        })
    }

    pub fn arrow_schema(&self) -> SchemaRef {
        self.reader.schema()
    }

    // pub fn parquet_metadata(&self) -> Arc<ParquetMetaData> {
    //     self.parquet_metadata.clone()
    // }

    // pub fn file_metadata(&self) -> &FileMetaData {
    //     self.parquet_metadata.file_metadata()
    // }

    // pub fn parquet_schema(&self) -> &SchemaDescriptor {
    //     self.file_metadata().schema_descr()
    // }
}
