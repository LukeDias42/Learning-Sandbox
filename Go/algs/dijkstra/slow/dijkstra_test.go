package main

import (
	"testing"
)

func TestGetPath(t *testing.T) {
	a := node{city: "a"}
	f := node{city: "f"}
	g := node{city: "g"}
	z := node{city: "z"}
	predecessors := map[node]node{
		a: f,
		g: z,
		f: g,
	}
	path := getPath(a, predecessors)
	expected := []node{a, f, g, z}

	if len(path) != len(expected) {
		t.Fatalf("Different amount of items in Path (%d) %v and the expected (%d) %v", len(path), path, len(expected), expected)
	}
	for i := range len(path) {
		if path[i] != expected[i] {
			t.Fatalf("Path %v was not equal to expected %v", path, expected)
		}
	}
}

func TestGetPathLonger(t *testing.T) {
	a := node{city: "a"}
	b := node{city: "b"}
	c := node{city: "c"}
	d := node{city: "d"}
	e := node{city: "e"}
	f := node{city: "f"}
	g := node{city: "g"}
	h := node{city: "h"}

	predecessors := map[node]node{
		a: b,
		b: c,
		c: d,
		d: e,
		e: f,
		f: g,
		g: h,
	}
	path := getPath(a, predecessors)
	expected := []node{a, b, c, d, e, f, g, h}

	if len(path) != len(expected) {
		t.Fatalf("Different amount of items in Path (%d) %v and the expected (%d) %v", len(path), path, len(expected), expected)
	}
	for i := range len(path) {
		if path[i] != expected[i] {
			t.Fatalf("Path %v was not equal to expected %v", path, expected)
		}
	}
}

func TestNextClosestNode(t *testing.T) {
	minasTirith := node{city: "Minas Tirith"}
	isengard := node{city: "Isengard"}
	gondor := node{city: "Gondor"}
	mirkwood := node{city: "Mirkwood"}

	distances := map[node]float64{
		minasTirith: 4,
		isengard:    2,
		gondor:      3,
		mirkwood:    1,
	}

	unvisited := map[node]bool{
		minasTirith: true,
		isengard:    false,
		gondor:      true,
		mirkwood:    false,
	}

	closestNode := nextClosestNode(distances, unvisited)
	expected := gondor

	if closestNode != expected {
		t.Fatalf("Next Closest Node %v was not equal to expected %v", closestNode, expected)
	}
}
