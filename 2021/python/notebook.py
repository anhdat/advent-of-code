# %%
# Helpers from norvig's pytudes
from collections import Counter, defaultdict, namedtuple, deque
from itertools import groupby, permutations, combinations, product, chain
from functools import lru_cache, reduce
from typing import Dict, Tuple, Set, List, Iterator, Optional, Union, Iterable
from typing import NamedTuple
from dataclasses import dataclass
import statistics

import operator
import math
import ast
import sys
import re
import copy

# Parsing


def data(day: int, parser=str, sep="\n") -> list:
    "Split the day's input file into sections separated by `sep`, and apply `parser` to each."
    sections = open(f"../inputs/day{day:02}/input.txt").read().rstrip().split(sep)
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
# Day 1

in1: list[int] = data(1, int)


def day1_1(nums: list[int]):
    return quantify(x > y for (x, y) in zip(nums[1:], nums))


def day1_2(nums: list[int]):
    return day1_1([sum(t) for t in zip(nums[: len(nums) - 2], nums[1:], nums[2:])])


do(1, 1374, 1418)
# %%
# Day 2

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
# Day 5

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
assert day5_1(lines) == 5


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


assert day5_2(lines) == 12
do(5, 5690, 17741)
# %%
# Day 6

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
assert day8_1(test_lines) == 26

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
    signals = [cat(sorted(v)) for v in signals]
    gs = {k: set(v) for k, v in groupby(sorted(signals, key=len), key=len)}
    sf = lambda s: set(first(s))

    one = sf(gs[2])
    seven = sf(gs[3])
    four = sf(gs[4])
    eight = sf(gs[7])
    three = sf([p for p in gs[5] if len(set(p) & one) == 2])
    six = sf([p for p in gs[6] if len(set(p) & one) == 1])
    nine = sf([p for p in gs[6] if len(set(p) & three) == 5])
    five = sf([p for p in gs[5] if len(set(p) & (four - one)) == 2])
    two = gs[5] - {cat(sorted(five))} - {cat(sorted(three))}
    zero = gs[6] - {cat(sorted(six))} - {cat(sorted(nine))}

    a = [
        cat(sorted(i))
        for i in (zero, one, two, three, four, five, six, seven, eight, nine)
    ]
    return a


def output_to_num(output: OutVal, m: list[str]) -> int:
    output = cat(sorted(output))
    return first([i for i, n in enumerate(m) if output == n])


lines = test_lines
m = signals_to_map(lines[0][0])
assert [output_to_num(n, m) for n in lines[0][1]] == [8, 3, 9, 4]
m = signals_to_map(lines[1][0])
assert [output_to_num(n, m) for n in lines[1][1]] == [9, 7, 8, 1]


def day8_2(lines: list[SignalLine]) -> int:
    s = 0
    for l in lines:
        m = signals_to_map(l[0])
        nums = "".join([str(n) for n in [output_to_num(n, m) for n in l[1]]])
        s += int(nums)
    return s


do(8, 255, 982158)
# %%
# Day 9

in9: list[list[int]] = data(9, lambda v: [int(i) for i in v])
Point = NamedTuple("Point", x=int, y=int)
Table = list[list[int]]

directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]


def neighbors(p: Point, table: Table) -> list[Point]:
    return [
        Point(y=p.y + dy, x=p.x + dx)
        for dx, dy in directions
        if 0 <= p.x + dx < len(table[0]) and 0 <= p.y + dy < len(table)
    ]


def is_low_point(p: Point, table: Table) -> bool:
    neighbor_vals = [table[p.y][p.x] for p in neighbors(p, table)]
    return all([table[p.y][p.x] < i for i in neighbor_vals])


def day9_1(table: list[list[int]]) -> int:
    return sum(
        [
            table[r][c] + 1
            for r in range(len(table))
            for c in range(len(table[r]))
            if is_low_point(Point(x=c, y=r), table)
        ]
    )


test_lines = """2199943210
3987894921
9856789892
8767896789
9899965678""".splitlines()
lines = [[int(i) for i in s] for s in test_lines]
lines
assert day9_1(lines) == 15


