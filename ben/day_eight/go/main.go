package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"
	"strconv"
	"strings"
)

type Direction int

const (
	RIGHT Direction = iota
	DOWN
	LEFT
	UP
)

type TreeCounter struct {
	grid   [][]int
	width  int
	height int
}

func NewTreeCounter(r io.Reader) TreeCounter {
	scan := bufio.NewScanner(r)
	grid := make([][]int, 0)
	for scan.Scan() {
		row := make([]int, 0)
		for _, val := range strings.Split(scan.Text(), "") {
			converted, _ := strconv.Atoi(val)
			row = append(row, converted)
		}
		grid = append(grid, row)
	}
	counter := TreeCounter{
		grid:   grid,
		height: len(grid),
		width:  len(grid[0]),
	}

	return counter
}

func (tc *TreeCounter) isVisibleTowards(i, j int, direction Direction) (bool, int) {
	treeHeight := tc.grid[i][j]
	treeCount := 0

	for {
		switch direction {
		case RIGHT:
			j += 1
		case UP:
			i += -1
		case LEFT:
			j -= 1
		case DOWN:
			i += 1
		}
		if i < 0 || i >= tc.height || j < 0 || j >= tc.width {
			break
		}
		treeCount += 1
		if treeHeight <= tc.grid[i][j] {
			return false, treeCount
		}
	}
	return true, treeCount
}

func (tc *TreeCounter) count() int {
	count := tc.width*2 + (tc.height-2)*2
	for i := 1; i != len(tc.grid)-1; i++ {
		for j := 1; j != len(tc.grid[i])-1; j++ {
			for dir := 0; dir != 4; dir++ {
				visible, _ := tc.isVisibleTowards(i, j, Direction(dir))
				if visible {
					count += 1
					break
				}
			}
		}
	}
	return count
}

func (tc *TreeCounter) getHighestScores() int {
	highest := 1
	for i := 0; i != len(tc.grid); i++ {
		for j := 0; j != len(tc.grid[i]); j++ {
			totalScores := 1
			for dir := 0; dir != 4; dir++ {
				_, scores := tc.isVisibleTowards(i, j, Direction(dir))
				totalScores *= scores
			}
			if highest < totalScores {
				highest = totalScores
			}
		}
	}
	return highest
}

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	counter := NewTreeCounter(f)
	//fmt.Println(counter.count())
	fmt.Println(counter.getHighestScores())
}
