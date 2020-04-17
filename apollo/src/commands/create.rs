use crate::commands::{Command, CreateGraph};
use crate::commands::utils;
use crate::commands::utils::get_auth;
use crate::graphql;

impl Command for CreateGraph {
    fn run(&self) {
        let auth_token = match get_auth() {
            Err(e) => {
                println!("Error authenticating: {}", e);
                return;
            }
            Ok(token) => token,
        };
        let gql_client = graphql::client::ApolloCloudClient::new(
            String::from("https://engine-staging-graphql.apollographql.com/api/graphql"),
            auth_token,
        );

        let accounts = match gql_client.get_org_memberships() {
            Ok(a) => a,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };
        let accounts_pretty = format!("[ {} ]", accounts.clone().into_iter().collect::<Vec<String>>().join(", "));
        let account = if accounts.is_empty() {
            println!("You are not a member of any organization");
            return;
        } else if accounts.len() == 1 {
            String::from(accounts.iter().next().unwrap())
        } else {
            let mut prompt_string = format!("Please choose an organization to own the graph from the following list\n{}", accounts_pretty);
            loop {
                let chosen_account = utils::get_user_input(&prompt_string).unwrap();
                if accounts.contains(&chosen_account) {
                    break chosen_account;
                } else {
                    prompt_string = String::from("Invalid choice; please try again:");
                }

            }
        };
        let graph_id = utils::get_user_input("Choose a name for your graph (cannot be changed)").unwrap();
        println!("You have chosen {}. Excellent selection.", graph_id);
    }
}
