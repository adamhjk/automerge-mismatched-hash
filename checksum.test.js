const Automerge = require("automerge");
const { default: fetch } = require("node-fetch");

test("good doc is transferred", async () => {
  let res = await fetch("http://localhost:3031/good_doc");
  let buffer = await res.arrayBuffer();
  let bytes = new Uint8Array(buffer);
  let doc = Automerge.load(bytes);
  expect(doc.bobo).toEqual({ friendly: true, job: "clown" });
});

test("bad doc is transferred", async () => {
  let res = await fetch("http://localhost:3031/bad_doc");
  let buffer = await res.arrayBuffer();
  let bytes = new Uint8Array(buffer);
  let doc = Automerge.load(bytes);
  expect(doc.name).toEqual("descriptive-meeting-0524");
});
