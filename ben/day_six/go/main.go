package main

import (
	"errors"
	"fmt"
	"io/ioutil"
	"log"
	"path/filepath"
)

func solution(packet []byte, distinctLevel int) (int, error) {
	pos := 0

	for {
		bmap := map[byte]int{}
		for i, b := range packet[pos : pos+distinctLevel] {
			newPos, ok := bmap[b]
			if ok {
				pos = newPos + 1
				break
			} else {
				if i == distinctLevel-1 {
					return pos + distinctLevel, nil
				}
				bmap[b] = pos + i
			}
		}
		if pos >= len(packet) {
			break
		}
	}

	return -1, errors.New("could not find a marker")
}

func main() {
	absPath, _ := filepath.Abs("../input.txt")
	input, err := ioutil.ReadFile(absPath)
	if err != nil {
		log.Fatalf("failed to read: %s", err.Error())
	}

	markerAt, err := solution(input, 14)
	if err != nil {
		log.Fatalf("failed: %s", err.Error())
	}
	fmt.Printf("%d\n", markerAt)
}
