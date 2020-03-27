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
pub struct FullProfile {
    name: String,
    address: Address,
}

impl Profile {
    fn entry(self) -> Entry {
        Entry::App("profile".into(), self.into())
    }
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Score {
    score: String,
    message: String,
    author_address: Address,
}

impl Score {
    fn entry(self) -> Entry {
        Entry::App("score".into(), self.into())
    }
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct AmpedScore {
    score: String,
    message: String,
    author_address: Address,
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
        },
        links: [
            from!(
                "%agent_id",
                link_type: "agent->score",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            from!(
                holochain_anchors::ANCHOR_TYPE,
                link_type: "anchor->score",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            )
        ]
    )
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
    #[entry_def]
    fn anchor_def() -> ValidatingEntryType {
        holochain_anchors::anchor_definition()
    }
    #[zome_fn("hc_public")]
    fn get_user_scores(addr: Address) -> ZomeApiResult<Vec<Score>> {
        // get scores linked from a generic user
        let res = hdk::utils::get_links_and_load_type(
            &addr,
            LinkMatch::Exactly("agent->score"),
            LinkMatch::Any,
        )?;
        Ok(res)
    }
    #[zome_fn("hc_public")]
    fn get_my_scores() -> ZomeApiResult<Vec<Score>> {
        // get scores linked from the user
        let res = hdk::utils::get_links_and_load_type(
            &AGENT_ADDRESS,
            LinkMatch::Exactly("agent->score"),
            LinkMatch::Any,
        )?;
        Ok(res)
    }
    #[zome_fn("hc_public")]
    fn get_all_scores() -> ZomeApiResult<Vec<Score>> {
        // get scores linked from the anchor
        let anchor_address = holochain_anchors::anchor("score".into(), "score".into())?;
        hdk::utils::get_links_and_load_type(
            &anchor_address,
            LinkMatch::Exactly("anchor->score"),
            LinkMatch::Any,
        )
    }
    #[zome_fn("hc_public")]
    fn get_username(addr: Address) -> ZomeApiResult<String> {
        let wrapped_profile_array: Vec<Profile> = hdk::utils::get_links_and_load_type(
            &addr,
            LinkMatch::Exactly("agent->profile"),
            LinkMatch::Any,
        )?;
        let wrapped_profile = wrapped_profile_array.last();
        match wrapped_profile {
            Some(profile) => Ok(profile.name.clone()),
            None => Err(ZomeApiError::Internal("profile not found".to_string())),
        }
    }
    #[zome_fn("hc_public")]
    fn get_score_details(addr: Address) -> ZomeApiResult<AmpedScore> {
        // get an amped score given a regular score
        let score: Score = hdk::utils::get_as_type(addr)?;
        let wrapped_profile_array: Vec<Profile> = hdk::utils::get_links_and_load_type(
            &score.author_address,
            LinkMatch::Exactly("agent->profile"),
            LinkMatch::Any,
        )?;
        let wrapped_profile = wrapped_profile_array.last();
        let author_profile: &Profile = match wrapped_profile {
            Some(profile) => profile,
            None => {
                return Err(ZomeApiError::Internal("profile not found".to_string()));
            }
        };
        let result = AmpedScore {
            score: score.score.clone(),
            message: score.message.clone(),
            author_address: score.author_address.clone(),
            author_username: author_profile.name.clone(),
        };
        Ok(result)
    }
    #[zome_fn("hc_public")]
    fn publish_score(score: String, message: String) -> ZomeApiResult<bool> {
        // upload a score, link from the anchor and link from the user
        let score = Score {
            score,
            message,
            author_address: AGENT_ADDRESS.clone(),
        };
        let entry = score.entry();
        let address = hdk::commit_entry(&entry)?;
        let anchor_address = holochain_anchors::anchor("score".into(), "score".into())?;
        hdk::link_entries(&anchor_address, &address, "anchor->score", "")?;
        hdk::link_entries(&AGENT_ADDRESS, &address, "agent->score", "")?;
        Ok(true)
    }
    #[zome_fn("hc_public")]
    fn profile(name: String) -> ZomeApiResult<Address> {
        // create my profile
        let profile = Profile { name };
        let entry = profile.entry();
        let address = hdk::commit_entry(&entry)?;
        hdk::link_entries(&AGENT_ADDRESS, &address, "agent->profile", "")?;
        Ok(AGENT_ADDRESS.clone())
    }
    #[zome_fn("hc_public")]
    fn get_my_profile() -> ZomeApiResult<FullProfile> {
        //fetch profile linked from the agent address
        let mut res = hdk::utils::get_links_and_load_type::<Profile>(
            &AGENT_ADDRESS,
            LinkMatch::Exactly("agent->profile"),
            LinkMatch::Any,
        )?;

        match res.pop() {
            Some(profile) => Ok(FullProfile {
                name: profile.name,
                address: AGENT_ADDRESS.clone(),
            }),
            None => Err(ZomeApiError::Internal("No profile registered".to_string())),
        }
    }
}
