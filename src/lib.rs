use byteorder::LittleEndian;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{self, BufReader, Read, Seek, SeekFrom},
    path::Path,
};

type ByteString = Vec<u8>;
type ByteStr = [u8];

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
        }
        Ok(())
    }
    fn process_record<R: Read>(f: &mut R) -> io::Result<KeyValuePair> {
        let mut buffer = [0; 4];
        f.read_exact(&mut buffer).expect("failed to read checksum");
        let saved_checksum = u32::from_le_bytes(buffer);
        f.read_exact(&mut buffer);
        let key_len = u32::from_le_bytes(buffer);
        f.read_exact(&mut buffer);
        let val_len = u32::from_le_bytes(buffer);
        let data_len = key_len + val_len;
        Ok(KeyValuePair {
            key: key,
            value: value,
        })
    }
}