def find_basin(low_point: Point, table: Table) -> list[Point]:
    q = deque()
    q.append(low_point)
    bs = set()
    visisteds = set()
    while q:
        p = q.popleft()
        visisteds.add(p)
        bs.add(p)
        for np in neighbors(p, table):
            if table[np.y][np.x] < 9 and np not in visisteds:
                q.append(np)

    return list(bs)


def day9_2(table: list[list[int]]) -> int:
    low_points: list[Point] = [
        Point(x=c, y=r)
        for r in range(len(table))
        for c in range(len(table[r]))
        if is_low_point(Point(x=c, y=r), table)
    ]

    bss = [find_basin(p, table) for p in low_points]
    bss_lengths = sorted(list(map(len, bss)), reverse=True)
    return math.prod(bss_lengths[:3])


assert day9_2(lines) == 1134

do(9, 564)
# %%
# Day 10

in10: list[str] = data(10)

m = {
    ")": "(",
    "}": "{",
    "]": "[",
    ">": "<",
}


def is_corrupted(line: str) -> tuple[bool, str]:
    stack = []
    for c in line:
        if c in "([{<":
            stack.append(c)
        else:
            cc = stack.pop()
            if cc != m[c]:
                return (True, c)
    return (False, "")


def day10_1(lines: list[str]) -> int:
    points = {
        ")": 3,
        "}": 1197,
        "]": 57,
        ">": 25137,
    }
    cs = []
    for l in lines:
        (isc, c) = is_corrupted(l)
        if isc:
            cs.append(c)
    s = 0
    for k, v in Counter(cs).items():
        s += points[k] * v

    return s


def cal_inc_score(added: str) -> int:
    ps = {
        ")": 1,
        "]": 2,
        "}": 3,
        ">": 4,
    }
    s = 0
    for c in added:
        s *= 5
        s += ps[c]
    return s


cal_inc_score(r"}}]])})]")


def is_incomplete(line: str) -> tuple[bool, int]:
    m2 = {
        "(": ")",
        "{": "}",
        "[": "]",
        "<": ">",
    }
    stack = []
    for c in line:
        if c in "([{<":
            stack.append(c)
        else:
            cc = stack.pop()
            if cc != m[c]:
                return (False, c)
    stack.reverse()
    return (True, [m2[c] for c in stack])


def day10_2(lines):
    incs = []
    for l in lines:
        (is_inc, addeds) = is_incomplete(l)
        if is_inc:
            incs.append(addeds)
    inc_scores = [cal_inc_score(a) for a in incs]
    return statistics.median(inc_scores)


do(10, 339477, 3049320156)

# %%
# Day 11

directions = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
]
in11: list[list[int]] = data(11, lambda v: [int(a) for a in v])

Point = tuple[int, int]
Table = list[list[int]]


def next(m: Table) -> int:
    q = deque()
    for (r, c) in product(range(len(m)), range(len(m[0]))):
        m[r][c] += 1
        if m[r][c] > 9:
            q.append((c, r))

    flashes = 0
    while q:
        (c, r) = q.pop()
        flashes += 1
        m[r][c] = 0
        for (dx, dy) in directions:
            x = c + dx
            y = r + dy
            if 0 <= x < len(m[0]) and 0 <= y < len(m):
                if m[y][x] <= 9 and m[y][x] != 0:
                    m[y][x] += 1
                    if m[y][x] > 9:
                        q.append((x, y))
    return flashes


def day11_1(m: Table) -> int:
    m = copy.deepcopy(m)
    c = 0
    for _ in range(100):
        c += next(m)
    return c


m = """5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526""".splitlines()
m = [[int(i) for i in l] for l in m]
assert day11_1(m) == 1656


def day11_2(m: Table) -> int:
    c = 0
    i = 0
    while c != len(m) * len(m[0]):
        c = next(m)
        i += 1
    return i


do(11, 1603, 222)

# %%
# Day 12

Path = tuple[str, str]

in12: list[Path] = data(12, lambda l: (l.split("-")[0], l.split("-")[1]))
in12


def count_paths_until(v, visiteds, graph, path_count):
    if v.islower():
        visiteds.add(v)

    if v == "end":
        path_count[0] += 1
    else:
        i = 0
        while i < len(graph[v]):
            if graph[v][i] not in visiteds:
                count_paths_until(graph[v][i], visiteds, graph, path_count)
            i += 1

    if v.islower():
        visiteds.remove(v)


