package main

import (
	"bufio"
	"fmt"
	"os"
	"runtime"
	"strconv"
	"strings"
	"sync"
)

type Problem struct {
	target  []int
	buttons [][]int
	raw     string
}

func parseFile(filename string) []Problem {
	file, err := os.Open(filename)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	var problems []Problem
	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			continue
		}

		parts := strings.Fields(line)

		// Parse buttons (all parts except first and last)
		var buttons [][]int
		for _, part := range parts[1 : len(parts)-1] {
			// Remove parentheses
			inner := part[1 : len(part)-1]
			indices := strings.Split(inner, ",")
			var button []int
			for _, idx := range indices {
				n, _ := strconv.Atoi(idx)
				button = append(button, n)
			}
			buttons = append(buttons, button)
		}

		// Parse joltage (last part)
		joltageStr := parts[len(parts)-1]
		joltageStr = joltageStr[1 : len(joltageStr)-1] // Remove braces
		joltageVals := strings.Split(joltageStr, ",")
		var joltage []int
		for _, v := range joltageVals {
			n, _ := strconv.Atoi(v)
			joltage = append(joltage, n)
		}

		problems = append(problems, Problem{
			target:  joltage,
			buttons: buttons,
			raw:     line,
		})
	}

	return problems
}

// Encode state as string for map key
func encodeState(state []int) string {
	var sb strings.Builder
	for i, v := range state {
		if i > 0 {
			sb.WriteByte(',')
		}
		sb.WriteString(strconv.Itoa(v))
	}
	return sb.String()
}

// Convert buttons to deltas
func buttonsToDelta(buttons [][]int, size int) [][]int {
	deltas := make([][]int, len(buttons))
	for i, b := range buttons {
		delta := make([]int, size)
		for _, idx := range b {
			delta[idx]++
		}
		deltas[i] = delta
	}
	return deltas
}

func maxSlice(s []int) int {
	m := s[0]
	for _, v := range s[1:] {
		if v > m {
			m = v
		}
	}
	return m
}

func sliceEqual(a, b []int) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if a[i] != b[i] {
			return false
		}
	}
	return true
}

func copySlice(s []int) []int {
	c := make([]int, len(s))
	copy(c, s)
	return c
}

func prefill(deltas [][]int, target []int, count int, size int) map[string][]int {
	removed := make(map[string]bool)
	values := make(map[string][]int)

	start := make([]int, size)
	values[encodeState(start)] = start

	for round := 0; round < count; round++ {
		newValues := make(map[string][]int)

		for _, v := range values {
			for _, delta := range deltas {
				newV := copySlice(v)
				for i, d := range delta {
					newV[i] += d
				}

				key := encodeState(newV)

				if removed[key] {
					continue
				}

				// Check if any value is too high
				tooHigh := false
				for i, val := range newV {
					if val > target[i] {
						tooHigh = true
						break
					}
				}
				if tooHigh {
					removed[key] = true
					continue
				}

				// Check if we had this state in the previous round
				if _, exists := values[key]; exists {
					removed[key] = true
					continue
				}

				newValues[key] = newV
			}
		}

		values = newValues
	}

	return values
}

func solve(buttons [][]int, target []int, idx int, total int) int {
	size := len(target)
	deltas := buttonsToDelta(buttons, size)

	maxTarget := maxSlice(target)
	variants := prefill(deltas, target, maxTarget, size)

	fmt.Printf("[%d/%d] Build prefill, found %d variants\n", idx+1, total, len(variants))

	i := maxTarget

	for {
		newVariants := make(map[string][]int)

		for _, variant := range variants {
			// Check if we found the target
			if sliceEqual(variant, target) {
				return i
			}

			// Expand with each button
			for _, delta := range deltas {
				newVariant := copySlice(variant)
				valid := true
				for j, d := range delta {
					newVariant[j] += d
					if newVariant[j] > target[j] {
						valid = false
						break
					}
				}
				if !valid {
					continue
				}

				key := encodeState(newVariant)
				if _, exists := newVariants[key]; !exists {
					newVariants[key] = newVariant
				}
			}
		}

		i++
		variants = newVariants

		if len(variants) == 0 {
			return -1
		}
	}
}

type Result struct {
	index int
	value int
	raw   string
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: go run aoc_2.go <input_file>")
		return
	}

	problems := parseFile(os.Args[1])

	// Use all available CPUs
	numWorkers := runtime.NumCPU()
	runtime.GOMAXPROCS(numWorkers)

	// Channel for jobs and results
	jobs := make(chan int, len(problems))
	results := make(chan Result, len(problems))

	total := len(problems)

	// Start worker goroutines
	var wg sync.WaitGroup
	for w := 0; w < numWorkers; w++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for idx := range jobs {
				p := problems[idx]
				result := solve(p.buttons, p.target, idx, total)
				results <- Result{index: idx, value: result, raw: p.raw}
			}
		}()
	}

	// Send all jobs
	for i := range problems {
		jobs <- i
	}
	close(jobs)

	// Wait for all workers to finish in a separate goroutine
	go func() {
		wg.Wait()
		close(results)
	}()

	// Print results as they come in
	sum := 0
	completed := 0
	for r := range results {
		completed++
		if r.value >= 0 {
			fmt.Printf("[%d/%d] Found %d for %s\n", completed, total, r.value, r.raw)
			sum += r.value
		} else {
			fmt.Printf("[%d/%d] No solution for %s\n", completed, total, r.raw)
		}
	}

	fmt.Println(sum)
}
