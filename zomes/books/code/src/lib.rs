#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate holochain_core_types;
#[macro_use]
extern crate holochain_core_types_derive;
extern crate boolinator;

use boolinator::Boolinator;
use hdk::{
    holochain_core_types::{
        dna::zome::entry_types::Sharing,
        hash::HashString,
        json::JsonString,
        entry::Entry,
        entry::entry_type::EntryType,
        error::HolochainError,
        cas::content::Address,
    },
};


// see https://developer.holochain.org/api/0.0.2/hdk/ for info on using the hdk library

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
struct Book {
	name: String,
    author: String,
    genre: String,
    blurb: String,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
struct Collection {
    name: String,
}

define_zome! {

    entries: [
        entry!(
            name: "book",
            description: "a book",
            sharing: Sharing::Public,
            native_type: Book,
            validation_package: || hdk::ValidationPackageDefinition::Entry,
            validation: |book: Book, _ctx: hdk::ValidationData| {
                Ok(())
            },
            links: [
                to! (
                    "bookOwner",
                    tag: "owner",
                    validation_package: || hdk::ValidationPackageDefinition::Entry,
                    validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
                        Ok(())
                    }
                ),
                to! (
                    "bookBorrower",
                    tag: "borrower",
                    validation_package: || hdk::ValidationPackageDefinition::Entry,
                    validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
                        Ok(())
                    }
                ),
                to! (
                    "collection",
                    tag: "in collection",
                    validation_package: || hdk::ValidationPackageDefinition::Entry,
                    validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
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
                inputs: |name: String, author: String, genre: String, blurb: String|,
                outputs: |result: JsonString|,
                handler: handle_create_book
            }
            create_collection: {
                inputs: |name: String|,
                outputs: |result: JsonString|,
                handler: handle_create_collection
            }
            //retrieve all books (?)
            /*get_books: {
                inputs: |???: ????|,
                outputs: |result: JsonString|,
                handler: handle_get_books
            }request_to_borrow: {
                inputs: |**: **|,
                outputs: |result: JsonString|,
                handler: handle_request_to_borrow
            }accept_request_to_borrow: {
                inputs: |**: **|,
                outputs: |result: JsonString|,
                handler: handle_accept_request_to_borrow
            }mark_book_returned: {
                inputs: |**: **|,
                outputs: |result: JsonString|,
                handler: handle_mark_book_returned
            }
            add_book_to_collection: {
                inputs: |book: Book, collection: Collection|,
                outputs: |result: JsonString|,
                handler: handle_add_book_to_collection
            }*/
        }
    }
}

fn handle_create_book(name: String, author: String, genre: String, blurb: String) -> JsonString {
        let maybe_added = Entry::new(EntryType::App("book".into()), Book {
            name, author, genre, blurb
        });
        match hdk::commit_entry(&maybe_added) {
            Ok(address) => json!({"address": address}).into(),
            Err(hdk_err) => hdk_err.into()
        }
}

fn handle_create_collection(name: String) -> JsonString {
        let maybe_added = Entry::new(EntryType::App("collecition".into()), Collection {
            name,
        });
        match hdk::commit_entry(&maybe_added) {
            Ok(address) => json!({"address": address}).into(),
            Err(hdk_err) => hdk_err.into()
        }
}

