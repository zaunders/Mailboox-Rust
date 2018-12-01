// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const test = require('tape');
const Container = require('@holochain/holochain-nodejs');

// instantiate an app from the DNA JSON bundle
const app = Container.instanceFromNameAndDna("dist/bundle.json")

// activate the new instance
app.start()

test('description of example test', (t) => {
  // Make a call to a Zome function
  // indicating the capability and function, and passing it an input
  // const result = app.call("zome-name", "capability-name", "function-name", {})

  // check for equality of the actual and expected results
  // t.equal(result, "expected result!")

  // ends this test
  t.end()
})
const bookAddress = "QmWyA4MpWazSQBEh7WLTLdHPFCUk31hbcacnJr87LCWR9T"

create_book('create a book', (t) => {
  const result = app.call("books", "main", "create_book", { 
    name: "Climate - a new story",
    author: "Charles Eisenstein",
    genre: "Education",
    blurb: "A thriving biosphere through regeneration of ecosystems"})
  t.deepEqual(result, { address: bookAddress })
  t.end()
})
