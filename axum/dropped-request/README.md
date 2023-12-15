## About
This example shows how, when deployed on Shuttle, APIs that contain spaces fail.

To see this, send a GET request the following endpoints and you will get the state result:
 - GET @ `api/v1/hello` -> "Give you your name, please."
 - GET @ `api/v1/hello/world` -> "Hello, world!"
 - GET @ `api/v1/hello/the world` -> Network, no response
 - GET @ `api/v1/hello/the%20world` -> Network error, no reponse
 - GET @ `some/random/path` -> "Unknown path... try again"
