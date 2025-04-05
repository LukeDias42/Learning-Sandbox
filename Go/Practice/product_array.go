package main

// Question 238 from leetcode

func productExceptSelf(nums []int) []int {
	zeros := 0
	product := 1
	for _, num := range nums {
		if num == 0 {
			zeros += 1
			continue
		}
		product *= num
	}

	solution := make([]int, len(nums))
	for i, num := range nums {
		if zeros >= 1 {
			if num == 0 && zeros == 1 {
				solution[i] = product
				continue
			}
			solution[i] = 0
			continue
		}
		solution[i] = product / num
	}
	return solution
}
