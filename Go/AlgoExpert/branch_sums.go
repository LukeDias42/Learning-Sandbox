package main

type BinaryTree struct {
	Value int
	Left  *BinaryTree
	Right *BinaryTree
}

func BranchSums(root *BinaryTree) []int {
	sums := []int{}
	root.CalculateSums(&sums, 0)
	return sums
}

func (node *BinaryTree) CalculateSums(sums *[]int, totalNodeSum int) {
	if node == nil {
		return
	}
	totalNodeSum += node.Value

	if node.isLeafNode() {
		*sums = append(*sums, totalNodeSum)
	}
}

func (node *BinaryTree) isLeafNode() bool {
	if node != nil &&
		node.Left == nil &&
		node.Right == nil {
		return true
	}
	return false
}
