# Design - SCGuild

## 4 Services
* API - Stores all ship, trade, etc data about SC
    * Consider game-specific profile info for SC like user name
* Discord Bot - Stores discord unique ID and API oauth2 token
* Web app - Stores API oauth2 token
* User service - Stores all user profile data, oauth2 token revocation info, federated login tokens, etc

## Bot login possibilities

### Bot Forward to User Service (Preferred)

1. `/login`
2. Bot displays a link to an endpoint on the bot itself
3. Bot endpoint immediately forwards to User Service oauth2 process for login with discord (skipping the 'login with' screen)
4. User logs in via discord oauth2 and User Service forwards to callback endpoint on bot
5. Bot stores the API token and discord username
6. Bot displays completed page and sends/alters message in discord to indicate login complete

### Oauth2 Device Code

1. `/login`
2. Bot displays a url and a device code (e.g. 'X7KJ4')
3. User clicks link and is asked to login to User Service
4. User logs into user service and is forwarded to a callback on the bot
5. User enters device code
6. Bot stores token and discord userid, reports complete

## Order of Development

1. User Service
    * Social logins
    * FIDO2
    * 2FA
    * User profile
2. API
    * Start with ship/fleet database
    * Trade tracker
    * Consider game-specific username/profile info
3. Web App
    * Front end for API
4. Discord Bot
    * Bot end for API

## Further Thoughts and Questions

* All parts of the app can exist in the same binary to begin with as a modularized monolith using separate routes
* Bot will need either gateway or webhooks, need to decide which is better
* Is any message passing necessary? Not yet, at least.
* How to handle situations where the endpoints need to display user profile info? Separate query to user service or do I keep profile info inside the API? Could do game-specific profile separately.