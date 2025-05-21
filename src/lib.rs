use crc32fast::Hasher;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{self, BufReader, Read, Seek, SeekFrom},
    path::Path,
};

type ByteString = Vec<u8>;

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValuePair {
    pub key: ByteString,
    pub value: ByteString,
}

#[derive(Debug)]
pub struct ActionKV {
    f: File,
    pub index: HashMap<ByteString, u64>,
}

impl ActionKV {
    pub fn open(path: &Path) -> io::Result<Self> {
        let f = OpenOptions::new()
            .read(true)
            .create(true)
            .append(true)
            .open(path)?;
        let index = HashMap::new();
        Ok(ActionKV { f: f, index: index })
    }
    pub fn load(&mut self) -> io::Result<()> {
        let mut f = BufReader::new(&mut self.f);
        loop {
            let position = f.seek(SeekFrom::Current(0))?;
            let maybe_kv = ActionKV::process_record(&mut f);

            let kv = match maybe_kv {
                Ok(kv) => kv,
                Err(err) => match err.kind() {
                    io::ErrorKind::UnexpectedEof => {
                        break;
                    }
                    _ => return Err(err),
                },
            };
            self.index.insert(kv.key, position);
        }
        Ok(())
    }
    fn process_record<R: Read>(f: &mut R) -> io::Result<KeyValuePair> {
        let mut buffer = [0; 4];
        f.read_exact(&mut buffer).expect("failed to read check_sum");
        let saved_checksum = u32::from_le_bytes(buffer);
        f.read_exact(&mut buffer)
            .expect("failed to read key length");
        let key_len = u32::from_le_bytes(buffer);
        f.read_exact(&mut buffer)
            .expect("failed to read val length");
        let val_len = u32::from_le_bytes(buffer);
        let data_len = key_len + val_len;

        let mut data = ByteString::with_capacity(data_len as usize);
        {
            f.by_ref().take(data_len as u64).read_to_end(&mut data)?;
        }

        debug_assert_eq!(data.len(), data_len as usize);

        let mut hasher = Hasher::new();
        hasher.update(&data);
        let checksum = hasher.finalize();
        if checksum != saved_checksum {
            panic!(
                "data corruption encountered ({:08x} != {:08x})",
                checksum, saved_checksum
            );
        }
        let value = data.split_off(key_len as usize);
        let key = data;

        Ok(KeyValuePair {
            key: key,
            value: value,
        })
    }
}
