package day03

import "github.com/anhdat/advent-of-code/2020/go/utils"

func Part1(input string) int {
	grid := utils.ToGrid(input, '.')
	return count_trees(grid, 3, 1)
}

func count_trees(grid *utils.Grid, slopeX, slopeY int) int {
	count := 0
	_, w := grid.SizeX()
	_, h := grid.SizeY()
	x := 0
	y := 0
	for y <= h+1 {
		if grid.Get(x%(w+1), y) == '#' {
			count++
		}
		x += slopeX
		y += slopeY
	}
	return count
}

func Part2(input string) int {
	grid := utils.ToGrid(input, '.')
	slopes := [][]int{
		{1, 1},
		{3, 1},
		{5, 1},
		{7, 1},
		{1, 2},
	}
	product := 1
	for _, slope := range slopes {
		product *= count_trees(grid, slope[0], slope[1])
	}
	return product
}
