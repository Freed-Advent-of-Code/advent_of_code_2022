package main

import (
	"log"
	"os"
	"testing"
)

func TestSnake_Move(t *testing.T) {
	assertPosition := func(t testing.TB, got Position, want Position) {
		t.Helper()
		if got != want {
			t.Errorf("wanted %d, got %d", want, got)
		}
	}

	t.Run("move down", func(t *testing.T) {
		snake := NewSnake()
		snake.Move(D, 3)
		expected := Position{0, -2}
		got := snake.GetTailPosition()
		assertPosition(t, got, expected)
	})

	t.Run("move right", func(t *testing.T) {
		snake := NewSnake()
		snake.Move(R, 3)
		expected := Position{2, 0}
		got := snake.GetTailPosition()
		assertPosition(t, got, expected)
	})

	t.Run("move left", func(t *testing.T) {
		snake := NewSnake()
		snake.Move(L, 3)
		expected := Position{-2, 0}
		got := snake.GetTailPosition()
		assertPosition(t, got, expected)
	})

	t.Run("move up", func(t *testing.T) {
		snake := NewSnake()
		snake.Move(U, 3)
		expected := Position{0, 2}
		got := snake.GetTailPosition()
		assertPosition(t, got, expected)
	})

	snake := NewSnake()

	t.Run("move down", func(t *testing.T) {
		snake.Move(D, 3)
		expected := Position{0, -2}
		got := snake.GetTailPosition()
		assertPosition(t, got, expected)
	})

	t.Run("move right", func(t *testing.T) {
		snake.Move(R, 3)
		expected := Position{2, -3}
		got := snake.GetTailPosition()
		assertPosition(t, got, expected)
	})

	t.Run("move left", func(t *testing.T) {
		snake.Move(L, 3)
		expected := Position{1, -3}
		got := snake.GetTailPosition()
		assertPosition(t, got, expected)
	})

	t.Run("move up", func(t *testing.T) {
		snake.Move(U, 3)
		expected := Position{0, -1}
		got := snake.GetTailPosition()
		assertPosition(t, got, expected)
	})
}

func TestSnake_GetMoveCount(t *testing.T) {
	f, err := os.Open("test_input.txt")
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
	got := snake.GetMoveCount()
	expected := 13
	if got != expected {
		t.Errorf("wanted %d, got %d", expected, got)
	}
}
