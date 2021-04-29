package day04

import (
	"sort"
	"strings"

	"github.com/anhdat/advent-of-code/2020/go/utils"
)

func Part1(input string) int {
	count := 0
	passport_raws := utils.ToStringsWithSep(input, "\n\n")
	required_fields := []string{
		"byr",
		"iyr",
		"eyr",
		"hgt",
		"hcl",
		"ecl",
		"pid",
	}
	sort.Strings(required_fields)
	for _, p := range passport_raws {
		field_names := make(map[string]bool)
		for _, f := range strings.Fields(p) {
			name := strings.Split(f, ":")[0]
			field_names[name] = true
		}
		valid_fields_count := 0
		for _, f := range required_fields {
			_, ok := field_names[f]
			if ok {
				valid_fields_count++
			}
		}
		if valid_fields_count == len(required_fields) {
			count++
		}
	}
	return count
}
