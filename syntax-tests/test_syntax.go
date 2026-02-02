// Go Syntax Test File
// Testing Go syntax highlighting with various language features

package main

import (
	"fmt"
	"math"
	"sync"
	"time"
)

// Constants
const (
	MaxSize     = 1024
	AppName     = "TestApp"
	Version     = "1.0.0"
	Pi          = 3.14159
	StatusOK    = 200
	StatusError = 500
)

// iota enumeration
const (
	Sunday = iota
	Monday
	Tuesday
	Wednesday
	Thursday
	Friday
	Saturday
)

// Type definitions
type Person struct {
	Name   string
	Age    int
	Salary float64
}

type Employee struct {
	Person           // Embedded struct
	Department string
	Manager    *Employee
}

// Interface
type Shape interface {
	Area() float64
	Perimeter() float64
}

// Rectangle implements Shape
type Rectangle struct {
	Width  float64
	Height float64
}

func (r Rectangle) Area() float64 {
	return r.Width * r.Height
}

func (r Rectangle) Perimeter() float64 {
	return 2 * (r.Width + r.Height)
}

// Circle implements Shape
type Circle struct {
	Radius float64
}

func (c Circle) Area() float64 {
	return math.Pi * c.Radius * c.Radius
}

func (c Circle) Perimeter() float64 {
	return 2 * math.Pi * c.Radius
}

// Methods
func (p *Person) UpdateAge(newAge int) {
	p.Age = newAge
}

func (p Person) GetInfo() string {
	return fmt.Sprintf("%s is %d years old", p.Name, p.Age)
}

// Function with multiple return values
func divide(a, b float64) (float64, error) {
	if b == 0 {
		return 0, fmt.Errorf("division by zero")
	}
	return a / b, nil
}

// Named return values
func swap(a, b int) (x, y int) {
	x = b
	y = a
	return // Naked return
}

// Variadic function
func sum(numbers ...int) int {
	total := 0
	for _, num := range numbers {
		total += num
	}
	return total
}

// Higher-order function
func apply(fn func(int) int, value int) int {
	return fn(value)
}

// Closure
func makeAdder(x int) func(int) int {
	return func(y int) int {
		return x + y
	}
}

