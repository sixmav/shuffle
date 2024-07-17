# Shuffle

<p align="center">
<img src="https://i.ibb.co.com/1RL40CB/Shuffle-MVP-Requirement.png" alt="name" />
<p/>


<p align="center">
<a href="https://github.com/atapas/model-repo/blob/master/LICENSE" target="blank">
<img src="https://img.shields.io/github/license/atapas/model-repo?style=flat-square" alt="tryshape licence" />
<a href="https://github.com/atapas/model-repo/issues" target="blank">
<img src="https://img.shields.io/github/issues/atapas/model-repo?style=flat-square" alt="tryshape issues"/>
</a>
<a href="https://github.com/atapas/model-repo/pulls" target="blank">
<img src="https://img.shields.io/github/issues-pr/atapas/model-repo?style=flat-square" alt="tryshape pull-requests"/>
</a>

<p align="center">
    <a href="https://github.com/TryShape/tryshape/issues/new/choose">Report Bug</a>
    ·
    <a href="https://github.com/TryShape/tryshape/issues/new/choose">Request Feature</a>
</p>

# 👋 Introducing `Shuffle`

The objective of the `Shuffle` MVP is to validate the market need for a nightlife event discovery and booking app in Indonesia by providing essential features that allow users to find and RSVP for events, and venue owners to manage their events. The primary goal is to attract both users and venue owners, proving the concept's viability and generating initial traction.


 

# 🚀 Design

Here is the [🎨 Figma](https://www.figma.com/design/SJ1JLMf0q0tnFHKbyw9kbw/Shuffle?node-id=1%3A801&t=BuDnEakUxVAcD5O5-1) design of the app. We hope you enjoy it.



# 🔥 Features


### 🏗️ Customer Features


| **Sub Features**  | **Description**                                                                                                                                                      |
|------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| **Event Discovery**    | Users can browse events and venues. Filters by location, date, music genre, and vibes. Search functionality for specific events or venues.                            |
| **Event Details**      | Access event detail pages with information including date, time, venue, description, and music genre. View photos and videos from the venue or previous events.       |
| **RSVP Process**       | RSVP button on event detail pages. Redirect to WhatsApp chat with the venue admin for reservation. Post-RSVP prompt asking if the reservation was made. Option to set reminders for the event upon Google account login. |
| **User Account**       | Google account login for setting reminders and saving preferences. Basic user profile with name, email, saved events, and watchlist.                                  |
| **Feedback Mechanism** | Post-event feedback form for users to rate and review their experience.                                                                                              |

### 🏗️ Venue Owner Features

| **Sub Features**                        | **Description**                                                                                           |
|-------------------------------------------------|-----------------------------------------------------------------------------------------------------------|
| **Venue Registration**                          | Simple registration process with basic profile management. Basic venue profile with contact information, location, and description. |
| **Event Management**                            | Interface to add, edit, and manage events. View RSVP reports and statistics.                               |
| **Exposure and Promotion (Nice To Have)**       | Events listed on the platform for users to discover. Potential for featured listings (future enhancement). |



# 🧑‍🔧 Technical Requirement

### Platform
- Web-based application with a responsive design optimized for mobile users.
- **Front-end:** Developed using React.js.
- **Back-end:** Developed using Rust with Actix.
### Database
PostgreSQL, ensuring ACID compliance and advanced querying capabilities.

### Integrations
- WhatsApp API for direct user communication.
- Google OAuth for authentication.
- Google Calendar API for event reminders.
- OpenAI for GPT Model implementation.



# 🛡️ License

This project is licensed under the MIT License - see the [`LICENSE`](LICENSE) file for details.


If you find something is missing, `Shuffle` is listening. Please create a feature request [from here](https://github.com/atapas/model-repo/issues/new/choose).

# 🏃‍♀️ Deploy

====== COMMING SOON ======


# 🤝 Contributing to `shuffle`

> Please read [`CONTRIBUTING`](CONTRIBUTING.md) for details on our [`CODE OF CONDUCT`](CODE_OF_CONDUCT.md), and the process for submitting pull requests to us.


