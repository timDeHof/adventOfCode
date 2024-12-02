/**
 * Advent of Code 2023
 * Day 06
 *
 * URL: [Link to the challenge for Day {{DAY_NUMBER}}]
 */
// Import necessary libraries or modules
// import readFile from "./utils.js"
import fs from "fs";
// Function to solve part 2 of the challenge

const calculateWays = (time, distance) => {
  let ways = [];
  for (let i = 0; i <= time; i++) {
    let millimeters = i * (time - i);
    if (millimeters > distance) {
      ways.push(i);
    }
  }
  return ways;
};
export const solvePart2 = () => {
  fs.readFile("input.txt", "utf-8", (err, data) => {
    const races = data.split("\n");
    const time = Number(
      races[0].split("Time:").pop().trim().split(/\s+/).join(""),
    );
    const distance = Number(
      races[1].split("Distance:").pop().trim().split(/\s+/).join(""),
    );
    let totalWays = calculateWays(time, distance);
    console.log(totalWays.length);
  });
};
solvePart2();
