package day02

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPart1(t *testing.T) {
	r := Part1(`1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc`)
	assert.Equal(t, 2, r)
}

func TestPart2(t *testing.T) {
	r := Part2(`1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc`)
	assert.Equal(t, 1, r)
}
