package utils

func PanicOnErr(err error) {
	if err != nil {
		panic(err)
	}
}

func IntMax(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func IntMin(a, b int) int {
	if a < b {
		return a
	}
	return b
}
