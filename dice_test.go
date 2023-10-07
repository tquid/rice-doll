package main

import "testing"

func TestCorrectSizeFromInteger(t *testing.T) {
	d6 := NewIntDie(6)
	if d6.GetSize() != 6 {
		t.Errorf("Expected size 6, got %d", d6.GetSize())
	}
}

func TestCorrectDieFromInteger(t *testing.T) {
	d6 := NewIntDie(6)
	expected := []Face{
		{"1", 1},
		{"2", 2},
		{"3", 3},
		{"4", 4},
		{"5", 5},
		{"6", 6},
	}

	if len(d6.GetFaces()) != len(expected) {
		t.Errorf("Expected %v, got %v", expected, d6.GetFaces())
		return
	}

	for i, face := range d6.GetFaces() {
		if face.sign != expected[i].sign || face.value != expected[i].value {
			t.Errorf("Expected %v, got %v", expected, d6.GetFaces())
			return
		}
	}
}
