# TODOs

## Small Bugs

- [ ] Overflow on shortnames in selector

## Architecture

### UI

- [ ] Better reactive design
    - [x] Select should preserve scroll
    - [ ] Diagram reactive elements
- [ ] Query and view instead of total sync
    - [ ] Filter media in selector by query
- [ ] Hover selector for metadata
- [ ] TODO list dashboard

### API

- [ ] Persist storage on server
    - [x] storage abstraction
    - [x] in-mem kv storage
    - [ ] on-disk kv storage
    - [ ] sqlite storage
    - [ ] garbage collection instead of deletion
- [ ] Improve metadata automation
    - [ ] `ffprobe` to mine metadata
    - [ ] Use collections for nested directories

## Maintenance

- [ ] Split resumable uploads into a separate crate
- [ ] figure out `trunk` is storing things and cache them in builds
- [ ] clean up old code
    - [ ] move code sections into named functions

## Greenfield

- [ ] Job manager
    - [ ] Display `ffmpeg` progress
    - [ ] Resumable conversions
    - [ ] Confirmation for anything destructive
- [ ] Upload manager
    - [ ] Resumable
- [ ] Use object storage
- [ ] Alternative protocols (ws,quic)

## PR

- [x] Dry demo 
- [ ] User docs
- [x] GitHub pages automation

## Under Consideration

- [x] better data model and sync procedure (CRDT?)
    - [x] database in browser storage
    - [x] only pull out-of-date items from server
    - [ ] ~~treat browser storage as write-back cache~~
- [ ] ~~Consider HTTP/2 for event streams~~