// Main function
func main() {
	// Number literals
	decimal := 42
	hex := 0xFF
	octal := 0o77
	binary := 0b1010_1011
	bigNum := 1234567890
	
	// Floating point
	pi := 3.14159
	e := 2.718281828
	scientific := 1.23e10
	
	// Complex numbers
	complex1 := 3 + 4i
	complex2 := complex(5, 6)
	
	// String literals
	str := "Hello, Go!"
	rawStr := `This is a raw string
that can span multiple lines
and include "quotes" without escaping`
	
	// Rune (character) literals
	ch := 'A'
	unicode := '世'
	escape := '\n'
	
	// Boolean and nil
	flag := true
	success := false
	var ptr *int = nil
	
	// Type inference with :=
	message := "Type inferred"
	count := 10
	
	// Multiple assignment
	x, y := 10, 20
	x, y = y, x // Swap
	
	// Array
	var array [5]int
	array = [5]int{1, 2, 3, 4, 5}
	arrayInit := [...]int{1, 2, 3} // Length inferred
	
	// Slice
	slice := []int{1, 2, 3, 4, 5}
	slicePart := slice[1:4]
	
	// Make slice
	dynamicSlice := make([]int, 5, 10) // length 5, capacity 10
	
	// Append to slice
	slice = append(slice, 6, 7, 8)
	
	// Map
	ages := map[string]int{
		"Alice": 25,
		"Bob":   30,
		"Charlie": 35,
	}
	
	// Make map
	scores := make(map[string]int)
	scores["test1"] = 90
	scores["test2"] = 85
	
	// Check map key
	value, exists := ages["Alice"]
	if exists {
		fmt.Println("Alice's age:", value)
	}
	
	// Struct initialization
	person := Person{
		Name:   "Alice",
		Age:    25,
		Salary: 50000.0,
	}
	
	// Anonymous struct
	point := struct {
		X int
		Y int
	}{10, 20}
	
	// Pointer
	ptr2 := &person
	ptr2.Age = 26
	
	// If statement
	if decimal > 40 {
		fmt.Println("Greater than 40")
	} else if decimal > 30 {
		fmt.Println("Greater than 30")
	} else {
		fmt.Println("30 or less")
	}
	
	// If with short statement
	if result, err := divide(10, 2); err == nil {
		fmt.Println("Result:", result)
	}
	
	// Switch statement
	switch decimal {
	case 0:
		fmt.Println("Zero")
	case 42:
		fmt.Println("The answer")
	default:
		fmt.Println("Other number")
	}
	
	// Switch with no condition (like if-else chain)
	switch {
	case decimal < 10:
		fmt.Println("Less than 10")
	case decimal < 50:
		fmt.Println("Less than 50")
	default:
		fmt.Println("50 or more")
	}
	
	// Type switch
	var i interface{} = "hello"
	switch v := i.(type) {
	case int:
		fmt.Println("Integer:", v)
	case string:
		fmt.Println("String:", v)
	default:
		fmt.Println("Unknown type")
	}
	
	// For loop (traditional)
	for i := 0; i < 10; i++ {
		fmt.Print(i, " ")
	}
	fmt.Println()
	
	// For loop (while style)
	i := 0
	for i < 5 {
		i++
	}
	
	// Infinite loop
	for {
		if i > 10 {
			break
		}
		i++
	}
	
	// Range over slice
	for index, value := range slice {
		fmt.Printf("Index: %d, Value: %d\n", index, value)
	}
	
	// Range over map
	for key, value := range ages {
		fmt.Printf("%s: %d\n", key, value)
	}
	
	// Range with _ to ignore index
	for _, value := range slice {
		fmt.Println(value)
	}
	
	// Defer statement
	defer fmt.Println("This executes last")
	
	// Multiple defers (execute in LIFO order)
	defer fmt.Println("Third")
	defer fmt.Println("Second")
	defer fmt.Println("First")
	
	// Goroutine
	go func() {
		fmt.Println("Running in goroutine")
	}()
	
	// Channel
	ch := make(chan int)
	go func() {
		ch <- 42 // Send to channel
	}()
	value2 := <-ch // Receive from channel
	fmt.Println("Received:", value2)
	
	// Buffered channel
	buffered := make(chan int, 2)
	buffered <- 1
	buffered <- 2
	fmt.Println(<-buffered)
	fmt.Println(<-buffered)
	
	// Select statement
	ch1 := make(chan int)
	ch2 := make(chan int)
	
	go func() {
		time.Sleep(100 * time.Millisecond)
		ch1 <- 1
	}()
	
	select {
	case val := <-ch1:
		fmt.Println("Received from ch1:", val)
	case val := <-ch2:
		fmt.Println("Received from ch2:", val)
	case <-time.After(200 * time.Millisecond):
		fmt.Println("Timeout")
	}
	
	// WaitGroup for synchronization
	var wg sync.WaitGroup
	
	for i := 0; i < 5; i++ {
		wg.Add(1)
		go func(id int) {
			defer wg.Done()
			fmt.Printf("Worker %d\n", id)
		}(i)
	}
	
	wg.Wait()
	
	// Mutex
	var mutex sync.Mutex
	counter := 0
	
	mutex.Lock()
	counter++
	mutex.Unlock()
	
	// Error handling
	if result, err := divide(10, 0); err != nil {
		fmt.Println("Error:", err)
	} else {
		fmt.Println("Result:", result)
	}
	
	// Panic and recover
	defer func() {
		if r := recover(); r != nil {
			fmt.Println("Recovered from:", r)
		}
	}()
	
	// Type assertion
	var inter interface{} = "hello"
	str2, ok := inter.(string)
	if ok {
		fmt.Println("String:", str2)
	}
	
	// Built-in functions
	length := len(slice)
	capacity := cap(slice)
	fmt.Printf("Length: %d, Capacity: %d\n", length, capacity)
	
	// Make and new
	sliceNew := make([]int, 5)
	ptrNew := new(int)
	*ptrNew = 42
	
	// Copy
	dest := make([]int, len(slice))
	copy(dest, slice)
	
	// Delete from map
	delete(ages, "Alice")
	
	// Closure example
	adder := makeAdder(10)
	fmt.Println(adder(5)) // 15
	
	// Anonymous function
	result := func(a, b int) int {
		return a + b
	}(5, 3)
	
	fmt.Println("Result:", result)
	
	fmt.Println("Program completed")
}

// Exported function (starts with capital letter)
func ProcessData(data []byte) error {
	if len(data) == 0 {
		return fmt.Errorf("empty data")
	}
	return nil
}

// Unexported function (starts with lowercase letter)
func helperFunction() {
	fmt.Println("Helper function")
}
