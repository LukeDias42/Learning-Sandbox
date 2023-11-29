package main

func NodeDepths(root *BinaryTree) int {
	sum := 0
	root.calculateSum(&sum, 0)
	return sum
}

func (node *BinaryTree) calculateSum(sum *int, depth int) {
	if node == nil {
		return
	}
	*sum += depth

	if node.isLeafNode() {
		return
	}

	node.Left.calculateSum(sum, depth+1)
	node.Right.calculateSum(sum, depth+1)
}
