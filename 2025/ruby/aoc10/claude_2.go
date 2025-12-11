package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"runtime"
	"sort"
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

func solve(buttons [][]int, target []int) int {
	size := len(target)

	// Check if target is all zeros
	allZero := true
	for _, t := range target {
		if t != 0 {
			allZero = false
			break
		}
	}
	if allZero {
		return 0
	}

	// Convert buttons to delta vectors
	deltas := make([][]int, len(buttons))
	for i, b := range buttons {
		delta := make([]int, size)
		for _, idx := range b {
			delta[idx]++
		}
		deltas[i] = delta
	}

	// Sort deltas by efficiency (total contribution per press, descending)
	indices := make([]int, len(deltas))
	for i := range indices {
		indices[i] = i
	}
	sort.Slice(indices, func(i, j int) bool {
		sumI, sumJ := 0, 0
		for _, v := range deltas[indices[i]] {
			sumI += v
		}
		for _, v := range deltas[indices[j]] {
			sumJ += v
		}
		return sumI > sumJ
	})

	sortedDeltas := make([][]int, len(deltas))
	for i, idx := range indices {
		sortedDeltas[i] = deltas[idx]
	}

	// Precompute which deltas can contribute to each position (for remaining deltas)
	// canContrib[idx][j] = true if any delta from idx onwards can contribute to position j
	canContrib := make([][]bool, len(sortedDeltas)+1)
	for i := range canContrib {
		canContrib[i] = make([]bool, size)
	}
	for idx := len(sortedDeltas) - 1; idx >= 0; idx-- {
		for j := 0; j < size; j++ {
			canContrib[idx][j] = canContrib[idx+1][j] || sortedDeltas[idx][j] > 0
		}
	}

	best := math.MaxInt32

	var dfs func(remaining []int, idx int, presses int)
	dfs = func(remaining []int, idx int, presses int) {
		// Check if done
		allZero := true
		for _, r := range remaining {
			if r != 0 {
				allZero = false
				break
			}
		}
		if allZero {
			if presses < best {
				best = presses
			}
			return
		}

		if idx >= len(sortedDeltas) {
			return
		}

		if presses >= best {
			return
		}

		// Pruning: check if remaining positions can be filled
		for j, r := range remaining {
			if r > 0 && !canContrib[idx][j] {
				return
			}
		}

		delta := sortedDeltas[idx]

		// Calculate max uses for this button
		maxUses := math.MaxInt32
		for j, d := range delta {
			if d > 0 {
				allowed := remaining[j] / d
				if allowed < maxUses {
					maxUses = allowed
				}
			}
		}
		if maxUses > 500 {
			maxUses = 500
		}

		// Try from max down to 0
		for uses := maxUses; uses >= 0; uses-- {
			newRemaining := make([]int, size)
			valid := true
			for j := 0; j < size; j++ {
				newRemaining[j] = remaining[j] - delta[j]*uses
				if newRemaining[j] < 0 {
					valid = false
					break
				}
			}
			if !valid {
				continue
			}

			dfs(newRemaining, idx+1, presses+uses)
		}
	}

	dfs(target, 0, 0)

	if best == math.MaxInt32 {
		return -1
	}
	return best
}

type Result struct {
	index int
	value int
	raw   string
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: go run claude_2.go <input_file>")
		return
	}

	problems := parseFile(os.Args[1])

	// Use all available CPUs
	numWorkers := runtime.NumCPU()
	runtime.GOMAXPROCS(numWorkers)

	// Channel for jobs and results
	jobs := make(chan int, len(problems))
	results := make(chan Result, len(problems))

	// Start worker goroutines
	var wg sync.WaitGroup
	for w := 0; w < numWorkers; w++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for idx := range jobs {
				p := problems[idx]
				result := solve(p.buttons, p.target)
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
	total := len(problems)
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
