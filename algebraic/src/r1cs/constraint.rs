use crate::bellman_ce::ScalarEngine;
use crate::r1cs::header::Header;
use crate::r1cs::utils::read_field;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Read, Result};

pub type Constraint<E> = (
    Vec<(usize, <E as ScalarEngine>::Fr)>,
    Vec<(usize, <E as ScalarEngine>::Fr)>,
    Vec<(usize, <E as ScalarEngine>::Fr)>,
);

pub fn read_constraint_vec<R: Read, E: ScalarEngine>(
    mut reader: R,
    header: &Header,
) -> Result<Vec<(usize, E::Fr)>> {
    let n_vec = reader.read_u32::<LittleEndian>()? as usize;
    let mut vec = Vec::with_capacity(n_vec);
    for _ in 0..n_vec {
        vec.push((
            reader.read_u32::<LittleEndian>()? as usize,
            read_field::<&mut R, E>(&mut reader)?,
        ));
        // sort by key
        vec.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    }
    Ok(vec)
}

pub(crate) fn read_constraints<R: Read, E: ScalarEngine>(
    mut reader: R,
    size: u64,
    header: &Header,
) -> Result<Vec<Constraint<E>>> {
    // todo check section size
    let mut vec = Vec::with_capacity(header.n_constraints as usize);
    for _ in 0..header.n_constraints {
        vec.push((
            read_constraint_vec::<&mut R, E>(&mut reader, header)?,
            read_constraint_vec::<&mut R, E>(&mut reader, header)?,
            read_constraint_vec::<&mut R, E>(&mut reader, header)?,
        ));
    }
    Ok(vec)
}
