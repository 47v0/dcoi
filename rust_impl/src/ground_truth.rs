use crate::rules::Operation;



// Ground truth for binary, same-width operations.
//
// Try every possible set of bits to keep in the COI.
// Then allow every dropped bit to change arbitrarily.
//
// The minimum set that always preserves output [h:l]
// is the ground truth.
pub fn ground_truth( op: Operation, x: u64, y: u64, w: u32, h: u32, l: u32,) -> (Vec<u32>, Vec<u32>) {

    let limit = 1u64 << w;
    let s = op.eval(x, y, w);
    let target = (s >> l) & ((1u64 << (h-l + 1)) - 1);


    // try every possible set of x bits to keep in coi
      // try every possible set of y bits to keep in coi
       // try every possible replacement x2 for dropped bits
        // try every possible replacement y2 for dropped bits
    // calculate the new result of x2 and y2, and get the output slice [h:l]
    // if changing removed bits can change the output, then this coi does not work


    let mut best_keep_x: Vec<u32> = Vec::new();
    let mut best_keep_y: Vec<u32> = Vec::new();

    let mut best_size = 2 * w + 1;


    for x_i in 0..limit {
        let mut keep_x: Vec<u32> = Vec::new();
        for i in 0..w {
            if ((x_i >> i) & 1) == 1 {
                keep_x.push(i);
            }
        }
        for y_i in 0..limit {
            let mut keep_y: Vec<u32> = Vec::new();
            for i in 0..w {
                if ((y_i >> i) & 1) == 1 {
                    keep_y.push(i);
                }
            }
            if keep_x.len() + keep_y.len() >= best_size as usize
            {
                continue;
            }

            let mut works = true;

            for x2 in 0..limit {
                let mut x_matches = true;
                for i in &keep_x {

                    if ((x2 >> *i) & 1) != ((x >> *i) & 1)
                    {
                        x_matches = false;
                        break;
                    }
                }


                if !x_matches {
                    continue;
                }


                for y2 in 0..limit {

                    let mut y_matches = true;


                    for i in &keep_y {

                        if ((y2 >> *i) & 1) != ((y >> *i) & 1)
                        {
                            y_matches = false;
                            break;
                        }
                    }


                    if !y_matches {
                        continue;
                    }


                    let s2 = op.eval(x2,y2,w);

                    let out = (s2 >> l) & ((1u64 << (h - l + 1)) - 1);


                    // if output changes, this fails
                    if out != target {

                        works = false;
                        break;
                    }
                }


                if !works {
                    break;
                }
            }


            if works {

                best_keep_x = keep_x.clone();
                best_keep_y = keep_y.clone();

                best_size = keep_x.len() as u32 + keep_y.len() as u32;
            }
        }
    }


    (best_keep_x,best_keep_y)
}


// Allow every 'dropped' bit to change arbitrarily,
// see if output changes
pub fn test_rule( op: Operation, x: u64, y: u64, w: u32, h: u32, l: u32, keep_x: &[u32], keep_y: &[u32],) -> (bool, Option<u64>, Option<u64>) {
    let limit = 1u64 << w;
    let s = op.eval(x, y, w);
    let target =(s >> l) & ((1u64 << (h - l + 1)) - 1);
    for x2 in 0..limit {
        for y2 in 0..limit {
            // checking every x2,y2, but only if they preserve coi bits.
            // x2 must match x on every kept bit
            let mut matches = true;
            for i in keep_x {
                if ((x2 >> *i) & 1) != ((x >> *i) & 1)
                {
                    matches = false;
                }
            }


            // y2 must match y on every kept bit
            for i in keep_y {
                if ((y2 >> *i) & 1) != ((y >> *i) & 1)
                {
                    matches = false;
                }
            }


            if !matches {
                continue;
            }


            // compute the output slice for this new x2, y2,
            // see if it matches correct output slice
            let s2 = op.eval( x2,y2,w);


            let out = (s2 >> l) & ((1u64 << (h - l + 1)) - 1);
            if out != target {

                return (false,Some(x2),Some(y2));
            }
        }
    }


    (true,None,None,)
}