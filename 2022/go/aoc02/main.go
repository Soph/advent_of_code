package main

import (
	"fmt"
	"os"
	"strings"
)

func score1(play string) int {
	parts := strings.Split(play, " ")
	score := 0
	switch parts[1] {
	case "X":
		score += 1
	case "Y":
		score += 2
	case "Z":
		score += 3
	}
	switch play {
	case "A X":
		score += 3
	case "B Y":
		score += 3
	case "C Z":
		score += 3
	case "A Z":
		score += 0
	case "B X":
		score += 0
	case "C Y":
		score += 0
	case "A Y":
		score += 6
	case "B Z":
		score += 6
	case "C X":
		score += 6
	}
	//fmt.Printf("%s: %d\n", play, score);
	return score
}

// A - Rock, B - Paper, C - Scissor
// X - loose, Y - draw, Z - Win
func score2(play string) int {
	score := 0
	switch play {
	case "A X":
		score += 3
	case "A Y":
		score += 4
	case "A Z":
		score += 8
	case "B X":
		score += 1
	case "B Y":
		score += 5
	case "B Z":
		score += 9
	case "C X":
		score += 2
	case "C Y":
		score += 6
	case "C Z":
		score += 7
	}
	//fmt.Printf("%s: %d\n", play, score);
	return score
}

func main() {
	fileName := os.Args[1]
	dat, err := os.ReadFile(fileName)
	if err != nil {
		panic(err)
	}

	plays_strings := strings.Split(string(dat), "\n")

	total_score1 := 0
	total_score2 := 0
	for _, play := range plays_strings {
		total_score1 += score1(play)
		total_score2 += score2(play)
	}

	fmt.Printf("Result1: %d\n", total_score1)
	fmt.Printf("Result2: %d\n", total_score2)
}
