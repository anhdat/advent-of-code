# %%
# Helpers from norvig's pytudes
from collections import Counter, defaultdict, namedtuple, deque
from itertools import groupby, permutations, combinations, product, chain
from functools import lru_cache
from typing import Dict, Tuple, Set, List, Iterator, Optional, Union, Iterable
from typing import NamedTuple
from dataclasses import dataclass

import operator
import math
import ast
import sys
import re

# Parsing


def data(day: int, parser=str, sep="\n") -> list:
    "Split the day's input file into sections separated by `sep`, and apply `parser` to each."
    sections = (
        open(f"/Users/dat/Developer/advent-of-code/2021/inputs/day{day:02}/input.txt")
        .read()
        .rstrip()
        .split(sep)
    )
    return [parser(section) for section in sections]


def ints(text: str) -> Tuple[int]:
    "Return a tuple of all the integers in text."
    return tuple(map(int, re.findall("-?[0-9]+", text)))


def atom(text: str) -> Union[float, int, str]:
    "Parse text into a single float or int or str."
    try:
        val = float(text)
        return round(val) if round(val) == val else val
    except ValueError:
        return text


def atoms(text: str, ignore=r"", sep=None) -> Tuple[Union[int, str]]:
    "Parse text into atoms (numbers or strs), possibly ignoring a regex."
    if ignore:
        text = re.sub(ignore, "", text)
    return tuple(map(atom, text.split(sep)))


# Execution


def do(day, *answers) -> Dict[int, int]:
    "E.g., do(3) returns {1: day3_1(in3), 2: day3_2(in3)}. Verifies `answers` if given."
    g = globals()
    got = []
    for part in (1, 2):
        fname = f"day{day}_{part}"
        if fname in g:
            got.append(g[fname](g[f"in{day}"]))
            if len(answers) >= part:
                assert (
                    got[-1] == answers[part - 1]
                ), f"{fname}(in{day}) got {got[-1]}; expected {answers[part - 1]}"
    return got


class timeit:
    from datetime import datetime

    def __enter__(self):
        self.tic = self.datetime.now()

    def __exit__(self, *args, **kwargs):
        print("runtime: {}".format(self.datetime.now() - self.tic))


def quantify(iterable, pred=bool) -> int:
    "Count the number of items in iterable for which pred is true."
    return sum(1 for item in iterable if pred(item))


def first(iterable, default=None) -> object:
    "Return first item in iterable, or default."
    return next(iter(iterable), default)


def rest(sequence) -> object:
    return sequence[1:]


def multimap(items: Iterable[Tuple]) -> dict:
    "Given (key, val) pairs, return {key: [val, ....], ...}."
    result = defaultdict(list)
    for (key, val) in items:
        result[key].append(val)
    return result


def dotproduct(A, B) -> float:
    return sum(a * b for a, b in zip(A, B))


def mapt(fn, *args):
    "map(fn, *args) and return the result as a tuple."
    return tuple(map(fn, *args))


cat = "".join
flatten = chain.from_iterable

# %%

in1: list[int] = data(1, int)


def day1_1(nums: list[int]):
    return quantify(x > y for (x, y) in zip(nums[1:], nums))


def day1_2(nums: list[int]):
    return day1_1([sum(t) for t in zip(nums[: len(nums) - 2], nums[1:], nums[2:])])


do(1, 1374, 1418)
# %%

Command = tuple[str, int]


def parse_command(line: str) -> Command:
    (direction, amount) = line.split()
    return (direction, int(amount))


in2: list[Command] = data(2, parse_command)


def day2_1(commands: list[Command]):
    x = 0
    z = 0
    for c in commands:
        match c[0]:
            case "forward":
                x += c[1]
            case "down":
                z += c[1]
            case "up":
                z -= c[1]

    return x * z


def day2_2(commands: list[Command]):
    x = 0
    z = 0
    aim = 0
    for c in commands:
        match c[0]:
            case "forward":
                x += c[1]
                z += aim * c[1]
            case "down":
                aim += c[1]
            case "up":
                aim -= c[1]

    return x * z


do(2, 2073315, 1840311528)

# %%
# Day 3
in3: list[str] = data(3)

lines = """00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010""".splitlines()


def day3_1(lines: list[str]):
    count_zeros = [g.count("0") > len(lines) / 2 for g in zip(*lines)]
    gamma = ["0" if c else "1" for c in count_zeros]
    epsilon = ["1" if c else "0" for c in count_zeros]
    return int("".join(gamma), 2) * int("".join(epsilon), 2)


assert day3_1(lines) == 198


