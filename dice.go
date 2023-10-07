package main

import (
	"fmt"
	"math/rand"
	"strconv"
)

type DieValue int16
type DieSize int

type Face struct {
	sign  string
	value DieValue
}

// Creates a "sign" (printable representation) and value to apply to a Die
func NewFace(sign string, value DieValue) Face {
	return Face{sign, value}
}

func IntFace(value DieValue) Face {
	return Face{strconv.Itoa(int(value)), value}
}

type Die struct {
	faces     []Face
	shownFace *Face
}

// Completely arbitrary die with any Faces
func NewDie(faces []Face) *Die {
	return &Die{faces, nil}
}

// Typical die with integer sides and values
func NewRangeDie(numbers []DieValue) *Die {
	faces := make([]Face, len(numbers))
	for i, n := range numbers {
		faces[i] = IntFace(n)
	}
	return NewDie(faces)
}

// Even more typical die numbered 1..n, incrementing by 1
func NewIntDie(size DieSize) *Die {
	sides := make([]DieValue, size)
	for i := 0; i < int(size); i++ {
		sides[i] = DieValue(i + 1)
	}
	return NewRangeDie(sides)
}

func (d *Die) Roll() *Die {
	d.shownFace = &d.faces[rand.Intn(len(d.faces))]
	return d
}

func (d *Die) GetShownFace() *Face {
	return d.shownFace
}

func (d *Die) GetSize() DieSize {
	return DieSize(len(d.faces))
}

func (d *Die) GetFaces() []Face {
	return d.faces
}

func main() {
	goofyFaces := []Face{
		{"😃", 0},
		{"😄", 1},
		{"😁", 2},
		{"😆", 3},
		{"😅", 4},
		{"😂", 5},
	}
	d := NewDie(goofyFaces)
	fmt.Printf("Got a die with %d sides with a roll showing %s, and a value of %d\n", d.GetSize(), d.Roll().GetShownFace().sign, d.Roll().GetShownFace().value)
}
