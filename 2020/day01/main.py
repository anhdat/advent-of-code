def list_from_input():
    a = []
    with open("input") as f:
        for l in f.readlines():
            a.append(int(l))
    return a

def find_num(nums, total):
    s = set([total - n for n in nums])
    for num in nums:
        if num in s:
            return num

def part_1():
    a = list_from_input()
    found = find_num(a, 2020)
    print(found, 2020 - found)
    print(found * (2020 - found))

def part_2():
    a = list_from_input()
    for num in a:
        sub_nums = [n for n in a if n != num]
        found = find_num(sub_nums, 2020 - num)
        if found:
            print(found, num, 2020 - num - found)
            print(found * num * (2020 - num - found))
            return

part_1()
part_2()
