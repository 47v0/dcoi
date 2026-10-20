def ground_truth(x, y, w, h, l):
    s = (x + y) % (2 ** w)
    target = (s >> l) & ((2 ** (h - l + 1)) - 1)
    # try every possible set of x bits to keep in coi
      # try every possible set of y bits to keep in coi
       #  try every possible replacement x2 for dropped bitts
        # try every possible replacement y2 for dropped bits
    # calculate the new sum of x2 and y2, and get the output slice [h:l]
    # if changing removed bits can change the output, then this coi does not work

    best_keep_x = None
    best_keep_y = None
    best_size = 2 * w + 1
    for x_i in range(2 ** w):
        keep_x = []
        for i in range(w):
            if (x_i >> i) & 1: # check if mask is 1
                keep_x.append(i)
        for y_i in range(2 ** w):
            keep_y = []
            for i in range(w):
                if (y_i >> i) & 1:
                    keep_y.append(i)
            if len(keep_x) + len(keep_y) >= best_size:
                continue
            works = True
            for x2 in range(2 ** w):
                x_matches = True
                for i in keep_x:
                    if ((x2 >> i) & 1) != ((x >> i) & 1):
                        x_matches = False
                        break
                if not x_matches:
                    continue
                for y2 in range(2 ** w):
                    y_matches = True
                    for i in keep_y:
                        if ((y2 >> i) & 1) != ((y >> i) & 1):
                            y_matches = False
                            break
                    if not y_matches:
                        continue
                    s2 = (x2 + y2) % (2 ** w)
                    out = (s2 >> l) & ((2 ** (h - l + 1)) - 1)
                    # if output changes, this fails
                    if out != target:
                        works = False
                        break
                if not works:
                    break
            if works:
                best_keep_x = keep_x
                best_keep_y = keep_y
                best_size = len(keep_x) + len(keep_y)
    return best_keep_x, best_keep_y

def paper_rule(h):
    keep_x = list(range(h + 1))
    keep_y = list(range(h + 1))
    return keep_x, keep_y

from visual import show_xy
from visual import BVADD_new

def new_rule(x, y, h, l):
    return BVADD_new(x,y,h,l)

# Allow every 'dropped' bit to change arbitrarily, see if output changes
def test_rule(x, y, w, h, l, keep_x, keep_y):
    s = (x + y) % (2 ** w)
    target = (s >> l) & ((2**(h - l + 1)) - 1)

    for x2 in range(2 ** w):
        for y2 in range(2 ** w):
            # checking every x1,x2, but only if they perserve coi bits.

            # x2 must match x on every kept bit
            matches = True
            for i in keep_x:
                if ((x2 >> i) & 1) != ((x >> i) & 1):
                    matches = False
            # y2 must match y on every kept bit
            for i in keep_y:
                if ((y2 >> i) & 1) != ((y >> i) & 1):
                    matches = False

            if not matches:
                continue

            # compute the output slice for this new x2, y2, see if it matches correct output slice
            s2 = (x2 + y2) % (2 ** w)
            # print("s2:",format(s2, f"0{w}b"))
            out = (s2 >> l) & ((2**(h - l + 1)) - 1)
            # print("out",format(out, f"0{w}b"))

            if out != target:
                return False, x2, y2

    return True, None, None



def test_all_comb(w, h, l):
    new_better = 0
    optimal = 0
    unoptimal_operands = []
    for x in range(2 ** w):
        for y in range(2 ** w):

            px, py = paper_rule(h)
            nx, ny = new_rule(x, y, h, l)
            optimal_x, optimal_y = ground_truth(x,y,w,h,l)
            # new_ok, bad_x, bad_y = test_rule(x, y, w, h, l, nx, ny)
            # if not new_ok:
            #     new_failures += 1

            #     print("NEW RULE FAIL")
            #     print("x  =", format(x, f"0{w}b"))
            #     print("y  =", format(y, f"0{w}b"))
            #     print("x2 =", format(bad_x, f"0{w}b"))
            #     print("y2 =", format(bad_y, f"0{w}b"))
            #     print("kept x bits:", nx)
            #     print("kept y bits:", ny)
            #     break

            if (len(nx)+len(ny)) < (len(px)+len(py)):
                print("x  =", format(x, f"0{w}b"))
                print("y  =", format(y, f"0{w}b"))
                print("kept x bits:", nx)
                print("kept y bits:", ny)
                new_better += 1
                show_xy(x,y,4,nx,ny)
            if len(optimal_x) + len(optimal_y) == (len(nx)+len(ny)):
                optimal += 1
            else:
                unoptimal_operands.append((x,y,nx,ny,optimal_x,optimal_y))

    print("total optimal:",optimal)
    print('-----------------------------------------------------------------')
    print("UNOPTIMAL")
    for x, y, nx, ny, optimal_x, optimal_y in unoptimal_operands:
            print("\nx =", format(x, f"0{w}b"))
            print("y =", format(y, f"0{w}b"))
            print("\nNEW RULE:")
            show_xy(x, y, w, nx, ny)
            print("GROUND TRUTH:")
            show_xy(x, y, w, optimal_x, optimal_y)






test_all_comb(w=4,h=3,l=2)



# eval framework, generalize ground truth to all rules
# incorrect, imprecise, and precise