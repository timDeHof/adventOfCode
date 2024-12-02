import fs from "fs";

fs.readFile("example.txt", "utf8", (err, data) => {
  const grid = data.split("\n").map((row) => row.split(""));
  console.log(grid);
});
