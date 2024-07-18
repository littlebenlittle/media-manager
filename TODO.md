# TODOs

## Architecture

### UI

- [x] Better reactive design
    - [x] Select should preserve scroll
- [x] Filter media in selector by query
- [x] Media detial on hover
- [ ] Toaster notifications
    - [ ] new media
    
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

- [x] Split resumable uploads into a separate crate
- [ ] API should return schema-appropriate media URLs
    - [ ] Use env var in API to set scheme

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
