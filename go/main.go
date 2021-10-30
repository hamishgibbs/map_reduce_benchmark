package main

import (
  "fmt"
  "io/ioutil"
  "log"
  "os"
  "strings"
  "strconv"
  )

type userdata struct {
    user uint64
    friends  []uint8
}

func main() {
  // read csv file and extract tuple of (user, friends)
  files, err := ioutil.ReadDir("../data")
  check(err)

  var s []userdata

  for _, file := range files {

      dat := ReadDataFile("../data/" + file.Name())

      user := strings.Replace(file.Name(), ".csv", "", 1)
      user_parsed, err := strconv.ParseUint(user, 10, 64)
      check(err)
      s = append(s, userdata{user_parsed, dat})

      // read this file and return a tuple of (u, f)
  }
  //fmt.Print(s)

  Map(s[0])

  // Map user and friends to {(u, u): [f]}

  // combine all friend maps into one

  // reduce based on intersectino of pairs of friends

  println(Hello("Yo"))
}

func Map(user userdata) {
  //var user_map[[]uint64][]uint64

  sum := 0
	for i := 0; i < len(user.friends); i++ {
		sum += i
	}
  fmt.Print(sum)

}


func ReadDataFile(name string) []uint8 {
  dat, err := os.ReadFile(name)
  check(err)
  return dat
}

func check(e error) {
    if e != nil {
        log.Fatal(e)
    }
}

func Hello(name string) string {
    // Return a greeting that embeds the name in a message.
    message := fmt.Sprintf("Hi, %v. Welcome!", name)
    return message
}
