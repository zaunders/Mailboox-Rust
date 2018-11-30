#![feature(try_from)]
use std::convert::TryFrom;

#[macro_use]
extern crate hdk;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate holochain_core_types_derive;
#[macro_use]
extern crate serde_json;

use hdk::holochain_core_types::{
    hash::HashString,
    error::HolochainError,
    entry::Entry,
    dna::zome::entry_types::Sharing,
    entry::entry_type::EntryType,
    json::JsonString,
    cas::content::Address
};


// see https://developer.holochain.org/api/0.0.2/hdk/ for info on using the hdk library

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
struct Book {
	name: String;
    author: String;
    genre: String;
    blurb: String;
    user: String
}

#[derive](Serialize, Deserialize, Debug, DefaultJson)]
struct Collection {
    name: String
}

define_zome! {
    entries: [
        entry!(
            name: "book",
            description: "a physical book",
            sharing: Sharing::Public,
            native_type: Book,
            validation_package: || hdk::ValidationPackageDefinition::Entry,
            validation: |book: Book, _ctx: hdk::ValidationData| {
                ok(())
            },
            links: [
                /*
                to!(
                    "bookOwner",
                    tag: "owner"
                    validation_package: || hdk:ValidationPackageDefinition::Entry,
                    validation: |base: Address, target: Address, _ctx: hdk:ValidationData| {
                        Ok(())
                    }
                )
                to!(
                    "bookBorrower",
                    tag: "borrower"
                    validation_package: || hdk:ValidationPackageDefinition::Entry,
                    validation: |base: Address, target: Address, _ctx: hdk:ValidationData| {
                        Ok(())
                    }
                )*/
                to!(
                    "collection",
                    tag: "inCollection"
                    validation_package: || hdk:ValidationPackageDefinition::Entry,
                    validation: |base: Address, target: Address, _ctx: hdk:ValidationData| {
                        Ok(())
                    }
                )
            ]
        )
    ]

    genesis: || { Ok(()) }

    functions: {
        main (Public) {
            create_book: {
                inputs: |book: Book|,
                outputs: |result: JsonString|,
                handler: handle_create_book
            }
            create_collection: {
                inputs: |collection: Collection|,
                outputs: |result: JsonString|,
                handler: handle_create_collection
            }
            //retrieve all books (?)
            /*get_books: {
                inputs: |???: ????|,
                outputs: |result: JsonString|,
                handler: handle_get_books
            }*/
            /*request_to_borrow: {
                inputs: |**: **|,
                outputs: |result: JsonString|,
                handler: handle_request_to_borrow
            }*/
            /*accept_request_to_borrow: {
                inputs: |**: **|,
                outputs: |result: JsonString|,
                handler: handle_accept_request_to_borrow
            }*/
            /*mark_book_returned: {
                inputs: |**: **|,
                outputs: |result: JsonString|,
                handler: handle_mark_book_returned
            }*/
            add_book_to_collection: {
                inputs: |book: Book, collection: Collection|
                outputs: |result: JsonString|,
                handler: handle_add_book_to_collection
            }

        }
    }
}
