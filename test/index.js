// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const test = require('tape');
const Container = require('@holochain/holochain-nodejs');

// instantiate an app from the DNA JSON bundle
const app = Container.loadAndInstantiate("dist/bundle.json")

// activate the new instance
app.start()

const bookClimateAddress = "QmNgH7iApZXnWwBnTcufXjQB61Y6uhQHLTT6wrdXFanBt8"
const collectionLearningAddress = "QmUcZSjYzsFQNRYsVH4NQhhjF3FNEFEosHE4isxSNHPwMg"
const bookshelf = "QmdRvAfumSyggTi149xsyiUDbFXPaHtByYuyxCCwYrvkuu"
const viktorAddress = "QmVkSK5TmPdzGjKUkBBwqrxtcjQ1wA9Ze3a2YSzPU8gxEG"

test('initialize application', (t) => {
  const result = app.call("books", "main", "init", {})
  t.deepEqual(result, {address: bookshelf})
  t.end()
})

test('create a book', (t) => {
  const result = app.call("books", "main", "create_book", { 
    name: "Climate - a new story",
    author: "Charles Eisenstein",
    genre: "Education",
    blurb: "A thriving biosphere through regeneration of ecosystems"})
  t.deepEqual(result, { address: bookClimateAddress })
  t.end()
})

/* test with link to shelf
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
})*/

test('create a collection', (t) => {
  const result = app.call("books", "main", "create_collection", { name: "Learning"})
  t.deepEqual(result, {address: collectionLearningAddress})
  t.end()
})


test('create a user', (t) => {
  const result = app.call("books", "main", "create_user", {
    name: "Viktor Z",
    street: "Backavägen 8",
    zip: "26868",
    city: "Röstånga",
    country: "Sweden",
  })
  t.deepEqual(result, {address: viktorAddress})
  t.end()
})


test('get a book', (t) => {
  const result = app.call("books", "main", "get_book", {address: bookClimateAddress})
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

/*
test('add book to collection', (t) => {
  const result = app.call("books", "main", "add_book_to_collection", {
    book_address: bookClimateAddress, 
    collection_address: collectionLearningAddress})
  t.deepEqual(result, { success: true })
  t.end()
})
*/


test('add book to collection', (t) => {
  const result = app.call("books", "main", "add_book_to_collection", {
    base: collectionLearningAddress,
    target: bookClimateAddress,
    tag: "includes book"
  })
  const result2 = app.call("books", "main", "add_book_to_collection", {
    base: bookClimateAddress,
    target: collectionLearningAddress,
    tag: "is in collection"
  })
  t.deepEqual(result2, { success: true })
  t.end()
})

test('get books in collection', (t) => {
  const result = app.call("books", "main", "get_books_in_collection", {
    collection_address: collectionLearningAddress, 
    tag: "includes book"})
  t.deepEqual(result, { addresses: [ 'QmNgH7iApZXnWwBnTcufXjQB61Y6uhQHLTT6wrdXFanBt8' ] })
  t.end()
})

test('get collections that book is in', (t) => {
  const result = app.call("books", "main", "get_collections_book_is_in", {
    book_address: bookClimateAddress,
    tag: "is in collection"
  })
  t.deepEqual(result, { addresses: [ 'QmUcZSjYzsFQNRYsVH4NQhhjF3FNEFEosHE4isxSNHPwMg' ] })
  t.end()
})
