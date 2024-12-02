/**
 * Advent of Code 2023
 * Day 15
 *
 * URL: [Link to the challenge for Day {{DAY_NUMBER}}]
 */
import fs from "fs";
// Function to solve part 1 of the challenge
function toHash(sequence, initialValue) {
  let currentValue = initialValue;
  for (let i = 0; i < sequence.length; i++) {
    currentValue += sequence.charCodeAt(i);
    currentValue *= 17;
  }
  return currentValue % 256;
}
fs.readFile("example.txt", "utf-8", (err, data) => {
  const input = data.split(",");
  // const boxes = new Array(256).fill("").map(() => []);
  //   const input = ["HASH"];
  let sumSeq = input.map((sequence) => toHash(sequence, 0));

  console.log(sumSeq.reduce((acc, cv) => acc + cv, 0));
});
//# sourceMappingURL=dayTemplate.js.map
