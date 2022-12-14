package main

import (
	"log"
	"os"
	"testing"
)

func TestPart1(t *testing.T) {
	f, err := os.Open("test_input.txt")
	if err != nil {
		log.Fatal(err)
	}
	counter := NewTreeCounter(f)
	got := counter.count()
	if counter.count() != 21 {
		t.Errorf("got %d wanted %d", got, 21)
	}
}

func TestPart2(t *testing.T) {

}
