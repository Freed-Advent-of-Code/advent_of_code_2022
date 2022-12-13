package main

import (
	"bufio"
	"errors"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type File struct {
	Name string
	Size int
}

type Directory struct {
	Name        string
	Files       []*File
	Parent      *Directory
	Directories map[string]*Directory
}

func (d *Directory) AddDirectory(dir *Directory) {
	d.Directories[dir.Name] = dir
}

func (d *Directory) AddFile(f *File) {
	d.Files = append(d.Files, f)
}

func (d *Directory) GetTotalSize() int {
	total := 0
	for _, f := range d.Files {
		total += f.Size
	}
	for _, dir := range d.Directories {
		total += dir.GetTotalSize()
	}

	return total
}

type FileSystem struct {
	Root       *Directory
	WorkingDir *Directory
}

func (fs *FileSystem) ChangeDir(to string) error {
	switch to {
	case "..":
		if fs.WorkingDir.Parent != nil {
			fs.WorkingDir = fs.WorkingDir.Parent
		}
	case "/":
		dir := fs.WorkingDir
		for dir.Parent != nil {
			dir = dir.Parent
		}
		fs.WorkingDir = dir
	default:
		dir, ok := fs.WorkingDir.Directories[to]
		if !ok {
			return errors.New(fmt.Sprintf("No dir named %s found!", to))
		}
		fs.WorkingDir = dir
	}

	return nil
}

func (fs *FileSystem) MakeDir(name string) {
	newDir := &Directory{
		Name:        name,
		Files:       make([]*File, 0),
		Parent:      fs.WorkingDir,
		Directories: make(map[string]*Directory, 0),
	}
	fs.WorkingDir.AddDirectory(newDir)
}

func (fs *FileSystem) AddFile(name string, size int) {
	newFile := File{
		Name: name,
		Size: size,
	}
	fs.WorkingDir.AddFile(&newFile)
}

func CdCommand(fs *FileSystem, to string) {
	if err := fs.ChangeDir(to); err != nil {
		fs.MakeDir(to)
		_ = fs.ChangeDir(to)
	}
}

func GetTotalSizeOfAtMost(dir *Directory, max int) int {
	result := 0
	for _, d := range dir.Directories {
		result += GetTotalSizeOfAtMost(d, max)
		size := d.GetTotalSize()
		if size <= max {
			result += size
		}
	}
	return result
}

func GetTotalSizeOfAtLeast(dir *Directory, min int, stored ...int) int {
	result := 0
	if len(stored) > 0 {
		result = stored[0]
	}
	for _, d := range dir.Directories {
		size := d.GetTotalSize()
		if size < min {
			continue
		}
		if result == 0 {
			result = size
		} else if size < result {
			result = size
		}
		result = GetTotalSizeOfAtLeast(d, min, result)
	}

	return result
}

func main() {
	root := &Directory{Name: "/", Files: make([]*File, 0), Parent: nil, Directories: make(map[string]*Directory, 0)}
	fs := FileSystem{
		Root:       root,
		WorkingDir: root,
	}

	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	scanner := bufio.NewScanner(f)

	for scanner.Scan() {
		line := scanner.Text()
		tokens := strings.Split(line, " ")

		switch tokens[0] {
		case "$":
			cmd := tokens[1]
			if cmd == "cd" {
				CdCommand(&fs, tokens[2])
			}
		case "dir":
			fs.MakeDir(tokens[1])
		case "ls":
			continue
		default:
			size, err := strconv.Atoi(tokens[0])
			if err != nil {
				log.Fatal(err)
			}
			fs.AddFile(tokens[1], size)
		}
	}
	if err := scanner.Err(); err != nil {
		log.Fatalf("Error while reading file: %s", err)
	}

	total := GetTotalSizeOfAtMost(fs.Root, 100000)
	fmt.Println(total)

	min := 30000000 - (70000000 - fs.Root.GetTotalSize())
	part2 := GetTotalSizeOfAtLeast(fs.Root, min)
	fmt.Println(part2)
}
