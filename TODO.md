# TODOs

## Architecture

### UI

- [ ] Better reactive design
    - [x] Select should preserve scroll
- [ ] Query and view instead of total sync
    - [ ] Filter media in selector by query
    
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

- [ ] Dry demo 
- [ ] User docs
- [ ] GitHub pages automation

## Under Consideration

- [x] better data model and sync procedure (CRDT?)
    - [x] database in browser storage
    - [x] only pull out-of-date items from server
    - [ ] ~~treat browser storage as write-back cache~~
- [ ] ~~Consider HTTP/2 for event streams~~
