use std::{fs::File, io::{Write}};
use anyhow::{anyhow};

use futures::StreamExt;
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};

/**
 * 向ipfs添加文件，返回ipfs文件名
 */
pub async fn add_file(file_path: &str) -> anyhow::Result<String>{
    let client = IpfsClient::default();
    let file = File::open(file_path).expect("could not read source file!");

    match client.add(file).await {
        Ok(file) => {
            eprintln!("added file: {:?}", file);
            return Ok(file.name);
        },
        Err(e) => {
            eprintln!("error adding file: {}", e);
            return Err(anyhow!("error adding file: {}", e));
        }
    }
}

/**
 * 从ipfs下载文件
 */
pub async fn get_file(qm_hash: &str, file_path: &str) -> anyhow::Result<()> {
    let client = IpfsClient::default();
    let mut file = File::options().create_new(true).append(true).open(file_path).expect("can not open file");

    let mut ipfs_stream = client.get(qm_hash);
    while let Some(value) = ipfs_stream.next().await {
        file.write_all(&value?)?;
    }
    anyhow::Ok(())
} 

#[cfg(test)]
mod tests {
    use crate::infrastructure::ipfs::add_file;

    use super::get_file;

    /**
     * added file: AddResponse { name: "QmbxQJkxyJGbLnDqpHUJRYzgVY6g1yXp3RgpaLzBpHwdAL", hash: "QmbxQJkxyJGbLnDqpHUJRYzgVY6g1yXp3RgpaLzBpHwdAL", size: "567727" }
     * "QmbxQJkxyJGbLnDqpHUJRYzgVY6g1yXp3RgpaLzBpHwdAL"
     */
    #[test]
    fn test_add_file() {
        let result = tokio_test::block_on(add_file("d:/raft.pdf"));
        assert_eq!(result.is_ok(), true);
    }

    /**
     * get_file("/ipfs/QmbxQJkxyJGbLnDqpHUJRYzgVY6g1yXp3RgpaLzBpHwdAL", "d:/QmbxQJkxyJGbLnDqpHUJRYzgVY6g1yXp3RgpaLzBpHwdAL.pdf")也支持
     */
    #[test]
    fn test_get_file() {
        let result = tokio_test::block_on(get_file("QmbxQJkxyJGbLnDqpHUJRYzgVY6g1yXp3RgpaLzBpHwdAL", "d:/QmbxQJkxyJGbLnDqpHUJRYzgVY6g1yXp3RgpaLzBpHwdAL.pdf"));
        assert_eq!(result.is_ok(), true);
    }
}
