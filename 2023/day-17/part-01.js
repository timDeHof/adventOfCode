/**
 * Advent of Code 2023
 * Day {{DAY_NUMBER}}
 *
 * URL: [Link to the challenge for Day {{DAY_NUMBER}}]
 */
// Import necessary libraries or modules

import fs from "fs";

fs.readFile("example.txt", "utf-8", (err, data) => {
  const input = data.split("\n").map((line) => line.split(""));
  console.log(input);
  let heatLoss = 0;
  for (let i = 0; i < input.length; i++) {
    for (let j = 0; j < input[i].length; j++) {}
  }
  console.log(heatLoss);
});
