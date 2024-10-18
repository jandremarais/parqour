use arrow::{
    array::RecordBatch,
    util::display::{ArrayFormatter, FormatOptions},
};
use parquet::{
    arrow::arrow_reader::{
        ArrowReaderMetadata, ParquetRecordBatchReader, ParquetRecordBatchReaderBuilder,
    },
    file::metadata::{ParquetMetaDataReader, RowGroupMetaData},
};
use ratatui::widgets::Row;

use crate::error::Result;
use std::{fs::File, sync::Arc};

pub struct Viewer {
    pub version: String,
    pub num_rows: i64,
    pub num_cols: usize,
    pub num_row_groups: usize,
    pub created_by: String,
    pub file_kv_data: Vec<(String, String)>,
    pub schema_table_data: Vec<[String; 9]>,
    pub max_col_name_width: usize,
    // parquet_metadata: Arc<ParquetMetaData>,
    pub file_stem: String,
    pub row_groups: Vec<RowGroupMetaData>,
    pub reader: ParquetRecordBatchReader,
    pub batch: RecordBatch,
    // pub batch_table:
    pub batch_size: usize,
    pub selected_row: usize,
    pub selected_col: usize,
    pub row_offset: usize,
    pub visible_rows: usize,
    pub col_offset: usize,
    pub visible_cols: usize,
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
        let num_row_groups = parquet_metadata.num_row_groups();
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

        let batch_size = 64;
        let arrow_metadata =
            ArrowReaderMetadata::try_new(Arc::clone(&parquet_metadata), Default::default())?;
        let builder = ParquetRecordBatchReaderBuilder::new_with_metadata(file, arrow_metadata)
            .with_batch_size(batch_size);
        let mut reader = builder.build()?;

        // TODO: consider keeping only the necessary data
        let row_groups = parquet_metadata.row_groups().to_vec();

        let batch = reader.next().unwrap()?;

        Ok(Self {
            version,
            num_rows,
            num_cols,
            num_row_groups,
            created_by,
            file_kv_data,
            schema_table_data,
            max_col_name_width,
            row_groups,
            file_stem,
            reader,
            batch,
            batch_size,
            selected_row: 0,
            selected_col: 0,
            row_offset: 0,
            visible_rows: 50,
            col_offset: 0,
            visible_cols: 10,
        })
    }
}
