use std::path::Path;
use tokio::fs;
use crate::{config::StorageConfig,error::AppError};

pub fn sanitize_export_filename(filename:&str)->Option<String>{if filename.contains("..")||filename.contains('/')||filename.contains('\\'){None}else if filename.ends_with(".csv"){Some(filename.to_string())}else{None}}
pub async fn store_local_export(config:&StorageConfig,filename:&str,content:&[u8])->Result<String,AppError>{let safe=sanitize_export_filename(filename).ok_or_else(||AppError::BadRequest("invalid filename".into()))?;fs::create_dir_all(&config.local_exports_dir).await.map_err(|_|AppError::Internal)?;let path=Path::new(&config.local_exports_dir).join(&safe);fs::write(path,content).await.map_err(|_|AppError::Internal)?;Ok(format!("/api/v1/exports/{safe}"))}
