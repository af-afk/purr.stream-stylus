
# Purr Stream Stylus Edition

## User story

```mermaid
flowchart TD
    User -->|Make donation| Contract[Donation contract]
    Contract -->|Sends the leaderboard| User
    Cron[Cron worker] -->|Resets the counter for the leaderboard| Contract
    Operator -->|Receives the donations| Contract
```
