use crate::reorder::{self, Pebble};

pub struct Stat {
    pub nr_xors: usize,
    pub nr_memacc: usize,
    pub nr_page_transfer: usize,
}

// https://en.wikipedia.org/wiki/Iverson_bracket
fn iverson(b: bool) -> usize {
    if b {
        1
    } else {
        0
    }
}

pub fn analyze(program: &Vec<(Pebble, Vec<Pebble>)>) -> Stat {
    let mut stat = Stat {
        nr_xors: 0,
        nr_memacc: 0,
        nr_page_transfer: 0,
    };

    // page queue
    let mut ru = reorder::RecentlyUse::new();

    for (t, vars) in program {
        // t <- XOR(vars)
        assert!(vars.len() > 1);

        // t <- XOR(a, b, c)
        // => 2 xor since (a + b) + c
        // => 4 memory accesses t, a, b, c
        stat.nr_xors += vars.len() - 1;
        stat.nr_memacc += vars.len() + 1;

        for v in vars {
            // read access
            if ru.is_hot(v) {
                // cache hit
            } else {
                // check evict
                stat.nr_page_transfer += iverson(ru.len() >= 8);

                // load
                stat.nr_page_transfer += 1;
            }
            ru.access(v.clone());
        }
        // write access
        if !ru.is_hot(t) {
            // check evict for allocate
            stat.nr_page_transfer += iverson(ru.len() >= 8);
        }
        ru.access(t.clone());
    }

    stat
}
