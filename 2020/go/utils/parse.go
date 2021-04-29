package utils

import (
	"fmt"
	"reflect"
	"regexp"
	"strconv"
	"strings"
)

func MustAtoi(s string) int {
	v, err := strconv.Atoi(s)
	PanicOnErr(err)
	return v
}

func ToStrings(input string) []string {
	return ToStringsWithSep(input, "\n")
}

func ToStringsWithSep(input, sep string) []string {
	var r []string
	for _, line := range strings.Split(input, sep) {
		if len(line) == 0 {
			continue
		}
		r = append(r, strings.TrimSpace(line))
	}
	return r
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

func ToGrid(input string, empty interface{}) *Grid {
	grid := NewGrid(empty)

	for y, line := range strings.Split(input, "\n") {
		for x, rune := range line {
			grid.Set(x, y, rune)
		}
	}

	return grid
}

func ParseToStruct(re *regexp.Regexp, input string, target interface{}) error {
	m := re.FindStringSubmatch(input)
	if m == nil {
		return fmt.Errorf("not match")
	}

	for i, name := range re.SubexpNames() {
		if i == 0 {
			continue
		}
		var field reflect.Value = reflect.ValueOf(target).Elem().FieldByName(name)

		kind := field.Kind()
		switch kind {
		case reflect.String:
			field.SetString(m[i])
		case reflect.Int:
			v, err := strconv.Atoi(m[i])
			if err != nil {
				return err
			}
			field.SetInt(int64(v))
		case reflect.Uint8:
			field.SetUint(uint64(m[i][0]))
		default:
			panic(fmt.Sprintf("unknown kind: %s", kind))
		}
	}
	return nil
}
