package day03

import "github.com/anhdat/advent-of-code/2020/go/utils"

func Part1(input string) int {
	grid := utils.ToGrid(input, '.')
	count := 0
	_, w := grid.SizeX()
	_, h := grid.SizeY()
	x := 0
	y := 0
	for y <= h+1 {
		if grid.Get(x%(w+1), y) == '#' {
			count++
		}
		x += 3
		y += 1
	}
	return count
}