def day12_1(paths: list[Path]) -> int:
    graph = defaultdict(list)
    for p in paths:
        graph[p[0]].append(p[1])
        graph[p[1]].append(p[0])
    path_count = [0]
    visiteds = set()
    count_paths_until("start", visiteds, graph, path_count)

    return path_count[0]


def count_paths_bfs(graph) -> int:
    q = deque()

    path = ["start"]
    q.append(path.copy())

    def should_add(vertex: str, path: list[str]) -> bool:
        return (
            vertex.isupper()
            or vertex not in path
            or (
                vertex not in ("start", "end")
                and all(v <= 1 for k, v in Counter(path).items() if k.islower())
            )
        )

    count = 0
    while 0 < len(q) < 100000:
        path = q.popleft()
        last = path[len(path) - 1]

        if last == "end":
            count += 1
        else:
            for current_vertex in graph[last]:
                if should_add(current_vertex, path):
                    newpath = path.copy()
                    newpath.append(current_vertex)
                    q.append(newpath)
    return count


def day12_2(paths: list[Path]) -> int:
    graph = defaultdict(list)
    for p in paths:
        graph[p[0]].append(p[1])
        graph[p[1]].append(p[0])
    return count_paths_bfs(graph)


do(12, 4754, 143562)
# %%
# Day 13

Point = tuple[int, int]
FoldingIns = tuple[str, int]
Manual = tuple[list[Point], list[FoldingIns]]


def parse_folding(sections: list[str]) -> Manual:
    points = [ints(l) for l in sections[0].splitlines()]
    inss = []
    for l in sections[1].splitlines():
        cs = l.split("=")
        inss.append((cs[0][-1], int(cs[1])))
    return (points, inss)


test_lines = """6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5""".split(
    "\n\n"
)
test_man = parse_folding(test_lines)
in13 = parse_folding(data(13, str, "\n\n"))


def fold_manual(ps: list[Point], ins: FoldingIns) -> set[Point]:
    direction, v = ins
    if direction == "x":
        return {(v - abs(x - v), y) for (x, y) in ps}
    else:
        return {(x, v - abs(y - v)) for (x, y) in ps}


def day13_1(man: Manual) -> int:
    return len(fold_manual(man[0], man[1][0]))


def day13_2(man: Manual) -> int:
    ps, inss = man

    ps = reduce(fold_manual, inss, ps)

    # Print manual
    max_y = max(p[1] for p in ps)
    max_x = max(p[0] for p in ps)
    m = [
        ["#" if (x, y) in ps else "." for x in range(max_x + 1)]
        for y in range(max_y + 1)
    ]
    print("\n".join(["".join(r) for r in m]))
    return 0


do(13, 675, 0)

# %%
# Day 14

InsertionRules = dict[str, str]
Instruction = tuple[str, InsertionRules]


def parse_polymer(sections: list[str]) -> Instruction:
    template, raw_rules = sections
    rules = {}
    for l in raw_rules.splitlines():
        cs = l.split(" -> ")
        rules[(cs[0][0], cs[0][1])] = cs[1]
    return (template, rules)


in14 = parse_polymer(data(14, str, "\n\n"))
in14


def day14(instruction: Instruction, steps=int) -> int:
    template, rules = instruction

    # Init counts from the starting template
    pair_counts = defaultdict(int, Counter(zip(template[:-1], template[1:])))
    char_counts = defaultdict(int, Counter(template))

    # Insert and track counts
    for _ in range(steps):
        for pair, count in pair_counts.copy().items():
            pair_counts[pair] -= count
            c = rules[pair]
            char_counts[c] += count
            a, b = pair
            pair_counts[(a, c)] += count
            pair_counts[(c, b)] += count

    return max(char_counts.values()) - min(char_counts.values())


def day14_1(instruction: Instruction) -> int:
    return day14(instruction, 10)


def day14_2(instruction: Instruction) -> int:
    return day14(instruction, 40)


test_sections = """NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C""".split(
    "\n\n"
)
test_ins = parse_polymer(test_sections)
assert day14_1(test_ins) == 1588
do(14, 2712, 8336623059567)
