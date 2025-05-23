use byteorder::{LittleEndian, WriteBytesExt};
use crc32fast::Hasher;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{self, BufReader, BufWriter, ErrorKind, Read, Result, Seek, SeekFrom, Write},
    path::Path,
};

type ByteString = Vec<u8>;
type Bytestr = [u8];

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
    fn process_record<R: Read>(f: &mut R) -> Result<KeyValuePair> {
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
    pub fn get(&mut self, key: &Bytestr) -> Result<Option<ByteString>> {
        let position = match self.index.get(key) {
            None => return Ok(None),
            Some(position) => *position,
        };
        let kv = self.get_at(position)?;
        Ok(Some(kv.value))
    }
    pub fn get_at(&mut self, position: u64) -> Result<KeyValuePair> {
        let mut f = BufReader::new(&mut self.f);
        f.seek(SeekFrom::Start(position))?;
        let kv = ActionKV::process_record(&mut f)?;
        Ok(kv)
    }
    pub fn find(&mut self, target: &Bytestr) -> Result<Option<(u64, ByteString)>> {
        let mut f = BufReader::new(&mut self.f);
        let mut found: Option<(u64, ByteString)> = None;
        loop {
            let position = f.seek(SeekFrom::Current(0))?;
            let maybe_kv = ActionKV::process_record(&mut f);
            let kv = match maybe_kv {
                Ok(kv) => kv,
                Err(err) => match err.kind() {
                    ErrorKind::UnexpectedEof => {
                        break;
                    }
                    _ => return Err(err),
                },
            };
            if kv.key == target {
                found = Some((position, kv.value))
            }
        }
        Ok(found)
    }
    pub fn insert_but_ignore_index(&mut self, key: &Bytestr, value: &Bytestr) -> io::Result<u64> {
        let mut f = BufWriter::new(&mut self.f);

        let key_len = key.len();
        let val_len = value.len();
        let mut tmp = ByteString::with_capacity(key_len + val_len);
        for byte in key {
            tmp.push(*byte);
        }
        for byte in value {
            tmp.push(*byte);
        }
        let mut hasher = Hasher::new();
        hasher.update(&tmp);
        let checksum = hasher.finalize();

        let next_byte = SeekFrom::End(0);
        let current_position = f.seek(SeekFrom::Current(0))?;
        f.seek(next_byte)?;
        f.write_u32::<LittleEndian>(checksum)?;
        f.write_u32::<LittleEndian>(key_len as u32)?;
        f.write_u32::<LittleEndian>(val_len as u32)?;
        f.write_all(&tmp)?;

        Ok(current_position)
    }
    pub fn insert(&mut self, key: &Bytestr, value: &Bytestr) -> Result<()> {
        let position = self.insert_but_ignore_index(key, value)?;
        self.index.insert(key.to_vec(), position);
        Ok(())
    }
    #[inline]
    pub fn delete(&mut self, key: &Bytestr) -> Result<()> {
        self.insert(key, b"")
    }
    #[inline]
    pub fn update(&mut self, key: &Bytestr, value: &Bytestr) -> Result<()> {
        self.insert(key, value)
    }
}
