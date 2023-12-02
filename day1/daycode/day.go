package daycode

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
	"sync"
)

type LineDigit struct {
	Position int
	Value    int
}

func DayResult(path string) string {
	lines := readInput(path)
	results := make(chan int)
	wg := &sync.WaitGroup{}

	for _, line := range lines {
		wg.Add(1)
		go processLine(line, wg, results)
	}

	go func() {
		wg.Wait()
		close(results)
	}()

	var finalList []int
	for result := range results {
		finalList = append(finalList, result)
	}

	return sumList(finalList)
}

func processLine(line string, wg *sync.WaitGroup, results chan<- int) {
	defer wg.Done()
	var digits []string
	sep := strings.Split(line, "")
	for pos, char := range sep {
		_, err := strconv.Atoi(char)
		if err == nil {
			digit := LineDigit{Position: pos, Value: int(char[0]) - 48}
			fmt.Println(digit)
			digits = append(digits, char)
		}
	}
	doubleDigit, _ := strconv.Atoi(digits[0] + digits[len(digits)-1])
	results <- doubleDigit
}

func readInput(path string) []string {
	lines := make([]string, 0)
	file, err := os.Open(path)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	return lines
}

func sumList(list []int) string {
	sum := 0
	for _, num := range list {
		sum += num
	}
	return strconv.Itoa(sum)
}

func findStringDigit(line string) []LineDigit {
	var digits []LineDigit
	numbers := []string{"one", "two", "three", "four", "five", "six", "seven", "eight", "nine"}
	for i, number := range numbers {
		if strings.Contains(line, number) {
			// find the position of the number
			occurrances := IndexOfSubstring(line, number)
			for _, occurrance := range occurrances {
				digits = append(digits, LineDigit{Position: occurrance, Value: i + 1})
			}
		}
	}
	return digits
}

func IndexOfSubstring(str, subStr string) []int {
	var occurrences []int
	for i := 0; i < len(str); i++ {
		if str[i:i+len(subStr)] == subStr {
			occurrences = append(occurrences, i)
		}
	}
	return occurrences
}
