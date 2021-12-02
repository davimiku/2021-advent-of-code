def parse_line(line: str):
    split = line.split(' ')
    direction = split[0]
    amount = int(split[1])

    return direction, amount


def main():
    depth = 0
    horizontal_position = 0
    aim = 0

    with open('./src/2/data.txt', 'r') as file:
        lines = file.readlines()
        for line in lines:
            direction, amount = parse_line(line)
            if direction == 'down':
                aim += amount
            elif direction == 'up':
                aim -= amount
            elif direction == 'forward':
                horizontal_position += amount
                depth += aim * amount

    print('depth:', depth)
    print('horizontal_position:', horizontal_position)
    print('multiplied:', depth * horizontal_position)


if __name__ == '__main__':
    main()
