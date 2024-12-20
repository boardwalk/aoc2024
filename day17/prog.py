#!/usr/bin/env python3
from typing import Optional, Tuple

OUTPUT = [2, 4, 1, 3, 7, 5, 1, 5, 0, 3, 4, 1, 5, 5, 3, 0]

def get_val(lst: list[Optional[bool]], i: int) -> Optional[bool]:
    try:
        return lst[i]
    except KeyError:
        return None

class Bitlen:
    len: Optional[int]
    def __init__(self) -> None:
        self.len = None
    def __str__(self) -> str:
        return f'Bitlen = {self.len}'
pass

class State:
    # reg to step to bit to value
    # bit 0 is the least sig bit
    data: list[list[list[Optional[bool]]]]
    # reg to step to len
    lengths: list[list[Bitlen]]
    # the length of the inner list is an upper limit on the length of that value

    def __init__(self) -> None:
        self.data = []
        self.lengths = []
        self.ip = 7
        self.num_unprints = 0
        self.step = 0

    def score(self) -> int:
        score = 0
        for reg_data in self.data:
            for step_data in reg_data:
                for bit in step_data:
                    if bit is not None:
                        score += 1
        return score

    def cleanup(self) -> None:
        for reg_data in self.data:
            for step_data in reg_data:
                step_data_clean = list(step_data)
                for i, val in enumerate(step_data):
                    if val is None:
                        continue
                    while i >= len(step_data_clean):
                        step_data_clean.append(None)
                    step_data_clean[i] = val
                step_data.clear()
                step_data.extend(step_data_clean)

    # return true if there was a reset
    def step_back(self) -> bool:
        # print(f'on step {self.step}, ip is {self.ip}, num_unprints is {self.num_unprints}')
        # print(f'data is {self.data}')
        a, a_len = self.get_carried(0, self.step)
        b, b_len = self.get_carried(1, self.step)
        c, c_len = self.get_carried(2, self.step)
        match self.ip:
            case 0:
                # b = a & 7
                # make ourselves ignorant of high bits of b
                for i in range(len(b)):
                    if i >= 2:
                        b[i] = None
            case 1:
                # b ^= 3
                if 0 < len(b) and b[0] is not None:
                    b[0] = not b[0]
                if 1 < len(b) and b[1] is not None:
                    b[1] = not b[1]
            case 2:
                # c >>= b
                # i think we must know b exactly to undo this...
                print(b_len)
            case 3:
                # b ^= 5
                if 0 < len(b) and b[0] is not None:
                    b[0] = not b[0]
                if 2 < len(b) and b[2] is not None:
                    b[2] = not b[2]
            case 4:
                # a >>= 3
                a.insert(0, None)
                a.insert(0, None)
                a.insert(0, None)
                if a_len.len is not None:
                    a_len.len += 3
            case 5:
                pass
                # b ^= c
                for i in range(len(b)):
                     lhs = b[i]
                     rhs = get_val(c, i)
                     if lhs is not None and rhs is not None:
                         b[i] = lhs^rhs
                     else:
                        b[i] = None
            case 6:
                # out a & 8
                self.num_unprints += 1
                val = OUTPUT[-self.num_unprints]
                while len(a) < 3:
                    a.append(None)
                a[0] = val & 1 != 0
                a[1] = val & 2 != 0
                a[2] = val & 4 != 0
            case 7:
                pass
            case _:
                raise RuntimeError('bad ip')

        if self.num_unprints == len(OUTPUT):
            # a and b are 0 on entrace
            a.clear()
            b.clear()
            a_len.len = 0
            b_len.len = 0
            self.cleanup()
            self.num_unprints = 0
            self.ip = 7
            self.step = 0
            return True

        if self.ip > 0:
            self.ip -= 1
        else:
            self.ip = 7
        self.step += 1
        return False


    def get(self, reg: int, step: int) -> list[Optional[bool]]:
        while reg >= len(self.data):
            self.data.append([])

        while step >= len(self.data[reg]):
            self.data[reg].append([])
        return self.data[reg][step]

    def get_len(self, reg:int, step: int) -> Bitlen:
        while reg >= len(self.lengths):
            self.lengths.append([])
        while step >= len(self.lengths[reg]):
            self.lengths[reg].append(Bitlen())
        return self.lengths[reg][step]

    def get_carried(self, reg: int, step: int) -> Tuple[list[Optional[bool]], Bitlen]:
        before = self.get(reg, step)
        after = self.get(reg, step + 1)
        before_len = self.get_len(reg, step)
        after_len = self.get_len(reg, step + 1)
        after.clear()
        after.extend(before)
        after_len.len = before_len.len
        return (after, after_len)

def eval(a, out):
    b = 0
    c = 0
    cycles = 0
    while True:
        # 0
        b = a&7 # discard all but low 3 bits
        # 1
        b ^= 3 # flip bits (0b11)
        # 2
        c >>= b # drop b lsbs
        # 3
        b ^= 5 # flip bits (0b101)
        # 4
        a >>= 3 # drop 3 lsbs
        # 5
        b ^= c
        # 6
        out.append(b%8)
        # 7
        cycles += 1
        if a == 0:
            break
    print(f'{cycles}')

def main():
    out = []
    eval(21539243, out)
    print(out)
    # st = State()
    # for _i in range(250):
    #     print(st.score())
    #     st.step_back()
    # print(f'a = {st.data[0][-1]}')


if __name__ == '__main__':
    main()
