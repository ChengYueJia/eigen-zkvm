use crate::bellman_ce::{Field, PrimeField, PrimeFieldRepr, ScalarEngine};
use std::io::{Error, ErrorKind, Read, Result};

pub fn read_field<R: Read, E: ScalarEngine>(mut reader: R) -> Result<E::Fr> {
    let mut repr = E::Fr::zero().into_repr();
    repr.read_le(&mut reader)?;
    let fr = E::Fr::from_repr(repr).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
    Ok(fr)
}

// TODO: why does the `read_to_end` not work?
pub fn read_to_string<R: Read>(mut reader: R) -> String {
    let mut name_buf = vec![1u8; 1];
    let mut buf = vec![];
    loop {
        let name_size_res = reader.read_exact(&mut name_buf);
        if name_buf[0] != 0 {
            buf.push(name_buf[0]);
        } else {
            break;
        }
    }
    String::from_utf8_lossy(&buf).to_string()
}
