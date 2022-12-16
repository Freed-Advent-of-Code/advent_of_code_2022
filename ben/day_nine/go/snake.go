package main

import "math"

type Direction string

const (
	R Direction = "R"
	D Direction = "D"
	L Direction = "L"
	U Direction = "U"
)

func absDiff(x, y int) int {
	if x >= y {
		return x - y
	}
	return y - x
}

type Position struct {
	X int
	Y int
}

type Snake struct {
	head        Position
	tail        Position
	tailHistory map[Position]bool
}

func NewSnake() Snake {
	return Snake{
		head:        Position{0, 0},
		tail:        Position{0, 0},
		tailHistory: map[Position]bool{Position{0, 0}: true},
	}
}

func (s *Snake) Move(d Direction, dist int) {
	switch d {
	case R:
		s.head.X += dist
		s.followStraightMove()
	case D:
		s.head.Y -= dist
		s.followStraightMove()
	case L:
		s.head.X -= dist
		s.followStraightMove()
	case U:
		s.head.Y += dist
		s.followStraightMove()
	}
}

func (s *Snake) followStraightMove() {
	if absDiff(s.tail.X, s.head.X) <= 1 && absDiff(s.tail.Y, s.head.Y) <= 1 {
		return
	}
	if absDiff(s.tail.X, s.head.X) > 1 {
		for absDiff(s.tail.X, s.head.X) > 1 {
			s.tail.X += int(math.Copysign(1.0, float64(s.head.X-s.tail.X)))
			s.tailHistory[Position{s.tail.X, s.head.Y}] = true
		}
		s.tail.Y = s.head.Y
	} else {
		for absDiff(s.tail.Y, s.head.Y) > 1 {
			s.tail.Y += int(math.Copysign(1.0, float64(s.head.Y-s.tail.Y)))
			s.tailHistory[Position{s.head.X, s.tail.Y}] = true
		}
		s.tail.X = s.head.X
	}
}

func (s *Snake) GetHeadPosition() Position {
	return s.head
}

func (s *Snake) GetTailPosition() Position {
	return s.tail
}

func (s *Snake) GetMoveCount() int {
	return len(s.tailHistory)
}
