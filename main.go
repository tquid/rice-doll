package main

import (
	"fmt"
	"math/rand"
	"time"

	"shinyhexagon.com/rice-doll/dice"
)

func main() {
	rand.Seed(time.Now().UnixNano())
	goofyFaces := []dice.Face{
		{Glyph: "😃", Value: 0},
		{Glyph: "😄", Value: 1},
		{Glyph: "😁", Value: 2},
		{Glyph: "😆", Value: 3},
		{Glyph: "😅", Value: 4},
		{Glyph: "😂", Value: 5},
	}
	d := dice.NewDie("goofy", goofyFaces)
	d.Roll(dice.BasicRoll)
	fmt.Printf("Got a die with %d sides with a roll showing %s, and a value of %d\n", d.GetSize(), d.ShownFace.Glyph, d.ShownFace.Value)
	// create a slice of five new dice
	d6s := make(dice.Hand, 5)
	// fill the slice with new dice
	for i := range d6s {
		d6s[i] = *dice.NewIntDie(6)
	}
	// roll each die in the slice
	for i := range d6s {
		d6s[i].Roll(dice.ExplodingRoll)
	}
	fmt.Printf("Explodey rolled five d6s: %s\n", d6s)
}
