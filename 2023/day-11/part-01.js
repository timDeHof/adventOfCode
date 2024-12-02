/**
 * Advent of Code 2023
 * Day 11
 *
 * URL: [Link to the challenge for Day 11]
 */
// Import necessary libraries or modules
import fs from "fs";

function getPathLength(start, end) {
  return Math.abs(start[0] - end[0]) + Math.abs(start[1] - end[1]);
}
// Function to solve part 1 of the challenge
const solvePart1 = () => {
  fs.readFile("input.txt", "utf-8", (err, data) => {
    if (err) {
      console.error("Error reading file:", err);
      return;
    }
    const input = data.split("\n").filter((n) => n);
    const grid = input.map((line) => line.split(""));

    const newRows = [];
    for (const row of grid) {
      if (row.some((cell) => cell === "#")) {
        newRows.push(row);
      } else {
        newRows.push(row);
        newRows.push(row);
      }
    }
    const newGrid = new Array(newRows.length).fill(0).map(() => []);
    console.log(newGrid);
    for (let i = 0; i < newRows[0].length; i++) {
      let empty = true;
      for (let j = 0; j < newRows.length; j++) {
        if (newRows[j][i] === "#") {
          empty = false;
          break;
        }
      }
      if (!empty) {
        for (let j = 0; j < newRows.length; j++) {
          newGrid[j].push(newRows[j][i]);
        }
      } else {
        for (let j = 0; j < newRows.length; j++) {
          newGrid[j].push(".");
          newGrid[j].push(".");
        }
      }
    }

    const galaxies = [];

    for (let i = 0; i < newGrid[0].length; i++) {
      for (let j = 0; j < newGrid.length; j++) {
        if (newGrid[j][i] === "#") {
          galaxies.push([i, j]);
        }
      }
    }

    let sum = 0;

    for (let i = 0; i < galaxies.length; i++) {
      for (let j = i + 1; j < galaxies.length; j++) {
        sum += getPathLength(galaxies[i], galaxies[j]);
      }
    }
    console.log(sum);
  });
};

// Invoke the function to solve part 1
solvePart1();