def find_life_supp_rating(lines, most_common=True):
    current_index = 0
    while len(lines) > 1:
        chars = [l[current_index] for l in lines]
        mcs = Counter(chars).most_common(2)
        if mcs[0][1] == mcs[1][1]:
            c_char = "1" if most_common else "0"
        else:
            c_char = mcs[0][0] if most_common else mcs[1][0]
        lines = [l for l in lines if l[current_index] == c_char]
        current_index += 1
    return lines[0]


assert find_life_supp_rating(lines) == "10111"
assert find_life_supp_rating(lines, most_common=False) == "01010"


def day3_2(lines: list[str]):
    return int(find_life_supp_rating(lines), 2) * int(
        find_life_supp_rating(lines, most_common=False), 2
    )


do(3, 4001724, 587895)

# %%
# Day 4

Ticket = list[list[int]]


class BingoGame:
    @classmethod
    def from_input(cls, sections):
        game = BingoGame()
        game.draws = [int(num) for num in sections[0].split(",")]
        game.tickets = [
            [[int(num) for num in line.split()] for line in section.splitlines()]
            for section in sections[1:]
        ]
        return game


bingo_test = """7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7""".split(
    "\n\n"
)
bingo_test
test_in4 = BingoGame.from_input(bingo_test)
test_in4


def day4_1(game: BingoGame) -> int:
    sets: list[tuple[set[int], int]] = []
    for i, ticket in enumerate(game.tickets):
        for row in ticket:
            sets.append((set(row), i))
        for col in range(len(ticket[0])):
            sets.append(({row[col] for row in ticket}, i))

    current_draws = set()
    remaining_draws = game.draws.copy()
    while True:
        last_draw = remaining_draws.pop(0)
        current_draws.add(last_draw)
        for (s, i) in sets:
            if len(s.intersection(current_draws)) == 5:
                return sum(set(flatten(game.tickets[i])) - current_draws) * last_draw


assert day4_1(test_in4) == 4512


def day4_2(game: BingoGame) -> int:
    sets: list[tuple[set[int], int]] = []
    for i, ticket in enumerate(game.tickets):
        for row in ticket:
            sets.append((set(row), i))
        for col in range(len(ticket[0])):
            sets.append(({row[col] for row in ticket}, i))

    current_draws = set()
    remaining_draws = game.draws.copy()
    winning_tickets = []
    while len(winning_tickets) < len(game.tickets):
        last_draw = remaining_draws.pop(0)
        current_draws.add(last_draw)

        for (s, i) in sets:
            if i in winning_tickets:
                continue

            if len(s.intersection(current_draws)) == 5:
                winning_tickets.append(i)
                if len(winning_tickets) == len(game.tickets):
                    return (
                        sum(set(flatten(game.tickets[i])) - current_draws) * last_draw
                    )


in4 = BingoGame.from_input(data(4, sep="\n\n"))
do(4, 74320, 17884)

# %%
Point = NamedTuple("Point", x=int, y=int)
Line = tuple[Point, Point]


def parse_line(line: str) -> Line:
    a, b, c, d = ints(line)
    return (Point(a, b), Point(c, d))


in5: list[Line] = data(5, parse_line)


def day5_1(lines: list[Line]):
    points: list[Point] = []
    for (p1, p2) in lines:
        if p1.x == p2.x or p1.y == p2.y:
            # since 2 points have the same x or y
            # the rectangle becomes a line
            points += [
                (x, y)
                for x in range(min(p1.x, p2.x), max(p1.x, p2.x) + 1)
                for y in range(min(p1.y, p2.y), max(p1.y, p2.y) + 1)
            ]
    return quantify(Counter(points).values(), lambda v: v > 1)


test_in5 = """0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2""".splitlines()
lines = [parse_line(l) for l in test_in5]
day5_1(lines)
# %%
def day5_2(lines: list[Line]):
    def diagonal_coordinates(c1: int, c2: int) -> list[int]:
        if c1 < c2:
            return [x for x in range(c1, c2 + 1)]
        return [x for x in range(c1, c2 - 1, -1)]

    points: list[Point] = []
    for (p1, p2) in lines:
        if p1.x == p2.x or p1.y == p2.y:
            # since 2 points have the same x or y
            # the rectangle becomes a line
            points += [
                (x, y)
                for x in range(min(p1.x, p2.x), max(p1.x, p2.x) + 1)
                for y in range(min(p1.y, p2.y), max(p1.y, p2.y) + 1)
            ]
        else:
            xs, ys = diagonal_coordinates(p1.x, p2.x), diagonal_coordinates(p1.y, p2.y)
            points += zip(xs, ys)
    return quantify(Counter(points).values(), lambda v: v > 1)


