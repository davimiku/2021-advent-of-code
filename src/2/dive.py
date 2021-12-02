from __future__ import annotations
from dataclasses import dataclass


@dataclass
class WithAmount:
    amount: int


class Depth(WithAmount):
    def __add__(self, other: 'Depth'):
        return Depth(self.amount + other.amount)


class HorizontalPosition(WithAmount):
    def __add__(self, other: 'HorizontalPosition'):
        return HorizontalPosition(self.amount + other.amount)


def parse_line(line: str) -> WithAmount:
    split = line.split(' ')
    direction = split[0]
    amount = int(split[1])

    if direction == 'forward':
        return HorizontalPosition(amount)
    if direction == 'down':
        return Depth(amount)
    if direction == 'up':
        return Depth(-amount)


def main():
    depth = Depth(0)
    horizontal_position = HorizontalPosition(0)
    with open('./src/2/data.txt', 'r') as file:
        lines = file.readlines()
        for line in lines:
            movement = parse_line(line)
            if isinstance(movement, Depth):
                depth += movement
            elif isinstance(movement, HorizontalPosition):
                horizontal_position += movement
    print(depth)
    print(horizontal_position)
    print('multiplied', depth.amount * horizontal_position.amount)


if __name__ == '__main__':
    main()
