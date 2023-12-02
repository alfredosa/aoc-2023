package tests

import (
	"alfredosa/aoc-2023/daycode"
	"testing"
)

func TestDayResult(t *testing.T) {
	result := daycode.DayResult("test_input.txt")
	expected := "142"
	if result != expected {
		t.Errorf("DayResult was incorrect, got: %s, want: %s.", result, expected)
	}
}
