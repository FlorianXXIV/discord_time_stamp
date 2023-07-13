# DiscordTimeStamp
This Discord bot prints out timestamps to make the timestamp feature of Discord more accesseable.
## Usage-Bot
Commands are prefixed with §
- time:
  - date as [year]-[month]-[day] (example 2023-07-05)
  - time as [hour]:[minute] (example 09:30)
  - offset as +/-[hour]:[minute] (example 02:00 or -02:00)
    - this is a UTC offset by default
    - Supports some Named timezones by their shorthands
      - (i.e. CEST GMT EET)
      - Since Timezone names overlap I recommend using UTC offset.
- time_rel:
  - date as [year]-[month]-[day] (example 2023-07-05)
  - time as [hour]:[minute] (example 09:30)
  - offset as +/-[hour]:[minute] (example 02:00 or -02:00)
    - this is a UTC offset by default
    - Supports some Named timezones by their shorthands
      - (i.e. CEST GMT EET)
      - Since Timezone names overlap I recommend using UTC offset.
  - gives you the relative time (i.e. in 42 years)
- time_f:
  - date as [year]-[month]-[day] (example 2023-07-05)
  - time as [hour]:[minute] (example 09:30)
  - offset as +/-[hour]:[minute] (example 02:00 or -02:00)
    - this is a UTC offset by default
    - Supports some Named timezones by their shorthands
      - (i.e. CEST GMT EET)
      - Since Timezone names overlap I recommend using UTC offset.
  - format as a single charakter
- stamp
  - gives you the timestamp as t:<unix_time_stamp> so you can copy it.
  - date as [year]-[month]-[day] (example 2023-07-05)
  - time as [hour]:[minute] (example 09:30)
  - offset as +/-[hour]:[minute] (example 02:00 or -02:00)
    - this is a UTC offset by default
    - Supports some Named timezones by their shorthands
      - (i.e. CEST GMT EET)
      - Since Timezone names overlap I recommend using UTC offset.
- stamp_rel
  - same as above but with relative time
  - date as [year]-[month]-[day] (example 2023-07-05)
  - time as [hour]:[minute] (example 09:30)
  - offset as +/-[hour]:[minute] (example 02:00 or -02:00)
    - this is a UTC offset by default
    - Supports some Named timezones by their shorthands
      - (i.e. CEST GMT EET)
      - Since Timezone names overlap I recommend using UTC offset.
- stamp_f
  - same as stamp but with a custom formatter at the end.
  - date as [year]-[month]-[day] (example 2023-07-05)
  - time as [hour]:[minute] (example 09:30)
  - offset as +/-[hour]:[minute] (example 02:00 or -02:00)
    - this is a UTC offset by default
    - Supports some Named timezones by their shorthands
      - (i.e. CEST GMT EET)
      - Since Timezone names overlap I recommend using UTC offset.
  - format as a single charakter