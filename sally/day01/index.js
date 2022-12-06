const findMaxSum = () => Math.max(...require("fs").readFileSync("sally/day01/input.txt").toString().split("\n\n").map((elf) => elf.split("\n").map(Number).reduce((sum, calories) => sum + calories)))

const getTopThreeSum = () => {
  const inputs = require("fs").readFileSync("sally/day01/input.txt").toString().split("\n\n").map((elf) => elf.split("\n").map(Number).reduce((sum, calories) => sum + calories))
  inputs.sort((cal1, cal2) => cal2 - cal1)
  return [inputs[0], inputs[1], inputs[2]].reduce((sum, calories) => sum + calories)
}



console.log(findMaxSum())
console.log(getTopThreeSum())