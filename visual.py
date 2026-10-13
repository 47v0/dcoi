def show_bits(value, width, keep):
    s = ""
    for i in range(width - 1, -1, -1):
        if i in keep:
            s += str((value >> i) & 1)
        else:
            s += "·"
    return s
def show_xy(x, y, w, keep_x, keep_y):
    print(" x  :", " ".join(format(x, f"0{w}b")))
    print("     ", " ".join(show_bits(x, w, keep_x)))
    print(" y  :", " ".join(format(y, f"0{w}b")))
    print("     ", " ".join(show_bits(y, w, keep_y)))
    print()


def AND(x, y):
    if x == 0:
        return ["x"]
    if y == 0:
        return ["y"]
    return ["x", "y"]

def OR(x, y):
    if x == 1:
        return ["x"]
    if y == 1:
        return ["y"]
    return ["x", "y"]

# Ante -> Conseq
def IMPLIES(ante, conseq):
    if ante == 0:
        return ["ante"]
    if conseq == 1:
        return ["conseq"]
    return ["ante", "conseq"]


def BVAND(x, y, h, l):
    keep_x = []
    keep_y = []

    for i in range(l, h + 1):
        xb = (x >> i) & 1
        yb = (y >> i) & 1

        if xb == 0:
            keep_x.append(i)
        elif yb == 0:
            keep_y.append(i)
        else:
            # 1 AND 1 = 1, so both must stay 1
            keep_x.append(i)
            keep_y.append(i)

    return keep_x, keep_y




def BVOR(x, y, h, l):
    keep_x = []
    keep_y = []

    for i in range(l, h + 1):
        xb = (x >> i) & 1
        yb = (y >> i) & 1

        if xb == 1:
            keep_x.append(i)

        elif yb == 1:
            keep_y.append(i)

        else:
            # 0 OR 0 = 0, need both zeros
            keep_x.append(i)
            keep_y.append(i)

    return keep_x, keep_y


# BVADD, pAPER RULE
def BVADD(x, y, h, l):
    keep_x = list(range(0, h + 1))
    keep_y = list(range(0, h + 1))
    return keep_x, keep_y
# new rule
def BVADD_new(x,y,h,l):
    keep_x = list(range(l, h + 1))
    keep_y = list(range(l, h + 1))
    # carry[i] = carry into bit i
    carry = [0] * (l + 1)

    # computing carry values (below l, we dont care abt s)
    for i in range(l):
        x_bit = (x >> i) & 1
        y_bit = (y >> i) & 1
        c_in = carry[i]
        c_out = ((x_bit & y_bit) | (x_bit & c_in) | (y_bit & c_in))

        carry[i + 1] = c_out

    # working backwards from the carry into bit l
    for i in range(l - 1, -1, -1):
        x_bit = (x >> i) & 1
        y_bit = (y >> i) & 1
        c_in = carry[i]

        # if x_i == y_i, they force the carry-out
        # regardless of the carry-in, so keep both and stop
        if x_bit == y_bit:
            keep_x.append(i)
            keep_y.append(i)
            break

        # otherwise x_i != y_i
        # carry-out equals carry-in
        #
        # keep whichever operand has the same value as c_in,
        # then continue downward to preserve c_in
        if x_bit == c_in:
            keep_x.append(i)
        else:
            keep_y.append(i)

    keep_x.sort()
    keep_y.sort()

    return keep_x, keep_y



print("BVAND")

x,y,w,h,l = 0b0010 ,0b0110 , 4 , 3 , 1

kx, ky = BVAND(x, y, h, l)
show_xy(x, y, w, kx, ky)


print("BVOR")

x = 0b1010
y = 0b0110

kx, ky = BVOR(x, y, h=3, l=1)
show_xy(x, y, 4, kx, ky)


print("BVADD")

x,y,w = 0b10110110, 0b00101101, 8
# kx, ky = BVADD(x, y, h=6, l=4)
kx, ky = BVADD_new(x,y,h=6,l=4)
print("x + y =", format((x + y) % (1 << w), f"0{w}b"))
show_xy(x, y, w, kx, ky)
