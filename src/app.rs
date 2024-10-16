use arrow::{array::RecordBatchReader, datatypes::SchemaRef};
use parquet::{
    arrow::arrow_reader::{
        ArrowReaderMetadata, ParquetRecordBatchReader, ParquetRecordBatchReaderBuilder,
    },
    file::metadata::{FileMetaData, ParquetMetaData, ParquetMetaDataReader},
    schema::types::SchemaDescriptor,
};

use crate::error::Result;
use std::{fs::File, sync::Arc};

pub struct Viewer {
    parquet_metadata: Arc<ParquetMetaData>,
    reader: ParquetRecordBatchReader,
    file_stem: Option<String>,
}

impl Viewer {
    pub fn new(file: File, name: Option<String>) -> Result<Self> {
        let parquet_metadata = ParquetMetaDataReader::new().parse_and_finish(&file)?;
        let parquet_metadata = Arc::new(parquet_metadata);
        // let schema = metadata.file_metadata().schema_descr();
        // dbg!(schema);

        let arrow_metadata =
            ArrowReaderMetadata::try_new(Arc::clone(&parquet_metadata), Default::default())?;
        // arrow_metadata.schema()
        // let metadata = ArrowReaderMetadata::load(&file, Default::default())?;
        // metadata.metadata
        // let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
        let builder = ParquetRecordBatchReaderBuilder::new_with_metadata(file, arrow_metadata);
        let reader = builder.build()?;
        // rdr.schema();
        // let schema = builder.schema();
        Ok(Self {
            parquet_metadata,
            reader,
            file_stem: name,
        })
    }

    pub fn arrow_schema(&self) -> SchemaRef {
        self.reader.schema()
    }

    pub fn parquet_metadata(&self) -> Arc<ParquetMetaData> {
        self.parquet_metadata.clone()
    }

    pub fn file_metadata(&self) -> &FileMetaData {
        self.parquet_metadata.file_metadata()
    }

    pub fn parquet_schema(&self) -> &SchemaDescriptor {
        self.file_metadata().schema_descr()
    }

    pub fn filename(&self) -> &str {
        if let Some(name) = &self.file_stem {
            name.as_str()
        } else {
            "no name"
        }
    }
}
