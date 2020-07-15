# Server side API
- Experimented with a barebones HTTP webserver using Rust's standard library as much as possible
- Only used third-party libraries for JSON conversion and SQLite drivers
- Unfortunately thread/pool handling and HTTP protocol handling took a while to implement, leaving little time for the frontend

# Database
- Used SQLite as it would be very lightweight and easy to setup on another machine
- It generates a file for its database store that can be easily inspected in the same folder

# Web app
- Created with Create React App and Typescript template
- Used MaterialUI for bootstrapping common components as I have worked with it recently
- Used React Router for handling multiple pages in a SPA
- Used fetch API and react hooks for lifecycling handling in stateless functional components

# Pages implemented
### Admin view
Add/remove/~~update~~/view employees
Add/~~update~~/view performance reviews
~~Assign employees to participate in another employee's performance review~~
### ~~Employee view~~