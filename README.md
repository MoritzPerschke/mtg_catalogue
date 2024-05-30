# Magic the gathering collection API
This is meant to be a simple wrapper around a Postgres14 database that provides an API endpoint to add a card to the database.
At some point in time there should exist a scanning app that finds out a unique identifier for a card to populate the database quicker.
Finally, a robot should automatically move cards one after the other in front of a camera to completely automate digitizing a collection.

## Used technology and setup
- PostgreSQL version 14 (upgrade this at some point)
- diesel ORM

## Documenting the process
### Setting up the database and ORM (16.05.2024)
I described what I wanted to do to chatgpt and went from there.
Since chatgpt is stupid, most things so far were not 100% accurate, but in fixing them I think I learned more than stupidly copy-pasting a blogpost.
The used database is PostgreSQL version 14, since that is what i had installed.
The schema is created using diesel's `table!` macro.
I won't include every field that scryfall returns in the database, but most relevant fields are implemented. Not all, things like color identity still require some thought on how to implement.
After creating the schema, it was migrated to the database using `diesel migration generate --diff-schema <schema name>` and `diesel migration run`.
I think that's all for today

### Fixing the ChatGPT errors (30.5.2024)
I recently whipped up the saga orchestrator for a uni project in rust using acitx-web, so after getting a bit more familiar with rust I looked at this again.
There wasn't really anything new I learned that helped me get it to a running state, but being more comfortable with the language meant debugging was way easier.
But now the whole thing is building at least, I didn't test it since I didn't want to install postgres on my laptop.

## Notes for further development
### Support different users
The maximum imaginable number of users of this project will be my playgroup.
Therefore implementing full username-password-email-... authentification will probably be overkill.
Instead take a book out of Mullvad's book (which I probably still need to pay for this month lol), and identify users only by their username.
It would be possible to have new users pick a name from a list of magic characters and run with that.

### Creating a frontend
This is intended to be an API-first project, but not everyone wants to mess around with curl, so a frontend will probably be needed at some point.
I don't like frontend development
