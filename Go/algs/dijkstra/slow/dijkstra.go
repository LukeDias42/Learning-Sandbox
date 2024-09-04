package main

type vertex struct {
	node
	distance float64
}

type node struct {
	city string
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
