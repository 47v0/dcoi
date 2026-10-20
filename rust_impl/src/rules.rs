#[derive(Debug, Clone, Copy)]
pub enum Operation {
    And,
    Or,
    Implies,
    BVAnd,
    BVNand,
    BVOr,
    BVNor,
    BVAdd,
    BVMul,
    BVUlt,
    BVUle,
    BVUgt,
    BVUge,
    Equal,
    Distinct,
    BVComp,
    // BVConcat,
    // BVIte
}


impl Operation {
    // Evaluate binary operations for the ground truth prototype.
    //
    // Eventually Patronus Expr will replace this Operation enum.
    pub fn eval(&self,x: u64,y: u64,w: u32) -> u64 {
        match self {

            Operation::And => {(x & 1) & (y & 1)}
            Operation::Or => {(x & 1) | (y & 1)}
            Operation::Implies => {(!(x & 1) | (y & 1)) & 1}
            Operation::BVAnd => {x & y}
            Operation::BVNand => {(!(x & y))}
            Operation::BVOr => {x | y}
            Operation::BVNor => {(!(x | y))}
            Operation::BVAdd => {x.wrapping_add(y)}
            Operation::BVMul => {x.wrapping_mul(y)}
            Operation::BVUlt => {(x < y) as u64}
            Operation::BVUle => {(x <= y) as u64}
            Operation::BVUgt => {(x > y) as u64}
            Operation::BVUge => {(x >= y) as u64}
            Operation::Equal => {(x == y) as u64}
            Operation::Distinct => {(x != y) as u64}
            Operation::BVComp => {(x == y) as u64}

        }
    }
}

pub fn paper_bvadd( h: u32,) -> (Vec<u32>, Vec<u32>) {
    let keep_x = (0..=h).collect();
    let keep_y = (0..=h).collect();
    (keep_x, keep_y)
}


// New BVADD rule
pub fn bvadd_new( x: u64, y: u64, h: u32, l: u32,) -> (Vec<u32>, Vec<u32>) {

    let mut keep_x: Vec<u32> = (l..=h).collect();
    let mut keep_y: Vec<u32> = (l..=h).collect();

    // carry[i] = carry into bit i
    let mut carry = vec![0u64; (l + 1) as usize];
    for i in 0..l {
        let x_bit = (x >> i) & 1;
        let y_bit = (y >> i) & 1;
        let c_in = carry[i as usize];
        let c_out = (x_bit & y_bit) | (x_bit & c_in) | (y_bit & c_in);
        carry[(i + 1) as usize] = c_out;
    }


    for i in (0..l).rev() {
        let x_bit = (x >> i) & 1;
        let y_bit = (y >> i) & 1;
        let c_in = carry[i as usize];

        if x_bit == y_bit {
            keep_x.push(i);
            keep_y.push(i);
            break;
        }


        if x_bit == c_in {
            keep_x.push(i);
        } else {
            keep_y.push(i);
        }
    }

    keep_x.sort();
    keep_y.sort();


    (keep_x, keep_y)
}