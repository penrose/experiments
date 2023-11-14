import * as fs from "fs/promises";

const tfjs = new Map();
const exclusions = JSON.parse(await fs.readFile("exclude.json", "utf8"));
for (const [commit, excludes] of Object.entries(exclusions)) {
  for (let i = 0; i < excludes.length; ++i) {
    const exclude = new Set(excludes[i]);
    const datas = await fs.readFile(`${commit}/data${i}.json`, "utf8");
    for (const [name, data] of Object.entries(JSON.parse(datas))) {
      if (!exclude.has(name)) {
        if (tfjs.has(name)) throw Error(`duplicate diagram: ${name}`);
        tfjs.set(name, data);
      }
    }
  }
}

const all = await fs.readFile("rose/data.json", "utf8");
const rose = new Map(Object.entries(JSON.parse(all)));

const getTime = (data) => data?.seconds?.optimizing ?? data?.totalSeconds ?? "";

const lines = ["name,tfjs,rose"];
for (const name of rose.keys())
  lines.push(`${name},${getTime(tfjs.get(name))},${getTime(rose.get(name))}`);
await fs.writeFile("data.csv", lines.join("\n"), "utf8");
