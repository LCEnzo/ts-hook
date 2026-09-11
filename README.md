# Timestamp Hook

Agents can get confused about when or if the user has been away from the 
session for a time, and generally lack a sense of time. To remedy this, a 
simple hook injects a timestamp after each user message.

As of 12/09/2026, something like it is an experimental feature in Codex 
CLI† and thus one can expect this project to reach end of life as soon as
both Claude Code and Codex CLI implement timestamps natively.

† it's named `current_time_reminder`
