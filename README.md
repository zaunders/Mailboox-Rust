# Mailboox - A distributed liberary powered by Holochain

## Application synopsis
Enter the books that are in your bookshelf in order to make them visible to your community. Users can request to borrow books from each other and if possible the owner can put the book in his/her Mailbox in order to make it easily available for the borrower to pick-up. 

## Installation
This application uses the Holochain distributed data storage engine as backend. It uses Distributed Hash Tables to create content addressable spaces where users are able to find the books that have been made available. 

In order to run the application (*not working yet!*) you will need to install Holochain on your machine. Find more information on this at https://github.com/holochain/holochain-rust

## Running for the UI
`hc run --port 3400 --package`

make sure to stop it and restart if you change the files

## Implementable features
- [ ] Lending time-outs (Maybe default at 2 months but editable on accepting request)
- [ ] Making borrowing history of the borrower visible to the owner of a book that is requested (return in time reputation)
- [ ] Optical Character Recognition for scanning books (read title, author)
- [ ] Meta-data import (Scrape open repositories for book metadata inmport and linking to outside resources)
- [ ] User reviews - Load published user reviews from the community when looking at a book in the application
- [ ] Geographic constraints, enable people to input their address as GPS coordinates in order to set max-distance in filtering
