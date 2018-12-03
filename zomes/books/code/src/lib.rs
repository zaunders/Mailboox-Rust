
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


use hdk::{
    AGENT_ADDRESS,
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
    book_owner: Address
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
struct Collection {
    name: String,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson)]
struct User {
    name: String,
    street: String,
    zip: String,
    city: String,
    country: String,
}

define_zome! {

    entries: [
        entry!(
			name: "anchor",
	        description: "",
	        sharing: Sharing::Public,
	        native_type: String,
	        validation_package: || {
	            hdk::ValidationPackageDefinition::Entry
	        },
	        validation: |name: String, _ctx: hdk::ValidationData| {
	        	Ok(())
	        },
            links: [
                to!(
	        		"book",
	        		tag: "in shelf",
	                validation_package: || {hdk::ValidationPackageDefinition::Entry },
                    validation: |_base: Address, _target: Address, _ctx: hdk::ValidationData| {
	                    Ok(())
	                }
	        	)
            ]
		),
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
                    "collection",
                    tag: "in collection",
                    validation_package: || hdk::ValidationPackageDefinition::Entry,
                    validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
                        Ok(())
                    }
                ),
                to! (
                    "user",
                    tag: "book requested by",
                    validation_package: || hdk::ValidationPackageDefinition::Entry,
                    validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
                        Ok(())
                    }
                )
            ]

        ),
        entry!(
            name: "collection",
            description: "a collection of books",
            sharing: Sharing::Public,
            native_type: Collection,
            validation_package: || hdk::ValidationPackageDefinition::Entry,
            validation: |collection: Collection, _ctx: hdk::ValidationData| {
                Ok(())
            },
            links: [
                to! (
                    "book",
                    tag: "has book",
                    validation_package: || hdk::ValidationPackageDefinition::Entry,
                    validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
                        Ok(())
                    }
                )
            ]

        ),
        entry!(
            name: "user",
            description: "an app user",
            sharing: Sharing::Public,
            native_type: User,
            validation_package: || hdk::ValidationPackageDefinition::Entry,
            validation: |collection: Collection, _ctx: hdk::ValidationData| {
                Ok(())
            },
            links: [
              to! (
                    //not implemented yet, users should be able to favorite books for others to see
                    "favoriteBook",
                    tag: "favorite",
                    validation_package: || hdk::ValidationPackageDefinition::Entry,
                    validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
                        Ok(())
                    }
                ),
                to! (
                    "book",
                    tag: "requests book",
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
            init: {
				inputs: | |,
				outputs: |result: JsonString|,
				handler: handle_init
			}
            create_book: {
                inputs: |name: String, author: String, genre: String, blurb: String, book_owner: Address, shelf_address: Address|,
                outputs: |result: JsonString|,
                handler: handle_create_book
            }
            create_collection: {
                inputs: |name: String|,
                outputs: |result: JsonString|,
                handler: handle_create_collection
            }
            create_user: {
                inputs: |name: String, street: String, zip: String, city: String, country: String|,
                outputs: |result: JsonString|,
                handler: handle_create_user
            }
            /* depreciated, owner now in book
            link_book_to_owner: {
                inputs: |book_address: Address, owner_address: Address|,
                outputs: |result: JsonString|,
                handler: handle_link_book_to_owner
            }
            */
            get_book: {
                inputs: |address: Address|,
                outputs: |result: JsonString|,
                handler: handle_get_book
            }
            get_user_data: {
                inputs: |address: Address|,
                outputs: |result: JsonString|,
                handler: handle_get_book
            }
            add_book_to_collection: {
                inputs: |book_address: Address, collection_address: Address|,
                outputs: |result: JsonString|,
                handler: handle_add_book_to_collection
            }
            get_books_in_collection: {
                inputs: |collection_address: Address, tag: String|,
                outputs: |result: JsonString|,
                handler: handle_get_books_in_collection
            }
            get_collections_book_is_in: {
                inputs: |book_address: Address, tag: String|,
                outputs: |result: JsonString|,
                handler: handle_get_collections_book_is_in
            }
            //retrieve all books linked to the anchor shelf
            get_books: {
                inputs: |shelf_address: Address, tag: String|,
                outputs: |result: JsonString|,
                handler: handle_get_books
            }
            get_owners: {
                inputs: |book_address: Address, tag: String|,
                outputs: |result: JsonString|,
                handler: handle_get_owners
            }
             request_to_borrow: {
                inputs: |borrower_address: Address, book_address: Address|,
                outputs: |result: JsonString|,
                handler: handle_request_to_borrow
            }
            /*
            get_current_user_address: {
				inputs: | |,
				outputs: |result: JsonString|,
				handler: handle_get_current_user_address
            }*/
            /*
           
            accept_request_to_borrow: {
                inputs: |**: **|,
                outputs: |result: JsonString|,
                handler: handle_accept_request_to_borrow
            }
            mark_book_returned: {
                inputs: |**: **|,
                outputs: |result: JsonString|,
                handler: handle_mark_book_returned
            }*/

        }
    }
}
fn handle_create_book(name: String, author: String, genre: String, blurb: String, book_owner: Address, shelf_address: Address) -> JsonString {
    let maybe_added = Entry::new(EntryType::App("book".into()), Book {
        name, author, genre, blurb, book_owner
    });
    match hdk::commit_entry(&maybe_added) {
        Ok(book_address) => {
            match hdk::link_entries(&shelf_address, &book_address, "in shelf") {
                Ok(_result) => json!({"address": &book_address}).into(),
                Err(hdk_error) => json!({"error" : hdk_error}).into()
            }
        },
        Err(hdk_err) => hdk_err.into()
    }
}


