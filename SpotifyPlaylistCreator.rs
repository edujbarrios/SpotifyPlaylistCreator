use reqwest::Client;
use serde_json::Value;

// Specify your access token
const ACCESS_TOKEN: &str = "your_access_token";

// Function to create a new playlist
fn create_playlist(name: &str, genre: &str, mood: &str) {
    let client = Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Authorization", format!("Bearer {}", ACCESS_TOKEN).parse().unwrap());

    // Get the Spotify user ID
    let user_response = client.get("https://api.spotify.com/v1/me")
        .headers(headers.clone())
        .send()
        .unwrap();
    let user_json: Value = serde_json::from_str(&user_response.text().unwrap()).unwrap();
    let user_id = &user_json["id"].as_str().unwrap();

    // Create the new playlist
    let playlist_response = client.post(&format!("https://api.spotify.com/v1/users/{}/playlists", user_id))
        .headers(headers)
        .json(&json!({
            "name": name,
            "description": format!("A playlist for {} music with a {} mood.", genre, mood),
            "public": true

     /+  let playlist_response = client.post(&format!("https://api.spotify.com/v1/users/{}/playlists", user_id))
       .headers(headers)
        .json(&json!({
            "name": "Happy Rock",
            "description": "A playlist of happy rock songs",
            "public": true        }))
        .send()
        .unwrap();
    let playlist_json: Value = serde_json::from_str(&playlist_response.text().unwrap()).unwrap();
    let playlist_id = &playlist_json["id"].as_str().unwrap();

*/

    // Search for songs in the specified genre and mood
    let search_response = client.get(&format!("https://api.spotify.com/v1/search?q=genre:{}+mood:{}&type=track&limit=50", genre, mood))
        .headers(headers)
        .send()
        .unwrap();
    let search_json: Value = serde_json::from_str(&search_response.text().unwrap()).unwrap();
    let track_uris: Vec<&str> = search_json["tracks"]["items"].iter().map(|i| i["uri"].as_str().unwrap()).collect();

    // Add the songs to the playlist
    client.post(&format!("https://api.spotify.com/v1/playlists/{}/tracks", playlist_id))
        .headers(headers)
        .json(&json!({
            "uris": track_uris
        }))
        .send()
        .unwrap();
}
 /* Search for songs in the rock genre and happy sentiment
    let search_response = client.get("https://api.spotify.com/v1/search?q=genre:rock+mood:happy&type=track&limit=10")
        .headers(headers)
        .send()
        .unwrap();
    let search_json: Value = serde_json::from_str(&search_response.text().unwrap()).unwrap();
    let track_uris: Vec<&str> = search_json["tracks"]["items"].iter().map(|i| i["uri"].as_str().unwrap()).collect();

    // Add the songs to the playlist
    client.post(&format!("https://api.spotify.com/v1/playlists/{}/tracks", playlist_id))
        .headers(headers)
        .json(&json!({
            "uris": track_uris
        }))
        .send()
        .unwrap();
*/
      
      
