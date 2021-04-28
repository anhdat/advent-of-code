package day02

import (
	"regexp"
	"strings"

	"github.com/anhdat/advent-of-code/2020/go/utils"
)

func Part1(input string) int {
	type Line struct {
		Min      int
		Max      int
		Char     string
		Password string
	}
	lines := utils.ToStrings(input)
	re := regexp.MustCompile(`(?P<Min>\d+)-(?P<Max>\d+) (?P<Char>.): (?P<Password>.*)`)
	valid := 0
	for _, line := range lines {
		var match Line
		err := utils.ParseToStruct(re, line, &match)
		utils.PanicOnErr(err)
		count := strings.Count(match.Password, match.Char)
		if count >= match.Min && count <= match.Max {
			valid++
		}
	}
	return valid
}

func Part2(input string) int {
	type Line struct {
		Pos1     int
		Pos2     int
		Char     byte
		Password string
	}
	lines := utils.ToStrings(input)
	re := regexp.MustCompile(`(?P<Pos1>\d+)-(?P<Pos2>\d+) (?P<Char>.): (?P<Password>.*)`)
	valid := 0
	for _, line := range lines {
		var match Line
		err := utils.ParseToStruct(re, line, &match)
		utils.PanicOnErr(err)
		if (match.Password[match.Pos1-1] == match.Char) != (match.Password[match.Pos2-1] == match.Char) {
			valid++
		}
	}
	return valid
}
