package main

import (
	"fmt"
	"os"
	"strconv"

	"github.com/anhdat/advent-of-code/2020/go/day01"
	"github.com/anhdat/advent-of-code/2020/go/day02"
	"github.com/anhdat/advent-of-code/2020/go/day03"
	"github.com/anhdat/advent-of-code/2020/go/day04"
	"github.com/anhdat/advent-of-code/2020/go/utils"
)

func main() {
	day, err := strconv.Atoi(os.Args[1])
	utils.PanicOnErr(err)
	input := utils.Readfile(day)
	switch day {
	case 1:
		fmt.Printf("Part 1: %d\n", day01.Part1(input))
		fmt.Printf("Part 2: %d\n", day01.Part2(input))
	case 2:
		fmt.Printf("Part 1: %d\n", day02.Part1(input))
		fmt.Printf("Part 2: %d\n", day02.Part2(input))
	case 3:
		fmt.Printf("Part 1: %d\n", day03.Part1(input))
		fmt.Printf("Part 2: %d\n", day03.Part2(input))
	case 4:
		fmt.Printf("Part 1: %d\n", day04.Part1(input))
		// fmt.Printf("Part 2: %d\n", day04.Part2(input))
	default:
		panic(fmt.Errorf("no such day: %d", day))
	}
}
