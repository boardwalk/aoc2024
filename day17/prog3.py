#!/usr/bin/env python3
import time

MIN_VAL = 0b1000000000000000000000000000000000000000000000
MAX_VAL = 0b111111111111111111111111111111111111111111111111
# EXPECTED = [6, 7, 5, 2, 1, 3, 5, 1, 7]
FINAL = [2, 4, 1, 3, 7, 5, 1, 5, 0, 3, 4, 1, 5, 5, 3, 0]


def eval(a, out):
    out.append((a ^ 0b110 ^ a >> (0b11 ^ a & 0b111)) & 0b111)
    if (a >> 3) != 0:
        eval(a=(a >> 3), out=out)

def main():
    out = []

    # part 1
    out.clear()
    eval(21539243, out)
    print(f'part 1 = {out}')
    # part 2
    out.clear()
    eval(214456852483120, out)
    print(f'part 2 = {out}')
    print(f'final  = {FINAL}')

if __name__ == '__main__':
    main()
