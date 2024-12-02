// Import necessary libraries or modules
import fs from "fs";
import { promisify } from "util";

const readFileAsync = promisify(fs.readFile);

// Function to solve part 1 of the challenge
function calcNewSequence(arr) {
  const newSeq = [];
  for (let i = 0; i < arr.length - 1; i++) {
    newSeq.push(Number(arr[i + 1]) - Number(arr[i]));
  }
  return newSeq;
}

function zeroTest(element) {
  return element === 0;
}

export const solvePart1 = async () => {
  try {
    const data = await readFileAsync("input.txt", "utf-8");
    const lines = data.split("\n").map((line) => line.split(" ").map(Number));

    function getHistory(line) {
      const history = [line];
      let current = line;

      while (true) {
        let next = [];
        for (let i = 0; i < current.length - 1; i++) {
          next.push(current[i + 1] - current[i]);
        }
        history.push(next);
        if (next.some((x) => x)) {
          current = next;
          next = [];
        } else {
          break;
        }
      }
      return history;
    }

    function getNext(history) {
      history.reverse();
      for (let i = 0; i < history.length - 1; i++) {
        const sum = history.at(i).at(-1) + history.at(i + 1).at(-1);
        history.at(i + 1).push(sum);
      }
      return history.at(-1).at(-1);
    }

    let res = lines.map((line) => {
      const lineHistory = getHistory(line);
      return getNext(lineHistory);
    });

    console.log(res.reduce((acc, cv) => acc + cv, 0));
  } catch (err) {
    console.error("Error reading file:", err);
  }
};

// Uncomment the line below to run the function
solvePart1();
