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
type RollFunc func(d *Die) Hand

type Face struct {
	Glyph string
	Value DieValue
}

type Die struct {
	Name      string
	Faces     []Face
	ShownFace *Face
}

type Hand []Die

// Creates a "glyph" (printable representation) and value to apply to a Die
func NewFace(glyph string, value DieValue) Face {
	return Face{glyph, value}
}

func IntFace(value DieValue) Face {
	return Face{strconv.Itoa(int(value)), value}
}

func NewHand(dice ...Die) Hand {
	return dice
}

func RollHand(h Hand, rf RollFunc) Hand {
	for i := range h {
		h[i].Roll(rf)
	}
	return h
}

func (dv DieValue) String() string {
	return strconv.Itoa(int(dv))
}

func (d *Die) String() string {
	if d.ShownFace != nil {
		// return fmt.Sprintf("%d", d.ShownFace.Value)
		return fmt.Sprintf("%s:%s", d.Name, d.ShownFace.Glyph)
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

func (d *Die) Roll(rf RollFunc) []Die {
	return rf(d)
}

func BasicRoll(d *Die) {
	d.ShownFace = &d.Faces[rand.Intn(len(d.Faces))]
	return
}

// We take a Hand, not a Die, for an exploding roll, since we may add dice to
// the hand
func ExplodingRoll(h *Hand) {
	newHand := NewHand()
	for _, d := range *h {
		BasicRoll(&d)
		newHand = append(newHand, d)
		for d.ShownFace.Value == d.GetMaxValue() {
			newDie := *NewIntDie(d.GetSize()) // create a new d6 die
			BasicRoll(&newDie)                // roll it
			newHand = append(newHand, newDie) // add it to the hand
			d = newDie
		}
	}
	*h = newHand
	return
}
