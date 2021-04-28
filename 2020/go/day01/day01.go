package day01

import "github.com/anhdat/advent-of-code/2020/go/utils"

func Part1(input string) int {
	nums := utils.ToInts(input, "\n")
	for _, num := range nums {
		for _, num2 := range nums {
			if num == num2 {
				continue
			}
			if num+num2 == 2020 {
				return num * num2
			}
		}
	}
	return 0
}
