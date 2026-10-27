use baa::{BitVecValue, WidthInt};

use crate::ground_truth::{ground_truth, test_rule};
use crate::rules::Operation;
// use std::io::Write;

pub type RuleFn = fn( &BitVecValue, &BitVecValue, WidthInt, WidthInt,) -> (Vec<WidthInt>, Vec<WidthInt>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Relation {
    NoOverlap,
    SomeOverlap,
    FullyOverlap,
    Same
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Classification {
    Precise,
    Imprecise,
    Incorrect,
}


#[derive(Debug)]
pub struct CaseResult {
    pub x: BitVecValue,
    pub y: BitVecValue,
    pub h: WidthInt,
    pub l: WidthInt,
    pub keep_x: Vec<WidthInt>,
    pub keep_y: Vec<WidthInt>,

    pub optimal_x: Vec<WidthInt>,
    pub optimal_y: Vec<WidthInt>,

    pub relation: Relation,
    pub classification: Classification,

    pub bad_x: Option<BitVecValue>,
    pub bad_y: Option<BitVecValue>,
}


#[derive(Debug, Default)]
pub struct Stats {
    pub total: usize,

    pub no_overlap: usize,
    pub some_overlap: usize,
    pub fully_overlap: usize,
    pub same: usize,

    pub precise: usize,
    pub imprecise: usize,
    pub incorrect: usize,

    pub rule_bits: usize,
    pub optimal_bits: usize,
}


#[derive(Debug)]
pub struct EvaluationResult {
    pub stats: Stats,
    pub cases: Vec<CaseResult>,
}


// helper; does coi A contain every bit in coi B?
fn coi_contains(a_x: &[WidthInt],a_y: &[WidthInt],b_x: &[WidthInt],b_y: &[WidthInt]
) -> bool {

    let x_contains = b_x.iter().all(|i: &u32| a_x.contains(i));
    let y_contains = b_y.iter().all(|i| a_y.contains(i));
    x_contains && y_contains
}


// Does either operand have any common coi bit?
fn coi_overlap(
    a_x: &[WidthInt],
    a_y: &[WidthInt],
    b_x: &[WidthInt],
    b_y: &[WidthInt],
) -> bool {

    let x_overlap = a_x.iter().any(|i: &u32| b_x.contains(i));
    let y_overlap = a_y.iter().any(|i| b_y.contains(i));

    x_overlap || y_overlap
}



fn compare_coi(
    keep_x: &[WidthInt],
    keep_y: &[WidthInt],
    optimal_x: &[WidthInt],
    optimal_y: &[WidthInt],
) -> Relation {

    let same_x = (keep_x.len() == optimal_x.len()) && keep_x.iter().all(|i| optimal_x.contains(i));
    let same_y = (keep_y.len() == optimal_y.len()) && keep_y.iter().all(|i| optimal_y.contains(i));

    if same_x && same_y {
        return Relation::Same;
    }

    if coi_contains(keep_x, keep_y, optimal_x, optimal_y) {
        return Relation::FullyOverlap;
    }

    if coi_overlap(keep_x, keep_y, optimal_x, optimal_y) {
        return Relation::SomeOverlap;
    }

    Relation::NoOverlap
}

// next add evaluate_ternary and so on.
pub fn evaluate_binary(op: Operation, rule: RuleFn, w: WidthInt, out_w: WidthInt) -> EvaluationResult {
    assert!(w < 64);

    let limit = 1u64 << w;
    let mut stats = Stats::default();
    let mut cases: Vec<CaseResult> = Vec::new();

    for x_raw in 0..limit {
        let x = BitVecValue::from_u64(x_raw, w);

        for y_raw in 0..limit {
            let y = BitVecValue::from_u64(y_raw, w);

            for h in 0..out_w {
                for l in 0..=h {
                    let (keep_x, keep_y) = rule(&x, &y, h, l);
                    let (optimal_x, optimal_y) = ground_truth(op, &x, &y, h, l);
                    let (rule_ok, bad_x, bad_y) = test_rule(op, &x, &y, h, l, &keep_x, &keep_y);

                    let relation = compare_coi(&keep_x, &keep_y, &optimal_x, &optimal_y);


                    let rule_size = keep_x.len() + keep_y.len();
                    let optimal_size = optimal_x.len() + optimal_y.len();


                    // incorrect: changing dropped bits can change output
                    // precise: sound and minimum-size
                    // imprecise: sound but keeps more bits than necessary
                    let classification =
                        if !rule_ok {
                            Classification::Incorrect
                        } else if rule_size == optimal_size {
                            Classification::Precise
                        } else {
                            Classification::Imprecise
                        };

                    if classification == Classification::Imprecise {

                    }


                    stats.total += 1;
                    stats.rule_bits += rule_size;
                    stats.optimal_bits += optimal_size;


                    match relation {
                        Relation::NoOverlap => stats.no_overlap += 1,
                        Relation::SomeOverlap => stats.some_overlap += 1,
                        Relation::FullyOverlap => stats.fully_overlap += 1,
                        Relation::Same => stats.same += 1,
                    }

                    
                    match classification {
                        Classification::Imprecise => stats.imprecise += 1,
                        Classification::Precise => stats.precise += 1,
                        Classification::Incorrect => stats.incorrect += 1,

                    }

                    cases.push(
                        CaseResult {
                            x: x.clone(),
                            y: y.clone(),
                            h,
                            l,
                            keep_x,
                            keep_y,
                            optimal_x,
                            optimal_y,
                            relation,
                            classification,
                            bad_x,
                            bad_y,
                        }
                    );
                }
            }
        }
    }


    EvaluationResult {
        stats,
        cases
    }
}