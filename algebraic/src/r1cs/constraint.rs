use crate::bellman_ce::ScalarEngine;
use crate::r1cs::utils::read_field;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Read, Result};

pub type Constraint<E> = (
    Vec<(usize, <E as ScalarEngine>::Fr)>,
    Vec<(usize, <E as ScalarEngine>::Fr)>,
    Vec<(usize, <E as ScalarEngine>::Fr)>,
);

pub(crate) fn read_constraint_vec<R: Read, E: ScalarEngine>(
    mut reader: R,
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
    size: u32,
) -> Result<Vec<Constraint<E>>> {
    // todo check section size
    let mut vec = Vec::with_capacity(size as usize);
    for _ in 0..size {
        vec.push((
            read_constraint_vec::<&mut R, E>(&mut reader)?,
            read_constraint_vec::<&mut R, E>(&mut reader)?,
            read_constraint_vec::<&mut R, E>(&mut reader)?,
        ));
    }
    Ok(vec)
}
