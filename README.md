# Via Discendi

Website for creating and following learning roadmaphs.

## MVP

### Use cases

#### auth
- [ ] user should be able to log in
- [ ] user should be able to sign up
---
#### roadmaps
- [ ] create roadmap
- [ ] add title
- [ ] set user as roadmap author 
- [ ] allow user to bookmark roadmap (replacement for likes)
- [ ] show roadmap bookmarks count
##### versions
- [ ] set default version to v0.0.0
- [ ] each time a roadmap is edited, it is published under a new version
- [ ] user can navigate between roadmap versions
- [ ] roadmap author can delete roadmap versions
- [ ] roadmap author can delete all roadmap versions
##### create areas
- [ ] create areas
- [ ] edit area title
- [ ] edit area description
- [ ] add area resources (books)
- [ ] link areas (one area to another)
- [ ] move area position
##### area status
- [ ] add status to area
- [ ] update status to "done" or "on course"

#### search roadmaps
- [ ] filter by title
- [ ] filter by author

### Data Modeling

#### user
- id
- email
- password (hashed)
- username
- name
    - first-name
    - last-name
- updated_at
- created_at

#### roadmap
- id
- version
- author
- author: fk(user.id)
- bookmarkcount
- title
- description
- created_at
- updated_at

#### area
- id
- roadmap_id: fk(roadmap.id)
- title
- description
- books: all books that reference this area
- position_x
- position_y

#### book
- id
- title
- link
- area: fk(area.id)

#### area edge
- from_area_id
- to_area_id

#### user progress
still need to work on planning this
