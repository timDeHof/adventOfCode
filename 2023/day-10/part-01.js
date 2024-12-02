/**
 * Advent of Code 2023
 * Day 10
 *
 * URL: [Link to the challenge for Day {{DAY_NUMBER}}]
 */
// Import necessary libraries or modules
import fs from "fs";
// Function to solve part 1 of the challenge
const getStartPoint = (input) => {
  for (let y = 0; y < input.length; y++) {
    const row = input[y];
    for (let x = 0; x < input.length; x++) {
      if (row[x] === "S") {
        return { x, y };
      }
    }
  }
};

const selectPath = (input, start) => {
  const { x, y } = start;

  if (
    y + 1 < input.length &&
    (input[y + 1][x] === "|" ||
      input[y + 1][x] === "L" ||
      input[y + 1][x] === "J")
  ) {
    return { dir: "SOUTH", y: y + 1, x: x };
  }

  if (
    y - 1 >= 0 &&
    (input[y - 1][x] === "|" ||
      input[y - 1][x] === "F" ||
      input[y - 1][x] === "7")
  ) {
    return { dir: "NORTH", y: y - 1, x: x };
  }

  return { dir: "EAST", y, x: x + 1 };
};

export const solvePart1 = () => {
  fs.readFile("input.txt", "utf-8", (err, data) => {
    if (err) {
      console.error("Error reading file:", err);
      return;
    }
    const input = data.split("\n");
    let start = getStartPoint(input);

    if (!start) {
      console.error("Start point not found.");
      return;
    }
    let { dir, x, y } = selectPath(input, start);
    let path = [start, { x, y }];
    let steps = 1;

    while (x !== start.x || y !== start.y) {
      let candidateX = 0;
      let candidateY = 0;

      switch (`${input[y][x]} : ${dir}`) {
        case "| : SOUTH":
          candidateY = 1;
          break;
        case "| : NORTH":
          candidateY = -1;
          break;
        case "- : EAST":
          candidateX = 1;
          break;
        case "- : WEST":
          candidateX = -1;
          break;
        case "L : SOUTH":
          candidateX = 1;
          break;
        case "L : WEST":
          candidateY = -1;
          break;
        case "J : SOUTH":
          candidateX = -1;
          break;
        case "J : EAST":
          candidateY = -1;
          break;
        case "7 : NORTH":
          candidateX = -1;
          break;
        case "7 : EAST":
          candidateY = 1;
          break;
        case "F : NORTH":
          candidateX = 1;
          break;
        case "F : WEST":
          candidateY = 1;
          break;
        default:
          break;
      }
      if (candidateY === 1) {
        dir = "SOUTH";
      } else if (candidateY === -1) {
        dir = "NORTH";
      } else if (candidateX === -1) {
        dir = "WEST";
      } else if (candidateX === 1) {
        dir = "EAST";
      }
      x += candidateX;
      y += candidateY;
      steps++;
      path.push({ x, y });

      // Check for loop termination condition
      if (x === start.x && y === start.y) {
        break; // Exit the loop if we've returned to the start
      }
    }

    console.log("Path: ", path);

    console.log("Total steps: ", steps / 2);
  });
};
solvePart1();
