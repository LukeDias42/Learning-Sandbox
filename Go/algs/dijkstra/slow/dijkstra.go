package main

import (
	"math"
)

type node struct {
	city string
}

func dijkstra(graph map[node]map[node]float64, src node, dest node) []node {
	unvisited := make(map[node]bool)
	distances := make(map[node]float64)
	predecessors := make(map[node]node)

	for k := range graph {
		distances[k] = math.MaxFloat64
		unvisited[k] = true
	}
	distances[src] = 0

	for len(unvisited) > 0 {
		closestNode := nextClosestNode(distances, unvisited)
		unvisited[closestNode] = false
		if closestNode == dest {
			return getPath(dest, predecessors)
		}
		for neighbor := range graph[closestNode] {
			totalDist := distances[closestNode] + graph[closestNode][neighbor]
			if distances[neighbor] > totalDist {
				distances[neighbor] = totalDist
				predecessors[neighbor] = closestNode

			}
		}
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

func getPath(dest node, predecessors map[node]node) []node {
	path := []node{}
	current := dest
	emptyNode := node{}
	for current != emptyNode {
		path = append(path, current)
		var ok bool
		current, ok = predecessors[current]
		if !ok {
			current = emptyNode
		}
	}
	for i := 0; i < len(path)/2; i++ {
		j := len(path) - i - 1
		path[i], path[j] = path[j], path[i]
	}
	return path
}
