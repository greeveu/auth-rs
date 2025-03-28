## Backend

- [x] Recursivly remove roles from users when role is deleted
- [x] Recursivly delete user bound data on user deletion
- [x] Return http status code in base response
- [x] Validate redirect URI (not sure if we already do this) -> Update: We did :3
- [x] Emails are not unique??? -> Fixed
- [x] Require at lease one oauth scope to authorize

## Frontend

- [x] Frontend missing auth flow id
- [x] User roles ~~dropdown~~ -> Selection Popup
- [x] Create / Edit users
- [x] Disable Users
- [ ] Registration
- [ ] ~~New "Your Profile" page style +~~ edit button
- [ ] Tooltips
- [ ] render invalid ouath url errors
- [ ] raw json viewer -> debug mode???

## Both

- [ ] Role descriptions
- [ ] Limit registations (registration codes with limited uses and autoroles?)
- [x] System admin settings (limited registrations, disable oauth application create for default users)
- [ ] Pagination

## ???

- [x] OAuth Connection expiery date is not created or calculated correctly (look further into this)
