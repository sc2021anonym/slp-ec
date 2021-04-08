use crate::reorder::*;
use crate::*;

fn calc_candidates(dag: &DAG, alloc: &Alloc) -> Vec<(Term, bool, usize, usize)> {
    // (term, ready?, #hot, #children)
    let mut summary = Vec::new();

    for (t, children) in dag {
        let mut ready = true;
        let mut hot = 0;
        for c in children {
            if c.is_const() || alloc.get(c).is_some() {
                if let Some(pos) = alloc.index(c) {
                    if pos < 48 {
                        hot += 1;
                    }
                } else {
                    // Constとかだと入ってないことがある
                }
            } else {
                ready = false;
            }
        }
        if !ready {
            summary.push((t.clone(), false, 0, 0));
        } else {
            summary.push((t.clone(), true, hot, children.len()));
        }
    }

    summary
}

pub fn deal_multislp2(
    slp: &MultiSLP,
    num_of_constants: usize,
    targets: Vec<Term>,
    strategy: Strategy,
) -> Vec<(Pebble, Vec<Pebble>)> {
    let mut pebble_computation: Vec<(Pebble, Vec<Pebble>)> = Vec::new();
    let mut dag = multislp_to_dag(slp);
    let original_len = dag.len();

    let mut alloc = Alloc::new(num_of_constants, targets, strategy);

    let mut outdegs = make_outdegrees(&dag);

    loop {
        if dag.is_empty() {
            break;
        }

        // dagがemptyでなければ必ず計算できるものがある
        let mut reduced: Vec<(Term, bool, usize, usize)> = calc_candidates(&dag, &alloc)
            .into_iter()
            .filter(|(_, ready, _, _)| *ready)
            .collect();

        // hotが最大のものでsort
        // reduced.sort_by_key(|(_, _, hot, _)| *hot); reduced.reverse();

        reduced.sort_by(|(_, _, hot1, children1), (_, _, hot2, children2)| {
            let ratio1 = (*hot1 as f64) / (*children1 as f64);
            let ratio2 = (*hot2 as f64) / (*children2 as f64);
            ratio2.partial_cmp(&ratio1).unwrap()
        });

        let (target, _, _, _) = &reduced[0];
        let mut first = Vec::new();
        let mut second = Vec::new();
        let mut third = Vec::new();

        let children: &BTreeSet<Term> = dag.get(&target).unwrap();
        for c in children {
            let idx = alloc.index(c).unwrap_or(300);
            if idx < 8 {
                first.push(c);
            } else if idx < 48 {
                second.push(c);
            } else {
                third.push(c);
            }
        }
        first.append(&mut second);
        first.append(&mut third);

        let mut pebbles = Vec::new();

        for c in first {
            pebbles.push(alloc.get(c).unwrap());
            alloc.access(&c);
            let mut_ref = outdegs.get_mut(c).unwrap();
            *mut_ref -= 1;
            if *mut_ref == 0 {
                alloc.try_release(c);
            }
        }
        let pebble = alloc.assign(&target);
        pebble_computation.push((pebble, pebbles));
        dag.remove(&target);
    }

    assert!(pebble_computation.len() == original_len);

    pebble_computation
}
