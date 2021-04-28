package day01

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPart1(t *testing.T) {
	r := Part1(`1721
979
366
299
675
1456`)
	assert.Equal(t, 514579, r)
}
func TestPart2(t *testing.T) {
	r := Part2(`1721
979
366
299
675
1456`)
	assert.Equal(t, 241861950, r)
}
