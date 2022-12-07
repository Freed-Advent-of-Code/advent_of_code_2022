package main

import (
	"testing"
)

func TestPart1(t *testing.T) {
	testCases := []struct {
		packet   []byte
		expected int
	}{
		{[]byte("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7},
		{[]byte("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5},
		{[]byte("nppdvjthqldpwncqszvftbrmjlhg"), 6},
		{[]byte("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10},
		{[]byte("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11},
	}
	for _, tc := range testCases {
		got, err := solution(tc.packet, 4)
		if err != nil {
			t.Errorf("got %g wanted %d", err, tc.expected)
		}
		if got != tc.expected {
			t.Errorf("got %d wanted %d", got, tc.expected)
		}
	}
}

func TestPart2(t *testing.T) {
	testCases := []struct {
		packet   []byte
		expected int
	}{
		{[]byte("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19},
		{[]byte("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23},
		{[]byte("nppdvjthqldpwncqszvftbrmjlhg"), 23},
		{[]byte("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29},
		{[]byte("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26},
	}
	for _, tc := range testCases {
		got, err := solution(tc.packet, 14)
		if err != nil {
			t.Errorf("case: %s, got %g wanted %d", tc.packet, err, tc.expected)
		}
		if got != tc.expected {
			t.Errorf("case: %s, got %d wanted %d", tc.packet, got, tc.expected)
		}
	}
}
