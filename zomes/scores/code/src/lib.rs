#![feature(proc_macro_hygiene)]

extern crate hdk;
use hdk::prelude::*;
use hdk::AGENT_ADDRESS;
use hdk_proc_macros::zome;
// use holochain_anchors;
// #[macro_use]
// extern crate hdk;
// extern crate serde;
// use hdk::prelude::*;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
// #[macro_use]
// use hdk_proc_macros::zome;
// extern crate holochain_json_derive;

// use hdk::holochain_core_types::{dna::entry_types::Sharing, entry::Entry};
// use hdk::{entry_definition::ValidatingEntryType, error::ZomeApiResult};

// use hdk::holochain_json_api::{error::JsonError, json::JsonString};

// see https://developer.holochain.org/api/0.0.42-alpha5/hdk/ for info on using the hdk library

// This is a sample zome that defines an entry type "MyEntry" that can be committed to the
// agent's chain via the exposed function create_my_entry

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Profile {
    name: String,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Score {
    content: String,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct AmpedScore {
    content: String,
    author_address: String,
    author_username: String,
}

fn profile_definition() -> ValidatingEntryType {
    entry!(
        name: "profile",
        description: "player profile",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Profile>| {
            Ok(())
        },
        links: [
            from!(
                "%agent_id",
                link_type: "agent->profile",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            )
        ]
    )
}
fn score_definition() -> ValidatingEntryType {
    entry!(
        name: "score",
        description: "Score for a game",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: | _validation_data: hdk::EntryValidationData<Score>| {
            Ok(())
        }
    )
}
impl Profile {
    fn entry(self) -> Entry {
        Entry::App("profile".into(), self.into())
    }
}
#[zome]
mod scores {
    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn profile_entry_def() -> ValidatingEntryType {
        profile_definition()
    }
    #[entry_def]
    fn score_entry_def() -> ValidatingEntryType {
        score_definition()
    }
    /*
    #[zome_fn("hc_public")]
    fn get_user_scores(addr: Address) -> ZomeApiResult<Vec<Score>> {
        // get scores linked from a generic user

    }
    #[zome_fn("hc_public")]
    fn get_my_scores() -> ZomeApiResult<Vec<Score>> {
        // get scores linked from the user
    }
    #[zome_fn("hc_public")]
    fn get_all_scores() -> ZomeApiResult<Vec<Score>> {
        // get scores linked from the anchor
    }
    #[zome_fn("hc_public")]
    fn get_score_details(addr: Address) -> ZomeApiResult<AmpedScore> {
        // get an amped score given a regular score
    }
    #[zome_fn("hc_public")]
    fn publish_score(points: i32, msg: String) -> ZomeApiResult<bool> {
        // upload a score, link from the anchor and link from the user
    }
    */
    #[zome_fn("hc_public")]
    fn profile(name: String) -> ZomeApiResult<Address> {
        // create my profile
        let profile = Profile { name };
        let entry = profile.entry();
        let address = hdk::commit_entry(&entry)?;
        hdk::link_entries(&AGENT_ADDRESS, &address, "agent->profile", "")?;
        Ok(address)
    }
    #[zome_fn("hc_public")]
    fn get_my_profile() -> ZomeApiResult<Profile> {
        //fetch profile linked from the agent address
        let mut res = hdk::utils::get_links_and_load_type(
            &AGENT_ADDRESS,
            LinkMatch::Exactly("agent->profile"),
            LinkMatch::Any,
        )?;

        match res.pop() {
            Some(profile) => Ok(profile),
            None => Err(ZomeApiError::Internal("No profile registered".to_string())),
        }
    }
}