day5_2(lines)
# %%
do(5, 5690, 17741)
# %%

in6: list[int] = data(6, int, sep=",")
in6


def count_fish(init_fish: list[int], days_count) -> int:
    fs = [0] * 9
    for f in init_fish:
        fs[f] += 1
    for _ in range(days_count):
        fs = fs[1:] + fs[:1]
        fs[6] += fs[8]
    return sum(fs)


def day6_1(nums: list[int]) -> int:
    return count_fish(nums, 80)


def day6_2(nums: list[int]) -> int:
    return count_fish(nums, 256)


nums = [3, 4, 3, 1, 2]
assert day6_1(nums) == 5934
do(6, 395627, 1767323539209)
# %%
# Day 7

in7: list[int] = data(7, int, ",")

nums = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14]
import statistics


def day7_1(nums: list[int]) -> int:
    m = statistics.median(nums)
    return int(sum(abs(m - i) for i in nums))


def day7_2(nums: list[int]) -> int:
    def cal(target: int) -> int:
        ns = [abs(target - i) for i in nums]
        fs = [(n * (n + 1) / 2) for n in ns]
        return int(sum(fs))

    avg = sum(nums) / len(nums)
    return min(cal(math.ceil(avg)), cal(math.floor(avg)))


assert day7_1(nums) == 37
assert day7_2(nums) == 168
do(7, 335271, 95851339)
# %%
# Day 8
SignalPattern = str
OutVal = str
SignalLine = (list[SignalPattern], list[OutVal])


def parse_signal_lines(lines: list[str]) -> list[SignalLine]:
    ls = []
    for l in lines:
        cs = l.split()
        ls.append((cs[:10], cs[11:]))
    return ls


in8: list[SignalLine] = parse_signal_lines(data(8))
in8


def day8_1(lines: list[SignalLine]) -> int:
    return sum(
        [quantify([len(s) for s in l[1]], lambda v: v in [2, 3, 4, 7]) for l in lines]
    )


test_input = """be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce""".splitlines()
test_lines = parse_signal_lines(test_input)
assert day8_1(lines) == 26

"""
  0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg

length
1: 2
7: 3
4: 4

2: 5
3: 5
5: 5

6: 6
0: 6
9: 6

8: 7
"""


def signals_to_map(signals: list[SignalPattern]):
    """
     0000
    1    2
    1    2
     3333
    4    5
    4    5
     6666
    """
    gs = {k: list(v) for k,v in groupby(sorted(signals, key=len), key=len)}
    sf = lambda s: set(first(s))

    one = sf(gs[2])
    seven = sf(gs[3])
    four = sf(gs[4])
    eight = sf(gs[7])
    three = sf([p for p in gs[5] if len(set(p) & one) == 2])
    six = sf([p for p in gs[6] if len(set(p) & one) == 1])
    nine = sf([p for p in gs[6] if len(set(p) & three) == 5])

    index_0 = seven - one
    index_2 = one - six
    index_5 = one - index_2
    index_4 = eight - nine
    index_1 = nine - three
    index_3 = four - (one | index_1)
    index_6 = three - (seven | index_3)
    return [
        first(i)
        for i in (index_0, index_1, index_2, index_3, index_4, index_5, index_6)
    ]


def output_to_num(output: OutVal, m: list[str]) -> int:
    """
     0000
    1    2
    1    2
     3333
    4    5
    4    5
     6666
    """
    nums = [
        set([m[0], m[1], m[2], m[4], m[5], m[6]]),  # 0
        set([m[2], m[5]]),
        set([m[0], m[2], m[3], m[4], m[6]]),  # 2
        set([m[0], m[2], m[3], m[5], m[6]]),
        set([m[1], m[2], m[3], m[5]]),  # 4
        set([m[0], m[1], m[3], m[5], m[6]]),
        set([m[0], m[1], m[3], m[4], m[5], m[6]]),  # 6
        set([m[0], m[2], m[5]]),
        set([m[0], m[1], m[2], m[3], m[4], m[5], m[6]]),  # 8
        set([m[0], m[1], m[2], m[3], m[5], m[6]]),
    ]

    output = set(output)
    for i, n in enumerate(nums):
        if len(output) == len(n) and len(output & n) == len(n):
            return i

    return -1


lines = test_lines


def day8_2(lines: list[SignalLine]) -> int:
    s = 0
    for l in lines:
        m = signals_to_map(l[0])
        nums = "".join([str(n) for n in [output_to_num(n, m) for n in l[1]]])
        s += int(nums)
    return s


do(8, 255, 982158)
