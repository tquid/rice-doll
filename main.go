package main

import (
	"fmt"
	"log"

	"github.com/tquid/rice-doll/dice"
)

func main() {
	goofyFaces := []dice.Face{
		{Glyph: "😃", Value: 0},
		{Glyph: "😄", Value: 1},
		{Glyph: "😁", Value: 2},
		{Glyph: "😆", Value: 3},
		{Glyph: "😅", Value: 4},
		{Glyph: "😂", Value: 5},
	}
	params := dice.NewDieParams{
		Faces:       goofyFaces,
		Name:        "GD6",
		Description: "Goofy die with emoji faces",
	}
	d, err := dice.NewDie(params)
	if err != nil {
		fmt.Println("Error creating goofy die")
	}
	d.Roll()
	fmt.Printf("Got a die with %d sides with a roll showing %s, and a value of %d\n", d.GetSize(), d.ShownFace.Glyph, d.ShownFace.Value)
	// create a slice of five new dice
	h := make(dice.Hand, 5)
	// fill the slice with new dice
	for i := range h {
		d, err := dice.NewIntDie(8)
		if err != nil {
			log.Fatalf("Error making new int die: %v", err)
		}
		h[i] = *d
	}
	for i := 0; i < 5; i++ {
		d, err := dice.NewIntDie(4)
		if err != nil {
			log.Fatalf("Error making new int die: %v", err)
		}
		h = append(h, *d)
	}
	gd, err := dice.NewDie(params)
	if err != nil {
		log.Fatalf("Error creating another goofy die: %v", err)
	}
	h = append(h, *gd)
	newHand := h.Roll()
	fmt.Printf("Explodey rolled: %s\n", newHand)
}
