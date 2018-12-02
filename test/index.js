// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const test = require('tape');
const Container = require('@holochain/holochain-nodejs');

// instantiate an app from the DNA JSON bundle
const app = Container.loadAndInstantiate("dist/bundle.json")

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

const bookshelf = "QmdRvAfumSyggTi149xsyiUDbFXPaHtByYuyxCCwYrvkuu"
test('initialize application', (t) => {
  const result = app.call("books", "main", "init", {})
  t.deepEqual(result, {address: bookshelf})
  t.end()
})

//const bookAddress = "QmNgH7iApZXnWwBnTcufXjQB61Y6uhQHLTT6wrdXFanBt8"
const bookshelfLinkAddress = "QmNgH7iApZXnWwBnTcufXjQB61Y6uhQHLTT6wrdXFanBt8"
test('create a book', (t) => {
  const result = app.call("books", "main", "create_book", { 
    name: "Climate - a new story",
    author: "Charles Eisenstein",
    genre: "Education",
    blurb: "A thriving biosphere through regeneration of ecosystems",
    shelf: "QmdRvAfumSyggTi149xsyiUDbFXPaHtByYuyxCCwYrvkuu"})
  t.deepEqual(result, { address: bookshelfLinkAddress })
  t.end()
})

const collectionAddress = "QmUcZSjYzsFQNRYsVH4NQhhjF3FNEFEosHE4isxSNHPwMg"
test('create a collection', (t) => {
  const result = app.call("books", "main", "create_collection", { name: "Learning"})
  t.deepEqual(result, {address: collectionAddress})
  t.end()
})

const userAddress = "QmVkSK5TmPdzGjKUkBBwqrxtcjQ1wA9Ze3a2YSzPU8gxEG"
test('create a user', (t) => {
  const result = app.call("books", "main", "create_user", {
    name: "Viktor Z",
    street: "Backavägen 8",
    zip: "26868",
    city: "Röstånga",
    country: "Sweden",
  })
  t.deepEqual(result, {address: userAddress})
  t.end()
})

// bookAddress = "QmNgH7iApZXnWwBnTcufXjQB61Y6uhQHLTT6wrdXFanBt8"
test('get a book', (t) => {
  const result = app.call("books", "main", "get_book", {address: "QmNgH7iApZXnWwBnTcufXjQB61Y6uhQHLTT6wrdXFanBt8"})
  t.deepEqual(result, { value: '{"name":"Climate - a new story","author":"Charles Eisenstein","genre":"Education","blurb":"A thriving biosphere through regeneration of ecosystems"}', entry_type: 'book' })
  t.end()
})
/*
test('retrieve a list of all books in dht', (t) => {
  const result = app.call("books", "main", "get_books", {})
  t.deepEqual(result, {addresses: [ "QmXWHWFiuNcz5mYGAVUJkU6jsLdybZc6ZKFykC5CoC8niZ" ]})
  t.end()
})
*/