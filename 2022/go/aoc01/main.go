package main

import (
    "fmt"
    "os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	fileName := os.Args[1]
    dat, err := os.ReadFile(fileName)
	if err != nil {
        panic(err)
    }

	elf_strings := strings.Split(string(dat), "\n\n");

	var elfs []int
	for _, elf_string := range elf_strings {
		elf := 0
		strings := strings.Split(string(elf_string), "\n");
		for _, str := range strings {
			number, err := strconv.Atoi(str)
			if err != nil {
				panic(err)
			}
			elf += number
		}
		elfs = append(elfs, elf)
	}
	sort.Ints(elfs)
	fmt.Printf("Part1: %d\n", elfs[len(elfs)-1])

	sum := 0
	for _,value := range elfs[len(elfs)-4:len(elfs)-1] {
		sum += value
	}
	fmt.Printf("Part2: %d\n", sum)
}