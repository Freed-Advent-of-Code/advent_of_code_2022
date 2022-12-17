package main
 
import (
    "bufio"
    "fmt"
    "os"
    "strconv"
    "math"
)

func sumCalories(){

}

func main() {
 
    readFile, err := os.Open("input.txt")
    defer readFile.Close()
  
    if err == nil {
        fileScanner := bufio.NewScanner(readFile)
        fileScanner.Split(bufio.ScanLines)
    
        var maximumCalory float64 = 0
        var calorySum float64 = 0
        for fileScanner.Scan() {
            var calory string = fileScanner.Text()
            if calory != ""{
                calory, err := strconv.ParseFloat(calory, 64)
                if err == nil {
                    calorySum = calorySum + calory
                }
            } else {
                maximumCalory =math.Max(maximumCalory, calorySum)
                calorySum = 0
            }
        }

        fmt.Println(maximumCalory)
    }
}