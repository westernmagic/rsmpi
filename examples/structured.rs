#![deny(warnings)]
extern crate mpi;

use mpi::{datatype::UserDatatype, traits::*};
use std::mem::size_of;

struct MyInts([i32; 3]);

unsafe impl Equivalence for MyInts {
    type Out = UserDatatype;
    fn equivalent_datatype() -> Self::Out {
        let int_datatype = i32::equivalent_datatype();
        mpi::datatype::UserDatatype::structured(
            &[1, 1, 1],
            &[(size_of::<i32>() * 2) as i64, size_of::<i32>() as i64, 0],
            &[&int_datatype, &int_datatype, &int_datatype],
        )
    }
}

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();

    let root_process = world.process_at_rank(0);

    if world.rank() == 0 {
        root_process.broadcast_into(&mut MyInts([3, 2, 1]));
    } else {
        let mut ints: [i32; 3] = [0, 0, 0];
        root_process.broadcast_into(&mut ints[..]);

        assert_eq!(1, ints[0]);
        assert_eq!(2, ints[1]);
        assert_eq!(3, ints[2]);
    }
}