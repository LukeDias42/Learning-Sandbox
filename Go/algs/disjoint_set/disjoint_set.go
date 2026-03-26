package main
import "fmt"

type DisjointSet struct {
	parent map[int]int
	rank   map[int]int
}

func (d *DisjointSet) MakeSet(n int) {
	d.parent = make(map[int]int)
	d.rank = make(map[int]int)
	for i := 0; i <= n; i++ {
		d.parent[i] = i
		d.rank[i] = 0
	}
}

func (d *DisjointSet) Find(x int) int {
	if x == d.parent[x] {
		return x
	}
	d.parent[x] = d.Find(d.parent[x])
	return d.parent[x]
}

func (d *DisjointSet) Union(x, y int) {
	xRoot := d.Find(x)
	yRoot := d.Find(y)
	if xRoot == yRoot {
		return
	}
	if d.rank[xRoot] < d.rank[yRoot] {
		d.parent[xRoot] = yRoot
	} else if d.rank[yRoot] < d.rank[xRoot] {
		d.parent[yRoot] = xRoot
	} else {
		d.parent[yRoot] = xRoot
		d.rank[xRoot] += 1
	}
}

func (d *DisjointSet) Print() {
	fmt.Println(d.parent)
}

func (d *DisjointSet) isConnected(x, y int) bool {
	return d.Find(x) == d.Find(y)
}

func main() {
	d := DisjointSet{}
	d.MakeSet(8)
	d.Print()
	d.Union(0, 1)
	d.Print()
	d.Union(2, 3)
	d.Print()
	d.Union(2, 4)
	d.Print()
	d.Union(1, 4)
	d.Print()
	d.Union(5, 6)
	d.Print()
	d.Union(5, 7)
	d.Print()
	d.Union(7, 8)
	d.Print()
	fmt.Println(d.isConnected(0, 1))
	d.Print()
	fmt.Println(d.isConnected(2, 3))
	d.Print()
	fmt.Println(d.isConnected(2, 4))
	d.Print()
	fmt.Println(d.isConnected(1, 4))
	d.Print()
	fmt.Println(d.isConnected(1, 5))
	d.Print()
	fmt.Println(d.isConnected(2, 5))
	d.Print()
	fmt.Println(d.isConnected(2, 5))
	d.Print()
	fmt.Println(d.isConnected(6, 5))
	d.Print()
	fmt.Println(d.isConnected(7, 5))
	d.Print()
	fmt.Println(d.isConnected(8, 5))
	d.Print()
	d.Union(0, 8)
	d.Print()
	fmt.Println(d.isConnected(8, 0))
	d.Print()
	fmt.Println(d.isConnected(1, 5))
	d.Print()
}