fn handle_create_collection(name: String) -> JsonString {
        let maybe_added = Entry::new(EntryType::App("collection".into()), Collection {
            name,
        });
        match hdk::commit_entry(&maybe_added) {
            Ok(address) => json!({"address": address}).into(),
            Err(hdk_err) => hdk_err.into()
        }
}
fn handle_create_user(name: String, street: String, zip: String, city: String, country: String,) -> JsonString {
    let maybe_added = Entry::new(EntryType::App("user".into()), User {name, street, zip, city, country});
    match hdk::commit_entry(&maybe_added) {
        Ok(address) => json!({"address": address}).into(),
        Err(hdk_err) => hdk_err.into()
    }
}
fn handle_get_book(address: Address) -> JsonString {
     match hdk::get_entry(address) {
        Ok(maybe_book) => maybe_book.and_then(|entry| Some(entry.serialize())).into(),
        Err(e) => e.into(),
    }
}

fn handle_get_user_data(address: Address) -> JsonString {
     match hdk::get_entry(address) {
        Ok(maybe_user) => maybe_user.and_then(|entry| Some(entry.serialize())).into(),
        Err(e) => e.into(),
    }
}

 fn handle_init() -> JsonString {
	let anchor_entry = Entry::new(EntryType::App("anchor".into()), json!("bookshelf"));
	match hdk::commit_entry(&anchor_entry) {
        Ok(address) => json!({"address": address}).into(),
        Err(hdk_err) => hdk_err.into()
    }
 }
/* how do I get current user address?
fn handle_get_current_user_address() -> JsonString {
    json!({address: &AGENT_ADDRESS}).into()
}*/

fn handle_add_book_to_collection(book_address: Address, collection_address: Address) -> JsonString {
    match (
        hdk::link_entries(&book_address, &collection_address, "in collection"),
	    hdk::link_entries(&collection_address, &book_address, "has book")
    ) {
        (Ok(_result),Ok(_result2)) => json!({"success": true}).into(),
        (Err(err1), Err(_)) => err1.into(),
		(Ok(_), Err(err2)) => err2.into(),
        (Err(err1), Ok(_)) => err1.into()
    }
}
/*depreciated, owner now in book
fn handle_link_book_to_owner(book_address: Address, owner_address: Address) -> JsonString {
    match (
        hdk::link_entries(&book_address, &owner_address, "owned by"),
	    hdk::link_entries(&owner_address, &book_address, "owns")
    ) {
        (Ok(_result),Ok(_result2)) => json!({"success": true}).into(),
        (Err(err1), Err(_)) => err1.into(),
		(Ok(_), Err(err2)) => err2.into(),
        (Err(err1), Ok(_)) => err1.into()
    }
}
*/
fn handle_get_books_in_collection(collection_address: Address, tag: String) -> JsonString {
    match hdk::get_links(&collection_address, tag)
    {
        Ok(result) => result.into(),
        Err(hdk_error) => hdk_error.into(),
    }
}

fn handle_get_collections_book_is_in (book_address: Address, tag: String) -> JsonString {
    match hdk::get_links(&book_address, tag)
    {
        Ok(result) => result.into(),
        Err(hdk_error) => hdk_error.into()
    }
}

fn handle_get_books(shelf_address: Address, tag: String) -> JsonString {
        match hdk::get_links(&shelf_address, tag)
    {
        Ok(result) => result.into(),
        Err(hdk_error) => hdk_error.into()
    }
}

fn handle_get_owners(book_address: Address, tag: String) -> JsonString {
        match hdk::get_links(&book_address, tag)
    {
        Ok(result) => result.into(),
        Err(hdk_error) => hdk_error.into()
    }
}

fn handle_request_to_borrow(book_address: Address, borrower_address: Address) -> JsonString {
        match (
        hdk::link_entries(&book_address, &borrower_address, "book requested by"),
	    hdk::link_entries(&borrower_address, &book_address, "requests book")
    ) {
        (Ok(_result),Ok(_result2)) => json!({"success": true}).into(),
        (Err(err1), Err(_)) => err1.into(),
		(Ok(_), Err(err2)) => err2.into(),
        (Err(err1), Ok(_)) => err1.into()
    }
}