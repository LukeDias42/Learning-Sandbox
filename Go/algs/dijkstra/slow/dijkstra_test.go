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
	expected := []node{z, g, f, a}

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
	expected := []node{h, g, f, e, d, c, b, a}

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

func TestDijkstra(t *testing.T) {
	minasTirith := node{city: "Minas Tirith"}
	isengard := node{city: "Isengard"}
	gondor := node{city: "Gondor"}
	mirkwood := node{city: "Mirkwood"}
	bree := node{city: "Bree"}
	lothlorien := node{city: "Lothlorien"}
	mistyMountains := node{city: "Misty Mountains"}

	graph := map[node]map[node]float64{
		minasTirith: {
			isengard: 4,
			gondor:   1,
		},
		isengard: {
			minasTirith: 4,
			bree:        4,
			mirkwood:    8,
		},
		gondor: {
			minasTirith:    1,
			bree:           2,
			mistyMountains: 8,
		},
		bree: {
			gondor:   2,
			isengard: 2,
			mirkwood: 4,
		},
		mirkwood: {
			bree:       4,
			isengard:   8,
			lothlorien: 2,
		},
		mistyMountains: {
			gondor:     6,
			lothlorien: 8,
		},
		lothlorien: {
			mistyMountains: 8,
			mirkwood:       2,
		},
	}
	path := dijkstra(graph, minasTirith, lothlorien)
	expected := []node{minasTirith, gondor, bree, mirkwood, lothlorien}
	if len(path) != len(expected) {
		t.Fatalf("Different amount of items in Path (%d) %v and the expected (%d) %v", len(path), path, len(expected), expected)
	}
	for i := range len(path) {
		if path[i] != expected[i] {
			t.Fatalf("Path %v was not equal to expected %v", path, expected)
		}
	}
}
