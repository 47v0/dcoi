use baa::{BitVecOps,BitVecValue,WidthInt};

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
    // ternary?
    pub fn eval(&self,x: &BitVecValue,y: &BitVecValue) -> BitVecValue {
        match self {

            Operation::And => BitVecValue::from_u64((x.is_true() && y.is_true()) as u64, 1),
            Operation::Or => BitVecValue::from_u64((x.is_true() || y.is_true()) as u64, 1),
            Operation::Implies => BitVecValue::from_u64((!x.is_true() || y.is_true()) as u64, 1),
            Operation::BVAnd => x.and(y),
            Operation::BVNand => x.and(y).not(),
            Operation::BVOr => x.or(y),
            Operation::BVNor => x.or(y).not(),
            Operation::BVAdd => x.add(y),
            Operation::BVMul => x.mul(y),
            Operation::BVUlt => BitVecValue::from_u64(x.is_less(y) as u64, 1),
            Operation::BVUle => BitVecValue::from_u64(x.is_less_or_equal(y) as u64, 1),
            Operation::BVUgt => BitVecValue::from_u64(x.is_greater(y) as u64, 1),
            Operation::BVUge => BitVecValue::from_u64(x.is_greater_or_equal(y) as u64, 1),
            Operation::Equal => BitVecValue::from_u64(x.is_equal(y) as u64, 1),
            Operation::Distinct => BitVecValue::from_u64(x.is_not_equal(y) as u64, 1),
            Operation::BVComp => BitVecValue::from_u64(x.is_equal(y) as u64, 1),

        }
    }
}

pub fn paper_bvadd(x: &BitVecValue, y: &BitVecValue, h: WidthInt, l: WidthInt) -> (Vec<WidthInt>, Vec<WidthInt>) {
    let keep_x = (0..=h).collect();
    let keep_y = (0..=h).collect();
    (keep_x, keep_y)
}


