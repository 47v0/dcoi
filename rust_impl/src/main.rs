mod ground_truth;
mod rules;
use ground_truth::{ ground_truth, test_rule};
use rules::{ bvadd_new, paper_bvadd, Operation};


fn show_bits( value: u64, w: u32, keep: &[u32],) -> String {

    let mut result = String::new();
    for i in (0..w).rev() {

        if keep.contains(&i) {
            result.push_str(&(((value >> i) & 1).to_string()));
        } else {
            result.push('·');
        }


        if i != 0 {
            result.push(' ');
        }
    }
    result
}




fn show_xy(x: u64,y: u64,w: u32,keep_x: &[u32],keep_y: &[u32]) {

    println!(
        " x  : {}",
        (0..w)
            .rev()
            .map(|i| ((x >> i) & 1).to_string())
            .collect::<Vec<_>>() //put into list
            .join(" ")
    );
    println!(
        "      {}",
        show_bits(x,w,keep_x)
    );



    println!(
        " y  : {}",
        (0..w)
            .rev()
            .map(|i| ((y >> i) & 1).to_string())
            .collect::<Vec<_>>() 
            .join(" ")
    );

    println!(
        "      {}",
        show_bits(y,w,keep_y)
    );

    println!();
}





fn test_all_comb(w: u32,h: u32,l: u32,) {

    let limit = 1u64 << w;
    let mut new_better = 0;
    let mut optimal = 0;
    let mut unoptimal_operands: Vec<(u64,u64,Vec<u32>,Vec<u32>,Vec<u32>,Vec<u32>)> = Vec::new();


    for x in 0..limit {
        for y in 0..limit {
            let (px, py) =paper_bvadd(h);
            let (nx, ny) =bvadd_new(x,y,h,l);
            let (optimal_x, optimal_y) = ground_truth(Operation::BVAdd,x,y,w,h,l);
            let (new_ok,bad_x,bad_y) = test_rule(Operation::BVAdd,x,y,w,h,l,&nx,&ny);
            if !new_ok {
                println!("NEW RULE FAIL");
                println!(
                    "x  = {:0width$b}",
                    x,
                    width = w as usize
                );
                println!(
                    "y  = {:0width$b}",
                    y,
                    width = w as usize
                );

                println!(
                    "x2 = {:0width$b}",
                    bad_x.unwrap(),
                    width = w as usize
                );

                println!(
                    "y2 = {:0width$b}",
                    bad_y.unwrap(),
                    width = w as usize
                );

                println!("kept x bits: {:?}", nx);
                println!("kept y bits: {:?}", ny);

                continue;
            }


            if nx.len() + ny.len()< px.len() + py.len()
            {
                new_better += 1;
            }


            if optimal_x.len() + optimal_y.len()== nx.len() + ny.len()
            {
                optimal += 1;

            } else {
                unoptimal_operands.push((x,y,nx,ny,optimal_x,optimal_y));
            }
        }
    }


    println!(
        "total x,y pairs: {}",
        limit * limit
    );

    println!(
        "new rule better than paper: {}",
        new_better
    );

    println!(
        "total optimal: {}",
        optimal
    );

    println!(
        "total unoptimal: {}",
        unoptimal_operands.len()
    );


    println!("-----------------------------------------------------------------");

    println!("UNOPTIMAL");

    for (x,y,nx,ny,optimal_x,optimal_y) in unoptimal_operands
    {
        println!( "\nx = {:0width$b}", x, width = w as usize);

        println!(
            "y = {:0width$b}",
            y,
            width = w as usize
        );


        println!("\nNEW RULE:");

        show_xy(x,y,w,&nx,&ny);

        println!("GROUND TRUTH:");

        show_xy(x,y,w,&optimal_x,&optimal_y);


        println!( "new size: {} | optimal size: {}", nx.len() + ny.len(), optimal_x.len() + optimal_y.len());

        println!(
            "----------------------------------------"
        );
    }
}


fn main() {
    test_all_comb(4, 3, 2);
}


// eval framework
// Want:
//      precise
//      imprecise
//      incorrect
//      
// evaluate.rs
// report.rs


// IDeas
// a struct for experiment configuration/results
// returning typed result enums for Precise / Imprecise / Incorrect

// patronus