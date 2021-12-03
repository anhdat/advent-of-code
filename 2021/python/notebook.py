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
