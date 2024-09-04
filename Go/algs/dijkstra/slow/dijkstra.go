package main

import (
	"fmt"
	"math"
)

type vertex struct {
	node
	distance float64
}

type node struct {
	city string
}

func dijkstra(graph map[node][]vertex, src node, dest node) []node {
	unvisited := make(map[node]bool)
	distances := make(map[node]float64)
	predecessors := make(map[node]node)
	fmt.Println(predecessors)

	for k := range graph {
		distances[k] = math.MaxFloat64
		unvisited[k] = true
	}
	distances[src] = 0

	for len(unvisited) > 0 {
	}

	return []node{}
}

func nextClosestNode(distances map[node]float64, unvisited map[node]bool) node {
	var closestNode node
	min_dist := math.MaxFloat64
	for v := range unvisited {
		if !unvisited[v] {
			continue
		}
		if min_dist > distances[v] {
			closestNode = v
			min_dist = distances[v]
		}
	}
	return closestNode
}

func getPath(source node, predecessors map[node]node) []node {
	path := []node{}
	current := source
	emptyNode := node{}
	for current != emptyNode {
		path = append(path, current)
		var ok bool
		current, ok = predecessors[current]
		if !ok {
			current = emptyNode
		}
	}
	return path
}
