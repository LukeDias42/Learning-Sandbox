package main

import "math"

type vertex struct {
	node
	distance float64
}

type node struct {
	city string
}

func nextClosestNode(distances map[node]float64, visited map[node]bool) node {
	var closestNode node
	min_dist := math.MaxFloat64
	for v := range visited {
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
