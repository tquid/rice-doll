package dice

import (
	"fmt"
	"math/rand"
	"sort"
	"strconv"
	"strings"
)

type DieValue int16
type DieSize int

type Face struct {
	Glyph string
	Value DieValue
}

// Creates a "glyph" (printable representation) and value to apply to a Die
func NewFace(glyph string, value DieValue) Face {
	return Face{glyph, value}
}

func IntFace(value DieValue) Face {
	return Face{strconv.Itoa(int(value)), value}
}

type Die struct {
	Name      string
	Faces     []Face
	ShownFace *Face
}

type Hand []Die

func (dv DieValue) String() string {
	return strconv.Itoa(int(dv))
}

func (d *Die) String() string {
	if d.ShownFace != nil {
		return fmt.Sprintf("%d", d.ShownFace.Value) // or any other representation you like
	}
	return "Not rolled yet"
}

func (h Hand) String() string {
	var builder strings.Builder
	builder.WriteString("[")
	for i, d := range h {
		builder.WriteString(d.String())
		if i < len(h)-1 {
			builder.WriteString(", ")
		}
	}
	builder.WriteString("]")
	return builder.String()
}

// Completely arbitrary die with any Faces
func NewDie(name string, faces []Face) *Die {
	d := &Die{name, faces, nil}
	d.sortFaces()
	return d
}

// Typical die with integer sides and values
func NewRangeDie(name string, numbers []DieValue) *Die {
	faces := make([]Face, len(numbers))
	for i, n := range numbers {
		faces[i] = IntFace(n)
	}
	return NewDie(name, faces)
}

// Even more typical die numbered 1..n, incrementing by 1
func NewIntDie(size DieSize) *Die {
	sides := make([]DieValue, size)
	for i := 0; i < int(size); i++ {
		sides[i] = DieValue(i + 1)
	}
	name := fmt.Sprintf("d%d", size)
	return NewRangeDie(name, sides)
}

func (d *Die) GetSize() DieSize {
	return DieSize(len(d.Faces))
}

// Sorts the faces of the die in ascending order by value
func (d *Die) sortFaces() {
	sort.Slice(d.Faces, func(i, j int) bool {
		return d.Faces[i].Value < d.Faces[j].Value
	})
}

func (d *Die) GetMaxValue() DieValue {
	return d.Faces[len(d.Faces)-1].Value
}

func (d *Die) Roll(strategy func(*Die) []Die) []Die {
	return strategy(d)
}

func BasicRoll(d *Die) []Die {
	d.ShownFace = &d.Faces[rand.Intn(len(d.Faces))]
	return []Die{*d}
}

func ExplodingRoll(d *Die) []Die {
	hand := d.Roll(BasicRoll)
	if d.ShownFace.Value == d.GetMaxValue() {
		fmt.Println("Exploding!")
		hand = append(hand, *d)
		d.Roll(ExplodingRoll)
	}
	return hand
}
