const asciiSub = (char1,char2) => char1.charCodeAt(0) - char2.charCodeAt(0)
const getTotalScore1 = () => require("fs").readFileSync("sally/day02/input.txt").toString().split("\n").map((game) => asciiSub(game[0], 'A') === asciiSub(game[2], 'X') ? (asciiSub(game[2], 'X') + 1) + 3 : (asciiSub(game[0], 'A') + 1) % 3 === asciiSub(game[2], 'X') ? (asciiSub(game[2], 'X') + 1) + 6 : asciiSub(game[2], 'X') + 1).reduce((sum, score) => sum + score)
const getTotalScore2 = () => 'TODO'

console.log(getTotalScore1())
console.log(getTotalScore2())
