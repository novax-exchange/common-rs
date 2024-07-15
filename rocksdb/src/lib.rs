use rocksdb::{TransactionDB, Options, TransactionDBOptions, DBCompactionStyle};

pub fn setup_rocks_db(use_fsync:bool, create_if_missing:bool) -> TransactionDB {
	let path = "./rocksdb";
	let mut opts = Options::default();
    opts.set_use_fsync(use_fsync);
	opts.create_if_missing(create_if_missing);
	let mut txn_db_opts = TransactionDBOptions::default();
	return TransactionDB::open(&opts, &txn_db_opts, path).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
