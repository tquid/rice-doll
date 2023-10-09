package parser

import (
	"bufio"
	"io"
)

// Implement a parser for dice definitions and roll expressions.

type Token int

const (
	ILLEGAL Token = iota
	EOF
	NUMBER
	DIE_QUANTITY   // e.g. "1" in "1d6"
	DIE_SIZE       // e.g. "6" in "1d6"
	OPEN_BRACKET   // "["
	CLOSE_BRACKET  // "]"
	D              // "d" or "D"
	DIE_DEFINITION // Freeform definition in brackets, e.g. "[1,2,3,4,5,6]"
	DIE_FACE       // Just "2" or potentially "😎:0", "😅:1", "🌟:3", etc.
	EXPLODE        // "!", indicating a die should be rolled again if it rolls its maximum value
	KEEP_HIGH      // "kh" optionally followed by a NUMBER, indicating the highest n dice should be kept
	KEEP_LOW       // "kl" optionally followed by a NUMBER, indicating the lowest n dice should be kept
	DROP_HIGH      // "dh" optionally followed by a NUMBER, indicating the highest n dice should be dropped
	DROP_LOW       // "dl" optionally followed by a NUMBER, indicating the lowest n dice should be dropped
	COUNT          // ">" or "<" followed by a NUMBER, indicating the value range to count
	PLUS           // "+" to be followed by NUMBER
	MINUS          // "-" to be followed by NUMBER
	QUALIFIER      // Any qualifiers along with quantities, e.g. "!kh2kl1+2"
)

var eof = rune(0)

// Scanner represents a lexical scanner.
type Scanner struct {
	r *bufio.Reader
}

// NewScanner returns a new instance of Scanner.
func NewScanner(r io.Reader) *Scanner {
	return &Scanner{r: bufio.NewReader(r)}
}

// read reads the next rune from the bufferred reader.
// Returns the rune(0) if an error occurs (or io.EOF is returned).
func (s *Scanner) read() rune {
	ch, _, err := s.r.ReadRune()
	if err != nil {
		return eof
	}
	return ch
}

// unread places the previously read rune back on the reader.
func (s *Scanner) unread() { _ = s.r.UnreadRune() }
