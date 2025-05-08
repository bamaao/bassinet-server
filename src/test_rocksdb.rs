use rocksdb::{DB};

/**
 * 测试RocksDB
 */
fn main() {
    let db = DB::open_default("e:/data/bassinet").unwrap();
        let put_operation = db.put(b"k1", b"v1111");
        if put_operation.is_ok() {
            println!("插入{:?}:{:?}成功!", b"k1", b"v1111");
        }

        let r = db.get(b"k1");

        println!("{:?}", r.unwrap().unwrap());

        let delete_operation= db.delete(b"k1");
        if delete_operation.is_ok() {
            println!("删除数据{:?}成功!", b"k1");
        }

        if db.get(b"k1").unwrap().is_none() {
            println!("删除数据{:?}确认成功!", b"k1");
        }
}