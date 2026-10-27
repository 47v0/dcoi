mod ground_truth;
mod rules;
mod evaluate;

use baa::{BitVecOps, BitVecValue, WidthInt};
use ground_truth::{ground_truth, test_rule};
// use std::fs;
// use std::io::Write;
use evaluate::{evaluate_binary, RuleFn};

use rules::{
    Operation,
    paper_and,
    paper_or,
    paper_implies,
    paper_bvand,
    paper_bvnand,
    paper_bvor,
    paper_bvnor,
    paper_bvadd,
    paper_bvmul,
    paper_bvult,
    paper_bvule,
    paper_bvugt,
    paper_bvuge,
    paper_equal,
    paper_distinct,
    paper_bvcomp,

    //playground; new proposed rules to test
    bvadd_new,

};

fn show_bits(value: &BitVecValue, keep: &[WidthInt]) -> String {

    let w = value.width();
    let mut result = String::new();
    for i in (0..w).rev() {

        if keep.contains(&i) {
            result.push(if value.is_bit_set(i) { '1' } else { '0' });
        } else {
            result.push('·');
        }


        if i != 0 {
            result.push(' ');
        }
    }
    result
}




fn show_xy(x: &BitVecValue, y: &BitVecValue, keep_x: &[WidthInt], keep_y: &[WidthInt]) {
    println!(
        " x  : {}",
        x.to_bit_str()
            .chars()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );




    println!("      {}", show_bits(x, keep_x));

    println!(
        " y  : {}",
        y.to_bit_str()
            .chars()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );

    println!("      {}", show_bits(y, keep_y));
    println!();
}

fn test_all_comb(w: WidthInt, h: WidthInt, l: WidthInt) {
    assert!(w < 64);

    let limit = 1u64 << w;

    let mut new_better = 0;
    let mut optimal = 0;

    let mut unoptimal_operands:
        Vec<(BitVecValue,BitVecValue,Vec<WidthInt>,Vec<WidthInt>,Vec<WidthInt>,Vec<WidthInt>)> = Vec::new();


    for x_raw in 0..limit {
        let x = BitVecValue::from_u64(x_raw, w);

        for y_raw in 0..limit {
            let y = BitVecValue::from_u64(y_raw, w);

            let (px, py) = paper_bvadd(&x, &y, h, l);
            let (nx, ny) = bvadd_new(&x, &y, h, l);

            let (optimal_x, optimal_y) =
                ground_truth(Operation::BVAdd, &x, &y, h, l);

            let (new_ok, bad_x, bad_y) =
                test_rule(Operation::BVAdd, &x, &y, h, l, &nx, &ny);


            if !new_ok {
                println!("NEW RULE FAIL");

                println!("x  = {}", x.to_bit_str());
                println!("y  = {}", y.to_bit_str());

                println!("x2 = {}", bad_x.unwrap().to_bit_str());
                println!("y2 = {}", bad_y.unwrap().to_bit_str());

                println!("kept x bits: {:?}", nx);
                println!("kept y bits: {:?}", ny);

                continue;
            }


            if nx.len() + ny.len() < px.len() + py.len() {
                new_better += 1;
            }


            if optimal_x.len() + optimal_y.len() == nx.len() + ny.len() {
                optimal += 1;

            } else {
                unoptimal_operands.push((
                    x.clone(),
                    y.clone(),
                    nx,
                    ny,
                    optimal_x,
                    optimal_y,
                ));
            }
        }
    }


    println!("total x,y pairs: {}", limit * limit);
    println!("new rule better than paper: {}", new_better);
    println!("total optimal: {}", optimal);
    println!("total unoptimal: {}", unoptimal_operands.len());


    println!("-----------------------------------------------------------------");

    println!("UNOPTIMAL");


    for (x, y, nx, ny, optimal_x, optimal_y) in unoptimal_operands {
        println!("\nx = {}", x.to_bit_str());
        println!("y = {}", y.to_bit_str());

        println!("\nNEW RULE:");
        show_xy(&x, &y, &nx, &ny);

        println!("GROUND TRUTH:");
        show_xy(&x, &y, &optimal_x, &optimal_y);

        println!(
            "new size: {} | optimal size: {}",
            nx.len() + ny.len(),
            optimal_x.len() + optimal_y.len()
        );

        println!("----------------------------------------");
    }
}





fn print_result(name: &str, result: &evaluate::EvaluationResult) {
    println!("\n{}", name, );
    println!("total: {}", result.stats.total);

    // println!("overlap:");
    // println!("  none:   {}", result.stats.no_overlap);
    // println!("  some:   {}", result.stats.some_overlap);
    // println!("  full:   {}", result.stats.fully_overlay);
    // println!("  same:   {}", result.stats.same);

    println!("quality:");
    println!("  precise:   {}", result.stats.precise);
    println!("  imprecise: {}", result.stats.imprecise);
    println!("  incorrect: {}", result.stats.incorrect);
}


fn main() {
    let w = 4;
    // let h = 3;
    // let l = 2;
    let experiments: Vec<(&str, Operation, RuleFn, WidthInt, WidthInt)> = vec![
        ("And",      Operation::And,     paper_and,      1, 1),
        ("Or",       Operation::Or,       paper_or,       1, 1),
        ("Implies",  Operation::Implies,  paper_implies,  1, 1),
        ("BVAnd",    Operation::BVAnd,    paper_bvand,    w, w),
        ("BVNand",   Operation::BVNand,   paper_bvnand,   w, w),
        ("BVOr",     Operation::BVOr,     paper_bvor,     w, w),
        ("BVNor",    Operation::BVNor,    paper_bvnor,    w, w),
        ("BVAdd",    Operation::BVAdd,    paper_bvadd,     w, w),
        ("BVMul",    Operation::BVMul,    paper_bvmul,     w, w),
        ("BVUlt",    Operation::BVUlt,   paper_bvult,      w, 1),
        ("BVUle",    Operation::BVUle,    paper_bvule,    w, 1),
        ("BVUgt",    Operation::BVUgt,    paper_bvugt,     w, 1),
        ("BVUge",    Operation::BVUge,    paper_bvuge,     w, 1),
        ("Equal",    Operation::Equal,    paper_equal,    w, 1),
        ("Distinct", Operation::Distinct, paper_distinct,  w, 1),
        ("BVComp",   Operation::BVComp,   paper_bvcomp,    w, 1),

        ("BVAdd New", Operation::BVAdd,    bvadd_new,      w, w),

    ];

    for (name, op, rule, in_w, out_w) in experiments {
        let result = evaluate_binary(op, rule, in_w, out_w);
        print_result(name, &result);
    }
    // test_all_comb(w, h, l);


}