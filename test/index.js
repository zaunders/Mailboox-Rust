// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const test = require('tape');
const Container = require('@holochain/holochain-nodejs');

// instantiate an app from the DNA JSON bundle
const app = Container.loadAndInstantiate("dist/bundle.json")

// activate the new instance
app.start()

const bookClimateAddress = "QmRMnPE86F4eBbTBNvJ9q9TPW5ZqoYqMzCzWc8ePQRmU7C"
const collectionLearningAddress = "QmUcZSjYzsFQNRYsVH4NQhhjF3FNEFEosHE4isxSNHPwMg"
const bookshelf = "QmdRvAfumSyggTi149xsyiUDbFXPaHtByYuyxCCwYrvkuu"
const viktorAddress = "QmVkSK5TmPdzGjKUkBBwqrxtcjQ1wA9Ze3a2YSzPU8gxEG"

const user1inputParams = {
  name: "Viktor Z",
  street: "Backavägen 8",
  zip: "26868",
  city: "Röstånga",
  country: "Sweden",
}
const user2inputParams = {
  name: "Joe smith",
  street: "Pineforest drive",
  zip: "56748",
  city: "London",
  country: "England",
}



test('initialize application', (t) => {
  const result = app.call("books", "main", "init", {})
  t.deepEqual(result, {address: bookshelf})
  t.end()
})

test('create a user', (t) => {
  const result = app.call("books", "main", "create_user", user1inputParams)
  t.deepEqual(result, {address: viktorAddress})
  t.end()
})

test('create a book', (t) => {
  // used to retrieve anchor address "shelf", which is a way to find all books in DHT
  const shelfAddress = app.call("books", "main", "init", {}).address

  const result = app.call("books", "main", "create_book", { 
    name: "Climate - a new story",
    author: "Charles Eisenstein",
    genre: "Education",
    blurb: "A thriving biosphere through regeneration of ecosystems",
    book_owner: viktorAddress,
    shelf_address: shelfAddress})
  t.deepEqual(result, { address: bookClimateAddress })
  t.end()
})

test('create a book collection', (t) => {
  const result = app.call("books", "main", "create_collection", { name: "Learning"})
  t.deepEqual(result, {address: collectionLearningAddress})
  t.end()
})


test('get data from a book address', (t) => {
  const result = app.call("books", "main", "get_book", {address: bookClimateAddress})
  t.deepEqual(result, { value: '{"name":"Climate - a new story","author":"Charles Eisenstein","genre":"Education","blurb":"A thriving biosphere through regeneration of ecosystems","book_owner":"QmVkSK5TmPdzGjKUkBBwqrxtcjQ1wA9Ze3a2YSzPU8gxEG"}', entry_type: 'book' })
  t.end()
})

test('get user data', (t) => {
  const result = app.call("books", "main", "get_user_data", {address: viktorAddress})
  t.deepEqual(result, { value: '{"name":"Viktor Z","street":"Backavägen 8","zip":"26868","city":"Röstånga","country":"Sweden"}', entry_type: 'user' })
  t.end()
})

test('retrieve a list of all books in dht', (t) => {
  const result = app.call("books", "main", "get_books", {shelf_address: bookshelf, tag: "in shelf"})
  t.deepEqual(result, { addresses: [ 'QmRMnPE86F4eBbTBNvJ9q9TPW5ZqoYqMzCzWc8ePQRmU7C' ] })
  t.end()
})


test('add book to collection', (t) => {
  const result = app.call("books", "main", "add_book_to_collection", {
    book_address: bookClimateAddress, 
    collection_address: collectionLearningAddress})
  t.deepEqual(result, { success: true })
  t.end()
})


test('get books in collection', (t) => {
  const result = app.call("books", "main", "get_books_in_collection", {
    collection_address: collectionLearningAddress, 
    tag: "has book"})
  t.deepEqual(result, { addresses: [ 'QmRMnPE86F4eBbTBNvJ9q9TPW5ZqoYqMzCzWc8ePQRmU7C' ] })
  t.end()
})

test('get collections that book is in', (t) => {
  const result = app.call("books", "main", "get_collections_book_is_in", {
    book_address: bookClimateAddress,
    tag: "in collection"
  })
  t.deepEqual(result, { addresses: [ 'QmUcZSjYzsFQNRYsVH4NQhhjF3FNEFEosHE4isxSNHPwMg' ] })
  t.end()
})

//Requesting to borrow should be implemented as SEND message, when available, request data does not need to be in the DHT 
//Now app needs to check it's agents user entry for each "owns" link, check for "is requested by" links, connected to each of those links
//requests can then be turned into loan entries, which can be marked deleted when returned
test('requst to borrow', (t) => {
  //create additional user that can borrow
  const borrower = app.call("books", "main", "create_user", user2inputParams)
  const result = app.call("books", "main", "request_to_borrow", {
    book_address: bookClimateAddress,
    borrower_address: borrower.address,
  })
  t.deepEqual(result, { success: true })
  t.end()
})

test('get requests to borrow made by user', (t) => {
  const borrower = app.call("books", "main", "create_user", user2inputParams)
  const result = app.call("books", "main", "get_requests_by_user", {
    borrower_address: borrower.address
  })
  t.deepEqual(result, { addresses: [ 'QmRMnPE86F4eBbTBNvJ9q9TPW5ZqoYqMzCzWc8ePQRmU7C' ] })
  t.end()
})

test('get requests made to borrow a specific book', (t) => {
  const result = app.call("books", "main", "get_book_requests", {
    book_address: bookClimateAddress
  })
  t.deepEqual(result, { addresses: [ 'QmdXyveCzH21Xq328NyLfqGvpcS56usftuR724qvmvknJD' ] })
  t.end()
})

test('create a loan entry', (t) => {
  const borrower = app.call("books", "main", "create_user", user2inputParams)
  const result = app.call("books", "main", "create_loan", { 
    book_address: bookClimateAddress,
    borrower_address: borrower.address
    })
  t.deepEqual(result, { address: 'QmWqBWdCuEnkFD53wGg8pjDnFg4BqsXKQz9bagTxYvXJyF' })
  t.end() 
})


/* not implemented functionally, can we just use entry metadata stamping?
test('get current user address', (t) => {
  const current_user = app.call("books", "main", "get_current_user_address", {})
  t.deepEqual(result, {address: bookshelf})
  t.end()
})*/

