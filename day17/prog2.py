#!/usr/bin/env python3
from typing import Optional
from enum import Enum
from abc import ABC, abstractmethod

EXPECTED = [6, 7, 5, 2, 1, 3, 5, 1, 7]

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

def eval(a, c, out):
    # 2,4 bst (1)
    b = a & 7
    # 1,3 bxl (2)
    b = b ^ 3
    # 7,5 cdv (3)
    c = a >> b
    # 1,5 bxl (4)
    b = b ^ 5
    # 0,3 adv (5)
    a = a >> 3
    # 4,1 bxc (6)
    b = b ^ c
    # 5,5 out (7)
    out.append(b % 8)
    # 3,0 jnz (8)
    if a != 0:
        eval(a=a, c=c, out=out)


def main():
    # st = State()
    # for i in range(5):
    #     st.step()
    #     print(f'{st}')
    out = []
    eval(a=21539243, c=0, out=out)
    print(out)
    assert out == EXPECTED

if __name__ == '__main__':
    main()