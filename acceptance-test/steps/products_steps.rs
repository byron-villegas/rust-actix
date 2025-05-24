use cucumber::{given, then, when};
use crate::World;

#[given(regex = r"An endpoint (.+)")]
async fn an_endpoint(world: &mut World, endpoint: String) {
    world.endpoint = endpoint.to_string();
}

#[when("I send a GET request to the endpoint")]
async fn i_send_a_get_request_to_the_endpoint(world: &mut World) {
    let url = format!("{}{}", world.host, world.endpoint);

    let response = reqwest::get(url).await.unwrap();
    world.response = Some(response);
}

#[then(regex = r"I should receive a (\d+) status code")]
async fn i_should_receive_a_status_code(world: &mut World, status_code: String) {
    if let Some(response) = &world.response {
        assert_eq!(status_code, response.status().as_str());
    } else {
        panic!("No response received");
    }
}