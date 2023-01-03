import requests

# Autenticación
client_id = "id"
client_secret = "secret"
grant_type = "client_credentials"

# Obtener un token de autenticación
auth_url = "https://accounts.spotify.com/api/token"
auth_response = requests.post(auth_url, {
  "grant_type": grant_type,
  "client_id": client_id,
  "client_secret": client_secret
})
access_token = auth_response.json()["access_token"]

# Definir el género y el nombre de la playlist
genre = "pop punk español"
playlist_name = "Mi playlist autogenerada de pop punk español"

# Crear la playlist
playlist_url = "https://api.spotify.com/v1/users/{user_id}/playlists"
headers = {"Authorization": f"Bearer {access_token}"}
response = requests.post(playlist_url, json={
  "name": playlist_name,
  "public": True
}, headers=headers)
playlist_id = response.json()["id"]

# Buscar canciones por género de forma aleatoria (inicia una "Radio" del género
search_url = "https://api.spotify.com/v1/search"
params = {
  "q": f"genre:{genre}",
  "type": "track",
  "limit": 15 #límite deseado
}
response = requests.get(search_url, params=params, headers=headers)
tracks = response.json()["tracks"]["items"]

# Agregar las canciones a la playlist
add_tracks_url = f"https://api.spotify.com/v1/playlists/{playlist_id}/tracks"
track_uris = [track["uri"] for track in tracks]
requests.post(add_tracks_url, json={"uris": track_uris}, headers=headers)

print(f"Se han agregado {len(tracks)} canciones de x-genero a la playlist {playlist_name}.")
