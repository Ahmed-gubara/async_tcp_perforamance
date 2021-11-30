package main

import (
	"bufio"
	"fmt"
	"math/rand"
	"net"
	"time"
)

const MIN = 1
const MAX = 100

func random() int {
	return rand.Intn(MAX-MIN) + MIN
}

func handleConnection(c net.Conn) {
	// fmt.Printf("Serving %s\n", c.RemoteAddr().String())
	defer c.Close()
	reader := bufio.NewReader(c)
	for {
		netData, err := reader.ReadString('\n')
		if err != nil {
			// fmt.Println(err)
			return
		}
		// fmt.Printf("recived %d\n", len(netData))
		result := findLongestString(netData) + "\n"
		c.Write([]byte(result))
	}
}
func findLongestString(s string) string {
	chars := []rune(s)
	leng := len(chars)
	charIndexes := make(map[rune]int)
	matchIndex := 0
	matchLength := 0
	index := 0
	for i := 0; i < leng; i++ {
		char := chars[i]
		charIndex, charExist := charIndexes[char]
		if charExist && charIndex >= index {
		} else {
			charExist = false
		}
		isTheEnd := charIndex == leng-1

		if charExist || isTheEnd {
			length := charIndex - index

			if !charExist {
				length++
			}
			if length > matchLength {
				matchIndex = index
				matchLength = length
			}
			if charExist {
				index = charIndexes[char] + 1
			}
		}

		charIndexes[char] = i
	}
	return string(chars[matchIndex : matchIndex+matchLength])

}

func main() {
	// arguments := os.Args
	// if len(arguments) == 1 {
	// 	fmt.Println("Please provide a port number!")
	// 	return
	// }

	PORT := ":8080"
	l, err := net.Listen("tcp4", PORT)
	if err != nil {
		fmt.Println(err)
		return
	}
	defer l.Close()
	rand.Seed(time.Now().Unix())
	fmt.Println("Waiting for connection!")
	for {
		c, err := l.Accept()
		if err != nil {
			// fmt.Println(err)
			return
		}
		go handleConnection(c)
	}
}
