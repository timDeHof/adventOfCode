/**
 * Advent of Code 2023
 * Day 06
 *
 * URL: [Link to the challenge for Day {{DAY_NUMBER}}]
 */
// Import necessary libraries or modules
// import readFile from "./utils.js"
import fs from "fs";
// Function to solve part 1 of the challenge
export const solvePart1 = () => {
  const input = fs.readFile("input.txt", "utf-8", (err, data) => {
    const races = data.split("\n");
    const times = races[0].split("Time:").pop().trim();
    const distances = races[1].split("Distance:").pop().trim();

    const timesArr = times.split(/\s+/).map(Number);
    const distArr = distances.split(/\s+/).map(Number);
    let totalWays = [];
    for (let i = 0; i < timesArr.length; i++) {
      const bestDist = distArr[i];
      const raceTime = timesArr[i];
      let ways = [];
      for (let i = 0; i <= raceTime; i++) {
        let remaining = raceTime - i;
        if (i * remaining > bestDist) {
          ways.push(i);
        }
      }
      totalWays.push(ways.length);
    }
    console.log(totalWays.reduce((acc, cv) => acc * cv, 1));
  });
};
solvePart1();
