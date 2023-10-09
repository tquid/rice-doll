package dice

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

	if len(d6.Faces) != len(expected) {
		t.Errorf("Expected %v, got %v", expected, d6.Faces)
		return
	}
	for i, face := range d6.Faces {
		if face.Glyph != expected[i].Glyph || face.Value != expected[i].Value {
			t.Errorf("Expected %v, got %v", expected, d6.Faces)
			return
		}
	}
}
