# TOOLS.md - Local Notes

Skills define _how_ tools work. This file is for _your_ specifics — the stuff that's unique to your setup.

## What Goes Here

Things like:

- Camera names and locations
- SSH hosts and aliases
- Preferred voices for TTS
- Speaker/room names
- Device nicknames
- Anything environment-specific

## Examples

```markdown
### Cameras

- living-room → Main area, 180° wide angle
- front-door → Entrance, motion-triggered

### SSH

- home-server → 192.168.1.100, user: admin

### TTS

- Preferred voice: "Nova" (warm, slightly British)
- Default speaker: Kitchen HomePod
```

## Why Separate?

Skills are shared. Your setup is yours. Keeping them apart means you can update skills without losing your notes, and share skills without leaking your infrastructure.

---

Add whatever helps you do your job. This is your cheat sheet.

## Related

- [Agent workspace](/concepts/agent-workspace)

### VPS

- zorbs.io → 203.161.41.93:22 → root (key auth via ~/.ssh/id_ed25519)
- SSH alias: `ssh zorbs` 
- Hostname: server1.zorbs.io
- OS: Ubuntu 24.04, 5.8GB RAM, 118GB disk
- Docker: zorbs (Axum+Postgres+nginx), ports 80/443

### Local zorbs dev server

- Port: 8080
- DB: Postgres via Docker (port 5432)
- Source: /home/zeta/.openclaw/workspace/zorbs
- Start: /tmp/start_zorbs.sh
- Log: /tmp/zorbs.log
