package utils

import (
	"strconv"
	"strings"
)

func MustAtoi(s string) int {
	v, err := strconv.Atoi(s)
	PanicOnErr(err)
	return v
}

func ToInts(input string, sep string) []int {
	var r []int
	for _, line := range strings.Split(input, sep) {
		if line != "" {
			r = append(r, MustAtoi(line))
		}
	}
	return r
}
