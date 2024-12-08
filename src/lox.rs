use std::{error::Error, path::PathBuf};

use bytes::Bytes;

use crate::error::{MyErrImpl, MyResult};

pub struct Lox {}
impl Lox {
    pub async fn run_file(path: PathBuf) -> MyResult<Bytes> {
        let r = tokio::fs::read(path).await?;
        let b = bytes::Bytes::from(r);
        Ok(b)
    }
    fn run(path: PathBuf) -> MyResult<()> {
        Ok(())
    }
 
}
