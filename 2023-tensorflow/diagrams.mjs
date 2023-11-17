#!/usr/bin/env node

import * as fs from "fs";

const commits = new Map();
const exclusions = JSON.parse(fs.readFileSync("exclude.json", "utf8"));
for (const [commit, excludes] of Object.entries(exclusions)) {
  for (let i = 0; i < excludes.length; ++i) {
    const exclude = new Set(excludes[i]);
    const data = JSON.parse(fs.readFileSync(`${commit}/data${i}.json`, "utf8"));
    for (const name of Object.keys(data)) {
      if (!exclude.has(name)) {
        commits.set(name, commit);
      }
    }
  }
}

let html = `<!DOCTYPE html>
<html>
  <head>
    <link rel="stylesheet" href="diagrams.css" />
  </head>
  <body>`;
const rose = JSON.parse(fs.readFileSync("rose/data.json", "utf8"));
for (const [name, data] of Object.entries(rose)) {
  if ("seconds" in data) {
    const commit = commits.get(name);
    html += `
    <div class="page">
      <div class="image-container">
        <div class="svg-wrapper">
          <img src="${commit}/${name}.svg" />
          <div class="caption">TensorFlow.js</div>
        </div>
        <div class="svg-wrapper">
          <img src="rose/${name}.svg" />
          <div class="caption">Rose</div>
        </div>
      </div>
      <div class="footer">${name}</div>
    </div>`;
  }
}
html += `
  </body>
</html>
`;
fs.writeFileSync("diagrams.html", html, "utf8");
