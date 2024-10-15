package dice

import "testing"

func TestErrorOnZeroDie(t *testing.T) {
	_, err := NewIntDie(0)
	if err == nil {
		t.Errorf("Attempt to create 0-sided die produced no error")
	}
}

func TestCorrectSizeFromInteger(t *testing.T) {
	d6, err := NewIntDie(6)
	if err != nil {
		t.Errorf("Unexpected error")
	}
	if d6.GetSize() != 6 {
		t.Errorf("Expected size 6, got %d", d6.GetSize())
	}
}

func TestCorrectDieFromInteger(t *testing.T) {
	d6, err := NewIntDie(6)
	if err != nil {
		t.Errorf("Unexpected error")
	}
	expected := []Face{
		{"1", 1},
		{"2", 2},
		{"3", 3},
		{"4", 4},
		{"5", 5},
		{"6", 6},
	}

	if len(d6.faces) != len(expected) {
		t.Errorf("Expected %v, got %v", expected, d6.faces)
		return
	}
	for i, face := range d6.faces {
		if face.Glyph != expected[i].Glyph || face.Value != expected[i].Value {
			t.Errorf("Expected %v, got %v", expected, d6.faces)
			return
		}
	}
}
