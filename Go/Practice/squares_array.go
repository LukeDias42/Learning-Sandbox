package main

// Question 977 from leetcode

func sortedSquares(nums []int) []int {
	right := len(nums) - 1
	left := 0
	solution := make([]int, len(nums))
	for i := range nums {
		if nums[right]*nums[right] > nums[left]*nums[left] {
			solution[len(nums)-1-i] = nums[right] * nums[right]
			right -= 1
		} else {
			solution[len(nums)-1-i] = nums[left] * nums[left]
			left += 1
		}
	}
	return solution
}
