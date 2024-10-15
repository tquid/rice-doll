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

type Roller interface {
	Roll(d *Die) Hand
}

type Faces []Face

func (f Faces) randomPick() *Face {
	return &f[rand.Intn(len(f))]
}

type Die struct {
	// A short name, e.g. "D6"
	Name string
	// A longer description, e.g. "Genesys Proficiency die"
	Description string
	// Each face of the die with a glyph (visual representation) and a value
	faces Faces
	// The face currently "facing up"
	ShownFace *Face
	// A function to roll the die
	Roller Roller
}

func (d *Die) Clone() *Die {
	newFaces := make(Faces, len(d.faces))
	copy(newFaces, d.faces)
	return &Die{Name: d.Name, faces: newFaces, Roller: d.Roller}
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

func (h Hand) Roll() Hand {
	for i := range h {
		h[i].Roll()
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

type NewDieParams struct {
	Name        string
	Description string
	Faces       Faces
	Roller      Roller
}

// Completely arbitrary die with any Faces
func NewDie(p NewDieParams) (*Die, error) {
	if len(p.Faces) == 0 {
		return nil, fmt.Errorf("creating a die with no faces is not allowed")
	}
	if p.Roller == nil {
		p.Roller = BasicRoller{}
	}
	d := &Die{
		Name:        p.Name,
		Description: p.Description,
		faces:       p.Faces,
		Roller:      p.Roller,
	}
	d.sortFaces()
	d.ShownFace = d.faces.randomPick()
	return d, nil
}

// Typical die with integer sides and values
func NewRangeDie(name string, numbers []DieValue) (*Die, error) {
	faces := make(Faces, len(numbers))
	var builder strings.Builder
	for i, n := range numbers {
		face := IntFace(n)
		faces[i] = face
		builder.WriteString(face.Glyph)
		if i < len(numbers)-1 {
			builder.WriteString(", ")
		}
	}
	nd, err := NewDie(NewDieParams{
		Name:        name,
		Description: fmt.Sprintf("D%d numbered %s", len(numbers), builder.String()),
		Faces:       faces,
	})
	if err != nil {
		return nil, fmt.Errorf("can't create new RangeDie: %w", err)
	}
	return nd, nil
}

// Even more typical die numbered 1..n, incrementing by 1
func NewIntDie(size DieSize) (*Die, error) {
	if int(size) == 0 {
		return nil, fmt.Errorf("can't make a die with no faces")
	}
	sides := make([]DieValue, size)
	for i := 0; i < int(size); i++ {
		sides[i] = DieValue(i + 1)
	}
	name := fmt.Sprintf("D%d", size)
	die, err := NewRangeDie(name, sides)
	if err != nil {
		return nil, fmt.Errorf("can't create new IntDie: %w", err)
	}
	return die, nil
}

func (d *Die) GetSize() DieSize {
	return DieSize(len(d.faces))
}

// Sorts the faces of the die in ascending order by value
func (d *Die) sortFaces() {
	sort.Slice(d.faces, func(i, j int) bool {
		return d.faces[i].Value < d.faces[j].Value
	})
}

func (d *Die) GetMaxValue() DieValue {
	return d.faces[len(d.faces)-1].Value
}

func (d *Die) Roll() Hand {
	return d.Roller.Roll(d)
}

type BasicRoller struct{}

func (r BasicRoller) Roll(d *Die) Hand {
	d.ShownFace = d.faces.randomPick()
	return Hand{*d}
}

type ExplodingRoller struct{}

func (r ExplodingRoller) Roll(d *Die) Hand {
	var hand Hand
	// Initial roll to get things started
	d.ShownFace = d.faces.randomPick()
	hand = append(hand, *d)

	for d.ShownFace.Value == d.GetMaxValue() {
		newDie := d.Clone()
		newDie.Roller = r
		newDie.Roll()
		hand = append(hand, *newDie)
		d = newDie
	}
	return hand
}
