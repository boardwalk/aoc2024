#!/usr/bin/env python3
from typing import Optional
from enum import Enum
from abc import ABC, abstractmethod
import time

EXPECTED = [6, 7, 5, 2, 1, 3, 5, 1, 7]
FINAL = [2,4,1,3,7,5,1,5,0,3,4,1,5,5,3,0]

def flip_bit(val: Optional[bool]) -> Optional[bool]:
    return not val if val is not None else None


class Register(Enum):
    A = 0
    B = 1
    C = 2


class Instruction(ABC):
    def __init__(self):
        self.num_bits = 0

    def reset(self) -> None:
        self.num_bits = 0

    @staticmethod
    @abstractmethod
    def sources() -> list[Register]:
        pass

    @staticmethod
    @abstractmethod
    def target() -> Register:
        pass

    @abstractmethod
    def forward(self, input: list[Optional[bool]]) -> Optional[bool]:
        pass

    @abstractmethod
    def back(self, input: list[Optional[bool]]) -> Optional[bool]:
        pass

class Instruction1(Instruction):
    @staticmethod
    def sources() -> list[Register]:
        return [Register.A]

    @staticmethod
    def target() -> Register:
        return Register.A

    # filter bits after 0,1,2
    def forward(self, input: list[Optional[bool]]) -> Optional[bool]:
        self.num_bits += 1
        return None if self.num_bits > 3 else input[0]

    # filter bits before 0,1,2
    def back(self, input: list[Optional[bool]]) -> Optional[bool]:
        self.num_bits += 1
        return None if self.num_bits < 3 else input[0]

class Instruction2(Instruction):
    @staticmethod
    def sources() -> list[Register]:
        return [Register.B]

    @staticmethod
    def target() -> Register:
        return Register.B

    # flip bits 0,1
    def forward(self, input: list[Optional[bool]]) -> Optional[bool]:
        self.num_bits += 1
        if self.num_bits in (1, 2):
            return flip_bit(input[0])
        else:
            return input

    def back(self, input: list[Optional[bool]]) -> Optional[bool]:
        # identical both ways
        return self.forward(input)

PROGRAM: list[Instruction] = [
    Instruction1(),
    Instruction2()
]


def make_bit_list(input: int) -> list[bool]:
    output = []
    while input != 0:
        output.append(input & 1 != 0)
        input>>=1
    return output

class State:
    def __init__(self):
        self.a: list[Optional[bool]] = make_bit_list(21539243)
        self.b: list[Optional[bool]] = []
        self.c: list[Optional[bool]] = []
        self.ip = 0
        self.out = []

    def get_reg(self, reg: Register) -> list[Optional[bool]]:
        match reg:
            case Register.A:
                return self.a
            case Register.B:
                return self.b
            case Register.C:
                return self.c
            case _:
                raise RuntimeError('bad register')

    def step(self) -> None:
        instr = PROGRAM[self.ip]
        instr.reset()
        sources = [list(self.get_reg(reg)) for reg in instr.sources()]
        target = self.get_reg(instr.target())
        target.clear()
        eval_len = min(len(source) for source in sources)
        for bit_num in range(eval_len):
            res = instr.forward([source[bit_num] for source in sources])
            target.append(res)
        self.ip += 1

    def __str__(self) -> str:
        return f'a={self.a}, b={self.b}, c={self.c}'

class Context:
    def __init__(self):
        self.remaining = list(EXPECTED)
        self.known = {}

    def shift_up(self):
        pass

def eval(a, out):
    out.append((a ^ 0b110 ^ a >> (0b11 ^ a & 0b111)) & 0b111)
    if (a >> 3) != 0:
        eval(a=(a >> 3), out=out)

def eval2(ctx: Context):
    # (1)
    # p4 = ((a ^ 0b11) & 0b111)
    # (2)
    # p3 = 0b110
    # (3)
    # p2 = p3 >> p4
    # (4)
    # p1 = p2 & 0b111
    # (5)
    # out.append(p1)

    # if (a >> 3) != 0:
    # (6)
    #     eval(a=(a >> 3), out=out)
    ctx.shift_up()
    val = ctx.remaining.pop()
    # (4) force p2 low bits
    # (3) choose all shifts



def cmp(out: list[int]) -> int:
    for l, r in zip(out, EXPECTED):
        if l + 4 > r:
            return 1
        elif r + 4 > l:
            return -1
        else:
            raise RuntimeError('jitter')




def out_to_num(out: list[int]) -> int:
    res = 0
    for v in out:
        res *8 + v
    return res

def dist(out: list[int]) -> int:
    for l, r in zip(out, EXPECTED):
        if l + 4 > r:
            return 1
        elif r + 4 > l:
            return -1
        else:
            raise RuntimeError('jitter')


def main():
    # ctx = []
    # for out in reversed(EXPECTED):
    #     eval2(ctx, out)
    # print(ctx)



    # st = State()
    # for i in range(5):
    #     st.step()
    #     print(f'{st}')
    out = []

    MIN_VAL = 0b1000000000000000000000000000000000000000000000
    MAX_VAL = 0b111111111111111111111111111111111111111111111111
    # a = (MAX_VAL - MAX_VAL) * 9 // 8 + MIN_VAL
    # a =  MAX_VAL - 242_290_604_621_823
    # 246_290_604_621_823
    lo = MIN_VAL
    hi = MAX_VAL

    # a = MIN_VAL + MAX_VAL // 2
    while True:
        m =(lo + hi) // 2
        out.clear()
        eval(a=m, out=out)
        print(f'a={m}, out={out}')

        out_num = out_to_num(out)
        expected_num = out_to_num(EXPECTED)
        dist = abs(out_num - expected_num)
        print(f'{dist}')
        assert len(out) == 16
        c = cmp(out)
        if c < 0:
            # go right
            print('left')
            lo = m  + 1
        else:
            print('right')
            hi = m - 1
        time.sleep(0.2)

        # a += 2 * 32
        # a -= 1
        # print(out)

    # print(len(out))
    print(out)
    # assert out == EXPECTED
    # ctx = Context()
    # eval2(ctx)

if __name__ == '__main__':
    main()