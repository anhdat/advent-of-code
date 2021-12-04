# %%
# Helpers from norvig's pytudes
from collections import Counter, defaultdict, namedtuple, deque
from itertools import permutations, combinations, product, chain
from functools import lru_cache
from typing import Dict, Tuple, Set, List, Iterator, Optional, Union, Iterable

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
    count_zeros = [g.count("0") > len(lines)/2 for g in zip(*lines)]
    gamma = ["0" if c else "1" for c in count_zeros]
    epsilon = ["1" if c else "0" for c in count_zeros]
    return int("".join(gamma), 2) * int("".join(epsilon), 2)

assert day3_1(lines) == 198

def find_life_supp_rating(lines, most_common=True):
    current_index = 0
    while len(lines)>1:
        chars = [l[current_index] for l in lines]
        mcs = Counter(chars).most_common(2)
        if mcs[0][1] == mcs[1][1]:
            c_char = '1' if most_common else '0'
        else:
            c_char = mcs[0][0] if most_common else mcs[1][0]
        lines = [l for l in lines if l[current_index] == c_char]
        current_index += 1
    return lines[0]


assert find_life_supp_rating(lines) == '10111'
assert find_life_supp_rating(lines, most_common=False) == '01010'

def day3_2(lines: list[str]):
    return int(find_life_supp_rating(lines), 2) * int(find_life_supp_rating(lines, most_common=False), 2)

do(3, 4001724, 587895)

# %%
# Day 4

Ticket = list[list[int]]


class BingoGame:
    def __init__(self) -> None:
        self.draws = []
        self.tickets = []

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
    remaining_draws = game.draws
    while True:
        last_draw = remaining_draws[0]
        remaining_draws = remaining_draws[1:]
        current_draws.add(last_draw)
        for (s, i) in sets:
            if len(s.intersection(current_draws)) == 5:
                return sum(set(flatten(game.tickets[i])) - current_draws) * last_draw


assert day4_1(test_in4) == 4512

in4 = BingoGame.from_input(data(4, sep="\n\n"))


def day4_2(game: BingoGame) -> int:
    sets: list[tuple[set[int], int]] = []
    for i, ticket in enumerate(game.tickets):
        for row in ticket:
            sets.append((set(row), i))
        for col in range(len(ticket[0])):
            sets.append(({row[col] for row in ticket}, i))

    current_draws = set()
    remaining_draws = game.draws
    winning_tickets = []
    while len(winning_tickets) < len(game.tickets):
        last_draw = remaining_draws[0]
        remaining_draws = remaining_draws[1:]
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


do(4, 74320, 17884)
