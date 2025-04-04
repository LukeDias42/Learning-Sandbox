package main

type BST struct {
	Value int

	Left  *BST
	Right *BST
}

func (tree *BST) FindClosestValue(target int) int {
	smallest_difference := tree.Value - target
	if tree.Left != nil {
		value := tree.Left.FindClosestValue(target)
		if Abs(smallest_difference) > Abs(value-target) {
			smallest_difference = value - target
		}
		if tree.Value == 15 {
		}
	}
	if tree.Right != nil {
		value := tree.Right.FindClosestValue(target)
		if Abs(smallest_difference) > Abs(value-target) {
			smallest_difference = value - target
		}
	}

	return smallest_difference + target
}

func Abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}
