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

type Command struct {
	direction Direction
	distance  int
}

func parseInput(r io.Reader) ([]Command, error) {
	scanner := bufio.NewScanner(r)

	commands := make([]Command, 0)
	for scanner.Scan() {
		line := strings.Split(scanner.Text(), " ")
		distance, err := strconv.Atoi(line[1])
		direction := Direction(line[0])
		if err != nil {
			return nil, err
		}
		commands = append(commands, Command{
			direction: direction,
			distance:  distance,
		})
	}

	return commands, nil
}

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	commands, err := parseInput(f)
	if err != nil {
		log.Fatal(err)
	}

	snake := NewSnake()

	for _, cmd := range commands {
		snake.Move(cmd.direction, cmd.distance)
	}

	fmt.Println(snake.GetMoveCount())
}
