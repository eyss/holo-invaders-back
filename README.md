# Holo Invaders
### (backend) 

Holochain backend for a space invaders game. The work of the backend is (as of 03/18/2020) to provide a mechanism to store and share game scores. The game mechanics will be stored in the frontend repo.

## Getting Started

To compile and run the backend make sure that your holochain version matches the one of the code by doing:

```
[nix-shell:~/holo-invaders]$ hc --version
hc 0.0.42-alpha5
```

## API

The data structures the system works with are:

```rust
    pub struct Profile {
        name: String,
    }
    
    pub struct Score {
        content: String,
    }

    pub struct AmpedScore {
        content: String,
        author_address: String,
        author_username: String,
    }
```

The functions to use from the frontend are:

```rust
    fn get_user_scores(addr: Address) -> ZomeApiResult<Vec<Score>> {
        // get scores linked from a generic user. The user address must be provided
    }
    fn get_my_scores() -> ZomeApiResult<Vec<Score>> {
        // get scores linked from the user
    }
    fn get_all_scores() -> ZomeApiResult<Vec<Score>> {
        // get all the scores in the system
    }
    fn get_score_details(addr: Address) -> ZomeApiResult<AmpedScore> {
        // get an amped score given a regular score
        // is the same score but with more details
    }
    fn publish_score(points: i32, msg: String) -> ZomeApiResult<bool> {
        // upload a score, link from the anchor and link from the user.
        // returns true when succeeding
    }
    fn profile(name: String) -> ZomeApiResult<Address> {
        // create my profile with my username
    }
    fn get_my_profile() -> ZomeApiResult<Profile> {
        // get my profile
    }
```


## Authors

* **Jhonatan Hernández** - *Initial work* - [github profile](https://github.com/JhonatanHern)