// New BVADD rule
pub fn bvadd_new(x: &BitVecValue, y: &BitVecValue, h: WidthInt, l: WidthInt) -> (Vec<WidthInt>, Vec<WidthInt>) {
    let mut keep_x: Vec<WidthInt> = (l..=h).collect();
    let mut keep_y: Vec<WidthInt> = (l..=h).collect();

    // carry[i] = carry into bit i
    let mut carry = vec![false; (l + 1) as usize];

    for i in 0..l {
        let x_bit = x.is_bit_set(i);
        let y_bit = y.is_bit_set(i);
        let c_in = carry[i as usize];
        let c_out = (x_bit && y_bit) || (x_bit && c_in) || (y_bit && c_in);
        carry[(i + 1) as usize] = c_out;
    }


    for i in (0..l).rev() {
        let x_bit = x.is_bit_set(i);
        let y_bit = y.is_bit_set(i);
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





// Boolean AND
pub fn paper_and(x: &BitVecValue, y: &BitVecValue, _h: WidthInt, _l: WidthInt) -> (Vec<WidthInt>, Vec<WidthInt>) {
    if !x.is_true() {
        return (vec![0], vec![]);
    }

    if !y.is_true() {
        return (vec![], vec![0]);
    }

    (vec![0], vec![0])
}


// Boolean OR
pub fn paper_or(x: &BitVecValue, y: &BitVecValue, _h: WidthInt, _l: WidthInt) -> (Vec<WidthInt>, Vec<WidthInt>) {
    if x.is_true() {
        return (vec![0], vec![]);
    }

    if y.is_true() {
        return (vec![], vec![0]);
    }

    (vec![0], vec![0])
}


// Boolean IMPLIES
pub fn paper_implies(x: &BitVecValue, y: &BitVecValue, _h: WidthInt, _l: WidthInt) -> (Vec<WidthInt>, Vec<WidthInt>) {
    if !x.is_true() {
        return (vec![0], vec![]);
    }

    if y.is_true() {
        return (vec![], vec![0]);
    }

    (vec![0], vec![0])
}


// BVAND
pub fn paper_bvand(x: &BitVecValue, y: &BitVecValue, h: WidthInt, l: WidthInt) -> (Vec<WidthInt>, Vec<WidthInt>) {
    let mut keep_x = Vec::new();
    let mut keep_y = Vec::new();

    for i in l..=h {
        let x_bit = x.is_bit_set(i);
        let y_bit = y.is_bit_set(i);

        if !x_bit {
            keep_x.push(i);
        } else if !y_bit {
            keep_y.push(i);
        } else {
            keep_x.push(i);
            keep_y.push(i);
        }
    }

    (keep_x, keep_y)
}


// BVNAND has the same coi as BVAND
pub fn paper_bvnand(x: &BitVecValue, y: &BitVecValue, h: WidthInt, l: WidthInt) -> (Vec<WidthInt>, Vec<WidthInt>) {
    paper_bvand(x, y, h, l)
}


// BVOR
pub fn paper_bvor(x: &BitVecValue, y: &BitVecValue, h: WidthInt, l: WidthInt) -> (Vec<WidthInt>, Vec<WidthInt>) {
    let mut keep_x = Vec::new();
    let mut keep_y = Vec::new();

    for i in l..=h {
        let x_bit = x.is_bit_set(i);
        let y_bit = y.is_bit_set(i);

        if x_bit {
            keep_x.push(i);
        } else if y_bit {
            keep_y.push(i);
        } else {
            keep_x.push(i);
            keep_y.push(i);
        }
    }

    (keep_x, keep_y)
}


// BVNOR has the same coi as BVOR
pub fn paper_bvnor(x: &BitVecValue, y: &BitVecValue, h: WidthInt, l: WidthInt) -> (Vec<WidthInt>, Vec<WidthInt>) {
    paper_bvor(x, y, h, l)
}


// BVMUL
pub fn paper_bvmul(x: &BitVecValue, y: &BitVecValue, _h: WidthInt, _l: WidthInt) -> (Vec<WidthInt>, Vec<WidthInt>) {
    let wx = x.width();
    let wy = y.width();

    let x_zero = x.is_zero();
    let y_zero = y.is_zero();

    if x_zero {
        return ((0..wx).collect(), vec![]);
    }

    if y_zero {
        return (vec![], (0..wy).collect());
    }

    ((0..wx).collect(), (0..wy).collect())
}


// BVULT / BVULE / BVUGT / BVUGE
//
// Track from the most-significant differing bit upward.
// If the operands are equal, keep all bits.
pub fn paper_bvcompare(x: &BitVecValue, y: &BitVecValue, _h: WidthInt, _l: WidthInt) -> (Vec<WidthInt>, Vec<WidthInt>) {
    let w = x.width();

    assert_eq!(w, y.width());

    let mut first_difference = None;

    for i in (0..w).rev() {
        if x.is_bit_set(i) != y.is_bit_set(i) {
            first_difference = Some(i);
            break;
        }
    }

    if let Some(i) = first_difference {
        return ((i..w).collect(), (i..w).collect());
    }

    ((0..w).collect(), (0..w).collect())
}


pub fn paper_bvult(x: &BitVecValue, y: &BitVecValue, h: WidthInt, l: WidthInt) -> (Vec<WidthInt>, Vec<WidthInt>) {paper_bvcompare(x, y, h, l)}
pub fn paper_bvule(x: &BitVecValue, y: &BitVecValue, h: WidthInt, l: WidthInt) -> (Vec<WidthInt>, Vec<WidthInt>) {paper_bvcompare(x, y, h, l)}
pub fn paper_bvugt(x: &BitVecValue, y: &BitVecValue, h: WidthInt, l: WidthInt) -> (Vec<WidthInt>, Vec<WidthInt>) {paper_bvcompare(x, y, h, l)}
pub fn paper_bvuge(x: &BitVecValue, y: &BitVecValue, h: WidthInt, l: WidthInt) -> (Vec<WidthInt>, Vec<WidthInt>) {paper_bvcompare(x, y, h, l)}

pub fn paper_equal(x: &BitVecValue, y: &BitVecValue, _h: WidthInt, _l: WidthInt) -> (Vec<WidthInt>, Vec<WidthInt>) {
    let w = x.width();
    assert_eq!(w, y.width());

    for i in 0..w {
        if x.is_bit_set(i) != y.is_bit_set(i) {
            return (vec![i], vec![i]);
        }
    }

    ((0..w).collect(), (0..w).collect())
}

pub fn paper_distinct(x: &BitVecValue, y: &BitVecValue, h: WidthInt, l: WidthInt) -> (Vec<WidthInt>, Vec<WidthInt>) {paper_equal(x, y, h, l)}
pub fn paper_bvcomp(x: &BitVecValue, y: &BitVecValue, h: WidthInt, l: WidthInt) -> (Vec<WidthInt>, Vec<WidthInt>) {paper_equal(x, y, h, l)}